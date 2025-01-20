use kinds::TokenKind;

pub mod kinds;

// Token = lexeme + some information
#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    kind: TokenKind, //type of the token
    lexeme: String,  //substring representation of the token
    line: usize,     //token line in the source
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}
