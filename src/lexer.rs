mod errors;

use slab_tree::*;
use colored::Colorize;
use crate::lexer::errors::*;

#[derive(Debug)]
pub enum Token { // each token minimally has a (usize, usize, u8) being (line number, character index, len)
	Section(String, usize, usize, u8), // sections are given as ".{name}"
	DefLabel(String, usize, usize, u8), // definition of labels, given as "{name}:" in data section but merely "{name}" in the text section
	Directive(String, usize, usize, u8), // directives of labels, given as ".{name}" following a DefLabel
	Instruction(String, usize, usize, u8),
	Register(String, usize, usize, u8),
	Number(u32, usize, usize, u8),
	Label(String, usize, usize, u8), // labels used outside definitons
	Empty,
	Placehold(char)
}

#[derive(Debug)]
pub enum Node { // the lexer will create a vector, containing either trees of tokens, or tokens
	Tree(Tree<Node>),
	Token(Token)
}

type LexRes<T> = Result<T, LexErr>;

struct Lexer { // one lexer exists for each line, since this is assembly
	text: String, // text of the line in whole
	number: usize, // line number
	buffer: String // buffer of symbols read in hopes of making sense later
}

pub fn lexify(program: &Vec<String>) -> LexRes<Vec<Node>> {
	let tokens = match tokenize(program) { // generally only returns Tokens, not Nodes
		Ok(tokens) => tokens,
		Err(err) => return Err(err)
	};

	// we will have to do a second pass to transform it into a hierarchial structure

	Ok(tokens)
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
					match consume_section(&mut idx, &lexer) {
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

				_ => {
					lexer.buffer.push(character);
					Node::Token(Token::Placehold(character))
				}
			});
			idx += 1;
		}
	}

	Ok(nodes)
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

	root.append(Node::Token(Token::Empty));

	Ok(Node::Tree(tree))
}

fn consume_section(idx: &mut usize, lexer: &Lexer) -> LexRes<Node> {
	*idx += 1; // consume our entry point already matched
	let mut name = String::new();

	while *idx < lexer.text.len() {
		let character = get_character(*idx, &lexer.text);

		match character {
			'a'..='z' => name.push(character),
			' ' => break, // space deliminates the section name
			_ => return Err(LexErr {
				msg: format!("Illegal symbol {}. Only alphabetic characters are legal in section name.", character.to_string().red()),
				line: lexer.number,
				character: *idx,
				len: 1
			})
		}

		*idx += 1;
	}

	let len = name.len();

	Ok(Node::Token(Token::Section(name, lexer.number, *idx - len, len as u8)))
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