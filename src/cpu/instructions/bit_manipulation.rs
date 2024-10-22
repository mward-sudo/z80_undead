use super::*;
use crate::cpu::flags::*;

impl Cpu {
    /// Tests the specified bit of the given value.
    pub fn bit(&mut self, bit: u8, value: u8) {
        let result = value & (1 << bit);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, true);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_PV, result == 0);
        self.set_flag(FLAG_S, bit == 7 && result != 0);
        self.set_flag(FLAG_Y, value & (1 << 5) != 0);
        self.set_flag(FLAG_X, value & (1 << 3) != 0);
    }

    /// Sets the specified bit of the given value.
    pub fn set_bit(&mut self, bit: u8, value: &mut u8) {
        *value |= 1 << bit;
    }

    /// Resets the specified bit of the given value.
    pub fn res_bit(&mut self, bit: u8, value: &mut u8) {
        *value &= !(1 << bit);
    }

    pub fn rlc(&mut self, value: u8) -> u8 {
        let result = value.rotate_left(1);
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x80 != 0);
        result
    }

    pub fn rrc(&mut self, value: u8) -> u8 {
        let result = value.rotate_right(1);
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x01 != 0);
        result
    }

    pub fn rl(&mut self, value: u8) -> u8 {
        let carry = self.get_flag(FLAG_C) as u8;
        let result = (value << 1) | carry;
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x80 != 0);
        result
    }

    pub fn rr(&mut self, value: u8) -> u8 {
        let carry = self.get_flag(FLAG_C) as u8;
        let result = (value >> 1) | (carry << 7);
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x01 != 0);
        result
    }

    pub fn sla(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x80 != 0);
        result
    }

    pub fn sra(&mut self, value: u8) -> u8 {
        let result = (value >> 1) | (value & 0x80);
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x01 != 0);
        result
    }

    pub fn srl(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.update_rotation_flags(result);
        self.set_flag(FLAG_C, value & 0x01 != 0);
        result
    }

    // New instructions

    pub fn rlca(&mut self) {
        self.a = self.rlc(self.a);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_N, false);
    }

    pub fn rrca(&mut self) {
        self.a = self.rrc(self.a);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_N, false);
    }

    pub fn rla(&mut self) {
        self.a = self.rl(self.a);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_N, false);
    }

    pub fn rra(&mut self) {
        self.a = self.rr(self.a);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_N, false);
    }

    fn update_rotation_flags(&mut self, result: u8) {
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, result.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit() {
        let mut cpu = Cpu::new();
        cpu.bit(3, 0b00001000);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));

        cpu.bit(3, 0b11110111);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_set_bit() {
        let mut cpu = Cpu::new();
        let mut value = 0b11110111;
        cpu.set_bit(3, &mut value);
        assert_eq!(value, 0b11111111);
    }

    #[test]
    fn test_res_bit() {
        let mut cpu = Cpu::new();
        let mut value = 0b11111111;
        cpu.res_bit(3, &mut value);
        assert_eq!(value, 0b11110111);
    }

    #[test]
    fn test_rotations() {
        let mut cpu = Cpu::new();

        // Test RLC
        let rlc_result = cpu.rlc(0b10000001);
        assert_eq!(rlc_result, 0b00000011);
        assert!(cpu.get_flag(FLAG_C));

        // Test RRC
        let rrc_result = cpu.rrc(0b10000001);
        assert_eq!(rrc_result, 0b11000000);
        assert!(cpu.get_flag(FLAG_C));

        // Test RL
        cpu.set_flag(FLAG_C, true);
        let rl_result = cpu.rl(0b10000000);
        assert_eq!(rl_result, 0b00000001);
        assert!(cpu.get_flag(FLAG_C));

        // Test RR
        cpu.set_flag(FLAG_C, true);
        let rr_result = cpu.rr(0b00000001);
        assert_eq!(rr_result, 0b10000000);
        assert!(cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_shifts() {
        let mut cpu = Cpu::new();

        // Test SLA
        let sla_result = cpu.sla(0b10000001);
        assert_eq!(sla_result, 0b00000010);
        assert!(cpu.get_flag(FLAG_C));

        // Test SRA
        let sra_result = cpu.sra(0b10000001);
        assert_eq!(sra_result, 0b11000000);
        assert!(cpu.get_flag(FLAG_C));

        // Test SRL
        let srl_result = cpu.srl(0b10000001);
        assert_eq!(srl_result, 0b01000000);
        assert!(cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_rlca() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000001;
        cpu.rlca();
        assert_eq!(cpu.a, 0b00000011);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_rrca() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000001;
        cpu.rrca();
        assert_eq!(cpu.a, 0b11000000);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_rla() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000001;
        cpu.set_flag(FLAG_C, true);
        cpu.rla();
        assert_eq!(cpu.a, 0b00000011);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_rra() {
        let mut cpu = Cpu::new();
        cpu.a = 0b10000001;
        cpu.set_flag(FLAG_C, true);
        cpu.rra();
        assert_eq!(cpu.a, 0b11000000);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_sla() {
        let mut cpu = Cpu::new();
        let result = cpu.sla(0b10000001);
        assert_eq!(result, 0b00000010);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_sra() {
        let mut cpu = Cpu::new();
        let result = cpu.sra(0b10000001);
        assert_eq!(result, 0b11000000);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_srl() {
        let mut cpu = Cpu::new();
        let result = cpu.srl(0b10000001);
        assert_eq!(result, 0b01000000);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
    }
}
