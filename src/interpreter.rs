
use crate::{
    error::{ BrainfuckError, },
    instruction::{ Instruction },
    components::{ Tape, Memory, IO, },
};
use std::{
    path::PathBuf,
    iter::FromIterator,
};

pub struct Interpreter {
    // Instructions and instruction pointer
    tape: Tape,

    // Logical memory and data pointer
    memory: Memory,

    // Input / Output
    io: IO,

    // Jump pointers
    // Depth increasing, pointers front to back
    jp: Vec<usize>,

} impl Interpreter {
    pub fn new(tape: Tape, input: Option<Vec<u8>>) -> Self {
        let io = if input.is_some() { IO::new(input.unwrap()) } else { IO::default() };
        Interpreter {
            tape,
            memory: Memory::new(),
            io,
            jp: Vec::new(),
        }
    }

    pub fn step(&mut self) -> Result<Option<Vec<u8>>, BrainfuckError> {
        let instruction = match self.tape.get_instruction() {
            Ok(i) => i,
            Err(_) => return Ok(Some(self.io.output())),
        };
        match instruction {
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


pub struct InterpreterOutput(Vec<u8>);
impl InterpreterOutput {
    pub fn new(v: Vec<u8>) -> Self { InterpreterOutput(v) }

    pub fn to_string(&self) -> String {
        String::from_iter(self.0.iter().map(|byte| *byte as char))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

pub fn eval_string(s: &'static str, input: Option<Vec<u8>>) -> Result<InterpreterOutput, BrainfuckError> {
    let mut interpreter = Interpreter::new(Tape::from_string(s)?, input);
    interpreter.eval()
}

pub fn eval_file(path: PathBuf,  input: Option<Vec<u8>>) -> Result<InterpreterOutput, BrainfuckError> {
    Ok(InterpreterOutput::new(Vec::new()))
}