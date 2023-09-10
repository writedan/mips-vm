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

    let mut program: Vec<String> = io::BufReader::new(file).lines().map(|l| {
        match l {
            Ok(line) => line,
            Err(why) => {
                println!("{} failed to open \"{}\": {}", "Error:".red().bold(), args.file.bright_black(), why);
                std::process::exit(0);
            }
        }
    }).collect();

    let mut final_line = match program.pop() {
        Some(line) => line,
        None => {
            println!("{} file has no final line?", "Error:".red().bold());
            std::process::exit(0);
        }
    };

    final_line.push_str(" # [auto-generated]"); // this is necessary to trigger the final line being read
    program.push(final_line);

    match lexer::tokenize(program) {
        Ok(tokens) => {
            println!("{:#?}", tokens);
            match parse::parse(tokens) {
                Ok(nodes) => {

                },
                Err(err) => println!("{}", err)
            }
        },
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