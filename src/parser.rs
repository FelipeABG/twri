use crate::ast::{Binary, Expr, Literal, Unary};
use crate::error::SyntaxError;
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

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.comparison()?;

        while let Tk::BangEqual | Tk::EqualEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.comparison()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.term()?;

        while let Tk::Greater | Tk::GreaterEqual | Tk::Less | Tk::LessEqual = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.term()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor()?;

        while let Tk::Minus | Tk::Plus = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.factor()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary()?;

        while let Tk::Slash | Tk::Star = self.peek().kind {
            let left = Box::new(expr.clone());
            let operator = self.next_token().clone();
            let right = Box::new(self.unary()?.clone());
            expr = Expr::Binary(Binary::new(left, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        while let Tk::Bang | Tk::Minus = self.peek().kind {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?.clone());
            return Ok(Expr::Unary(Unary::new(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        match self.peek().clone().kind {
            TokenKind::False => {
                self.next_token();
                Ok(Expr::Lit(Literal::False))
            }
            TokenKind::True => {
                self.next_token();
                Ok(Expr::Lit(Literal::True))
            }
            TokenKind::Nil => {
                self.next_token();
                Ok(Expr::Lit(Literal::Nil))
            }
            TokenKind::Number(n) => {
                self.next_token();
                Ok(Expr::Lit(Literal::Number(n)))
            }
            TokenKind::String(s) => {
                self.next_token();
                Ok(Expr::Lit(Literal::Str(s)))
            }
            TokenKind::LeftParen => {
                self.next_token();
                let expr = Box::new(self.expression()?);
                self.consume(Tk::RightParen, "Expected ')' after expression")?;
                Ok(Expr::Grouping(expr))
            }
            _ => Err(SyntaxError::new(
                self.peek().line,
                "Expected Expression",
                &self.peek().lexeme,
            )),
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) -> Result<(), SyntaxError> {
        if kind == self.peek().kind {
            self.next_token();
            return Ok(());
        }

        Err(SyntaxError::new(self.peek().line, msg, &self.peek().lexeme))
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
