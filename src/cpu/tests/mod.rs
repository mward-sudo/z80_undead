use super::*;
use crate::{
    cpu::{Cpu, Flags},
    memory::Memory,
    EmulatorError,
};

/// Test fixture for CPU validation tests
pub struct CpuTestFixture {
    pub cpu: Cpu,
    pub initial_state: CpuState,
}

#[derive(Debug, Clone)]
pub struct CpuState {
    pub registers: Registers,
    pub flags: Flags,
    pub pc: u16,
    pub sp: u16,
    pub cycles: u32,
}

impl CpuTestFixture {
    pub fn new() -> Self {
        let cpu = Cpu::new(Memory::new());
        let initial_state = CpuState {
            registers: cpu.registers.clone(),
            flags: cpu.flags.clone(),
            pc: cpu.get_pc(),
            sp: cpu.get_sp(),
            cycles: cpu.cycles,
        };

        Self { cpu, initial_state }
    }

    pub fn load_and_run(&mut self, program: &[u8]) -> Result<(), EmulatorError> {
        self.cpu.load_program(0, program)?;
        self.cpu.step()?;
        Ok(())
    }

    pub fn verify_state(&self, expected: CpuState) -> bool {
        self.cpu.registers == expected.registers
            && self.cpu.flags == expected.flags
            && self.cpu.get_pc() == expected.pc
            && self.cpu.get_sp() == expected.sp
            && self.cpu.cycles == expected.cycles
    }
}
