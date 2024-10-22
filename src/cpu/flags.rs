pub const FLAG_C: u8 = 0x01; // Carry Flag
pub const FLAG_N: u8 = 0x02; // Add/Subtract
pub const FLAG_PV: u8 = 0x04; // Parity/Overflow
pub const FLAG_H: u8 = 0x10; // Half Carry
pub const FLAG_Z: u8 = 0x40; // Zero
pub const FLAG_S: u8 = 0x80; // Sign

impl super::Cpu {
    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.f |= flag;
        } else {
            self.f &= !flag;
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.f & flag) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Cpu;

    #[test]
    fn test_flag_operations() {
        let mut cpu = Cpu::new();
        cpu.set_flag(FLAG_Z, true);
        assert!(cpu.get_flag(FLAG_Z));
        cpu.set_flag(FLAG_Z, false);
        assert!(!cpu.get_flag(FLAG_Z));
    }

    // Add more flag-related tests...
}
