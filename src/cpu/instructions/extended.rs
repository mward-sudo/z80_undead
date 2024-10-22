use super::*;
use crate::cpu::flags::*;

impl Cpu {
    pub fn ld_ix_nn(&mut self, value: u16) {
        self.ix = value;
    }

    pub fn ld_iy_nn(&mut self, value: u16) {
        self.iy = value;
    }

    pub fn ld_ix_d_n(&mut self, offset: i8, value: u8) {
        let address = self.ix.wrapping_add(offset as u16);
        self.write_byte(address, value);
    }

    pub fn ld_iy_d_n(&mut self, offset: i8, value: u8) {
        let address = self.iy.wrapping_add(offset as u16);
        self.write_byte(address, value);
    }

    pub fn ld_r_ix_d(&mut self, reg: Register, offset: i8) {
        let address = self.ix.wrapping_add(offset as u16);
        let value = self.read_byte(address);
        self.write_register(reg, value);
    }

    pub fn ld_r_iy_d(&mut self, reg: Register, offset: i8) {
        let address = self.iy.wrapping_add(offset as u16);
        let value = self.read_byte(address);
        self.write_register(reg, value);
    }

    pub fn ld_ix_d_r(&mut self, offset: i8, reg: Register) {
        let address = self.ix.wrapping_add(offset as u16);
        let value = self.read_register(reg);
        self.write_byte(address, value);
    }

    pub fn ld_iy_d_r(&mut self, offset: i8, reg: Register) {
        let address = self.iy.wrapping_add(offset as u16);
        let value = self.read_register(reg);
        self.write_byte(address, value);
    }

    pub fn add_ix(&mut self, value: u16) {
        let ix = self.ix;
        let result = ix.wrapping_add(value);
        self.update_flags_add_16(ix, value);
        self.ix = result;
    }

    pub fn add_iy(&mut self, value: u16) {
        let iy = self.iy;
        let result = iy.wrapping_add(value);
        let h_check = (iy & 0x0FFF) + (value & 0x0FFF);

        // Update the half-carry flag correctly
        self.set_flag(FLAG_H, h_check > 0x0FFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, result < iy);
        self.iy = result;
    }

    pub fn adc_ix(&mut self, value: u16) {
        let ix = self.ix;
        let carry = self.get_flag(FLAG_C) as u16;
        let result = ix.wrapping_add(value).wrapping_add(carry);
        self.update_flags_adc_16(ix, value, carry);
        self.ix = result;
    }

    pub fn adc_iy(&mut self, value: u16) {
        let iy = self.iy;
        let carry = self.get_flag(FLAG_C) as u16;
        let result = iy.wrapping_add(value).wrapping_add(carry);
        self.update_flags_adc_16(iy, value, carry);
        self.iy = result;
    }

    pub fn sbc_ix(&mut self, value: u16) {
        let ix = self.ix;
        let carry = self.get_flag(FLAG_C) as u16;
        let result = ix.wrapping_sub(value).wrapping_sub(carry);
        self.update_flags_sbc_16(ix, value, carry);
        self.ix = result;
    }

    pub fn sbc_iy(&mut self, value: u16) {
        let iy = self.iy;
        let carry = self.get_flag(FLAG_C) as u16;
        let result = iy.wrapping_sub(value).wrapping_sub(carry);
        self.update_flags_sbc_16(iy, value, carry);
        self.iy = result;
    }

    pub fn inc_ix(&mut self) {
        self.ix = self.ix.wrapping_add(1);
    }

    pub fn inc_iy(&mut self) {
        self.iy = self.iy.wrapping_add(1);
    }

    pub fn dec_ix(&mut self) {
        self.ix = self.ix.wrapping_sub(1);
    }

    pub fn dec_iy(&mut self) {
        self.iy = self.iy.wrapping_sub(1);
    }

    // 16-bit arithmetic operations
    pub fn add_hl(&mut self, value: u16) {
        let hl = self.get_hl();
        let result = hl.wrapping_add(value);
        self.update_flags_add_16(hl, value);
        self.set_hl(result);
    }

    pub fn adc_hl(&mut self, value: u16) {
        let hl = self.get_hl();
        let carry = self.get_flag(FLAG_C) as u16;
        let result = hl.wrapping_add(value).wrapping_add(carry);
        self.update_flags_adc_16(hl, value, carry);
        self.set_hl(result);
    }

    pub fn sbc_hl(&mut self, value: u16) {
        let hl = self.get_hl();
        let carry = self.get_flag(FLAG_C) as u16;
        let result = hl.wrapping_sub(value).wrapping_sub(carry);
        self.update_flags_sbc_16(hl, value, carry);
        self.set_hl(result);
    }

