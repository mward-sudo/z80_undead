use super::*;
use crate::cpu::flags::*;

impl Cpu {
    /// Performs a bitwise AND operation between the accumulator (A) and the given value.
    pub fn and_a(&mut self, value: u8) {
        self.a &= value;
        self.update_flags_logical();
        self.set_flag(FLAG_H, true);
    }

    /// Performs a bitwise OR operation between the accumulator (A) and the given value.
    pub fn or_a(&mut self, value: u8) {
        self.a |= value;
        self.update_flags_logical();
        self.set_flag(FLAG_H, false);
    }

    /// Performs a bitwise XOR operation between the accumulator (A) and the given value.
    pub fn xor_a(&mut self, value: u8) {
        self.a ^= value;
        self.update_flags_logical();
        self.set_flag(FLAG_H, false);
    }

    /// Compares the accumulator (A) with the given value by performing a subtraction
    /// without storing the result.
    pub fn cp_a(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        let half_carry = (self.a & 0xF) < (value & 0xF);

        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, half_carry);
        self.set_flag(FLAG_PV, (self.a ^ value) & (self.a ^ result) & 0x80 != 0);
        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_C, self.a < value);
        self.set_flag(FLAG_Y, result & (1 << 5) != 0);
        self.set_flag(FLAG_X, result & (1 << 3) != 0);
    }

    /// Updates flags common to AND, OR, and XOR operations.
    fn update_flags_logical(&mut self) {
        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_PV, self.a.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, false);
        self.set_flag(FLAG_Y, self.a & (1 << 5) != 0);
        self.set_flag(FLAG_X, self.a & (1 << 3) != 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0b11001100;
        cpu.and_a(0b10101010);
        assert_eq!(cpu.a, 0b10001000);
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_or_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0b11001100;
        cpu.or_a(0b10101010);
        assert_eq!(cpu.a, 0b11101110);
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_xor_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0b11001100;
        cpu.xor_a(0b10101010);
        assert_eq!(cpu.a, 0b01100110);
        assert!(!cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_cp_a() {
        let mut cpu = Cpu::new();
        cpu.a = 0x40;
        cpu.cp_a(0x40);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_N));

        cpu.a = 0x40;
        cpu.cp_a(0x41);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_N));
    }
}
