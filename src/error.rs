use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpErr {
    #[error("SyntaxError [line {line}] Error: {msg} '{place}'")]
    SyntaxError {
        line: usize,
        msg: String,
        place: String,
    },

    #[error("RuntimeError [line {line}] Error: {msg}")]
    RuntimeError { line: usize, msg: String },
}
