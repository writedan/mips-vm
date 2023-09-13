use std::fmt;

#[derive(Debug, Clone)]
pub struct CodeSegment {
	// identifiers the location of a token within the text of a program

	pub line: usize,		// line number
	pub idx: usize,			// character index
	pub len: usize			// how many characters included
						// line[idx..=len] being the whole code segment
}

#[derive(Debug, Clone)]
pub enum Token {
	Directive(String, CodeSegment), 	// given as ".{name}", controls how following instruction(s)/token(s) are interpreted
	DefLabel(String, CodeSegment),		// given as "{name}:", stores the following instruction as a named memory location
	Identifier(String, CodeSegment),	// some yet unknown identifier in the form {id}, will have to be parsed to its correct value
	Register(String, CodeSegment),		// ${id}

	StringLiteral(String, CodeSegment),
	NumberLiteral(i32, CodeSegment),	// numbers are always

	Newline,
	Empty
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Token::Directive(id, _) => write!(f, "Directive"),
			Token::DefLabel(id, _) => write!(f, "DefineLabel"),
			Token::Identifier(id, _) => write!(f, "Identifier"),
			Token::Register(id, _) => write!(f, "Register"),
			Token::StringLiteral(id, _) => write!(f, "StringLiteral"),
			Token::NumberLiteral(num, _) => write!(f, "NumberLiteral"),
			_ => panic!("Unknown token {:?}", self)
		};

		Ok(())
	}
}