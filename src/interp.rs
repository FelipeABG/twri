use std::any::Any;

use crate::{
    ast::{Binary, Expr, Literal, Unary},
    error::InterpErr,
    error::InterpErr as Ie,
    token::kinds::TokenKind as Tk,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(expr: Expr) -> Result<(), InterpErr> {
        match Self::evaluate(expr) {
            Ok(value) => {
                if let Some(value) = value.downcast_ref::<f64>() {
                    println!("{}", value);
                } else if let Some(value) = value.downcast_ref::<String>() {
                    println!("{}", value);
                } else if let Some(value) = value.downcast_ref::<bool>() {
                    println!("{}", value);
                } else if let Some(_) = value.downcast_ref::<()>() {
                    println!("nil");
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn evaluate(expr: Expr) -> Result<Box<dyn Any>, InterpErr> {
        match expr {
            Expr::Unary(unary) => evaluate_unary(unary),
            Expr::Binary(binary) => evaluate_binary(binary),
            Expr::Grouping(expr) => evaluate_grouping(*expr),
            Expr::Lit(literal) => evaluate_literal(literal),
        }
    }
}

fn evaluate_unary(u: Unary) -> Result<Box<dyn Any>, InterpErr> {
    let expr = Interpreter::evaluate(*u.right)?;
    match u.operator.kind {
        Tk::Bang => match expr.downcast::<bool>() {
            Ok(value) => Ok(Box::new(!*value)),
            Err(_) => Err(Ie::RuntimeError {
                line: u.operator.line,
                msg: "Operand must be a number".to_string(),
            }),
        },
        Tk::Minus => match expr.downcast::<f64>() {
            Ok(value) => Ok(Box::new(-*value)),
            Err(_) => Err(Ie::RuntimeError {
                line: u.operator.line,
                msg: "Operand must be a number".to_string(),
            }),
        },
        _ => Err(Ie::RuntimeError {
            line: u.operator.line,
            msg: "Invalid operator. Valid ones are '-' and '!'".to_string(),
        }),
    }
}

fn evaluate_binary(b: Binary) -> Result<Box<dyn Any>, InterpErr> {
    let left = Interpreter::evaluate(*b.left)?;
    let right = Interpreter::evaluate(*b.right)?;

    match b.operator.kind {
        Tk::Minus => Ok(Box::new(
            value_ref::<f64>(&left)? - value_ref::<f64>(&right)?,
        )),
        Tk::Slash => Ok(Box::new(
            value_ref::<f64>(&left)? - value_ref::<f64>(&right)?,
        )),
        Tk::Star => Ok(Box::new(
            value_ref::<f64>(&left)? * value_ref::<f64>(&right)?,
        )),
        Tk::Greater => Ok(Box::new(
            value_ref::<f64>(&left)? > value_ref::<f64>(&right)?,
        )),
        Tk::GreaterEqual => Ok(Box::new(
            value_ref::<f64>(&left)? >= value_ref::<f64>(&right)?,
        )),
        Tk::Less => Ok(Box::new(
            value_ref::<f64>(&left)? < value_ref::<f64>(&right)?,
        )),
        Tk::LessEqual => Ok(Box::new(
            value_ref::<f64>(&left)? <= value_ref::<f64>(&right)?,
        )),
        Tk::EqualEqual => {
            if is_same_type(&*left, &*right) {
                if let Ok(vl) = value_ref::<f64>(&left) {
                    let vr = value_ref::<f64>(&right)?;
                    return Ok(Box::new(vl == vr));
                }
                if let Ok(vl) = value_ref::<bool>(&left) {
                    let vr = value_ref::<bool>(&right)?;
                    return Ok(Box::new(vl == vr));
                }
                if let Ok(vl) = value_ref::<String>(&left) {
                    let vr = value_ref::<String>(&right)?;
                    return Ok(Box::new(vl == vr));
                }
                if let Ok(vl) = value_ref::<()>(&left) {
                    let vr = value_ref::<()>(&right)?;
                    return Ok(Box::new(vl == vr));
                }
            }

            Err(InterpErr::RuntimeError {
                line: b.operator.line,
                msg: "Cannot compare values of different types".to_string(),
            })
        }
        Tk::BangEqual => {
            if is_same_type(&*left, &*right) {
                if let Ok(vl) = value_ref::<f64>(&left) {
                    let vr = value_ref::<f64>(&right)?;
                    return Ok(Box::new(vl != vr));
                }
            }

            Err(InterpErr::RuntimeError {
                line: b.operator.line,
                msg: "Cannot compare values of different types".to_string(),
            })
        }
        Tk::Plus => {
            if left.is::<String>() && right.is::<String>() {
                return Ok(Box::new(
                    value_ref::<String>(&left)?.to_string() + value_ref::<String>(&right)?,
                ));
            }

            if left.is::<f64>() && right.is::<f64>() {
                return Ok(Box::new(
                    value_ref::<f64>(&left)? + value_ref::<f64>(&right)?,
                ));
            }

            Err(InterpErr::RuntimeError {
                line: b.operator.line,
                msg: "'+' operation require two two operands of the same type ('str' or 'number')"
                    .to_string(),
            })
        }
        _ => Err(InterpErr::RuntimeError {
            line: b.operator.line,
            msg: "Invalid operator.".to_string(),
        }),
    }
}

fn evaluate_grouping(expr: Expr) -> Result<Box<dyn Any>, InterpErr> {
    Interpreter::evaluate(expr)
}

fn evaluate_literal(expr: Literal) -> Result<Box<dyn Any>, InterpErr> {
    match expr {
        Literal::Str(s) => Ok(Box::new(s)),
        Literal::Number(n) => Ok(Box::new(n)),
        Literal::True => Ok(Box::new(true)),
        Literal::False => Ok(Box::new(false)),
        Literal::Nil => Ok(Box::new(())),
    }
}

fn value_ref<T: Any>(value: &Box<dyn Any>) -> Result<&T, InterpErr> {
    match value.downcast_ref::<T>() {
        Some(v) => Ok(v),
        None => Err(Ie::RuntimeError {
            line: 000,
            msg: "Invalid type".to_string(),
        }),
    }
}

fn is_same_type(o1: &dyn Any, o2: &dyn Any) -> bool {
    o1.type_id() == o2.type_id()
}
