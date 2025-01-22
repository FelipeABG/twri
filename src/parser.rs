use crate::token::kinds::TokenKind;
use crate::token::Token;

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn finished(&self) -> bool {
        return self.current >= self.tokens.len();
    }

    fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds.iter() {
            if self.check(kind) {
                self.next_token();
                return true;
            }
        }

        false
    }

    fn next_token(&mut self) -> &Token {
        if !self.finished() {
            self.current += 1
        };
        self.previous()
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.finished() {
            return false;
        };
        self.peek().kind == *kind
    }
}
