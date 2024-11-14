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
