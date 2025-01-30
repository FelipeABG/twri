use crate::error::InterpErr;
use crate::interp::Interpreter;
use crate::obj::Callable;
use crate::obj::LoxObject;
use format as fmt;

pub struct Clock {}

impl Callable for Clock {
    fn call(&self, _interp: &Interpreter, _args: Vec<LoxObject>) -> Result<LoxObject, InterpErr> {
        Ok(LoxObject::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as f64,
        ))
    }

    fn arity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        return fmt!("<native fn> clock");
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(Self {})
    }
}
