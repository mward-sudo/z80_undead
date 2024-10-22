use super::*;

impl Cpu {
    pub fn jp(&mut self, condition: bool, address: u16) {
        if condition {
            self.pc = address;
        }
    }

    pub fn jr(&mut self, condition: bool, offset: i8) {
        if condition {
            self.pc = self.pc.wrapping_add(offset as u16);
        }
    }

    pub fn call(&mut self, condition: bool, address: u16) {
        if condition {
            self.sp = self.sp.wrapping_sub(2);
            self.write_word(self.sp, self.pc);
            self.pc = address;
        }
    }

    pub fn ret(&mut self, condition: bool) {
        if condition {
            self.pc = self.read_word(self.sp);
            self.sp = self.sp.wrapping_add(2);
        }
    }

    pub fn rst(&mut self, address: u8) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, self.pc);
        self.pc = address as u16;
    }

    pub fn djnz(&mut self, offset: i8) {
        self.b = self.b.wrapping_sub(1);
        if self.b != 0 {
            self.pc = self.pc.wrapping_add(offset as u16);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jp() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1000;
        cpu.jp(true, 0x2000);
        assert_eq!(cpu.pc, 0x2000);

        cpu.pc = 0x1000;
        cpu.jp(false, 0x2000);
        assert_eq!(cpu.pc, 0x1000);
    }

    #[test]
    fn test_jr() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1000;
        cpu.jr(true, 10);
        assert_eq!(cpu.pc, 0x100A);

        cpu.pc = 0x1000;
        cpu.jr(true, -10);
        assert_eq!(cpu.pc, 0x0FF6);

        cpu.pc = 0x1000;
        cpu.jr(false, 10);
        assert_eq!(cpu.pc, 0x1000);
    }

    #[test]
    fn test_call_and_ret() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1000;
        cpu.sp = 0xFFFF;
        cpu.call(true, 0x2000);
        assert_eq!(cpu.pc, 0x2000);
        assert_eq!(cpu.sp, 0xFFFD);
        assert_eq!(cpu.read_word(cpu.sp), 0x1000);

        cpu.ret(true);
        assert_eq!(cpu.pc, 0x1000);
        assert_eq!(cpu.sp, 0xFFFF);
    }

    #[test]
    fn test_rst() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1000;
        cpu.sp = 0xFFFF;
        cpu.rst(0x08);
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(cpu.sp, 0xFFFD);
        assert_eq!(cpu.read_word(cpu.sp), 0x1000);
    }

    #[test]
    fn test_djnz() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x1000;
        cpu.b = 2;
        cpu.djnz(10);
        assert_eq!(cpu.pc, 0x100A);
        assert_eq!(cpu.b, 1);

        cpu.djnz(10);
        assert_eq!(cpu.pc, 0x100A);
        assert_eq!(cpu.b, 0);
    }

    #[test]
    fn test_halt() {
        let mut cpu = Cpu::new();
        cpu.halt();
        assert!(cpu.halted);
    }
}
