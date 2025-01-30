use crate::{error::InterpErr, interp::Interpreter};
use std::fmt::Display;

pub trait Callable {
    fn call(&self, interp: &Interpreter, args: Vec<LoxObject>) -> Result<LoxObject, InterpErr>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
    fn clone_box(&self) -> Box<dyn Callable>;
}

pub enum LoxObject {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
    Callable(Box<dyn Callable>),
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxObject::Number(a), LoxObject::Number(b)) => a == b,
            (LoxObject::Str(a), LoxObject::Str(b)) => a == b,
            (LoxObject::Bool(a), LoxObject::Bool(b)) => a == b,
            (LoxObject::Null, LoxObject::Null) => true,
            (LoxObject::Callable(_), LoxObject::Callable(_)) => false,
            _ => false,
        }
    }
}

impl Clone for LoxObject {
    fn clone(&self) -> Self {
        match self {
            LoxObject::Number(n) => LoxObject::Number(*n),
            LoxObject::Str(s) => LoxObject::Str(s.clone()),
            LoxObject::Bool(b) => LoxObject::Bool(*b),
            LoxObject::Null => LoxObject::Null,
            LoxObject::Callable(c) => LoxObject::Callable(c.clone()),
        }
    }
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LoxObject::Str(s) => format!("{s}"),
            LoxObject::Number(n) => format!("{n}"),
            LoxObject::Null => format!("null"),
            LoxObject::Bool(b) => format!("{b}"),
            LoxObject::Callable(c) => format!("{}", c.to_string()),
        };
        write!(f, "{msg}")
    }
}
