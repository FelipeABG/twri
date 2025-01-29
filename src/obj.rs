use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum LoxObject {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl Display for LoxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LoxObject::Str(s) => format!("{s}"),
            LoxObject::Number(n) => format!("{n}"),
            LoxObject::Null => format!("null"),
            LoxObject::Bool(b) => format!("{b}"),
        };
        write!(f, "{msg}")
    }
}
