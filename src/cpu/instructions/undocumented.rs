use super::*;
use crate::cpu::flags::*;

impl Cpu {
    /// Undocumented: SLL (Shift Left Logical)
    /// Similar to SLA but sets bit 0 to 1
    pub fn sll(&mut self, value: u8) -> u8 {
        let result = (value << 1) | 0x01;
        self.update_flags_logical_shift(result);
        self.set_flag(FLAG_C, value & 0x80 != 0);
        result
    }

    /// Undocumented: Set flag N based on bit 3 of register
    pub fn set_n_flag_bit3(&mut self, reg: Register) {
        let value = self.read_register(reg);
        self.set_flag(FLAG_N, value & 0x08 != 0);
    }

    /// Undocumented: Set flag N based on bit 5 of register
    pub fn set_n_flag_bit5(&mut self, reg: Register) {
        let value = self.read_register(reg);
        self.set_flag(FLAG_N, value & 0x20 != 0);
    }

    /// Undocumented: Load high byte of IX into A
    pub fn ld_a_ixh(&mut self) {
        self.a = (self.ix >> 8) as u8;
    }

    /// Undocumented: Load low byte of IX into A
    pub fn ld_a_ixl(&mut self) {
        self.a = self.ix as u8;
    }

    /// Undocumented: Load high byte of IY into A
    pub fn ld_a_iyh(&mut self) {
        self.a = (self.iy >> 8) as u8;
    }

    /// Undocumented: Load low byte of IY into A
    pub fn ld_a_iyl(&mut self) {
        self.a = self.iy as u8;
    }

    /// Undocumented: Load value into high byte of IX
    pub fn ld_ixh_n(&mut self, value: u8) {
        self.ix = (self.ix & 0x00FF) | ((value as u16) << 8);
    }

    /// Undocumented: Load value into low byte of IX
    pub fn ld_ixl_n(&mut self, value: u8) {
        self.ix = (self.ix & 0xFF00) | (value as u16);
    }

    /// Undocumented: Load value into high byte of IY
    pub fn ld_iyh_n(&mut self, value: u8) {
        self.iy = (self.iy & 0x00FF) | ((value as u16) << 8);
    }

    /// Undocumented: Load value into low byte of IY
    pub fn ld_iyl_n(&mut self, value: u8) {
        self.iy = (self.iy & 0xFF00) | (value as u16);
    }

    fn update_flags_logical_shift(&mut self, result: u8) {
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, result.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
    }

    /// Undocumented: IN F,(C) - Input to F register
    pub fn in_f_c(&mut self) {
        let value = self.read_byte(0xFF00 | (self.c as u16));

        // Only affects flags, doesn't store the value
        self.set_flag(FLAG_S, value & 0x80 != 0);
        self.set_flag(FLAG_Z, value == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, value.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_Y, value & (1 << 5) != 0);
        self.set_flag(FLAG_X, value & (1 << 3) != 0);
    }

    /// Undocumented: OUT (C),0 - Output 0 to port (C)
    pub fn out_c_0(&mut self) {
        self.write_byte(0xFF00 | (self.c as u16), 0);
    }

    /// Undocumented: Store high byte of IX in H and low byte in L
    pub fn ld_hl_ix(&mut self) {
        self.h = (self.ix >> 8) as u8;
        self.l = self.ix as u8;
    }

    /// Undocumented: Store high byte of IY in H and low byte in L
    pub fn ld_hl_iy(&mut self) {
        self.h = (self.iy >> 8) as u8;
        self.l = self.iy as u8;
    }

    /// Undocumented: Store H in high byte of IX and L in low byte
    pub fn ld_ix_hl(&mut self) {
        self.ix = ((self.h as u16) << 8) | (self.l as u16);
    }

    /// Undocumented: Store H in high byte of IY and L in low byte
    pub fn ld_iy_hl(&mut self) {
        self.iy = ((self.h as u16) << 8) | (self.l as u16);
    }

    /// Undocumented: Compare high byte of IX with A
    pub fn cp_ixh(&mut self) {
        let value = (self.ix >> 8) as u8;
        self.cp_a(value);
    }

    /// Undocumented: Compare low byte of IX with A
    pub fn cp_ixl(&mut self) {
        let value = self.ix as u8;
        self.cp_a(value);
    }

    /// Undocumented: Compare high byte of IY with A
    pub fn cp_iyh(&mut self) {
        let value = (self.iy >> 8) as u8;
        self.cp_a(value);
    }

    /// Undocumented: Compare low byte of IY with A
    pub fn cp_iyl(&mut self) {
        let value = self.iy as u8;
        self.cp_a(value);
    }

    /// Undocumented: Increment high byte of IX
    pub fn inc_ixh(&mut self) {
        let value = (self.ix >> 8) as u8;
        let result = self.inc(value);
        self.ix = (self.ix & 0x00FF) | ((result as u16) << 8);
    }