    pub fn inc_16(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    pub fn dec_16(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    // Helper functions for updating flags
    fn update_flags_add_16(&mut self, a: u16, b: u16) {
        let result = a.wrapping_add(b);
        self.set_flag(FLAG_H, (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, result < a);
    }

    fn update_flags_adc_16(&mut self, a: u16, b: u16, carry: u16) {
        let result = a.wrapping_add(b).wrapping_add(carry);
        self.set_flag(FLAG_S, (result & 0x8000) != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (a & 0x0FFF) + (b & 0x0FFF) + carry > 0x0FFF);
        self.set_flag(FLAG_PV, (a ^ b ^ 0x8000) & (a ^ result) & 0x8000 != 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (a as u32) + (b as u32) + (carry as u32) > 0xFFFF);
    }

    fn update_flags_sbc_16(&mut self, a: u16, b: u16, carry: u16) {
        let result = a.wrapping_sub(b).wrapping_sub(carry);
        self.set_flag(FLAG_S, (result & 0x8000) != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (a & 0x0FFF) < (b & 0x0FFF) + carry);
        self.set_flag(FLAG_PV, (a ^ b) & (a ^ result) & 0x8000 != 0);
        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_C, (a as u32) < (b as u32) + (carry as u32));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_ix_nn() {
        let mut cpu = Cpu::new();
        cpu.ld_ix_nn(0x1234);
        assert_eq!(cpu.ix, 0x1234);
    }

    #[test]
    fn test_ld_iy_nn() {
        let mut cpu = Cpu::new();
        cpu.ld_iy_nn(0x5678);
        assert_eq!(cpu.iy, 0x5678);
    }

    #[test]
    fn test_ld_ix_d_n() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1000;
        cpu.ld_ix_d_n(10, 0x42);
        assert_eq!(cpu.read_byte(0x100A), 0x42);
    }

    // Add more tests for IX and IY related instructions...

    #[test]
    fn test_add_ix_with_carry() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1000;
        cpu.add_ix(0x0234);
        assert_eq!(cpu.ix, 0x1234);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_inc_ix_overflow() {
        let mut cpu = Cpu::new();
        cpu.ix = 0xFFFF;
        cpu.inc_ix();
        assert_eq!(cpu.ix, 0x0000);
    }

    #[test]
    fn test_inc_iy_overflow() {
        let mut cpu = Cpu::new();
        cpu.iy = 0xFFFF;
        cpu.inc_iy();
        assert_eq!(cpu.iy, 0x0000);
    }

    #[test]
    fn test_dec_ix_underflow() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x0000;
        cpu.dec_ix();
        assert_eq!(cpu.ix, 0xFFFF);
    }

    #[test]
    fn test_adc_ix() {
        let mut cpu = Cpu::new();
        cpu.ix = 0xFFFE;
        cpu.set_flag(FLAG_C, true);
        cpu.adc_ix(0x0001);
        assert_eq!(cpu.ix, 0x0000);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_sbc_iy() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x1000;
        cpu.set_flag(FLAG_C, true);
        cpu.sbc_iy(0x0FFF);
        assert_eq!(cpu.iy, 0x0000);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_inc_ix() {
        let mut cpu = Cpu::new();
        cpu.ix = 0xFFFF;
        cpu.inc_ix();
        assert_eq!(cpu.ix, 0x0000);
    }

    #[test]
    fn test_dec_iy() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x0000;
        cpu.dec_iy();
        assert_eq!(cpu.iy, 0xFFFF);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1000);
        cpu.add_hl(0x0234);
        assert_eq!(cpu.get_hl(), 0x1234);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_adc_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1000);
        cpu.set_flag(FLAG_C, true);
        cpu.adc_hl(0x0001);
        assert_eq!(cpu.get_hl(), 0x1002);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_sbc_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1000);
        cpu.set_flag(FLAG_C, true);
        cpu.sbc_hl(0x0FFF);
        assert_eq!(cpu.get_hl(), 0x0000);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_add_ix() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1000;
        cpu.add_ix(0x0234);
        assert_eq!(cpu.ix, 0x1234);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_add_iy() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x0FFF;
        cpu.add_iy(0x0001);
        assert_eq!(cpu.iy, 0x1000);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H), "Half-carry flag should be set");
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_add_iy_with_carry() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x0FFF;
        cpu.add_iy(0x0001);
        println!("Test add_iy_with_carry: iy = {:04X}", cpu.iy);
        println!(
            "Flags: S={} Z={} H={} PV={} N={} C={}",
            cpu.get_flag(FLAG_S),
            cpu.get_flag(FLAG_Z),
            cpu.get_flag(FLAG_H),
            cpu.get_flag(FLAG_PV),
            cpu.get_flag(FLAG_N),
            cpu.get_flag(FLAG_C)
        );
        assert_eq!(cpu.iy, 0x1000);
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H), "Half-carry flag should be set");
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_inc_16() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.inc_16(0xFFFF), 0x0000);
    }

    #[test]
    fn test_dec_16() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.dec_16(0x0000), 0xFFFF);
    }

    #[test]
    fn test_inc_ix_no_overflow() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;
        cpu.inc_ix();
        assert_eq!(cpu.ix, 0x1235);
    }

    #[test]
    fn test_inc_iy_no_overflow() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x1234;
        cpu.inc_iy();
        assert_eq!(cpu.iy, 0x1235);
    }

    #[test]
    fn test_dec_ix_no_underflow() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;
        cpu.dec_ix();
        assert_eq!(cpu.ix, 0x1233);
    }

    #[test]
    fn test_dec_iy_no_underflow() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x1234;
        cpu.dec_iy();
        assert_eq!(cpu.iy, 0x1233);
    }
}
