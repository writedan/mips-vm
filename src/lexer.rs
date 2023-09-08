mod errors;

use slab_tree::*;
use colored::Colorize;
use crate::lexer::errors::*;

#[derive(Debug)]
pub enum Token { // each token minimally has a (usize, usize, u8) being (line number, character index, len)
	Section(String, usize, usize, u8), // sections are given as ".{name}"
	Label(String, usize, usize, u8), // labels of variables, given as "{name}:" in data section but merely "{name}" in the text section
	Directive(String, usize, usize, u8), // directives of labels, given as ".{name}"
	Instruction(String, usize, usize, u8),
	Register(String, usize, usize, u8),
	Number(u32, usize, usize, u8),
	Empty
}

#[derive(Debug)]
pub enum Node { // the lexer will create a vector, containing either trees of tokens, or tokens
	Tree(Tree<Token>),
	Token(Token)
}

type LexRes<T> = Result<T, LexErr>;

pub fn lexify(program: &Vec<String>) -> LexRes<Vec<Node>> {
	let mut nodes: Vec<Node> = Vec::new();

	for (line_num, line) in program.iter().enumerate() {
		let line = line.trim();
		let line = line.to_string(); // trim() produces a &str
		if line.len() == 0 { continue; }

		let mut idx = 0;
		let mut buffer = String::new();
		while idx < line.len() {
			let character = get_character(idx, &line);
			nodes.push(match character {
				'#' => consume_comment(&mut idx, &line),
				'.' => {
					match consume_section(&mut idx, line_num, &line) {
						Ok(node) => node,
						Err(err) => return Err(err)
					}
				},

				_ => {
					buffer.push(character);
					Node::Token(Token::Empty)
				}
			});
			idx += 1;
		}
	}

	Ok(nodes)
}

/*fn consume_label(idx: &mut usize, line_num: usize, line: &String) -> LexRes<Node> {
	*idx += 1; // consume the entry point
}*/

fn consume_section(idx: &mut usize, line_num: usize, line: &String) -> LexRes<Node> {
	*idx += 1; // consume our entry point already matched
	let mut name = String::new();

	while *idx < line.len() {
		let character = get_character(*idx, &line);

		match character {
			'a'..='z' => name.push(character),
			' ' => break, // space deliminates the section name
			_ => return Err(LexErr {
				msg: format!("Illegal symbol {}. Only alphabetic characters are legal in section name.", character.to_string().red()),
				line: line_num,
				character: *idx,
				len: 1
			})
		}

		*idx += 1;
	}

	let len = name.len();

	let root = Node::Token(Token::Section(name, line_num, *idx - len, len as u8));
	
}

fn consume_comment(idx: &mut usize, line: &String) -> Node {
	*idx += line.len();
	Node::Token(Token::Empty)
}

fn get_character(idx: usize, line: &String) -> char {
	match line.chars().nth(idx) {
		Some(character) => character,
		None => panic!("No character found at index {}", idx)
	}
}