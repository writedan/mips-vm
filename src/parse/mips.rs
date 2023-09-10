use crate::lexer::tokens::Token;

pub enum Register {
	Zero,								// zero = 0
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

pub enum Instruction { // first token is always the identifier, remainder are arguments
	LoadImmediate(Token, Register, Token), 	// token must be number
	LoadAddress(Token, Register, Token), 	// token must be identifier (label)
	SystemCall(Token)
}