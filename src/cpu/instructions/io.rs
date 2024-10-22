use super::*;
use crate::cpu::flags::*;

impl Cpu {
    pub fn in_a_n(&mut self) {
        let port = self.fetch_byte();
        self.in_r_c(Register::A, port);
    }

    pub fn in_r_c(&mut self, reg: Register, port: u8) -> u8 {
        let value = self.read_byte(0xFF00 | (port as u16));
        self.write_register(reg, value);

        // Update flags
        self.set_flag(FLAG_S, value & 0x80 != 0);
        self.set_flag(FLAG_Z, value == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, value.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);

        value
    }

    pub fn out_n_a(&mut self) {
        let port = self.fetch_byte();
        self.out_c_r(port, Register::A);
    }

    pub fn out_c_r(&mut self, port: u8, reg: Register) {
        let value = self.read_register(reg);
        // In a real system, this would write to an I/O device
        // For now, we'll simulate it by writing to a fixed memory location
        self.write_byte(0xFF00 | (port as u16), value);
    }

    pub fn ini(&mut self) {
        let port = self.c;
        let value = self.in_r_c(Register::A, port);
        let hl = self.get_hl();
        self.write_byte(hl, value);
        self.set_hl(hl.wrapping_add(1));
        self.b = self.b.wrapping_sub(1);

        self.set_flag(FLAG_Z, self.b == 0);
        self.set_flag(FLAG_N, true);
        let temp = value.wrapping_add((hl & 0xFF) as u8);
        self.set_flag(FLAG_H, temp < value);
        self.set_flag(FLAG_C, temp < value);
        self.set_flag(FLAG_PV, self.b != 0);
    }

    pub fn inir(&mut self) {
        while self.b != 0 {
            self.ini();
            self.c = self.c.wrapping_add(1);
            if self.b == 0 {
                self.set_flag(FLAG_PV, false);
                break;
            }
        }
    }

    pub fn outi(&mut self) {
        let hl = self.get_hl();
        let value = self.read_byte(hl);
        self.out_c_r(self.c, Register::B);
        self.set_hl(hl.wrapping_add(1));
        self.b = self.b.wrapping_sub(1);

        self.set_flag(FLAG_Z, self.b == 0);
        self.set_flag(FLAG_N, true);
        let l = (hl & 0xFF) as u8;
        self.set_flag(FLAG_H, l.wrapping_add(value) < l);
        self.set_flag(FLAG_C, l.wrapping_add(value) < l);
        self.set_flag(FLAG_PV, self.b != 0x7F);
    }

    pub fn otir(&mut self) {
        while self.b != 0 {
            self.outi();
            if self.b == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_a_n() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0xFF00, 0x42); // Simulate I/O port 0 containing 0x42
        cpu.pc = 0x1000;
        cpu.write_byte(0x1000, 0x00); // Port number 0

        cpu.in_a_n();

        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.pc, 0x1001);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_out_n_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0x42;
        cpu.pc = 0x1000;
        cpu.write_byte(0x1000, 0x00); // Port number 0

        cpu.out_n_a();

        assert_eq!(cpu.read_byte(0xFF00), 0x42);
        assert_eq!(cpu.pc, 0x1001);
    }

    #[test]
    fn test_ini() {
        let mut cpu = Cpu::new();
        cpu.b = 0x03;
        cpu.c = 0x10;
        cpu.set_hl(0x2000);
        cpu.write_byte(0xFF10, 0xAA); // Simulate I/O port 0x10 containing 0xAA

        cpu.ini();

        assert_eq!(cpu.read_byte(0x2000), 0xAA);
        assert_eq!(cpu.get_hl(), 0x2001);
        assert_eq!(cpu.b, 0x02);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_inir() {
        let mut cpu = Cpu::new();
        cpu.b = 0x03;
        cpu.c = 0x10;
        cpu.set_hl(0x2000);
        cpu.write_byte(0xFF10, 0xAA);
        cpu.write_byte(0xFF11, 0xBB);
        cpu.write_byte(0xFF12, 0xCC);

        cpu.inir();

        assert_eq!(cpu.read_byte(0x2000), 0xAA);
        assert_eq!(cpu.read_byte(0x2001), 0xBB);
        assert_eq!(cpu.read_byte(0x2002), 0xCC);
        assert_eq!(cpu.get_hl(), 0x2003);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x13);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_PV));
    }
}
