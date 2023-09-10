use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;
use colored::Colorize;

mod lexer;
mod errors;

/// A light-weight MIPS emulator and debugger.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The assembly file to be executed.
    file: String,

    /// Does not show the interface with stack, heap, and variables and runs the program straight without steps.
    #[arg(short, long, default_value_t = false)]
    no_debug: bool,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.file);

    let file = match File::open(path) {
        Err(why) => {
            println!("{} failed to open \"{}\": {}", "Error:".red().bold(), args.file.bright_black(), why);
            return;
        },

        Ok(file) => file,
    };

    let program: Vec<String> = io::BufReader::new(file).lines().map(|l| l.expect("Could not parse line.")).collect();
    match lexer::tokenize(program) {
        Ok(tokens) => println!("{:#?}", tokens),
        Err(err) => println!("{}", err)
    }
}

pub mod mips {
    // highlight snytax for one line of code
    pub fn syntax_highlight(code: String) -> String {
        // note that red is only to be used for errors
        code
    }
}