mod ast;

use crate::parse::ast::*;
use crate::lexer::tokens::CodeSegment;
use crate::lexer::tokens::Token;
use crate::errors;

use colored::Colorize;

type ParRes<T> = Result<T, errors::Err>;
type Return = ParRes<Vec<ASTNode<Token>>>;

pub fn parse(program: &Vec<Token>) -> Return {
	let mut nodes = Vec::new();

	let mut idx = 0;
	while idx < program.len() {
		let token = &program[idx];
		match token {
			Token::Register(id, segment) => {


			_ => {
				println!("{:?}", token)
			}
		}

		idx += 1;
	}

	Ok(nodes)
}

fn call_err(segment: &CodeSegment, msg: errors::Msg) -> Return {
	Err(errors::Err {
		segment: segment.clone(),
		errtype: errors::ErrType::Assemble,
		msg
	})
}