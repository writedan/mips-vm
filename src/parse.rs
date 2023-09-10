mod ast;
mod symbols;

use crate::parse::symbols::*;
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
		let symbol: Option<Symbol> = match token {
			Token::Register(id, segment) => {
				match id.as_str() {
					"0" | "zero" => Some(Symbol::Register(Register::Z, segment.clone())),
					"1" | "at" => Some(Symbol::Register(Register::AT, segment.clone())),
					"2" | "v0" => Some(Symbol::Register(Register::V0, segment.clone())),
					"3" | "v1" => Some(Symbol::Register(Register::V1, segment.clone())),
					"4" | "a0" => Some(Symbol::Register(Register::A0, segment.clone())),
					_ => {
						return call_err(segment, errors::Msg::One(format!("Unknown register \"{}\".", id.red())));
					}
				}
			}

			_ => None
		};

		match symbol {
			Some(symbol) => nodes.push(ASTNode::Node(symbol)),
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