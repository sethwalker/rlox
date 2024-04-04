use crate::lox::token::{Object, Token, TokenType};
use crate::lox::Lox;
use std::vec::Vec;

#[derive(Debug)]
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
            None,
            self.line,
        ));
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            '0'..='9' => {
                self.number();
            }
            _ => Lox::error(self.line, String::from("Unexpected character.")),
        }
    }

    fn is_digit(&self, c: &char) -> bool {
        // todo: same range as match above, duplication
        return ('0'..='9').contains(c);
    }

    fn number(&mut self) {
        loop {
            let c = &self.peek();
            if self.is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(&self.peek_next()) {
            // Consume the "."
            self.advance();

            // duplication again
            loop {
                let c = &self.peek();
                if self.is_digit(c) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let value = self.source_chars[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        self.add_token(
            TokenType::Number,
            Some(Object::Num(value.parse::<f64>().unwrap())),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, String::from("Unterminated string."));
            return;
        }

        // The closing ".
        self.advance();

        let start = self.start + 1;
        // Trim the surrounding quotes.
        let value = self.source_chars[start as usize..self.current as usize - 1]
            .iter()
            .collect::<String>();
        self.add_token(TokenType::String, Some(Object::Str(value)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_chars[self.current as usize];
    }

    fn peek_next(&self) -> char {
        if self.current as usize + 1 >= self.source_chars.len() {
            return '\0';
        }
        return self.source_chars[self.current as usize + 1];
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source_chars[self.current as usize] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char {
        // TODO: bad casting, too much iterating
        let n = self.current as usize;
        self.current += 1;
        return self.source_chars[n];
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = self.source_chars[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(ttype, text, literal, self.line));
    }
}
