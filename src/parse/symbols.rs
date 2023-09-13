use crate::lexer::tokens::CodeSegment;
use crate::parse::instructions::{Instruction, Register};

#[derive(Debug)]
pub enum Symbol {
	Directive(String, CodeSegment),
	DefLabel(String, CodeSegment),
	Label(String, CodeSegment),
	Instruction(Instruction, CodeSegment), // instruction and label must be parsed out here
	Register(Register, CodeSegment),
	StringLiteral(String, CodeSegment),
	NumberLiteral(i32, CodeSegment)
}