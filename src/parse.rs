mod ast;
mod symbols;
mod parsers;
mod instructions;

use crate::parse::symbols::*;
use crate::parse::parsers::Parser;
use crate::parse::ast::*;
use crate::lexer::tokens::CodeSegment;
use crate::lexer::tokens::Token;
use crate::errors;

use colored::Colorize;

type ParRes<T> = Result<T, errors::Err>;
type Return = ParRes<Vec<ASTNode<Symbol>>>;

pub fn parse_one(idx: &mut usize, program: &Vec<Token>) -> ParRes<ASTNode<Symbol>> {
	let token = &program[*idx];
	let symbol: Option<ASTNode<Symbol>> = match token {
		Token::Directive(id, segment) => {
			match parsers::Directive::parse(idx, &program) {
				Ok(node) => Some(node),
				Err(err) => return Err(err)
			}
		},

		Token::DefLabel(id, segment) => {
			match parsers::DefLabel::parse(idx, &program) {
				Ok(node) => Some(node),
				Err(err) => return Err(err)
			}
		},

		Token::Identifier(id, segment) => {
			// we can assume this to be an instruction, because labels can only occur after instructions
			// thus labels will be consumed elsewhere
			// labels included here will fail to validate as instructions for one reason or anotehr
			match parsers::Instruction::parse(idx, &program) {
				Ok(node) => Some(node),
				Err(err) => return Err(err)
			}
		}

		_ => {
			let msg = errors::Msg::One(format!("Unexpected token {}.", token.to_string().red()));
			return Err(errors::Err {
				segment: extract_segment(token),
				errtype: errors::ErrType::Assemble,
				msg
			});
		}
	};

	match symbol {
		Some(symbol) => Ok(symbol),
		None => {
			let msg = errors::Msg::One(format!("Token failed to parse."));
			return Err(errors::Err {
				segment: extract_segment(token),
				errtype: errors::ErrType::Assemble,
				msg
			});
		}
	}
}

pub fn parse(program: &Vec<Token>) -> Return {
	let mut nodes = Vec::new();

	let mut idx = 0;
	while idx < program.len() {
		match parse_one(&mut idx, program) {
			Ok(symbol) => nodes.push(symbol),
			Err(err) => return Err(err)
		}

		idx += 1;
	}

	Ok(nodes)
}

pub fn transform(nodes: Vec<ASTNode<Symbol>>) -> BaseASTree<Symbol> {
	let mut basetree = BaseASTree::<Symbol>::new();
	for node in nodes {
		basetree.add_node(node);
	}

	return basetree;
}

fn call_err(segment: &CodeSegment, msg: errors::Msg) -> Return {
	Err(errors::Err {
		segment: segment.clone(),
		errtype: errors::ErrType::Assemble,
		msg
	})
}

fn extract_segment(token: &Token) -> CodeSegment {
	match token {
		Token::Directive(_, segment) => segment.clone(),
		Token::DefLabel(_, segment) => segment.clone(),
		Token::Identifier(_, segment) => segment.clone(),
		Token::Register(_, segment) => segment.clone(),
		Token::StringLiteral(_, segment) => segment.clone(),
		Token::NumberLiteral(_, segment) => segment.clone(),
		_ => {
			panic!("Illegal token {:?}", token);
		}
	}
}