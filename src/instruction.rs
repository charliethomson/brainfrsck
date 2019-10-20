

use crate::error::BrainfuckError;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Output,
    Input,
    SetJump,
    Jump,
    EOF,
    NOP,
    Debug,
} impl Instruction {
    pub fn from_char(instruction: char) -> Self {
        match instruction {
            '>' => Instruction::IncPtr,
            '<' => Instruction::DecPtr,
            '+' => Instruction::IncVal,
            '-' => Instruction::DecVal,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => Instruction::SetJump,
            ']' => Instruction::Jump,
            ';' => Instruction::EOF,
            '#' => Instruction::Debug,
            _ => Instruction::NOP,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Instruction::IncPtr => '>',
            Instruction::DecPtr => '<',
            Instruction::IncVal => '+',
            Instruction::DecVal => '-',
            Instruction::Output => '.',
            Instruction::Input => ',',
            Instruction::SetJump => '[',
            Instruction::Jump => ']',
            Instruction::EOF => ';',
            Instruction::NOP => 'N',
            Instruction::Debug => '#',
        }
    }
} impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}