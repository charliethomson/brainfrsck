use crate::{
    instruction::Instruction,
    error::BrainfuckError,
};
use std::string::ToString;
use std::fmt;
use std::fmt::Debug;



const MEMORY_SIZE: usize = 30_000;



#[derive(Debug)]
pub struct Tape {
    stream: Vec<Instruction>,
    ptr: usize,
} impl Tape {
    pub fn new(mut stream: Vec<Instruction>) -> Self {
        if !stream.ends_with(&[Instruction::EOF]) {
            stream.push(Instruction::EOF);
        }
        Tape {
            stream,
            ptr: 0usize,
        }
    }

    pub fn from_string<S: ToString>(s: S) -> Result<Self, BrainfuckError> {

        // TODO: This needs cleaned, is ugly <3
        let mut stream: Vec<Instruction> = s.to_string()
            .chars()
            .map(|c| Instruction::from_char(c))
            .collect();

        if !stream.ends_with(&[Instruction::EOF]) {
            stream.push(Instruction::EOF);
        }

        Ok(Self::new(stream))
    }

    pub fn get_instruction(&mut self) -> Result<Instruction, BrainfuckError> {
        self.ptr += 1;
        match self.stream.get(self.ptr-1) {
            Some(n) => Ok(*n),
            None => Err(BrainfuckError::new("Instruction pointer seems to have lost control :( (InstPtrOOBError)"))
        }
    }

    pub fn jump_back(&mut self) -> Result<(), BrainfuckError> {
        // Current state: self.ptr is one above the Jump instruction

        self.ptr -= 1;

        let mut depth = 0;

        while self.ptr != 0 {
            match self.stream.get(self.ptr) {
                Some(Instruction::SetJump) => { depth -= 1; },
                Some(Instruction::Jump) => { depth += 1; },
                _ => (),
            };

            if depth == 0 {
                return Ok(())
            } else {
                self.ptr -= 1;
            }
        }

        Err(BrainfuckError::new("Mismatched jump operations"))
    }

    pub fn jump_forward(&mut self) -> Result<(), BrainfuckError> {

        // We are currently on the instruction right after the SetJump, so initialise depth with 1
        let mut depth = 1;

        while self.ptr != self.stream.len() {
            match self.stream.get(self.ptr) {
                Some(Instruction::SetJump) => { depth += 1; },
                Some(Instruction::Jump) => { depth -= 1; },
                _ => ()
            };

            if depth == 0 {
                return Ok(());
            } else {
                self.ptr += 1;
            }
        }

        Err(BrainfuckError::new("Mismatched jump operations"))
    }

} impl Default for Tape {
    fn default() -> Self {
        Self::new(Vec::new())
    }
} impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tape - ptr {}; stream {:?};", self.ptr, self.stream)
    }
}

/// Component of the interpreter that handles virtual memory management (
pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
    ptr: usize,
} impl Memory {

    /// Constructor for the Memory component
    pub fn new() -> Self {
        let bytes = [0u8; MEMORY_SIZE];
        Memory {
            bytes,
            ptr: 0usize
        }
    }

    /// Called when the `.` instruction is encountered.
    /// Attempts to pull the value at the data pointer from memory
    /// _Should_ never panic, will panic if the data pointer is greater than 30,000, however, the
    /// data pointer should never go higher than that (the program will return Err in `memory.inc_ptr()`
    /// if the data pointer will be higher than 30,000)
    pub fn pull(&self) -> u8 {
        self.bytes[self.ptr]
    }

    /// Called when the `,` instruction is encountered
    /// Attempts to push the value into the memory location at the data pointer
    /// See `memory.pull()` for safety
    pub fn push(&mut self, value: u8) {
        self.bytes[self.ptr] = value;
    }

    /// Called when the `>` instruction is encountered
    /// Attempts to increment the data pointer
    /// Returns `Err` if the data pointer would pass the end of memory if incremented
    pub fn inc_ptr(&mut self) -> Result<(), BrainfuckError> {
        if self.ptr == MEMORY_SIZE {
            Err(BrainfuckError::new("Attempt to increment data pointer past the end of memory"))
        } else {
            self.ptr += 1;
            Ok(())
        }
    }

    /// Called when the `<` instruction is encountered
    /// Attempts to decrement the data pointer
    /// Returns `Err` if the data pointer is zero
    pub fn dec_ptr(&mut self) {
        if self.ptr != 0 {
            self.ptr -= 1;
        }
    }

    /// Called when the `+` instruction is encountered
    /// Attempts to increment the data pointer
    /// Returns `Err` if the data pointer would pass the end of memory if incremented
    pub fn inc_val(&mut self) -> Result<(), BrainfuckError> {
        if self.ptr == MEMORY_SIZE {
            Err(BrainfuckError::new("Attempt to increment data pointer past the end of memory"))
        } else {
            self.bytes[self.ptr] = self.bytes[self.ptr].wrapping_add(1);
            Ok(())
        }
    }

    /// Called when the `-` instruction is encountered
    /// Attempts to decrement the data pointer
    /// Returns `Err` if the data pointer is zero
    pub fn dec_val(&mut self) {
        self.bytes[self.ptr] = self.bytes[self.ptr].wrapping_sub(1);
    }
} impl Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory {{\n\tbytes: {:?},\n\tptr: {}\n}}", self.bytes.iter().map(|v| *v).collect::<Vec<u8>>(), self.ptr)
    }
}

/// """Wrapper""" for the Memory component of the interpreter
/// Handles the actual IO and such of the interpreter
#[derive(Debug)]
pub struct IO {
    input: Vec<u8>,
    output: Vec<u8>,
} impl IO {

    /// Constructor for the IO component
    pub fn new(input: Vec<u8>) -> Self {
        IO {
            input,
            output: Vec::new(),
        }
    }

    /// Write a single byte from the front of `io.input` into `memory`
    pub fn write_to(&mut self, memory: &mut Memory) {
        if self.input.len() != 0 {
            memory.push(self.input.remove(0));
        }
    }

    /// Write a single byte from `memory` onto the back of `io.output`
    pub fn pull_from(&mut self, memory: &mut Memory) {
        self.output.push(memory.pull());
    }

    pub fn output(&self) -> Vec<u8> {
        self.output.clone()
    }

} impl Default for IO {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}