mod ast;
mod symbols;
mod parsers;

use crate::parse::symbols::*;
use crate::parse::parsers::Parser;
use crate::parse::ast::*;
use crate::lexer::tokens::CodeSegment;
use crate::lexer::tokens::Token;
use crate::errors;

use colored::Colorize;

type ParRes<T> = Result<T, errors::Err>;
type Return = ParRes<Vec<ASTNode<Symbol>>>;

pub fn parse(program: &Vec<Token>) -> Return {
	let mut nodes = Vec::new();

	let mut idx = 0;
	while idx < program.len() {
		let token = &program[idx];
		let symbol: Option<ASTNode<Symbol>> = match token {
			Token::Directive(id, segment) => {
				match parsers::Directive::parse(&mut idx, &program) {
					Ok(node) => Some(node),
					Err(err) => return call_err(segment, err)
				}
			}
			_ => None
		};

		match symbol {
			Some(symbol) => nodes.push(symbol),
			None => {}
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