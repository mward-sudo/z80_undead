//! System module handles the integration between CPU, memory, and I/O devices.

use crate::{cpu::Cpu, memory::Memory, Result};

/// Represents the system bus and coordinates component interaction
pub struct System {
    cpu: Cpu,
}

impl System {
    /// Creates a new System instance
    pub fn new() -> Self {
        let memory = Memory::new();
        let cpu = Cpu::new(memory);

        Self { cpu }
    }

    /// Executes one system tick
    pub fn tick(&mut self) -> Result<()> {
        self.cpu.step()
    }

    /// Loads a program into memory
    pub fn load_program(&mut self, program: &[u8]) -> Result<()> {
        // For now, always load at address 0
        // TODO: Implement proper program loading logic
        self.cpu.load_program(0, program)
    }
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmulatorError;

    #[test]
    fn test_system_initialization() {
        let system = System::default();
        assert_eq!(system.cpu.get_pc(), 0);
    }

    #[test]
    fn test_program_execution() {
        let mut system = System::default();
        let program = [0x00]; // NOP instruction

        system.load_program(&program).unwrap();
        system.tick().unwrap();
    }

    #[test]
    fn test_invalid_program() {
        let mut system = System::default();
        let program = [0xFF]; // Invalid opcode

        system.load_program(&program).unwrap();
        let result = system.tick();
        assert!(matches!(result, Err(EmulatorError::InvalidOpcode(0xFF))));
    }
}
