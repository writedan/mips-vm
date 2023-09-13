use crate::lexer::tokens::{Token, CodeSegment};
use crate::parse::parsers::Parser;
use crate::parse::parsers;
use crate::parse::symbols::*;
use crate::parse::ast::*;
use crate::parse;
use crate::errors;


#[derive(Debug)]
pub enum Instruction {
	LoadImmediate,
	LoadAddress,
	SystemCall
}

#[derive(Debug)]
pub enum Register {
	Z0,									// zero = 0
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

pub fn parse_syscall(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
	let instruction = Symbol::Instruction(Instruction::SystemCall, parse::extract_segment(&tokens[*idx]));
	Ok(ASTNode::Node(instruction))
}

pub fn parse_la(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
	*idx += 1;
	let register = match parsers::Register::parse(idx, tokens) {
		Ok(node) => node,
		Err(err) => return Err(err)
	};

	*idx += 1;
	let value = match parsers::Label::parse(idx, tokens) {
		Ok(node) => node,
		Err(err) => return Err(err)
	};

	let head_segment = parse::extract_segment(&tokens[*idx - 2]);
	let tail_segment = parse::extract_segment(&tokens[*idx]);
	let full_segment = CodeSegment {
		line: head_segment.line,
		idx: head_segment.idx,
		len: (tail_segment.len + tail_segment.idx) - head_segment.idx
	};

	let instruction = Symbol::Instruction(Instruction::LoadAddress, full_segment.clone());

	let mut tree = ASTree::<Symbol>::new(instruction);
	tree.add_node(register);
	tree.add_node(value);

	Ok(ASTNode::Tree(tree))
}


pub fn parse_li(idx: &mut usize, tokens: &Vec<Token>) -> Result<ASTNode<Symbol>, errors::Err> {
	*idx += 1;
	let register = match parsers::Register::parse(idx, tokens) {
		Ok(node) => node,
		Err(err) => return Err(err)
	};

	*idx += 1;
	let value = match parsers::NumberLiteral::parse(idx, tokens) {
		Ok(node) => node,
		Err(err) => return Err(err)
	};

	let head_segment = parse::extract_segment(&tokens[*idx - 2]);
	let tail_segment = parse::extract_segment(&tokens[*idx]);
	let full_segment = CodeSegment {
		line: head_segment.line,
		idx: head_segment.idx,
		len: (tail_segment.len + tail_segment.idx) - head_segment.idx
	};

	let instruction = Symbol::Instruction(Instruction::LoadImmediate, full_segment.clone());

	let mut tree = ASTree::<Symbol>::new(instruction);
	tree.add_node(register);
	tree.add_node(value);

	Ok(ASTNode::Tree(tree))
}