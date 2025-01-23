use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct SyntaxError {
    line: usize,
    msg: String,
    place: String,
}

impl SyntaxError {
    pub fn new(line: usize, msg: &str, place: &str) -> Self {
        Self {
            line,
            msg: msg.to_string(),
            place: place.to_string(),
        }
    }
}

impl Error for SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "> {} [line {}] Error: {} at '{}'",
            std::any::type_name::<Self>().split("::").last().unwrap(),
            self.line,
            self.msg,
            self.place
        )
    }
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "> {} [line {}] Error: {} '{}'",
            std::any::type_name::<Self>().split("::").last().unwrap(),
            self.line,
            self.msg,
            self.place
        )
    }
}
