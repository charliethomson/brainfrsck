
use std::{
    fmt,
    error::Error
};

pub struct BrainfuckError {
    msg: String,
} impl BrainfuckError {
    pub fn new<S: Into<String> + Sized>(msg: S) -> Self {
        BrainfuckError { msg: msg.into() }
    }
} impl fmt::Debug for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Brainfuck error: {}", self.msg)
    }
} impl fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Brainfuck error: {}", self.msg)
    }
} impl Error for BrainfuckError {  }

