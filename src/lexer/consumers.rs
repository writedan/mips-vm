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
		*idx += lexer.text.len();
		Ok(Token::Empty)
	}
}

pub struct Directive{}
impl Consumer for Directive {
	fn consume(idx: &mut usize, lexer: &mut Lexer) -> Result<Token, errors::Err> {
		*idx += 1; // skip the initial "."

		if lexer.text.len() > 0 {
			let msgs = errors::Msg::Many(vec![
				format!("Trying to consume directive but buffer = {}", lexer.buffer),
				"Buffer should be empty.".to_string()
			]);
			return lexer.error(lexer.text.len(), 0, msgs);
		}

		while *idx < lexer.text.len() {
			let character = lexer.idx(*idx);
			match character {
				'A'..='z' => lexer.buffer.push(character),
				' ' => break,
				_ => {
					let msgs = errors::Msg::Many(vec![
						format!("Illegal symbol \"{}\" while consuming directive.", character.to_string().red()),
						"Directives may only have alphabetic numbers.".to_string()
					]);
					return lexer.error(*idx, 1, msgs);
				}
			}

			*idx += 1;
		}

		let identifier = String::from(&lexer.buffer);
		Ok(Token::Directive(identifier, CodeSegment {
			line: lexer.line,
			idx: *idx - lexer.len(),
			len: lexer.len()
		}))
	}
}