pub mod tokens;
mod consumers;

use crate::lexer::consumers::Consumer;
use crate::lexer::tokens::*;

use crate::errors;

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
			line: text,
			msg
		})
	}

	// return buffer size
	pub fn len(&self) -> usize {
		self.buffer.len()
	}
}

type LexRes<T> = Result<T, errors::Err>;

fn tokenize(program: Vec<String>) -> LexRes<Vec<Token>> {
	let mut tokens: Vec<Token> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {
		let line = line.trim();
		let line = line.to_string();
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
				},

				'.' => {
					match consumers::Directive::consume(&mut idx, &mut lexer) {
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
	}

	Ok(tokens)
}