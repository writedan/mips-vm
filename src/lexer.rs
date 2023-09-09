mod tokens;

struct Lexer {
	// the lexer is initialized for each line

	line: usize, 	// line number
	text: String,	// line text
	buffer: String,	// encountered symbols that cannot yet be tokenized
}

