use crate::lexer::tokens::Token;
use crate::parse::symbols::{self, *};
use crate::parse::ast::*;
use crate::errors;
use crate::parse;

use colored::Colorize;

pub trait Parser {
	fn parse(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err>;
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
							format!("Unexpected token {}.", token),
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
			let msg = errors::Msg::One(format!("Expected directive token, found {}", token));
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