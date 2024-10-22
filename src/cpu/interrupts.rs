use super::Cpu;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptMode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
}

impl Cpu {
    /// Handle non-maskable interrupt (NMI)
    pub fn handle_nmi(&mut self) {
        if self.halted {
            self.halted = false;
            self.pc = self.pc.wrapping_add(1);
        }

        self.iff2 = self.iff1;
        self.iff1 = false;

        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, self.pc);
        self.pc = 0x0066; // NMI handler address
    }

    /// Handle maskable interrupt (INT)
    pub fn handle_interrupt(&mut self) {
        if !self.iff1 {
            return;
        }

        if self.halted {
            self.halted = false;
            self.pc = self.pc.wrapping_add(1);
        }

        self.iff1 = false;
        self.iff2 = false;

        match self.interrupt_mode {
            0 => self.handle_interrupt_mode0(),
            1 => self.handle_interrupt_mode1(),
            2 => self.handle_interrupt_mode2(),
            _ => panic!("Invalid interrupt mode"),
        }
    }

    fn handle_interrupt_mode0(&mut self) {
        // In mode 0, the interrupting device places an instruction on the data bus
        // For simulation, we'll just call RST 38H as that's what most devices did
        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, self.pc);
        self.pc = 0x0038;
    }

    fn handle_interrupt_mode1(&mut self) {
        // Mode 1 is simple: just execute RST 38H
        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, self.pc);
        self.pc = 0x0038;
    }

    fn handle_interrupt_mode2(&mut self) {
        // Mode 2 uses the I register and the vector supplied by the interrupting device
        // For simulation, we'll use 0xFF as the vector (common value)
        let vector = 0xFF;
        let address = ((self.i as u16) << 8) | (vector as u16);
        let jump_address = self.read_word(address);

        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, self.pc);
        self.pc = jump_address;
    }

    /// Enable interrupts
    pub fn ei(&mut self) {
        self.iff1 = true;
        self.iff2 = true;
    }

    /// Disable interrupts
    pub fn di(&mut self) {
        self.iff1 = false;
        self.iff2 = false;
    }

    /// Set interrupt mode
    pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
        self.interrupt_mode = mode as u8;
    }

    /// Return from non-maskable interrupt
    pub fn retn(&mut self) {
        self.pc = self.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        self.iff1 = self.iff2;
    }

    /// Return from maskable interrupt
    pub fn reti(&mut self) {
        self.pc = self.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        // Some sources say RETI also enables interrupts
        self.iff1 = true;
        self.iff2 = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nmi() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.sp = 0x2000;
        cpu.iff1 = true;
        cpu.iff2 = true;

        cpu.handle_nmi();

        assert_eq!(cpu.pc, 0x0066);
        assert_eq!(cpu.read_word(0x1FFE), 0x1234);
        assert_eq!(cpu.sp, 0x1FFE);
        assert!(!cpu.iff1);
        assert!(cpu.iff2);
    }

    #[test]
    fn test_interrupt_mode1() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.sp = 0x2000;
        cpu.iff1 = true;
        cpu.interrupt_mode = InterruptMode::Mode1 as u8;

        cpu.handle_interrupt();

        assert_eq!(cpu.pc, 0x0038);
        assert_eq!(cpu.read_word(0x1FFE), 0x1234);
        assert_eq!(cpu.sp, 0x1FFE);
        assert!(!cpu.iff1);
        assert!(!cpu.iff2);
    }

    #[test]
    fn test_interrupt_mode2() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.sp = 0x2000;
        cpu.iff1 = true;
        cpu.i = 0x20;
        cpu.interrupt_mode = InterruptMode::Mode2 as u8;
        cpu.write_word(0x20FF, 0x4567); // Vector table entry

        cpu.handle_interrupt();

        assert_eq!(cpu.pc, 0x4567);
        assert_eq!(cpu.read_word(0x1FFE), 0x1234);
        assert_eq!(cpu.sp, 0x1FFE);
        assert!(!cpu.iff1);
        assert!(!cpu.iff2);
    }

    #[test]
    fn test_ei_di() {
        let mut cpu = Cpu::new();

        cpu.di();
        assert!(!cpu.iff1);
        assert!(!cpu.iff2);

        cpu.ei();
        assert!(cpu.iff1);
        assert!(cpu.iff2);
    }

    #[test]
    fn test_retn() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1FFE;
        cpu.write_word(0x1FFE, 0x1234);
        cpu.iff2 = true;
        cpu.iff1 = false;

        cpu.retn();

        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0x2000);
        assert!(cpu.iff1);
    }

    #[test]
    fn test_reti() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1FFE;
        cpu.write_word(0x1FFE, 0x1234);
        cpu.iff1 = false;
        cpu.iff2 = false;

        cpu.reti();

        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0x2000);
        assert!(cpu.iff1);
        assert!(cpu.iff2);
    }

    #[test]
    fn test_interrupt_disabled() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.sp = 0x2000;
        cpu.iff1 = false;
        cpu.interrupt_mode = InterruptMode::Mode1 as u8;

        cpu.handle_interrupt();

        // Nothing should change when interrupts are disabled
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.sp, 0x2000);
    }

    #[test]
    fn test_interrupt_from_halt() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1234;
        cpu.sp = 0x2000;
        cpu.iff1 = true;
        cpu.halted = true;
        cpu.interrupt_mode = InterruptMode::Mode1 as u8;

        cpu.handle_interrupt();

        assert_eq!(cpu.pc, 0x0038);
        assert!(!cpu.halted);
    }
}
