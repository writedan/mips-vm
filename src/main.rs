use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use clap::Parser;
use colored::Colorize;

mod lexer;

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

    let lexed_program = lexer::lexify(&program);
    if let Err(error) = lexed_program {
        let prelude = format!("{} ({}) on line {} at {}. ", "Error".red().bold(), "syntax error".bright_black(), error.line + 1, error.character + 1);
        print!("{}", prelude);
        let mut line = &program[error.line];
        let range = error.character..error.len;
        for idx in 0..line.len() {
            if range.contains(&idx) {
                print!("{}", line.chars().nth(idx).expect("This should not fail.").to_string().bright_black());
            } else {
                print!("{}", line.chars().nth(idx).expect("This should not fail."));
            }
        }
        println!();
        println!("{} {}", " ".repeat(prelude.len() - 21 + error.character), error.msg);
        return;
    }

    println!("{:#?}", lexed_program);
}