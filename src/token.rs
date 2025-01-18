use kinds::TokenKind;
use std::any::Any;

pub mod kinds;

// Token = lexeme + some information
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,               //type of the token
    lexeme: String,                //substring representation of the token
    literal: Option<Box<dyn Any>>, //Object representation of a literal (if the token is one)
    line: usize,                   //token line in the source
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
