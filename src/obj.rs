use std::fmt::Display;

use crate::call::Callable;

#[derive(PartialEq, Clone)]
pub enum LoxObject {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
    Callable(Callable),
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LoxObject::Str(s) => format!("{s}"),
            LoxObject::Number(n) => format!("{n}"),
            LoxObject::Null => format!("null"),
            LoxObject::Bool(b) => format!("{b}"),
            _ => format!("object print not implemented yet"),
        };
        write!(f, "{msg}")
    }
}
