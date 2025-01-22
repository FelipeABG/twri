use core::panic;

use crate::ast::{Binary, Expr, Literal, Unary};
use crate::token::kinds::TokenKind;
use crate::token::kinds::TokenKind as Tk;
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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Tk::BangEqual | Tk::EqualEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.comparison().clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Tk::Greater | Tk::GreaterEqual | Tk::Less | Tk::LessEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.term().clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while let Tk::Minus | Tk::Plus = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.factor().clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while let Tk::Slash | Tk::Star = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.unary().clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        while let Tk::Bang | Tk::Minus = self.peek().kind {
            let operator = self.previous().clone();
            let right = Box::new(self.unary().clone());
            return Expr::Unary(Unary::new(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        match self.peek().clone().kind {
            TokenKind::False => {
                self.next_token();
                Expr::Lit(Literal::False)
            }
            TokenKind::True => {
                self.next_token();
                Expr::Lit(Literal::True)
            }
            TokenKind::Nil => {
                self.next_token();
                Expr::Lit(Literal::Nil)
            }
            TokenKind::Number(n) => {
                self.next_token();
                Expr::Lit(Literal::Number(n))
            }
            TokenKind::String(s) => {
                self.next_token();
                Expr::Lit(Literal::Str(s))
            }
            TokenKind::LeftParen => {
                self.next_token();
                let expr = Box::new(self.expression());
                self.consume(Tk::RightParen, "Expected ')' after expression");
                Expr::Grouping(expr)
            }
            _ => panic!("Expected Expression"),
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) {
        if kind == self.peek().kind {
            self.next_token();
            return;
        }

        panic!("{msg}")
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

    fn next_token(&mut self) -> &Token {
        if !self.finished() {
            self.current += 1
        };
        self.previous()
    }
}
