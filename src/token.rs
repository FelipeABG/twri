use std::any::Any;

use kinds::TokenKind;

pub mod kinds;

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Box<dyn Any>>,
    line: usize,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
