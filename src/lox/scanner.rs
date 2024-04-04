use crate::lox::token::{Object, Token, TokenType};
use crate::lox::Lox;
use std::vec::Vec;

pub struct Scanner {
    source_chars: Vec<char>,
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let source_str = source.to_string();
        let source_chars = source_str.chars().collect();
        Self {
            source: source_str,
            source_chars: source_chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current as usize >= self.source.len();
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            Object {},
            self.line,
        ));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Object {}),
            _ => Lox::error(self.line, String::from("Unexpected character.")),
        }
    }

    fn advance(&mut self) -> char {
        // TODO: bad casting, too much iterating
        let n = self.current as usize;
        self.current += 1;
        return self.source_chars[n];
    }

    fn add_token(&mut self, ttype: TokenType, literal: Object) {
        let text = self.source_chars[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(ttype, text, literal, self.line));
    }
}
