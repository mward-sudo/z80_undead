use crate::cpu::flags::*;
use crate::cpu::Cpu;

impl Cpu {
    pub fn nop(&self) {
        // No operation
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    /// Complement the accumulator (A)
    pub fn cpl(&mut self) {
        self.a = !self.a;
        self.set_flag(FLAG_H, true);
        self.set_flag(FLAG_N, true);
    }

    /// Complement the carry flag
    pub fn ccf(&mut self) {
        let carry = self.get_flag(FLAG_C);
        self.set_flag(FLAG_C, !carry);
        self.set_flag(FLAG_H, carry);
        self.set_flag(FLAG_N, false);
    }

    /// Set the carry flag
    pub fn scf(&mut self) {
        self.set_flag(FLAG_C, true);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_N, false);
    }

    /// Set interrupt mode to 0
    pub fn im_0(&mut self) {
        self.interrupt_mode = 0;
    }

    /// Set interrupt mode to 1
    pub fn im_1(&mut self) {
        self.interrupt_mode = 1;
    }

    /// Set interrupt mode to 2
    pub fn im_2(&mut self) {
        self.interrupt_mode = 2;
    }

    /// Rotate Left Digit
    pub fn rld(&mut self) {
        let hl_address = self.get_hl();
        let m = self.read_byte(hl_address);

        println!("RLD Debug - Initial:");
        println!("  A: {:02X}", self.a);
        println!("  (HL): {:02X}", m);
        println!("  HL address: {:04X}", hl_address);

        let new_a = (self.a & 0xF0) | (m >> 4);
        let new_m = ((m << 4) & 0xF0) | (self.a & 0x0F);

        println!("RLD Debug - After calculation:");
        println!("  New A: {:02X}", new_a);
        println!("  New (HL): {:02X}", new_m);

        self.a = new_a;
        self.write_byte(hl_address, new_m);

        println!("RLD Debug - After update:");
        println!("  Final A: {:02X}", self.a);
        println!("  Final (HL): {:02X}", self.read_byte(hl_address));

        // Update flags
        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_H, false);

        // Simple parity calculation
        let parity = (self.a.count_ones() & 1) == 0;
        self.set_flag(FLAG_PV, parity);

        self.set_flag(FLAG_N, false);

