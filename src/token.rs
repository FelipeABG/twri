use kinds::TokenKind;

pub mod kinds;

// Token = lexeme + some information
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind, //type of the token
    pub lexeme: String,  //substring representation of the token
    pub line: usize,     //token line in the source
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}
