use crate::cpu::flags::*;
use crate::cpu::Cpu;

impl Cpu {
    pub fn ldi(&mut self) {
        let value = self.read_byte(self.get_hl());
        self.write_byte(self.get_de(), value);
        self.set_hl(self.get_hl().wrapping_add(1));
        self.set_de(self.get_de().wrapping_add(1));
        self.set_bc(self.get_bc().wrapping_sub(1));

        let n = self.a.wrapping_add(value);
        self.set_flag(FLAG_H | FLAG_N, false);
        self.set_flag(FLAG_PV, self.get_bc() != 0);
        self.set_flag(FLAG_Y, n & 0x02 != 0);
        self.set_flag(FLAG_X, n & 0x08 != 0);
    }

    pub fn ldir(&mut self) {
        loop {
            self.ldi();
            if self.get_bc() == 0 {
                break;
            }
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    pub fn ldd(&mut self) {
        let value = self.read_byte(self.get_hl());
        self.write_byte(self.get_de(), value);
        self.set_hl(self.get_hl().wrapping_sub(1));
        self.set_de(self.get_de().wrapping_sub(1));
        self.set_bc(self.get_bc().wrapping_sub(1));

        let n = self.a.wrapping_add(value);
        self.set_flag(FLAG_H | FLAG_N, false);
        self.set_flag(FLAG_PV, self.get_bc() != 0);
        self.set_flag(FLAG_Y, n & 0x02 != 0);
        self.set_flag(FLAG_X, n & 0x08 != 0);
    }

    pub fn lddr(&mut self) {
        loop {
            self.ldd();
            if self.get_bc() == 0 {
                break;
            }
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    pub fn cpi(&mut self) {
        let value = self.read_byte(self.get_hl());
        let result = self.a.wrapping_sub(value);
        self.set_hl(self.get_hl().wrapping_add(1));
        self.set_bc(self.get_bc().wrapping_sub(1));

        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (self.a & 0x0F) < (value & 0x0F));
        self.set_flag(FLAG_PV, self.get_bc() != 0);
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Y, result & 0x02 != 0);
        self.set_flag(FLAG_X, result & 0x08 != 0);
    }

    pub fn cpir(&mut self) {
        loop {
            self.cpi();
            if self.get_flag(FLAG_Z) || self.get_bc() == 0 {
                break;
            }
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    pub fn cpd(&mut self) {
        let value = self.read_byte(self.get_hl());
        let result = self.a.wrapping_sub(value);
        self.set_hl(self.get_hl().wrapping_sub(1));
        self.set_bc(self.get_bc().wrapping_sub(1));

        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (self.a & 0x0F) < (value & 0x0F));
        self.set_flag(FLAG_PV, self.get_bc() != 0);
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Y, result & 0x02 != 0);
        self.set_flag(FLAG_X, result & 0x08 != 0);
    }

    pub fn cpdr(&mut self) {
        loop {
            self.cpd();
            if self.get_flag(FLAG_Z) || self.get_bc() == 0 {
                break;
            }
            self.pc = self.pc.wrapping_sub(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldi() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1000);
        cpu.set_de(0x2000);
        cpu.set_bc(0x0003);
        cpu.a = 0x10;
        cpu.write_byte(0x1000, 0x42);

        cpu.ldi();

        assert_eq!(cpu.read_byte(0x2000), 0x42);
        assert_eq!(cpu.get_hl(), 0x1001);
        assert_eq!(cpu.get_de(), 0x2001);
        assert_eq!(cpu.get_bc(), 0x0002);
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_ldir() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1000);
        cpu.set_de(0x2000);
        cpu.set_bc(0x0003);
        cpu.a = 0x10;
        cpu.write_byte(0x1000, 0x42);
        cpu.write_byte(0x1001, 0x43);
        cpu.write_byte(0x1002, 0x44);

        cpu.ldir();

        assert_eq!(cpu.read_byte(0x2000), 0x42);
        assert_eq!(cpu.read_byte(0x2001), 0x43);
        assert_eq!(cpu.read_byte(0x2002), 0x44);
        assert_eq!(cpu.get_hl(), 0x1003);
        assert_eq!(cpu.get_de(), 0x2003);
        assert_eq!(cpu.get_bc(), 0x0000);
        assert!(!cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_ldd() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1002);
        cpu.set_de(0x2002);
        cpu.set_bc(0x0003);
        cpu.a = 0x10;
        cpu.write_byte(0x1002, 0x42);

        cpu.ldd();

        assert_eq!(cpu.read_byte(0x2002), 0x42);
        assert_eq!(cpu.get_hl(), 0x1001);
        assert_eq!(cpu.get_de(), 0x2001);
        assert_eq!(cpu.get_bc(), 0x0002);
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_lddr() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1002);
        cpu.set_de(0x2002);
        cpu.set_bc(0x0003);
        cpu.a = 0x10;
        cpu.write_byte(0x1000, 0x42);
        cpu.write_byte(0x1001, 0x43);
        cpu.write_byte(0x1002, 0x44);

        cpu.lddr();

        assert_eq!(cpu.read_byte(0x2000), 0x42);
        assert_eq!(cpu.read_byte(0x2001), 0x43);
        assert_eq!(cpu.read_byte(0x2002), 0x44);
        assert_eq!(cpu.get_hl(), 0x0FFF);
        assert_eq!(cpu.get_de(), 0x1FFF);
        assert_eq!(cpu.get_bc(), 0x0000);
        assert!(!cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_cpi() {
        let mut cpu = Cpu::new();
        cpu.a = 0x42;
        cpu.set_hl(0x1000);
        cpu.set_bc(0x0003);
        cpu.write_byte(0x1000, 0x42);

        cpu.cpi();

        assert_eq!(cpu.get_hl(), 0x1001);
        assert_eq!(cpu.get_bc(), 0x0002);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_cpir() {
        let mut cpu = Cpu::new();
        cpu.a = 0x42;
        cpu.set_hl(0x1000);
        cpu.set_bc(0x0003);
        cpu.write_byte(0x1000, 0x41);
        cpu.write_byte(0x1001, 0x42);

        cpu.cpir();

        assert_eq!(cpu.get_hl(), 0x1002);
        assert_eq!(cpu.get_bc(), 0x0001);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_cpd() {
        let mut cpu = Cpu::new();
        cpu.a = 0x42;
        cpu.set_hl(0x1002);
        cpu.set_bc(0x0003);
        cpu.write_byte(0x1002, 0x42);

        cpu.cpd();

        assert_eq!(cpu.get_hl(), 0x1001);
        assert_eq!(cpu.get_bc(), 0x0002);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_cpdr() {
        let mut cpu = Cpu::new();
        cpu.a = 0x42;
        cpu.set_hl(0x1002);
        cpu.set_bc(0x0003);
        cpu.write_byte(0x1000, 0x42);
        cpu.write_byte(0x1001, 0x41);
        cpu.write_byte(0x1002, 0x40);

        cpu.cpdr();

        assert_eq!(cpu.get_hl(), 0x0FFF);
        assert_eq!(cpu.get_bc(), 0x0000);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_PV));
    }
}
