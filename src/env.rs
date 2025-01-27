use crate::{ast::Literal, error::InterpErr, token::Token};
use format as fmt;
use std::collections::HashMap;

type Value = Literal;

pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    pub fn get(&self, key: Token) -> Result<Value, InterpErr> {
        match self.variables.get(&key.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(InterpErr::RuntimeError {
                line: key.line,
                msg: fmt!("Undefined variable '{}'", key.lexeme),
            }),
        }
    }
}
