use crate::lexer::tokens::*;
use crate::lexer::{self, *};
use crate::errors;
use colored::Colorize;

pub trait Consumer {
	fn consume(idx: &mut usize, lexer: &mut Lexer)  -> Result<Token, errors::Err>;
}

pub struct Comment{}
impl Consumer for Comment {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		*idx += lexer.text.len(); // as soon as we encounter a comment, skip to the end
		Ok(Token::Empty) // this result is discarded
	}
}

pub struct Directive{}
impl Consumer for Directive {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		*idx += 1; // skip the initial "."

		if let Err(err) = lexer.verify_buffer("directive") {
			return Err(err);
		}

		while *idx < lexer.text.len() {
			let character = lexer.idx(*idx);
			match character {
				'A'..='z' => lexer.buffer.push(character),
				' ' => break,
				_ => {
					let msgs = errors::Msg::Many(vec![
						format!("Illegal symbol \"{}\" while consuming directive.", character.to_string().red()),
						"Directives may only have alphabetic names.".to_string()
					]);
					return lexer.error(*idx, 1, msgs);
				}
			}

			*idx += 1;
		}

		let identifier = String::from(&lexer.buffer);
		let len = identifier.len();
		lexer.buffer.clear(); // remove our work
		Ok(Token::Directive(identifier, CodeSegment {
			line: lexer.line,
			idx: *idx - len - 1,
			len: len + 1
		}))
	}
}

pub struct DefLabel{}
impl Consumer for DefLabel {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		// label definitions are given as "{name}:", so we have to use the buffered value
		let identifier = lexer.buffer.drain(..).collect::<String>(); // we retain the value but clear the buffer
		let len = identifier.len();
		Ok(Token::DefLabel(identifier, CodeSegment {
			line: lexer.line,
			idx: *idx - len,
			len: len + 1
		}))
	}
}

pub struct Identifier{}
impl Consumer for Identifier {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		let identifier = lexer.buffer.drain(..).collect::<String>(); // we retain the value but clear the buffer
		let len = identifier.len();
		Ok(Token::Identifier(identifier, CodeSegment {
			line: lexer.line,
			idx: *idx - len,
			len: len
		}))
	}
}

pub struct Register{}
impl Consumer for Register {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		*idx += 1; // registers are always ${id}, skip the $
		
		if let Err(err) = lexer.verify_buffer("register") {
			return Err(err);
		}

		while *idx < lexer.text.len() {
			let character = lexer.idx(*idx);
			match character {
				'a'..='z' => lexer.buffer.push(character),
				'0'..='9' => lexer.buffer.push(character),
				',' => break,
				_ => {
					let msgs = errors::Msg::Many(vec![
						format!("Illegal symbol \"{}\" while consuming register.", character.to_string().red()),
						"Registers may only have alphanumeric names.".to_string()
					]);
					return lexer.error(*idx, 1, msgs);
				}
			}
			*idx += 1;
		}

		if lexer.len() > 2 {
			let msgs = errors::Msg::Many(vec![
				format!("Illegal register form \"{}\".", lexer.buffer.red()),
				"Registers' names do not exceed two characters.".to_string()
			]);
			return lexer.error(*idx - lexer.len(), lexer.len(), msgs);
		}


		let identifier = String::from(&lexer.buffer);
		let len = identifier.len();
		lexer.buffer.clear(); // remove our work
		Ok(Token::Register(identifier, CodeSegment {
			line: lexer.line,
			idx: *idx - len - 1,
			len: len + 1
		}))
	}
}

pub struct StringLiteral {}
impl Consumer for StringLiteral {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		*idx += 1; // remove initial "

		if let Err(err) = lexer.verify_buffer("string") {
			return Err(err);
		}

		while *idx < lexer.text.len() {
			let character = lexer.idx(*idx);
			match character {
				'"' => break,
				_ => lexer.buffer.push(character)
			}

			*idx += 1;
		}

		let string = String::from(&lexer.buffer);
		let len = string.len();
		lexer.buffer.clear();
		Ok(Token::StringLiteral(string, CodeSegment {
			line: lexer.line,
			idx: *idx - len - 1,
			len: len + 2
		}))
	}
}

pub struct NumberLiteral {}
impl Consumer for NumberLiteral {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		if let Err(err) = lexer.verify_buffer("number") {
			return Err(err);
		}

		while *idx < lexer.text.len() {
			let character = lexer.idx(*idx);
			match character {
				'0'..='9' => lexer.buffer.push(character),
				',' => break,
				'#' => {
					Comment::consume(idx, lexer);
				},
				'-' => lexer.buffer.push(character),
				' ' => match skip_whitespace(idx, lexer) {
					Ok(token) => {},
					Err(err) => return Err(err)
				},
				_ => {
					let msgs = errors::Msg::One(format!("Illegal character \"{}\" while consuming number.", character));
					return lexer.error(*idx, 1, msgs);
				}
			}

			*idx += 1;
		}

		let number = String::from(&lexer.buffer);
		let len = number.len();
		let index = lexer.text.find(&number).unwrap();
		lexer.buffer.clear();
		let number = match number.parse::<i32>() {
			Ok(number) => number,
			Err(err) => {
				let msgs = errors::Msg::One(format!("Could not cast {} to a number: {}", number.red(), err));
				return lexer.error(index, len, msgs);
			}
		};

		Ok(Token::NumberLiteral(number, CodeSegment {
			line: lexer.line,
			idx: index,
			len: len
		}))
	}
}

// skips whitespace until one of a comma, comment, or line end is found
fn skip_whitespace(idx: &mut usize, lexer: &Lexer) -> Result<Token, errors::Err> {
	while *idx < lexer.text.len() {
		let character = lexer.idx(*idx);
		match character {
			' ' => {},
			',' => break,
			'#' => {
				*idx -= 1; // put comment back into view
				break;
			},
			_ => {
				let msgs = errors::Msg::One(format!("Unexpected character {} while reading whitespace.", character));
				return lexer.error(*idx, 1, msgs);
			}
		}

		*idx += 1;
	}

	Ok(Token::Empty)
}