// Token = lexeme + some information
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //single char Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    //single or double char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String(String),
    Number(f64),

    //keywords
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    Return,
    Super,
    This,
    True,
    Let,
    While,

    Eof,
}