        println!("RLD Debug - Final flags:");
        println!("  S: {}", self.get_flag(FLAG_S));
        println!("  Z: {}", self.get_flag(FLAG_Z));
        println!("  H: {}", self.get_flag(FLAG_H));
        println!("  PV: {}", self.get_flag(FLAG_PV));
        println!("  N: {}", self.get_flag(FLAG_N));
    }

    /// Rotate Right Digit
    pub fn rrd(&mut self) {
        let hl_address = self.get_hl();
        let mut value = self.read_byte(hl_address);

        let high_nibble = (value & 0xF0) >> 4;
        let low_nibble = value & 0x0F;

        // Rotate the nibbles
        value = (self.a & 0x0F) << 4 | high_nibble;
        self.a = (self.a & 0xF0) | low_nibble;

        self.write_byte(hl_address, value);

        // Update flags
        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_H, false);
        let parity = (self.a.count_ones() & 1) == 0;
        self.set_flag(FLAG_PV, parity);
        self.set_flag(FLAG_N, false);
    }

    /// Exchange the contents of DE and HL
    pub fn ex_de_hl(&mut self) {
        let temp = self.get_de();
        self.set_de(self.get_hl());
        self.set_hl(temp);
    }

    /// Exchange the contents of AF and AF'
    pub fn ex_af_af_prime(&mut self) {
        std::mem::swap(&mut self.a, &mut self.a_alt);
        std::mem::swap(&mut self.f, &mut self.f_alt);
    }

    /// Exchange the contents of BC, DE, and HL with BC', DE', and HL'
    pub fn exx(&mut self) {
        std::mem::swap(&mut self.b, &mut self.b_alt);
        std::mem::swap(&mut self.c, &mut self.c_alt);
        std::mem::swap(&mut self.d, &mut self.d_alt);
        std::mem::swap(&mut self.e, &mut self.e_alt);
        std::mem::swap(&mut self.h, &mut self.h_alt);
        std::mem::swap(&mut self.l, &mut self.l_alt);
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

    #[test]
    fn test_cpl() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10101010;
        cpu.cpl();
        assert_eq!(cpu.a, 0b01010101);
        assert!(cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_ccf() {
        let mut cpu = Cpu::new();
        cpu.set_flag(FLAG_C, true);
        cpu.ccf();
        assert!(!cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));

        cpu.ccf();
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
    }

    #[test]
    fn test_scf() {
        let mut cpu = Cpu::new();
        cpu.set_flag(FLAG_C, false);
        cpu.scf();
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_im_0() {
        let mut cpu = Cpu::new();
        cpu.im_0();
        assert_eq!(cpu.interrupt_mode, 0);
    }

    #[test]
    fn test_im_1() {
        let mut cpu = Cpu::new();
        cpu.im_1();
        assert_eq!(cpu.interrupt_mode, 1);
    }

    #[test]
    fn test_im_2() {
        let mut cpu = Cpu::new();
        cpu.im_2();
        assert_eq!(cpu.interrupt_mode, 2);
    }

    #[test]
    fn test_rld() {
        let mut cpu = Cpu::new();
        cpu.a = 0x12;
        cpu.set_hl(0x2000);
        cpu.write_byte(0x2000, 0x34);
        cpu.rld();
        assert_eq!(cpu.a, 0x13, "Accumulator should be 0x13");
        assert_eq!(cpu.read_byte(0x2000), 0x42, "Memory at HL should be 0x42");

        // Test flags
        assert!(!cpu.get_flag(FLAG_S), "S flag should be reset");
        assert!(!cpu.get_flag(FLAG_Z), "Z flag should be reset");
        assert!(!cpu.get_flag(FLAG_H), "H flag should be reset");
        assert!(!cpu.get_flag(FLAG_PV), "PV flag should be reset"); // Changed this line
        assert!(!cpu.get_flag(FLAG_N), "N flag should be reset");

        // Test with A = 0x00
        cpu.a = 0x00;
        cpu.write_byte(0x2000, 0x00);
        cpu.rld();
        assert_eq!(cpu.a, 0x00, "Accumulator should be 0x00");
        assert_eq!(cpu.read_byte(0x2000), 0x00, "Memory at HL should be 0x00");
        assert!(cpu.get_flag(FLAG_Z), "Z flag should be set");
        assert!(cpu.get_flag(FLAG_PV), "PV flag should be set for A = 0x00");
    }

    #[test]
    fn test_rrd() {
        let mut cpu = Cpu::new();
        cpu.a = 0x12;
        cpu.set_hl(0x2000);
        cpu.write_byte(0x2000, 0x34);
        cpu.rrd();
        assert_eq!(cpu.a, 0x14);
        assert_eq!(cpu.read_byte(0x2000), 0x23);
    }

    #[test]
    fn test_ex_de_hl() {
        let mut cpu = Cpu::new();
        cpu.set_de(0x1234);
        cpu.set_hl(0x5678);
        cpu.ex_de_hl();
        assert_eq!(cpu.get_de(), 0x5678);
        assert_eq!(cpu.get_hl(), 0x1234);
    }

    #[test]
    fn test_ex_af_af_prime() {
        let mut cpu = Cpu::new();
        cpu.a = 0x12;
        cpu.f = 0x34;
        cpu.a_alt = 0x56;
        cpu.f_alt = 0x78;
        cpu.ex_af_af_prime();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.f, 0x78);
        assert_eq!(cpu.a_alt, 0x12);
        assert_eq!(cpu.f_alt, 0x34);
    }

    #[test]
    fn test_exx() {
        let mut cpu = Cpu::new();
        cpu.b = 0x11;
        cpu.c = 0x22;
        cpu.d = 0x33;
        cpu.e = 0x44;
        cpu.h = 0x55;
        cpu.l = 0x66;
        cpu.b_alt = 0xAA;
        cpu.c_alt = 0xBB;
        cpu.d_alt = 0xCC;
        cpu.e_alt = 0xDD;
        cpu.h_alt = 0xEE;
        cpu.l_alt = 0xFF;
        cpu.exx();
        assert_eq!(cpu.b, 0xAA);
        assert_eq!(cpu.c, 0xBB);
        assert_eq!(cpu.d, 0xCC);
        assert_eq!(cpu.e, 0xDD);
        assert_eq!(cpu.h, 0xEE);
        assert_eq!(cpu.l, 0xFF);
        assert_eq!(cpu.b_alt, 0x11);
        assert_eq!(cpu.c_alt, 0x22);
        assert_eq!(cpu.d_alt, 0x33);
        assert_eq!(cpu.e_alt, 0x44);
        assert_eq!(cpu.h_alt, 0x55);
        assert_eq!(cpu.l_alt, 0x66);
    }
}
