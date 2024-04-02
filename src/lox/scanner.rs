use crate::lox::token::Token;
use std::vec::Vec;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(self) -> Vec<Token> {
        return Vec::new();
    }
}
