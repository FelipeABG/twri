use std::slice::Iter;

use crate::{
    error::SyntaxError,
    token::{kinds::TokenKind, Token},
};

// MENTAL MODEL:
// parses each character of the source string into the language tokens
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
            start: 0,
        }
    }

    pub fn tokenized(&mut self) -> Result<Iter<'_, Token>, SyntaxError> {
        while !self.finished() {
            self.start = self.current;

            self.process_next()?
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), self.line));
        Ok(self.tokens.iter())
    }

    fn process_next(&mut self) -> Result<(), SyntaxError> {
        let c = self.next_char();

        match c {
            //single char tokens
            '(' => Ok(self.add_token(TokenKind::LeftParen)),
            ')' => Ok(self.add_token(TokenKind::RightParen)),
            '{' => Ok(self.add_token(TokenKind::LeftBrace)),
            '}' => Ok(self.add_token(TokenKind::RightBrace)),
            ',' => Ok(self.add_token(TokenKind::Comma)),
            '.' => Ok(self.add_token(TokenKind::Dot)),
            ';' => Ok(self.add_token(TokenKind::Semicolon)),
            '+' => Ok(self.add_token(TokenKind::Plus)),
            '-' => Ok(self.add_token(TokenKind::Minus)),
            '*' => Ok(self.add_token(TokenKind::Star)),
            //single or double char tokens
            '!' => {
                let kind = if self.complement('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                Ok(self.add_token(kind))
            }
            '=' => {
                let kind = if self.complement('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                Ok(self.add_token(kind))
            }
            '>' => {
                let kind = if self.complement('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                Ok(self.add_token(kind))
            }
            '<' => {
                let kind = if self.complement('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                Ok(self.add_token(kind))
            }
            '/' => {
                if self.complement('/') {
                    //if it is a comment, skip the line
                    while self.peek() != '\n' && !self.finished() {
                        self.next_char();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenKind::Slash))
                }
            }
            //literals
            '"' => self.string(),
            //meaningless chars
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                //if it is a new line, increment the line number;
                self.line += 1;
                Ok(())
            }
            _ => Err(SyntaxError::new(self.line, "Unexpected character", "")),
        }
    }

    fn string(&mut self) -> Result<(), SyntaxError> {
        while self.peek() != '"' && !self.finished() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.next_char();
        }

        if self.finished() {
            return Err(SyntaxError::new(self.line, "Unterminated string", ""));
        }

        self.next_char();

        let value = &self.source[self.start + 1..self.current - 1];
        let lexeme = &self.source[self.start..self.current];

        self.tokens.push(Token::new(
            TokenKind::String(value.to_string()),
            lexeme.to_string(),
            self.line,
        ));

        Ok(())
    }

    fn finished(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.finished() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token(&mut self, ty: TokenKind) {
        let lexeme = &self.source[self.start..self.current];
        let tk = Token::new(ty, lexeme.to_string(), self.line);
        self.tokens.push(tk);
    }

    fn next_char(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn complement(&mut self, expected: char) -> bool {
        if self.finished() {
            return false;
        };
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        };

        self.current += 1;
        true
    }
}
