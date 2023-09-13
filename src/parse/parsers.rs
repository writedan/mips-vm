use crate::lexer::tokens::Token;
use crate::parse::instructions;
use crate::parse::symbols::{self, *};
use crate::parse::ast::*;
use crate::errors;
use crate::parse;

use colored::Colorize;

pub trait Parser {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err>;
}

pub struct Label;
impl Parser for Label {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		if let Token::Identifier(id, segment) = &tokens[*idx] {
			let symbol = Symbol::Label(symbols::Label {
				id: id.to_string()
			}, segment.clone());

			return Ok(ASTNode::Node(symbol));
		}

		let msg = errors::Msg::Many(vec![
			format!("Unexpected token {}.", &tokens[*idx].to_string().red()),
			"Expected label.".to_string()
		]);

		Err(errors::Err{
			segment: parse::extract_segment(&tokens[*idx]),
			msg,
			errtype: errors::ErrType::Assemble
		})
	}
}

pub struct NumberLiteral;
impl Parser for NumberLiteral {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		if let Token::NumberLiteral(num, segment) = &tokens[*idx] {
			let symbol = Symbol::NumberLiteral(symbols::NumberLiteral {
				value: *num
			}, segment.clone());

			return Ok(ASTNode::Node(symbol));
		}

		let msg = errors::Msg::Many(vec![
			format!("Unexpected token {}.", &tokens[*idx].to_string().red()),
			"Expected number literal.".to_string()
		]);

		Err(errors::Err{
			segment: parse::extract_segment(&tokens[*idx]),
			msg,
			errtype: errors::ErrType::Assemble
		})
	}
}

pub struct Register;
impl Parser for Register {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		if let Token::Register(id, segment) = &tokens[*idx] {
			let register = match id.as_str() {
				 "0" | "zero" => instructions::Register::Z0,
				 "1" | "at" => instructions::Register::AT,
				 "2" | "v0" => instructions::Register::V0,
				 "3" | "v1" => instructions::Register::V1,
				 "4" | "a0" => instructions::Register::A0,
				 "5" | "a1" => instructions::Register::A1,
				 "6" | "a2" => instructions::Register::A2,
				 "7" | "a3" => instructions::Register::A3,
				 "8" | "t0" => instructions::Register::T0,
				 "9" | "t1" => instructions::Register::T1,
				"10" | "t2" => instructions::Register::T2,
				"11" | "t3" => instructions::Register::T3,
				"12" | "t4" => instructions::Register::T4,
				"13" | "t5" => instructions::Register::T5,
				"14" | "t6" => instructions::Register::T6,
				"15" | "t7" => instructions::Register::T7,

				_ => {
					return Err(errors::Err {
						segment: segment.clone(),
						msg: errors::Msg::One(format!("Unknown register {}.", id.red())),
						errtype: errors::ErrType::Assemble
					});
				}
			};

			let symbol = Symbol::Register(register, segment.clone());

			return Ok(ASTNode::Node(symbol));
		}

		let msg = errors::Msg::Many(vec![
			format!("Unexpected token {}.", &tokens[*idx].to_string().red()),
			"Expected register.".to_string()
		]);

		Err(errors::Err{
			segment: parse::extract_segment(&tokens[*idx]),
			msg,
			errtype: errors::ErrType::Assemble
		})
	}
}

pub struct Instruction;
impl Parser for Instruction {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		if let Token::Identifier(id, segment) = &tokens[*idx] {
			match id.as_str() {
				"li" => return instructions::parse_li(idx, tokens),

				"la" => return instructions::parse_la(idx, tokens),

				"syscall" => return instructions::parse_syscall(idx, tokens),

				_ => {
					let msg = errors::Msg::One(format!("Unknown instruction {}.", id.red()));
					return Err(errors::Err {
						segment: parse::extract_segment(&tokens[*idx]),
						msg,
						errtype: errors::ErrType::Assemble
					});
				}
			}
		}

