use crate::{ast::Literal, error::InterpErr, interp::Interpreter};

type Value = Literal;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&mut self, interp: &mut Interpreter, args: Vec<Value>) -> Result<Value, InterpErr>;
}
