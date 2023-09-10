mod ast;
mod mips;

use crate::parse::ast::*;
use crate::lexer::tokens::CodeSegment;
use crate::lexer::tokens::Token;
use crate::errors;

use colored::Colorize;

type ParRes<T> = Result<T, errors::Err>;
type Return = ParRes<Vec<ASTNode<Token>>>;

pub fn parse(program: &Vec<Token>) -> Return {
	let mut nodes = Vec::new();

	for token in program {
		println!("{:#?}", token);
	}

	Ok(nodes)
}

fn call_err(token: Token, segment: CodeSegment, msg: errors::Msg) -> Return {
	Err(errors::Err {
		segment,
		errtype: errors::ErrType::Assemble,
		msg
	})
}