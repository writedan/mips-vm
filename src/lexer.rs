pub mod tokens;
mod consumers;

use crate::lexer::consumers::Consumer;
use crate::lexer::tokens::*;

use crate::errors;

use colored::Colorize;

pub struct Lexer {
	// the lexer is initialized for each line

	line: usize, 	// line number
	text: String,	// line text
	buffer: String,	// encountered symbols that cannot yet be tokenized
}

impl Lexer {
	fn idx(&self, idx: usize) -> char {
		match self.text.chars().nth(idx) {
			Some(character) => character,
			None => panic!("No character found at index {}", idx)
		}
	}

	pub fn error(&self, idx: usize, len: usize, msg: errors::Msg) -> std::result::Result<Token, errors::Err> {
		let text = String::from(&self.text);
		std::result::Result::Err(errors::Err {
			segment: CodeSegment {
				line: self.line,
				idx,
				len
			},
			errtype: errors::ErrType::Syntax,
			msg
		})
	}

	// return buffer size
	pub fn len(&self) -> usize {
		self.buffer.len()
	}

	pub fn verify_buffer(&self, consumer: &str) -> Result<Token, errors::Err> {
		if self.buffer.len() > 0 {
			let msgs = errors::Msg::Many(vec![
				format!("Trying to consume {} but buffer length = {}", consumer, self.buffer.len()),
				format!("buffer = \"{}\"", self.buffer.red()),
				"Buffer should be empty.".to_string()
			]);
			return self.error(self.text.len(), 0, msgs);
		}

		Ok(Token::Empty)
	}
}

type LexRes<T> = Result<T, errors::Err>;

pub fn tokenize(program: &Vec<String>) -> LexRes<Vec<Token>> {
	let mut tokens: Vec<Token> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {
		let line = line.trim();
		let line = line.to_string();
		let line = format!("{} # auto-generated", line);
		let mut lexer = Lexer {
			line: line_num,
			text: line,
			buffer: String::new()
		};

		let mut idx = 0;
		while idx < lexer.text.len() {
			let character = lexer.idx(idx);
			let token: Option<Token> = match character {
				'#' => {
					consumers::Comment::consume(&mut idx, &mut lexer);
					None
				}

				'.' => {
					match consumers::Directive::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				':' => {
					match consumers::DefLabel::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				'$' => {
					match consumers::Register::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				'"' => {
					match consumers::StringLiteral::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				'0'..='9' | '-' => {
					match consumers::NumberLiteral::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				' ' => {
					match consumers::Identifier::consume(&mut idx, &mut lexer) {
						Ok(token) => Some(token),
						Err(err) => return Err(err)
					}
				}

				_ => {
					lexer.buffer.push(character);
					None
				}
			};

			if let Some(token) = token {
				tokens.push(token);
			}

			idx += 1;
		}

		if lexer.buffer.len() > 0 {
			let msg = errors::Msg::Many(vec![
				format!("Line ended but buffer length = {}", lexer.buffer.len()),
				format!("buffer = \"{}\"", lexer.buffer.red()),
				"Buffer should be empty.".to_string()
			]);
			let text = String::from(lexer.text);
			return std::result::Result::Err(errors::Err {
				segment: CodeSegment {
					line: lexer.line,
					idx,
					len: 0
				},
				errtype: errors::ErrType::Syntax,
				msg
			});
		}
	}

	Ok(tokens)
}