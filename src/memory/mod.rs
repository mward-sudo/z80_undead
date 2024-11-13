//! Memory module handles memory management and addressing.

use crate::Result;

const MEMORY_SIZE: usize = 0x10000; // 64KB memory space

/// Represents the memory management unit
pub struct Memory {
    ram: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates a new Memory instance
    pub fn new() -> Self {
        Self {
            ram: vec![0; MEMORY_SIZE],
        }
    }

    /// Reads a byte from memory
    pub fn read_byte(&self, address: u16) -> Result<u8> {
        Ok(self.ram[address as usize])
    }

    /// Writes a byte to memory
    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<()> {
        self.ram[address as usize] = value;
        Ok(())
    }

    /// Loads data into memory at specified address
    pub fn load(&mut self, address: u16, data: &[u8]) -> Result<()> {
        let start = address as usize;
        let end = start + data.len();

        if end > MEMORY_SIZE {
            return Err(crate::EmulatorError::MemoryError(address));
        }

        self.ram[start..end].copy_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmulatorError;

    #[test]
    fn test_memory_initialization() {
        let memory = Memory::new();
        assert_eq!(memory.ram.len(), MEMORY_SIZE);
        assert!(memory.ram.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn test_read_write_byte() {
        let mut memory = Memory::new();
        let address = 0x1234;
        let value = 0x42;

        memory.write_byte(address, value).unwrap();
        assert_eq!(memory.read_byte(address).unwrap(), value);
    }

    #[test]
    fn test_load_program() {
        let mut memory = Memory::new();
        let program = [0x00, 0x01, 0x02, 0x03];
        let address = 0x1000;

        memory.load(address, &program).unwrap();

        for (i, &byte) in program.iter().enumerate() {
            assert_eq!(memory.read_byte(address + i as u16).unwrap(), byte);
        }
    }

    #[test]
    fn test_load_program_overflow() {
        let mut memory = Memory::new();
        let program = [0x00; MEMORY_SIZE + 1];
        let result = memory.load(0, &program);
        assert!(matches!(result, Err(EmulatorError::MemoryError(_))));
    }
}
