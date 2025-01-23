use std::any::Any;

use crate::{
    ast::{Binary, Expr, Literal, Unary},
    error::RuntimeError,
    token::kinds::TokenKind as Tk,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(expr: Expr) {}

    fn evaluate(expr: Expr) -> Result<Box<dyn Any>, RuntimeError> {
        match expr {
            Expr::Unary(unary) => Self::evaluate_unary(unary),
            Expr::Binary(binary) => Self::evaluate_binary(binary),
            Expr::Grouping(expr) => Self::evaluate_grouping(*expr),
            Expr::Lit(literal) => Self::evaluate_literal(literal),
        }
    }

    fn evaluate_unary(u: Unary) -> Result<Box<dyn Any>, RuntimeError> {
        todo!()
    }

    fn evaluate_binary(expr: Binary) -> Result<Box<dyn Any>, RuntimeError> {
        todo!()
    }

    fn evaluate_grouping(expr: Expr) -> Result<Box<dyn Any>, RuntimeError> {
        Self::evaluate(expr)
    }

    fn evaluate_literal(expr: Literal) -> Result<Box<dyn Any>, RuntimeError> {
        match expr {
            Literal::Str(s) => Ok(Box::new(s)),
            Literal::Number(n) => Ok(Box::new(n)),
            Literal::True => Ok(Box::new(true)),
            Literal::False => Ok(Box::new(false)),
            Literal::Nil => Ok(Box::new(())),
        }
    }
}
