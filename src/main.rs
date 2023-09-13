use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;
use colored::Colorize;

mod lexer;
mod errors;
mod parse;

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

    let program: Vec<String> = io::BufReader::new(file).lines().map(|l| {
        match l {
            Ok(line) => line,
            Err(why) => {
                println!("{} failed to open \"{}\": {}", "Error:".red().bold(), args.file.bright_black(), why);
                std::process::exit(0);
            }
        }
    }).collect();

    match lexer::tokenize(&program) {
        Ok(tokens) => {
            match parse::parse(&tokens) {
                Ok(nodes) => {
                    let tree = parse::transform(nodes);
                    println!("{:#?}", tree);
                },
                Err(err) => handle_err(program, err)
            }
        },
        Err(err) => handle_err(program, err)
    }
}

fn handle_err(program: Vec<String>, err: errors::Err) {
    let line = &program[err.segment.line];
    println!("{}", errors::DisplayableErr::new(err, line));
}

pub mod mips {
    // highlight snytax for one line of code
    pub fn syntax_highlight(code: String) -> String {
        // note that red is only to be used for errors
        code
    }
}