
//! Holds support for the `Instruction` enum

use std::fmt;


/// Provides an interface for different instructions.
/// Essentially a front end enum for this mapping: 
/// - IncPtr  => '>'
/// - DecPtr  => '<'
/// - IncVal  => '+'
/// - DecVal  => '-'
/// - Output  => '.'
/// - Input   => ','
/// - SetJump => '['
/// - Jump    => ']'
/// - EOF     => ';'
/// - Debug   => '#'
/// - NOP     => Any other character
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

    /// Gets an `Instruction` from a `char`
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

    /// Gets a `char` from an `Instruction`
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