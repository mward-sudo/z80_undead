//! CPU module handles Z80 CPU emulation including registers, flags, and instruction execution.

mod instruction;

use crate::{memory::Memory, Result};
use instruction::create_nop;

/// Represents the Z80 CPU state
pub struct Cpu {
    // Program Counter
    pc: u16,
    // Stack Pointer
    sp: u16,
    // Memory reference
    memory: Memory,
}

impl Cpu {
    /// Creates a new CPU instance with initialized memory
    pub fn new(memory: Memory) -> Self {
        Self {
            pc: 0,
            sp: 0xFFFF, // Stack starts at top of memory
            memory,
        }
    }

    /// Executes a single instruction
    pub fn step(&mut self) -> Result<()> {
        // Fetch
        let opcode = self.memory.read_byte(self.pc)?;

        // Decode
        let instruction = match opcode {
            0x00 => create_nop(),
            _ => return Err(crate::EmulatorError::InvalidOpcode(opcode)),
        };

        // Execute
        (instruction.execute)(self)?;

        // Increment PC
        self.pc = self.pc.wrapping_add(instruction.length as u16);

        Ok(())
    }

    /// Returns the current program counter value
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Loads a program into memory at the specified address
    pub fn load_program(&mut self, address: u16, program: &[u8]) -> Result<()> {
        self.memory.load(address, program)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(Memory::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmulatorError;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::default();
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0xFFFF);
    }

    #[test]
    fn test_nop_execution() {
        let mut cpu = Cpu::default();
        let program = [0x00]; // NOP instruction
        cpu.load_program(0, &program).unwrap();

        let initial_pc = cpu.get_pc();
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), initial_pc + 1);
    }

    #[test]
    fn test_invalid_opcode() {
        let mut cpu = Cpu::default();
        let program = [0xFF]; // Invalid opcode
        cpu.load_program(0, &program).unwrap();

        let result = cpu.step();
        assert!(matches!(result, Err(EmulatorError::InvalidOpcode(0xFF))));
    }

    #[test]
    fn test_program_loading() {
        let mut cpu = Cpu::default();
        let program = [0x00, 0x01, 0x02];
        let address = 0x100;

        cpu.load_program(address, &program).unwrap();
        assert_eq!(cpu.memory.read_byte(address).unwrap(), 0x00);
        assert_eq!(cpu.memory.read_byte(address + 1).unwrap(), 0x01);
        assert_eq!(cpu.memory.read_byte(address + 2).unwrap(), 0x02);
    }
}
