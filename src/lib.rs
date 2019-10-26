
//! This is an implementation of an interpreter for the brainfuck "programming language".


pub mod instruction;
pub mod components;
pub mod interpreter;
pub mod error;
pub mod util;

pub mod prelude {
    //! Basic functions and structs you may need along the way
    pub use crate::{
        instruction::{ Instruction, },
        components::{ Tape, Memory, IO },
        interpreter::{ Interpreter, eval_string, },
        error::BrainfuckError,
        util::ToBytes,
    };
}