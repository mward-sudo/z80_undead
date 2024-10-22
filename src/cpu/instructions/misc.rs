use crate::cpu::Cpu;

impl Cpu {
    pub fn nop(&self) {
        // No operation
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn di(&mut self) {
        self.iff1 = false;
        self.iff2 = false;
    }

    pub fn ei(&mut self) {
        self.iff1 = true;
        self.iff2 = true;
    }

    // Implement other miscellaneous instructions (EX, DAA, CPL, CCF, SCF, etc.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halt() {
        let mut cpu = Cpu::new();
        cpu.halt();
        assert!(cpu.halted);
    }
}
