use super::super::flags::*;
use super::*;

impl Cpu {
    // 8-bit arithmetic operations
    pub fn add_a(&mut self, value: u8) {
        let a = self.a;
        let result = a.wrapping_add(value);
        self.update_flags_add(a, value, false);
        self.a = result;
    }

    pub fn adc_a(&mut self, value: u8) {
        let a = self.a;
        let carry = self.get_flag(FLAG_C) as u8;
        let result = a.wrapping_add(value).wrapping_add(carry);
        self.update_flags_add(a, value, carry != 0);
        self.a = result;
    }

    pub fn sub_a(&mut self, value: u8) {
        let a = self.a;
        let result = a.wrapping_sub(value);
        self.update_flags_sub(a, value, false);
        self.a = result;
    }

    pub fn sbc_a(&mut self, value: u8) {
        let a = self.a;
        let carry = self.get_flag(FLAG_C) as u8;
        let result = a.wrapping_sub(value).wrapping_sub(carry);
        self.update_flags_sub(a, value, carry != 0);
        self.a = result;
    }

    pub fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (value & 0x0F) == 0x0F);
        self.set_flag(FLAG_PV, value == 0x7F);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_Y, result & (1 << 5) != 0);
        self.set_flag(FLAG_X, result & (1 << 3) != 0);
        result
    }

    pub fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (value & 0x0F) == 0);
        self.set_flag(FLAG_PV, value == 0x80);
        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_Y, result & (1 << 5) != 0);
        self.set_flag(FLAG_X, result & (1 << 3) != 0);
        result
    }

    pub fn daa(&mut self) {
        let mut a = self.a;
        let mut adjust = if self.get_flag(FLAG_C) { 0x60 } else { 0 };

        if self.get_flag(FLAG_H) || (a & 0x0F) > 9 {
            adjust |= 0x06;
        }

        if self.get_flag(FLAG_C) || a > 0x99 {
            adjust |= 0x60;
        }

        if self.get_flag(FLAG_N) {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }

        self.set_flag(FLAG_S, a & 0x80 != 0);
        self.set_flag(FLAG_Z, a == 0);
        self.set_flag(FLAG_H, false); // Always reset H flag after DAA
        self.set_flag(FLAG_PV, a.count_ones() % 2 == 0);
        self.set_flag(FLAG_C, adjust >= 0x60);
        self.set_flag(FLAG_Y, a & (1 << 5) != 0);
        self.set_flag(FLAG_X, a & (1 << 3) != 0);

        self.a = a;
    }

    pub fn neg(&mut self) {
        let a = self.a;
        self.a = 0;
        self.sub_a(a);
    }

    fn update_flags_add(&mut self, a: u8, value: u8, carry: bool) {
        let result = (a as u16) + (value as u16) + (carry as u16);
        self.set_flag(FLAG_S, (result & 0x80) != 0);
        self.set_flag(FLAG_Z, (result & 0xFF) == 0);
        self.set_flag(FLAG_H, (a & 0x0F) + (value & 0x0F) + (carry as u8) > 0x0F);
        self.set_flag(FLAG_PV, (a ^ value ^ 0x80) & (a ^ result as u8) & 0x80 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, result > 0xFF);
        self.set_flag(FLAG_Y, result & (1 << 5) != 0);
        self.set_flag(FLAG_X, result & (1 << 3) != 0);
    }

    fn update_flags_sub(&mut self, a: u8, value: u8, carry: bool) {
        let result = (a as i16) - (value as i16) - (carry as i16);
        self.set_flag(FLAG_S, (result & 0x80) != 0);
        self.set_flag(FLAG_Z, (result & 0xFF) == 0);
        self.set_flag(FLAG_H, (a & 0x0F) < (value & 0x0F) + (carry as u8));
        self.set_flag(FLAG_PV, (a ^ value) & (a ^ result as u8) & 0x80 != 0);
        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_C, result < 0);
        self.set_flag(FLAG_Y, result & (1 << 5) != 0);
        self.set_flag(FLAG_X, result & (1 << 3) != 0);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    // ... (keep existing tests)
}
