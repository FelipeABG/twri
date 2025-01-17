use std::any::Any;

use crate::{
    error::SyntaxError,
    token::{kinds::TokenKind, Token},
};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenized(&mut self) -> Vec<&Token> {
        while !self.finished() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), None, self.line));
        self.tokens.iter().collect()
    }

    fn finished(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let char = self.next();
        match char {
            //single char tokens
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            // single or double char tokens
            _ => panic!(
                "{}",
                SyntaxError::new(self.line, "Unexpected character", "")
            ),
        }
    }

    fn next(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.generate_token(kind, None)
    }

    fn generate_token(&mut self, kind: TokenKind, literal: Option<Box<dyn Any>>) {
        let lexeme = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(kind, lexeme.to_string(), literal, self.line));
    }
}
