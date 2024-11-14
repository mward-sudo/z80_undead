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
    // Flags register
    flags: Flags,
    // Memory reference
    memory: Memory,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Flags {
    // Primary flags
    pub sign: bool,         // S: Bit 7 - Set if result is negative
    pub zero: bool,         // Z: Bit 6 - Set if result is zero
    pub half_carry: bool,   // H: Bit 4 - Set if carry from bit 3 to 4
    pub parity: bool,       // P/V: Bit 2 - Parity/Overflow flag
    pub add_subtract: bool, // N: Bit 1 - Set if last op was subtraction
    pub carry: bool,        // C: Bit 0 - Set if result overflowed
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    // Convert flags to a byte representation
    pub fn to_byte(&self) -> u8 {
        let mut result = 0u8;
        if self.sign {
            result |= 0b1000_0000;
        }
        if self.zero {
            result |= 0b0100_0000;
        }
        if self.half_carry {
            result |= 0b0001_0000;
        }
        if self.parity {
            result |= 0b0000_0100;
        }
        if self.add_subtract {
            result |= 0b0000_0010;
        }
        if self.carry {
            result |= 0b0000_0001;
        }
        result
    }

    // Set flags from a byte
    pub fn from_byte(&mut self, byte: u8) {
        self.sign = (byte & 0b1000_0000) != 0;
        self.zero = (byte & 0b0100_0000) != 0;
        self.half_carry = (byte & 0b0001_0000) != 0;
        self.parity = (byte & 0b0000_0100) != 0;
        self.add_subtract = (byte & 0b0000_0010) != 0;
        self.carry = (byte & 0b0000_0001) != 0;
    }
}

impl Cpu {
    /// Creates a new CPU instance with initialized memory
    pub fn new(memory: Memory) -> Self {
        Self {
            pc: 0,
            sp: 0xFFFF, // Stack starts at top of memory
            flags: Flags::default(),
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

    #[test]
    fn test_flags_to_byte() {
        let mut flags = Flags::new();
        flags.sign = true;
        flags.zero = true;
        flags.carry = true;
        assert_eq!(flags.to_byte(), 0b1100_0001);
    }

    #[test]
    fn test_flags_from_byte() {
        let mut flags = Flags::new();
        flags.from_byte(0b1100_0001);
        assert!(flags.sign);
        assert!(flags.zero);
        assert!(flags.carry);
        assert!(!flags.half_carry);
        assert!(!flags.parity);
        assert!(!flags.add_subtract);
    }

    #[test]
    fn test_arithmetic_flags() {
        let mut cpu = Cpu::new(Memory::default());

        // Test addition with carry
        cpu.update_arithmetic_flags(0x7F, 0x01, false, true);
        assert!(cpu.flags.sign);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.carry);
        assert!(cpu.flags.parity); // Overflow from positive to negative

        // Test subtraction with borrow
        cpu.update_arithmetic_flags(0x00, 0x01, false, false);
        assert!(cpu.flags.sign);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.parity);
    }
}
