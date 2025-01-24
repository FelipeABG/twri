use std::{collections::HashMap, slice::Iter};

use crate::{
    error::InterpErr,
    token::{Token, TokenKind},
};

// MENTAL MODEL:
// parses each character of the source string into the language tokens
// single char: just parses it directly
// single or double char: check if the next char is a match to the first char.
// literals: go to the end of the literal and parses it into the object representation
// unused: skip them

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
    keywords: HashMap<String, TokenKind>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".into(), TokenKind::And);
        keywords.insert("class".into(), TokenKind::Class);
        keywords.insert("else".into(), TokenKind::Class);
        keywords.insert("false".into(), TokenKind::False);
        keywords.insert("fn".into(), TokenKind::Fn);
        keywords.insert("for".into(), TokenKind::For);
        keywords.insert("if".into(), TokenKind::If);
        keywords.insert("null".into(), TokenKind::Null);
        keywords.insert("or".into(), TokenKind::Print);
        keywords.insert("return".into(), TokenKind::Return);
        keywords.insert("super".into(), TokenKind::Super);
        keywords.insert("this".into(), TokenKind::This);
        keywords.insert("true".into(), TokenKind::True);
        keywords.insert("var".into(), TokenKind::Var);
        keywords.insert("while".into(), TokenKind::While);

        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
            start: 0,
            keywords,
        }
    }

    pub fn tokenized(&mut self) -> Result<Iter<'_, Token>, InterpErr> {
        while !self.finished() {
            self.start = self.current;

            self.process_next()?
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), self.line));
        Ok(self.tokens.iter())
    }

    fn process_next(&mut self) -> Result<(), InterpErr> {
        let c = self.next_char();

        match c {
            //single char tokens
            '(' => {
                self.add_token(TokenKind::LeftParen);
                Ok(())
            }
            ')' => {
                self.add_token(TokenKind::RightParen);
                Ok(())
            }
            '{' => {
                self.add_token(TokenKind::LeftBrace);
                Ok(())
            }
            '}' => {
                self.add_token(TokenKind::RightBrace);
                Ok(())
            }
            ',' => {
                self.add_token(TokenKind::Comma);
                Ok(())
            }
            '.' => {
                self.add_token(TokenKind::Dot);
                Ok(())
            }
            ';' => {
                self.add_token(TokenKind::Semicolon);
                Ok(())
            }
            '+' => {
                self.add_token(TokenKind::Plus);
                Ok(())
            }
            '-' => {
                self.add_token(TokenKind::Minus);
                Ok(())
            }
            '*' => {
                self.add_token(TokenKind::Star);
                Ok(())
            }
            //single or double char tokens
            '!' => {
                let kind = if self.complement('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(kind);
                Ok(())
            }
            '=' => {
                let kind = if self.complement('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
                Ok(())
            }
            '>' => {
                let kind = if self.complement('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(kind);
                Ok(())
            }
            '<' => {
                let kind = if self.complement('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(kind);
                Ok(())
            }
            '/' => {
                if self.complement('/') {
                    //if it is a comment, skip the line
                    while self.peek() != '\n' && !self.finished() {
                        self.next_char();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenKind::Slash);
                    Ok(())
                }
            }
            //literals
            '"' => self.string(),
            c if c.is_numeric() => self.number(),
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            //meaningless chars
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            c => Err(InterpErr::SyntaxError {
                line: self.line,
                msg: "Unexpected character".to_string(),
                place: format!("{c}"),
            }),
        }
    }

    fn identifier(&mut self) -> Result<(), InterpErr> {
        while self.peek().is_alphanumeric() {
            self.next_char();
        }

        let lexeme = &self.source[self.start..self.current];

        match self.keywords.get(lexeme) {
            Some(kw) => {
                self.add_token(kw.clone());
                Ok(())
            }
            None => {
                self.add_token(TokenKind::Identifier);
                Ok(())
            }
        }
    }

    fn number(&mut self) -> Result<(), InterpErr> {
        while self.peek().is_numeric() {
            self.next_char();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.next_char();

            while self.peek().is_numeric() {
                self.next_char();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenKind::Number(value));
        Ok(())
    }

    fn string(&mut self) -> Result<(), InterpErr> {
        while self.peek() != '"' && !self.finished() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.next_char();
        }

        if self.finished() {
            return Err(InterpErr::SyntaxError {
                line: self.line,
                msg: "Unterminated string".to_string(),
                place: "".to_string(),
            });
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
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
