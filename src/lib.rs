
pub mod instruction;
pub mod components;
pub mod interpreter;
pub mod error;


pub mod prelude {
    pub use crate::{
        instruction::{ Instruction, },
        components::{ Tape, Memory, IO },
        interpreter::{ Interpreter, eval_string, },
        error::BrainfuckError,
    };
}