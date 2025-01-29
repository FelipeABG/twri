use crate::{error::InterpErr, interp::Interpreter, obj::LoxObject};

#[derive(Clone, PartialEq)]
pub struct Callable {}

impl Callable {
    pub fn call(&self, interp: &Interpreter, args: Vec<LoxObject>) -> Result<LoxObject, InterpErr> {
        todo!()
    }

    pub fn arity(&self) -> usize {
        todo!()
    }
}
