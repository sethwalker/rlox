#[derive(Debug)]
pub struct Lox;

mod scanner;
mod token;

use scanner::Scanner;
use token::Token;

impl Lox {
    pub fn run(source: &str) {
        let scanner: Scanner = Scanner::new(&source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        for token in tokens.iter() {
            println!("{}", token);
        }
    }
}