		let msg = errors::Msg::Many(vec![
			format!("Unexpected token {}.", &tokens[*idx].to_string().red()),
			"Expected identifier as instruction.".to_string()
		]);

		Err(errors::Err{
			segment: parse::extract_segment(&tokens[*idx]),
			msg,
			errtype: errors::ErrType::Assemble
		})
	}
}

pub struct DefLabel{}
impl Parser for DefLabel {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		if let Token::DefLabel(id, segment) = &tokens[*idx] {
			let symbol = Symbol::DefLabel(symbols::DefLabel {
				id: id.to_string()
			}, segment.clone());
			let mut tree = ASTree::<Symbol>::new(symbol);

			*idx += 1; // the label will attach to the memory location of the next symbol, whether instruction or directive-allocation
			match parse::parse_one(idx, tokens) {
				Ok(symbol) => tree.add_node(symbol),
				Err(err) => return Err(err)
			}

			return Ok(ASTNode::Tree(tree));
		}

		let msg = errors::Msg::Many(vec![
			format!("Unexpected token {}", &tokens[*idx].to_string().red()),
			format!("Expected label definition.")
		]);

		Err(errors::Err {
			segment: parse::extract_segment(&tokens[*idx]),
			msg,
			errtype: errors::ErrType::Assemble
		})
	}
}

pub struct Directive {}
impl Parser for Directive {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
		let token = &tokens[*idx];
		if let Token::Directive(id, segment) = token {
			let symbol = Symbol::Directive(symbols::Directive{
				id: id.to_string()
			}, segment.clone());
			let mut tree = ASTree::<Symbol>::new(symbol);

			match id.as_str() {
				"data" | "text" => {
					// parse the tokens until the next such directive or until all tokens are added
					// every token belongs to one of these in a tree
					match parse_until_next_directive(idx, tokens) {
						Ok(nodes) => {
							for node in nodes {
								tree.add_node(node);
							}
						},
						Err(err) => {
							return Err(err);
						}
					}
				},
				"asciiz" => {
					// next token must be a string literal
					*idx += 1;
					let token = &tokens[*idx];
					if let Token::StringLiteral(string, segment) = token {
						let node = Symbol::StringLiteral(StringLiteral {
							content: string.to_string()
						}, segment.clone());
						tree.add_node(ASTNode::Node(node));
					} else {
						let msg = errors::Msg::Many(vec![
							format!("Unexpected token {}.", token.to_string().red()),
							format!("Expected string literal.")
						]);
						return Err(errors::Err {
							segment: parse::extract_segment(token),
							msg,
							errtype: errors::ErrType::Assemble
						});
					}
				}
				_ => {
					let msg = errors::Msg::One(format!("Unknown directive {}.", id.red()));
					return Err(errors::Err{
						segment: segment.clone(),
						errtype: errors::ErrType::Assemble,
						msg
					});
				}
			};

			Ok(ASTNode::Tree(tree))
		} else {
			let msg = errors::Msg::One(format!("Expected directive token, found {}", token.to_string().red()));
			return Err(errors::Err{
				segment: parse::extract_segment(&token),
				errtype: errors::ErrType::Assemble,
				msg
			});
		}
	}
}

fn parse_until_next_directive(idx: &mut usize, tokens: &Vec<Token>) -> Result<Vec<ASTNode<Symbol>>, errors::Err> {
	*idx += 1; // skip initial token
	let mut nodes: Vec<Token> = Vec::new();
	while *idx < tokens.len() {
		let token = &tokens[*idx];
		if let Token::Directive(id, segment) = token {
			match id.as_str() {
				"data" | "text" => break,
				_ => nodes.push(token.clone())
			}
		} else {
			nodes.push(token.clone());
		}
		*idx += 1;
	}
	match parse::parse(&nodes) {
		Ok(nodes) => Ok(nodes),
		Err(err) => {Err(err)}
	}
}