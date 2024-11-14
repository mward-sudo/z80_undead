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
    /// Length of the instruction in bytes
    pub length: u8,
    /// Function pointer to the instruction's execution code
    pub execute: ExecuteFn,
    pub mnemonic: &'static str,
    pub instruction_type: InstructionType,
    /// Number of T-states (clock cycles) the instruction takes
    pub t_states: u32,
}

impl Instruction {
    /// Creates a new instruction with the specified metadata
    pub fn new(
        length: u8,
        execute: ExecuteFn,
        mnemonic: &'static str,
        instruction_type: InstructionType,
        t_states: u32,
    ) -> Self {
        Self {
            length,
            execute,
            mnemonic,
            instruction_type,
            t_states,
        }
    }
}

/// Creates the NOP instruction metadata
pub fn create_nop() -> Instruction {
    Instruction::new(
        1,                        // length
        |_cpu| Ok(()),            // execute fn
        "NOP",                    // mnemonic
        InstructionType::Control, // instruction type
        4,                        // t_states
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

pub trait FlagUtils {
    fn update_sign_flag(&mut self, result: u8);
    fn update_zero_flag(&mut self, result: u8);
    fn update_parity_flag(&mut self, result: u8);
    fn update_carry_flag(&mut self, result: u16);
    fn update_half_carry_flag(&mut self, a: u8, b: u8, carry: bool);
    fn update_overflow_flag(&mut self, a: u8, b: u8, result: u8);
}

impl FlagUtils for Cpu {
    fn update_sign_flag(&mut self, result: u8) {
        self.flags.sign = (result & 0x80) != 0;
    }

    fn update_zero_flag(&mut self, result: u8) {
        self.flags.zero = result == 0;
    }

    fn update_parity_flag(&mut self, result: u8) {
        // Count number of 1 bits - if even, set parity flag
        let ones = result.count_ones();
        self.flags.parity = ones % 2 == 0;
    }

    fn update_carry_flag(&mut self, result: u16) {
        self.flags.carry = result > 0xFF;
    }

    fn update_half_carry_flag(&mut self, a: u8, b: u8, carry: bool) {
        // Half carry occurs on carry from bit 3 to 4
        let carry_in = if carry { 1 } else { 0 };
        self.flags.half_carry = (((a & 0x0F) + (b & 0x0F) + carry_in) & 0x10) != 0;
    }

    fn update_overflow_flag(&mut self, a: u8, b: u8, result: u8) {
        // Overflow occurs when:
        // - Adding two positives gives negative
        // - Adding two negatives gives positive
        let a_neg = (a & 0x80) != 0;
        let b_neg = (b & 0x80) != 0;
        let r_neg = (result & 0x80) != 0;
        self.flags.parity = (a_neg == b_neg) && (a_neg != r_neg);
    }
}

// Helper functions for common flag updates
impl Cpu {
    pub fn update_sz_flags(&mut self, result: u8) {
        self.update_sign_flag(result);
        self.update_zero_flag(result);
    }

    pub fn update_szp_flags(&mut self, result: u8) {
        self.update_sign_flag(result);
        self.update_zero_flag(result);
        self.update_parity_flag(result);
    }

    pub fn update_arithmetic_flags(&mut self, a: u8, b: u8, carry: bool, is_add: bool) {
        let result = if is_add {
            let r = a as u16 + b as u16 + if carry { 1 } else { 0 };
            self.update_carry_flag(r);
            self.update_half_carry_flag(a, b, carry);
            r as u8
        } else {
            let r = a as i16 - b as i16 - if carry { 1 } else { 0 };
            self.flags.carry = r < 0;
            self.update_half_carry_flag(a, !b, !carry);
            r as u8
        };

        self.flags.add_subtract = !is_add;
        self.update_szp_flags(result);

        if is_add {
            self.update_overflow_flag(a, b, result);
        } else {
            self.update_overflow_flag(a, !b, result);
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
            1,                        // length
            |_cpu| Ok(()),            // execute fn
            "NOP",                    // mnemonic
            InstructionType::Control, // instruction type
            4,                        // t_states
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
