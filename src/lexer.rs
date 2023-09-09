mod errors;

use slab_tree::*;
use colored::Colorize;
use crate::lexer::errors::*;

#[derive(Debug)]
pub enum Token { // each token minimally has a (usize, usize, u8) being (line number, character index, len)
	DefLabel(String, usize, usize, u8), // definition of labels, given as "{name}: .{type} {value}" in .data 
	Directive(String, usize, usize, u8), // given as ".{name}"
	Instruction(String, usize, usize, u8),
	Register(String, usize, usize, u8),
	Number(u32, usize, usize, u8),
	String(String, usize, usize, u8),
	Label(String, usize, usize, u8), // labels used outside definitons
	Empty,
	Placehold(char)
}

#[derive(Debug)]
pub enum Node { // the lexer will create a vector, containing either trees of tokens, or tokens
	Tree(Tree<Node>),
	Token(Token)
}

struct TokenLocation {
	line: usize,
	character: usize,
	length: usize
}

type LexRes<T> = Result<T, LexErr>;

struct Lexer { // one lexer exists for each line, since this is assembly
	text: String, // text of the line in whole
	number: usize, // line number
	buffer: String // buffer of symbols read in hopes of making sense later
}

impl Token {
	fn remove(&self) -> bool {
		match self {
			Token::Empty => true,
			Token::Placehold(_) => true,
			_ => false
		}
	}
}

impl Node {
	fn remove(&self) -> bool {
		match self {
			Node::Token(token) => token.remove(),
			_ => false
		}
	}
}

pub fn read(program: &Vec<String>) -> LexRes<Vec<Node>> {
	let mut nodes = match tokenize(program) { // generally only returns Tokens, not Nodes
		Ok(nodes) => nodes.into_iter().filter(|n| !n.remove()).collect(),
		Err(err) => return Err(err)
	};

	Ok(nodes)
}

fn tokenize(program: &Vec<String>) -> LexRes<Vec<Node>> {
	let mut nodes: Vec<Node> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {
		let line = line.trim();
		let line = line.to_string(); // trim() produces a &str
		if line.len() == 0 { continue; }

		let mut lexer = Lexer {
			text: line,
			number: line_num,
			buffer: String::new()
		};

		let mut idx = 0;
		while idx < lexer.text.len() {
			let character = get_character(idx, &lexer.text);
			nodes.push(match character {
				'#' => consume_comment(&mut idx, &lexer),
				'.' => {
					match consume_directive(&mut idx, &lexer) {
						Ok(node) => node,
						Err(err) => return Err(err)
					}
				},
				':' => {
					match consume_def_label(&mut idx, &mut lexer) {
						Ok(node) => node,
						Err(err) => return Err(err)
					}
				},
				' ' => {
					match consume_instruction(&mut idx, &mut lexer) {
						Ok(node) => node,
						Err(err) => return Err(err)
					}
				},
				_ => {
					lexer.buffer.push(character);
					(Node::Token(Token::Placehold(character)))
				}
			});
			idx += 1;
		}

		if lexer.buffer.len() > 0 {
			match consume_instruction(&mut idx, &mut lexer) {
				Ok(node) => nodes.push(node),
				Err(err) => return Err(err)
			}
		}
	}

	Ok(nodes)
}

