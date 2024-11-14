//! Instruction module handles Z80 instruction metadata and execution.

use super::Cpu;
use crate::Result;
use std::fmt;

/// Function type for instruction execution
pub type ExecuteFn = fn(&mut Cpu) -> Result<()>;

/// Represents a Z80 instruction's metadata and execution function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    // Basic instruction types
    Load,       // Load/transfer instructions
    Arithmetic, // Arithmetic operations
    Logic,      // Logical operations
    Rotate,     // Rotation/shift operations
    BitManip,   // Bit manipulation
    Jump,       // Jump instructions
    Call,       // Subroutine calls
    Return,     // Return instructions
    IO,         // Input/Output operations
    Control,    // CPU control instructions
    Exchange,   // Register exchange
    Block,      // Block transfer/search
    Special,    // Special/misc instructions
}

#[derive(Debug)]
pub struct Instruction {
    /// Number of T-states (clock cycles) the instruction takes
    pub cycles: u8,
    /// Length of the instruction in bytes
    pub length: u8,
    /// Function pointer to the instruction's execution code
    pub execute: ExecuteFn,
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
}

impl Instruction {
    /// Creates a new instruction with the specified metadata
    pub fn new(
        cycles: u8,
        length: u8,
        execute: ExecuteFn,
        mnemonic: &'static str,
        instruction_type: InstructionType,
    ) -> Self {
        Self {
            cycles,
            length,
            execute,
            mnemonic,
            instruction_type,
        }
    }
}

/// Creates the NOP instruction metadata
pub fn create_nop() -> Instruction {
    Instruction::new(
        4,
        1,
        |cpu| {
            // NOP does nothing but advance PC, which is handled by the CPU
            Ok(())
        },
        "NOP",
        InstructionType::Control,
    )
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.mnemonic, self.instruction_type)
    }
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load => write!(f, "Load"),
            Self::Arithmetic => write!(f, "Arithmetic"),
            Self::Logic => write!(f, "Logic"),
            Self::Rotate => write!(f, "Rotate"),
            Self::BitManip => write!(f, "Bit Manipulation"),
            Self::Jump => write!(f, "Jump"),
            Self::Call => write!(f, "Call"),
            Self::Return => write!(f, "Return"),
            Self::IO => write!(f, "I/O"),
            Self::Control => write!(f, "Control"),
            Self::Exchange => write!(f, "Exchange"),
            Self::Block => write!(f, "Block"),
            Self::Special => write!(f, "Special"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn test_instruction_creation() {
        let nop = create_nop();
        assert_eq!(nop.cycles, 4);
        assert_eq!(nop.length, 1);
    }

    #[test]
    fn test_nop_execution() {
        let mut cpu = Cpu::new(Memory::new());
        let nop = create_nop();

        // Execute NOP instruction
        (nop.execute)(&mut cpu).unwrap();

        // NOP should not affect any CPU state except PC
        // PC advancement is handled by the CPU step function
    }

    #[test]
    fn test_instruction_display() {
        let instruction = Instruction::new(
            4,                        // cycles
            1,                        // length
            |_cpu| Ok(()),            // execute fn
            "NOP",                    // mnemonic
            InstructionType::Control, // instruction type
        );
        assert_eq!(instruction.to_string(), "NOP (Control)");
    }

    #[test]
    fn test_instruction_type_display() {
        assert_eq!(InstructionType::Load.to_string(), "Load");
        assert_eq!(InstructionType::Arithmetic.to_string(), "Arithmetic");
        assert_eq!(InstructionType::BitManip.to_string(), "Bit Manipulation");
    }
}
