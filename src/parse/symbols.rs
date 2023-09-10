use crate::lexer::tokens::CodeSegment;

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
pub enum Register {
	Z,									// zero = 0
	AT,									// reserved for assembler
	V0, V1,								// values
	A0, A1, A2, A3,						// arguments
	T0, T1, T2, T3, T4, T5, T6, T7,		// temporary
	S0, S1, S2, S3, S4, S5, S6, S7,		// saved
	T8, T9,								// additional temporaries
	K0, K1,								// reserved by operating system
	GP,									// global pointer
	SP,									// stack pointer
	FP,									// frame pointer
	RA,									// return address
}

#[derive(Debug)]
pub struct Directive {
	id: String
}

#[derive(Debug)]
pub struct DefLabel {
	id: String
}

#[derive(Debug)]
pub struct Label {
	id: String
}

#[derive(Debug)]
pub enum Instruction {

}

#[derive(Debug)]
pub struct StringLiteral {
	content: String
}

#[derive(Debug)]
pub struct NumberLiteral {
	value: i32
}