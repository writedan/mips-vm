struct CodeSegment {
	// identifiers the location of a token within the text of a program

	line: usize,		// line number
	idx: usize,			// character index
	len: usize			// how many characters included
						// line[idx..=len] being the whole code segment
}

pub enum Token {
	Directive(String, CodeSegment), 	// given as ".{name}", controls how following instruction(s)/token(s) are interpreted
	DefLabel(String, CodeSegment),		// given as "{name}:", stores the following instruction as a named memory location
	Instruction(String, CodeSegment),	// {name} {arg1}, {arg2}, arguments must be registers or labels
	Register(String, CodeSegment),		// ${id}
	Label(String, CodeSegment),			// refers to a named memory location
	
	StringLiteral(String, CodeSegment),
	NumberLiteral(i32, CodeSegment),	// numbers are always
}