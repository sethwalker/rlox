#[derive(Debug)]
pub struct Lox {}

mod scanner;
mod token;

use scanner::Scanner;
use token::Token;

impl Lox {
    pub fn run(source: &str) {
        let mut scanner: Scanner = Scanner::new(&source);
        let tokens: &Vec<Token> = scanner.scan_tokens();

        for token in tokens.iter() {
            println!("{}", token);
        }
    }

    fn report(line: u32, whilst: String, message: String) {
        println!("[line {}] Error{}: {}", line, whilst, message);
    }

    pub fn error(line: u32, message: String) {
        //report(line, "", message);
    }
}
