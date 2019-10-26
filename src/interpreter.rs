
//! The interpreter itself and others relating to it

use crate::{
    error::{ BrainfuckError, },
    instruction::{ Instruction },
    components::{ Tape, Memory, IO, },
};
use std::{
    iter::FromIterator,
    fmt::{ self, Debug, Display },
};


/// The main 'interpreter' structure.
/// Handles interactions between the tape, memory, and IO
pub struct Interpreter {
    // Instructions and instruction pointer
    tape: Tape,

    // Logical memory and data pointer
    memory: Memory,

    // Input / Output
    io: IO,
} impl Interpreter {

    pub fn new(tape: Tape, input: Option<Vec<u8>>) -> Self {
        let io = if input.is_some() { IO::new(input.unwrap()) } else { IO::default() };
        Interpreter {
            tape,
            memory: Memory::new(),
            io,
        }
    }

    /// Gets the current instruction, advances the instruction pointer, handles the instructions
    fn step(&mut self) -> Result<Option<Vec<u8>>, BrainfuckError> {

        match self.tape.get_instruction() {
            Instruction::IncPtr => {
                self.memory.inc_ptr()?;
            },
            Instruction::DecPtr => {
                self.memory.dec_ptr();
            },
            Instruction::IncVal => {
                self.memory.inc_val();
            },
            Instruction::DecVal => {
                self.memory.dec_val();
            },
            Instruction::Input => {
                self.io.write_to(&mut self.memory);
            },
            Instruction::Output => {
                self.io.pull_from(&mut self.memory);
            },
            Instruction::SetJump => {
                if self.memory.pull() == 0 {
                    self.tape.jump_forward()?;
                }
            },
            Instruction::Jump => {
                if self.memory.pull() != 0 {
                    self.tape.jump_back()?;
                }
            },
            Instruction::EOF => {
                return Ok(Some(self.io.output()))
            },
            Instruction::NOP => {
                return Ok(None)
            },
            Instruction::Debug => {
                eprintln!("{:?};\n{:?};\n{:?};", self.memory, self.io, self.tape)
            }
        };
        Ok(None)
    }

    /// Evaluates the code the interpreter was given and outputs the result
    pub fn eval(&mut self) -> Result<InterpreterOutput, BrainfuckError> {
        loop {
            match self.step() {
                Err(e) => return Err(e),
                Ok(Some(output)) => return Ok(InterpreterOutput(output)),
                Ok(None) => (),
            }
        }
    }
}

/// The output type of the brainfuck interpreter
pub struct InterpreterOutput(Vec<u8>);
impl InterpreterOutput {
    /// Output the data as a `String`
    pub fn to_string(&self) -> String {
        String::from_iter(self.0.iter().map(|byte| *byte as char))
    }

    /// Output the data as a `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}
/// format the data as a `Vec<u8>`
impl Debug for InterpreterOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
} 
/// format the data as a `String`
impl Display for InterpreterOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Evaluate brainfuck code with an optional input
pub fn eval_string(code: &'static str, input: Option<Vec<u8>>) -> Result<InterpreterOutput, BrainfuckError> {
    Interpreter::new(Tape::from_string(code), input).eval()
}