fn consume_instruction(idx: &mut usize, lexer: &mut Lexer) -> LexRes<Node> {
	let identifier = lexer.buffer.drain(..).collect::<String>(); // we retain the value but clear the buffer
	let start = lexer.text.find(&identifier).unwrap();
	let label = Node::Token(Token::Instruction(identifier, lexer.number, start, *idx as u8));

	let mut tree = TreeBuilder::new().with_root(label).build();
	let mut root = match tree.root_id() {
		Some(root) => root,
		None => {
			return Err(LexErr {
				msg: "Tree root vanished.".to_string(),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}
	};

	let mut root = tree.get_mut(root).unwrap();

	*idx += 1;

	// some code must be duplicated here until a better version is found
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		root.append(match character {
			'$' => match consume_register(idx, &lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			'0'..='9' => match consume_number(idx, &lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			'#' => consume_comment(idx, &lexer),
			'a'..='z' | 'A'..='Z' => match consume_label(idx, &lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			' ' => Node::Token(Token::Empty),
			_ => {
				return Err(LexErr {
					msg: format!("Unknown symbol \"{}\" while lexing instruction.", character.to_string().red().bold()),
					line: lexer.number,
					character: *idx,
					len: 1
				})
			}
		});

		*idx += 1;
	}

	Ok(Node::Tree(tree))
}

fn consume_label(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	let mut buffer = String::new();
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		match character {
			'a'..='z' | 'A'..='Z' => buffer.push(character),
			'0'..='9' => buffer.push(character),
			'_' => buffer.push(character),
			',' => break,
			' ' => match skip_whitespace(idx, lexer) {
				Err(err) => return Err(err),
				Ok(node) => {}
			},
			'#' => {
				consume_comment(idx, &lexer);
			},
			_ => return Err(LexErr {
				msg: format!("Illegal symbol \"{}\" while lexing label.", character.to_string().red()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}

		*idx += 1;
	}

	let len = buffer.len();
	Ok(Node::Token(Token::Label(buffer, lexer.number, *idx - len, len as u8)))
}

// consumes spaces until a comma-delimeter is reached
// this is kinda hacky but i don't see a better way to validate the code
fn skip_whitespace(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		match character {
			' ' => {},
			',' => break,
			'#' => {
				*idx -= 1;
				break;
			}
			_ => {
				return Err(LexErr {
					msg: format!("Unexpected symbol \"{}\". Comma-delimit instruction arguments.", character.to_string().red()),
					line: lexer.number,
					character:  *idx,
					len: 1
				})
			}
		}

		*idx += 1;
	}

	Ok(Node::Token(Token::Empty))
}

fn consume_number(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	let mut buffer = String::new();
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		match character {
			' ' => match skip_whitespace(idx, &lexer) {
				Err(err) => return Err(err),
				Ok(_) => {}
			},
			'#' => {consume_comment(idx, &lexer); ()},
			',' => break,
			'0'..='9' => buffer.push(character),
			_ => return Err(LexErr {
				msg: format!("Illegal symbol \"{}\". Only numeric characters are legal when beginning with a numeral.", character.to_string().red()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}
		*idx += 1;
	}

	let len = buffer.len();
	let buffer = match buffer.parse::<u32>() {
		Ok(int) => int,
		Err(err) => return Err(LexErr {
			msg: format!("An error occured trying to parse number \"{}\"", buffer),
			line: lexer.number,
			character: *idx,
			len: 1
		})
	};

	Ok(Node::Token(Token::Number(buffer, lexer.number, *idx - len, len as u8)))
}

fn consume_register(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	let mut buffer = String::new();
	*idx += 1;
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		match character {
			',' => break,
			' ' => {},
			'a'..='z' | 'A'..='Z' => buffer.push(character),
			'0'..='9' => buffer.push(character),
			_ => return Err(LexErr {
				msg: format!("Illegal symbol \"{}\". Only alphanumeric characters are legal in register identifier.", character.to_string().red()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}

		*idx += 1;
	}

	let len = buffer.len();
	Ok(Node::Token(Token::Register(buffer, lexer.number, *idx - len, len as u8)))
}

fn consume_def_label(idx: &mut usize, lexer: &mut Lexer) -> LexRes<Node> {
	let identifier = lexer.buffer.drain(..).collect::<String>(); // we retain the value but clear the buffer
	let start = lexer.text.find(&identifier).unwrap();
	let label = Node::Token(Token::DefLabel(identifier, lexer.number, start, *idx as u8));

	// anything can follow a label
	// and labels can be defined at any time
	// anything that follows a label is part of that label until another label is defined
	// for now we will be happy to collate merely what is on the same line

	*idx += 1; // go past where we currently are, i.e. avoid the ":" entry point

	let mut tree = TreeBuilder::new().with_root(label).build();
	let mut root = match tree.root_id() {
		Some(root) => root,
		None => {
			return Err(LexErr {
				msg: "Tree root vanished.".to_string(),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}
	};

	let mut root = tree.get_mut(root).unwrap();

	// some code must be duplicated here until a better version is found
	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		root.append(match character {
			'.' => match consume_directive(idx, &lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			' ' => {
				Node::Token(Token::Empty)
			},
			'A'..='z' => match consume_instruction(idx, lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			'"' => match consume_string(idx, &lexer) {
				Ok(node) => node,
				Err(err) => return Err(err)
			},
			'#' => consume_comment(idx, &lexer),
			_ => return Err(LexErr {
				msg: format!("Unknown symbol \"{}\" while lexing label definition.", character.to_string().red().bold()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		});

		*idx += 1;
	}

	Ok(Node::Tree(tree))
}

fn consume_string(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	*idx += 1; // consume the entry point
	let mut buffer = String::new();

	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);
		match character {
			'"' => match buffer.chars().last() {
				Some(character) => {
					if !(character == '\\') { break; }
					else { buffer.push(character); }
				},
				None => return Err(LexErr {
					msg: format!("Empty string defined here."),
					line: lexer.number,
					character: *idx - 1,
					len: 1
				})
			},
			_ => buffer.push(character)
		}

		*idx += 1;
	}

	let len = buffer.len();
	Ok(Node::Token(Token::String(buffer, lexer.number, *idx, len as u8)))
}

fn consume_directive(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	*idx += 1; // consume our entry point already matched
	let mut name = String::new();

	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);

		match character {
			'a'..='z' | 'A'..='Z' => name.push(character),
			' ' => break, // space deliminates the section name
			_ => return Err(LexErr {
				msg: format!("Illegal symbol \"{}\". Only alphabetic characters are legal in directive name.", character.to_string().red()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}

		*idx += 1;
	}

	let len = name.len();

	Ok(Node::Token(Token::Directive(name, lexer.number, *idx - len, len as u8)))
}

fn consume_comment(idx: &mut usize, lexer: &Lexer) -> Node {
	*idx += lexer.text.len();
	Node::Token(Token::Empty)
}

fn get_character(idx: usize, line: &String) -> char {
	match line.chars().nth(idx) {
		Some(character) => character,
		None => panic!("No character found at index {}", idx)
	}
}