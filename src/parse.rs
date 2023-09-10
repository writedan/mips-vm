mod ast;

use crate::parse::ast::*;
use crate::lexer::tokens::Token;
use crate::errors;

type ParRes<T> = Result<T, errors::Err>;

pub fn parse(program: Vec<Token>) -> ParRes<Vec<ASTNode<Token>>> {
	let mut nodes = Vec::new();
	Ok(nodes)
}