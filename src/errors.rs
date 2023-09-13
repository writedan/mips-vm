use std::fmt;

use crate::lexer::tokens::CodeSegment;

use crate::mips;

use colored::Colorize;

#[derive(Debug)]
pub enum Msg {
	One(String),
	Many(Vec<String>)
}

#[derive(Debug)]
pub enum ErrType {
	Syntax,
	Assemble
}

#[derive(Debug)]
pub struct Err {
	pub segment: CodeSegment,
	pub errtype: ErrType,
	pub msg: Msg
}

pub struct DisplayableErr {
	pub err: Err,
	pub line: String
}

impl DisplayableErr {
	pub fn new(err: Err, line: &String) -> DisplayableErr {
		DisplayableErr {
			err,
			line: line.to_string()
		}
	}
}

impl fmt::Display for DisplayableErr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let errtype = match self.err.errtype {
			ErrType::Syntax => "syntax error",
			ErrType::Assemble => "parse error"
		}.to_string().bright_black();

		let prelude = format!("{} ({}) on line {} at {}.", "Error".red().bold(), errtype, self.err.segment.line + 1, self.err.segment.idx + 1);

		let mut line = &self.line;
		let mut line = line.trim().to_string();
		let mut line = mips::syntax_highlight(line);

		write!(f, "{}\n", prelude);

		let range = self.err.segment.idx..(self.err.segment.idx + self.err.segment.len);

		for idx in 0..line.len() {
            if range.contains(&idx) {
                write!(f, "{}", line.chars().nth(idx).expect("This should not fail.").to_string().red());
            } else {
                write!(f, "{}", line.chars().nth(idx).expect("This should not fail."));
            }
        }

        write!(f, "\n");

        match &self.err.msg {
        	Msg::One(msg) => {
        		write!(f, "{}{} {}", " ".repeat(self.err.segment.idx), "^".repeat(self.err.segment.len).red().bold(), msg.bold());
        	},
        	Msg::Many(msgs) => {
        		for (idx, msg) in msgs.iter().enumerate() {
        			write!(f, "{}{} {}", " ".repeat(self.err.segment.idx), "^".repeat(self.err.segment.len).red().bold(), msg.bold());
        			if idx <(msgs.len() - 1) {
        				write!(f, "\n");
        			}
        		}
        	}
        }

        Ok(())
	}
}