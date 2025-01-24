use std::fmt::Display;

use crate::{
    ast::{Binary, Expr, Literal, Unary},
    error::InterpErr,
    error::InterpErr as Ie,
    token::TokenKind as Tk,
};

// Literals are a bit of syntax that produces a value. They exist in the source code.
// Values dont exist in the source code, they are computed in the interprerter.
type Value = Literal;

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Literal::Str(s) => format!("{s}"),
            Literal::Number(n) => format!("{n}"),
            Literal::Null => format!("null"),
            Literal::Bool(b) => format!("{b}"),
        };
        write!(f, "{msg}")
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(e: Expr) -> Result<Value, InterpErr> {
        match evaluate(e) {
            Ok(e) => {
                println!("{e}");
                Ok(e)
            }
            Err(e) => Err(e),
        }
    }
}

fn evaluate(e: Expr) -> Result<Value, InterpErr> {
    match e {
        Expr::Unary(unary) => unary_eval(unary),
        Expr::Binary(binary) => binary_eval(binary),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Lit(literal) => Ok(literal),
    }
}

fn binary_eval(b: Binary) -> Result<Value, InterpErr> {
    let left = evaluate(*b.left)?;
    let right = evaluate(*b.right)?;

    match b.operator.kind {
        Tk::Minus => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Number(l - r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::Slash => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Number(l / r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::Star => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Number(l * r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::Plus => {
            if let (Literal::Number(l), Literal::Number(r)) = (left.clone(), right.clone()) {
                return Ok(Value::Number(l + r));
            }

            if let (Literal::Str(l), Literal::Str(r)) = (left, right) {
                return Ok(Value::Str(l + &r));
            }

            rt_error(b.operator.line, "Operand must be 'string' or 'number'")
        }
        Tk::Greater => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Bool(l > r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::GreaterEqual => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Bool(l >= r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::Less => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Bool(l < r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::LessEqual => {
            if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                return Ok(Value::Bool(l <= r));
            }

            rt_error(b.operator.line, "Operands must be number")
        }
        Tk::BangEqual => Ok(Value::Bool(left != right)),
        Tk::EqualEqual => Ok(Value::Bool(left == right)),
        _ => rt_error(b.operator.line, "Invalid operator"),
    }
}

fn unary_eval(u: Unary) -> Result<Value, InterpErr> {
    let right = evaluate(*u.right)?;

    match u.operator.kind {
        Tk::Bang => Ok(Value::Bool(!truthy(right))),
        Tk::Minus => {
            if let Literal::Number(n) = right {
                return Ok(Value::Number(-n));
            }

            rt_error(u.operator.line, "Operand must be a number")
        }
        _ => rt_error(u.operator.line, "Ivalid operator"),
    }
}

fn rt_error(line: usize, msg: &str) -> Result<Value, InterpErr> {
    Err(Ie::RuntimeError {
        line,
        msg: msg.to_string(),
    })
}

fn truthy(v: Literal) -> bool {
    match v {
        Literal::Bool(b) => b,
        Literal::Null => false,
        _ => true,
    }
}
