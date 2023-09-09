mod tokens;
mod ast;

use crate::lexer::tokens::*;
use crate::lexer::ast::*;

struct Lexer {
	// the lexer is initialized for each line

	line: usize, 	// line number
	text: String,	// line text
	buffer: String,	// encountered symbols that cannot yet be tokenized
}

pub enum Msg {
	One(String),
	Many(Vec<String>)
}

pub struct LexErr {
	segment: CodeSegment,
	msg: Msg
}

pub enum Node {
	Tree(ASTree<Token>),
	Token(Token)
}

type LexRes<T> = Result<T, LexErr>;

pub fn read(program: Vec<String>) -> LexRes<Vec<Node>> {
	let nodes: Vec<Node> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {

	}

	Ok(nodes)
}