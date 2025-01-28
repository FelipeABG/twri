use crate::{ast::Literal, error::InterpErr, token::Token};
use format as fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Value = Literal;

#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            variables: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    pub fn assign(&mut self, key: Token, value: Value) -> Result<Value, InterpErr> {
        match self.variables.get(&key.lexeme) {
            Some(_) => Ok(self.variables.insert(key.lexeme, value).unwrap()),
            None => {
                if let Some(e) = &self.enclosing {
                    return RefCell::borrow_mut(e).assign(key, value);
                }

                Err(InterpErr::RuntimeError {
                    line: key.line,
                    msg: fmt!("Undefined variable '{}'", key.lexeme),
                })
            }
        }
    }

    pub fn get(&self, key: &Token) -> Result<Value, InterpErr> {
        match self.variables.get(&key.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if let Some(e) = &self.enclosing {
                    return RefCell::borrow_mut(e).get(key);
                };

                Err(InterpErr::RuntimeError {
                    line: key.line,
                    msg: fmt!("Undefined variable '{}'", key.lexeme),
                })
            }
        }
    }
}
