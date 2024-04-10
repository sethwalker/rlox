#[path = "../lox/lib.rs"]
mod lox;

use lox::Lox;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

pub fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    Lox::run(&contents);
}

pub fn run_prompt() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");
        Lox::run(&input);
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        return ExitCode::from(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }

    ExitCode::SUCCESS
}
