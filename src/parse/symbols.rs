use crate::lexer::tokens::CodeSegment;
use crate::parse::instructions::{Instruction, Register};

#[derive(Debug)]
pub enum Symbol {
	Directive(Directive, CodeSegment),
	DefLabel(DefLabel, CodeSegment),
	Label(Label, CodeSegment),
	Instruction(Instruction, CodeSegment), // instruction and label must be parsed out here
	Register(Register, CodeSegment),
	StringLiteral(StringLiteral, CodeSegment),
	NumberLiteral(NumberLiteral, CodeSegment)
}

#[derive(Debug)]
pub struct Directive {
	pub id: String
}

#[derive(Debug)]
pub struct DefLabel {
	pub id: String
}

#[derive(Debug)]
pub struct Label {
	pub id: String
}

#[derive(Debug)]
pub struct StringLiteral {
	pub content: String
}

#[derive(Debug)]
pub struct NumberLiteral {
	pub value: i32
}