    /// Undocumented: Increment low byte of IX
    pub fn inc_ixl(&mut self) {
        let value = self.ix as u8;
        let result = self.inc(value);
        self.ix = (self.ix & 0xFF00) | (result as u16);
    }

    /// Undocumented: Increment high byte of IY
    pub fn inc_iyh(&mut self) {
        let value = (self.iy >> 8) as u8;
        let result = self.inc(value);
        self.iy = (self.iy & 0x00FF) | ((result as u16) << 8);
    }

    /// Undocumented: Increment low byte of IY
    pub fn inc_iyl(&mut self) {
        let value = self.iy as u8;
        let result = self.inc(value);
        self.iy = (self.iy & 0xFF00) | (result as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sll() {
        let mut cpu = Cpu::new();
        let result = cpu.sll(0b10101010);
        assert_eq!(result, 0b01010101);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_ix_high_low() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;

        cpu.ld_a_ixh();
        assert_eq!(cpu.a, 0x12);

        cpu.ld_a_ixl();
        assert_eq!(cpu.a, 0x34);

        cpu.ld_ixh_n(0x56);
        assert_eq!(cpu.ix, 0x5634);

        cpu.ld_ixl_n(0x78);
        assert_eq!(cpu.ix, 0x5678);
    }

    #[test]
    fn test_iy_high_low() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x1234;

        cpu.ld_a_iyh();
        assert_eq!(cpu.a, 0x12);

        cpu.ld_a_iyl();
        assert_eq!(cpu.a, 0x34);

        cpu.ld_iyh_n(0x56);
        assert_eq!(cpu.iy, 0x5634);

        cpu.ld_iyl_n(0x78);
        assert_eq!(cpu.iy, 0x5678);
    }

    #[test]
    fn test_set_n_flag_bits() {
        let mut cpu = Cpu::new();
        cpu.write_register(Register::A, 0b00001000);
        cpu.set_n_flag_bit3(Register::A);
        assert!(cpu.get_flag(FLAG_N));

        cpu.write_register(Register::A, 0b00000000);
        cpu.set_n_flag_bit3(Register::A);
        assert!(!cpu.get_flag(FLAG_N));

        cpu.write_register(Register::A, 0b00100000);
        cpu.set_n_flag_bit5(Register::A);
        assert!(cpu.get_flag(FLAG_N));

        cpu.write_register(Register::A, 0b00000000);
        cpu.set_n_flag_bit5(Register::A);
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_in_f_c() {
        let mut cpu = Cpu::new();
        cpu.c = 0x10;
        cpu.write_byte(0xFF10, 0b10100101);

        cpu.in_f_c();

        assert!(cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(cpu.get_flag(FLAG_Y));
        assert!(!cpu.get_flag(FLAG_X));
    }

    #[test]
    fn test_out_c_0() {
        let mut cpu = Cpu::new();
        cpu.c = 0x10;
        cpu.write_byte(0xFF10, 0xFF);

        cpu.out_c_0();

        assert_eq!(cpu.read_byte(0xFF10), 0);
    }

    #[test]
    fn test_ld_hl_ix_iy() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;
        cpu.iy = 0x5678;

        cpu.ld_hl_ix();
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x34);

        cpu.ld_hl_iy();
        assert_eq!(cpu.h, 0x56);
        assert_eq!(cpu.l, 0x78);
    }

    #[test]
    fn test_ld_ix_iy_hl() {
        let mut cpu = Cpu::new();
        cpu.h = 0x12;
        cpu.l = 0x34;

        cpu.ld_ix_hl();
        assert_eq!(cpu.ix, 0x1234);

        cpu.h = 0x56;
        cpu.l = 0x78;
        cpu.ld_iy_hl();
        assert_eq!(cpu.iy, 0x5678);
    }

    #[test]
    fn test_cp_ix_iy_parts() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;
        cpu.iy = 0x5678;
        cpu.a = 0x12;

        cpu.cp_ixh();
        assert!(cpu.get_flag(FLAG_Z));

        cpu.cp_ixl();
        assert!(!cpu.get_flag(FLAG_Z));

        cpu.a = 0x56;
        cpu.cp_iyh();
        assert!(cpu.get_flag(FLAG_Z));

        cpu.cp_iyl();
        assert!(!cpu.get_flag(FLAG_Z));
    }

    #[test]
    fn test_inc_ix_iy_parts() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x12FF;
        cpu.iy = 0x56FF;

        cpu.inc_ixh();
        assert_eq!(cpu.ix, 0x13FF);

        cpu.inc_ixl();
        assert_eq!(cpu.ix, 0x1300);

        cpu.inc_iyh();
        assert_eq!(cpu.iy, 0x57FF);

        cpu.inc_iyl();
        assert_eq!(cpu.iy, 0x5700);
    }
}
