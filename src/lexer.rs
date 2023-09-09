pub mod tokens;
mod ast;

use crate::lexer::tokens::*;
use crate::lexer::ast::*;

use crate::errors;

struct Lexer {
	// the lexer is initialized for each line

	line: usize, 	// line number
	text: String,	// line text
	buffer: String,	// encountered symbols that cannot yet be tokenized
}

pub enum Node {
	Tree(ASTree<Token>),
	Token(Token)
}

type LexRes<T> = Result<T, errors::Err>;

fn tokenize(program: Vec<String>) -> LexRes<Vec<Node>> {
	let nodes: Vec<Node> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {
		let line = line.trim();
		let line = line.to_string();
		let lexer = Lexer {
			line: line_num,
			text: line,
			buffer: String::new()
		};
	}

	Ok(nodes)
}