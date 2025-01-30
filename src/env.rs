use crate::{error::InterpErr, obj::LoxObject, token::Token};
use format as fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, LoxObject>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            variables: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, key: &str, value: LoxObject) {
        self.variables.insert(key.to_string(), value);
    }

    pub fn assign(&mut self, key: Token, value: LoxObject) -> Result<LoxObject, InterpErr> {
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

    pub fn get(&self, key: &Token) -> Result<LoxObject, InterpErr> {
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
