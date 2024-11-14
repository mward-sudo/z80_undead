//! Instruction module handles Z80 instruction metadata and execution.

use super::Cpu;
use crate::Result;

/// Function type for instruction execution
pub type ExecuteFn = fn(&mut Cpu) -> Result<()>;

/// Represents a Z80 instruction's metadata and execution function
#[derive(Debug)]
pub struct Instruction {
    /// Number of T-states (clock cycles) the instruction takes
    pub cycles: u8,
    /// Length of the instruction in bytes
    pub length: u8,
    /// Function pointer to the instruction's execution code
    pub execute: ExecuteFn,
}

impl Instruction {
    /// Creates a new instruction with the specified metadata
    pub fn new(cycles: u8, length: u8, execute: ExecuteFn) -> Self {
        Self {
            cycles,
            length,
            execute,
        }
    }
}

/// Creates the NOP instruction metadata
pub fn create_nop() -> Instruction {
    Instruction::new(4, 1, |cpu| {
        // NOP does nothing but advance PC, which is handled by the CPU
        Ok(())
    })
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
}
