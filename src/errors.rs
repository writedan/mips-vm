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
	Syntax
}

#[derive(Debug)]
pub struct Err {
	pub segment: CodeSegment,
	pub errtype: ErrType,
	pub line: String,
	pub msg: Msg
}

impl fmt::Display for Err {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let errtype = match self.errtype {
			ErrType::Syntax => "syntax error"
		}.to_string().bright_black();

		let prelude = format!("{} ({}) on line {} at {}.", "Error".red().bold(), errtype, self.segment.line + 1, self.segment.idx + 1);

		let mut line = &self.line;
		let mut line = line.trim().to_string();
		let mut line = mips::syntax_highlight(line);

		write!(f, "{}\n", prelude);

		let range = self.segment.idx..(self.segment.idx + self.segment.len);

		for idx in 0..line.len() {
            if range.contains(&idx) {
                write!(f, "{}", line.chars().nth(idx).expect("This should not fail.").to_string().red());
            } else {
                write!(f, "{}", line.chars().nth(idx).expect("This should not fail."));
            }
        }

        write!(f, "\n");

        match &self.msg {
        	Msg::One(msg) => {
        		write!(f, "{}{} {}", " ".repeat(self.segment.idx), "^".repeat(self.segment.len).red().bold(), msg);
        	},
        	Msg::Many(msgs) => {
        		for (idx, msg) in msgs.iter().enumerate() {
        			write!(f, "{}{} {}", " ".repeat(self.segment.idx), "^".repeat(self.segment.len).red().bold(), msg);
        			if idx <(msgs.len() - 1) {
        				write!(f, "\n");
        			}
        		}
        	}
        }

        Ok(())
	}
}