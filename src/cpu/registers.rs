#[derive(Clone, Copy)]
pub enum Register {
    A, B, C, D, E, H, L, F,
}

#[derive(Clone, Copy)]
pub enum RegisterPair {
    BC, DE, HL, AF, SP, IX, IY,
}

#[derive(Clone, Copy)]
pub enum IndexRegister {
    IX, IY,
}

impl super::Cpu {
    pub fn read_register(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::F => self.f,
        }
    }

    pub fn write_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::F => self.f = value,
        }
    }

    pub fn read_register_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
            RegisterPair::AF => ((self.a as u16) << 8) | (self.f as u16),
            RegisterPair::SP => self.sp,
            RegisterPair::IX => self.ix,
            RegisterPair::IY => self.iy,
        }
    }

    pub fn write_register_pair(&mut self, pair: RegisterPair, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;
        match pair {
            RegisterPair::BC => { self.b = high; self.c = low; }
            RegisterPair::DE => { self.d = high; self.e = low; }
            RegisterPair::HL => { self.h = high; self.l = low; }
            RegisterPair::AF => { self.a = high; self.f = low; }
            RegisterPair::SP => self.sp = value,
            RegisterPair::IX => self.ix = value,
            RegisterPair::IY => self.iy = value,
        }
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::Cpu;

    #[test]
    fn test_register_operations() {
        let mut cpu = Cpu::new();
        cpu.write_register(Register::A, 0xAA);
        assert_eq!(cpu.read_register(Register::A), 0xAA);

        cpu.write_register_pair(RegisterPair::BC, 0xBBCC);
        assert_eq!(cpu.read_register_pair(RegisterPair::BC), 0xBBCC);
    }

    // Add more register-related tests...
}
