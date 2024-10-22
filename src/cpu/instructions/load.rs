use super::*;
use crate::cpu::RegisterPair;

impl Cpu {
    pub fn ld_r_r(&mut self, dest: Register, src: Register) {
        let value = self.read_register(src);
        self.write_register(dest, value);
    }

    pub fn ld_r_n(&mut self, reg: Register, value: u8) {
        self.write_register(reg, value);
    }

    pub fn ld_r_hl(&mut self, reg: Register) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.write_register(reg, value);
    }

    pub fn ld_hl_r(&mut self, reg: Register) {
        let address = self.get_hl();
        let value = self.read_register(reg);
        self.write_byte(address, value);
    }

    pub fn ld_hl_n(&mut self, value: u8) {
        let address = self.get_hl();
        self.write_byte(address, value);
    }

    pub fn ld_a_bc(&mut self) {
        let address = self.get_bc();
        let value = self.read_byte(address);
        self.write_register(Register::A, value);
    }

    pub fn ld_a_de(&mut self) {
        let address = self.get_de();
        let value = self.read_byte(address);
        self.write_register(Register::A, value);
    }

    pub fn ld_bc_a(&mut self) {
        let address = self.get_bc();
        let value = self.read_register(Register::A);
        self.write_byte(address, value);
    }

    pub fn ld_de_a(&mut self) {
        let address = self.get_de();
        let value = self.read_register(Register::A);
        self.write_byte(address, value);
    }

    pub fn ld_nn_a(&mut self, address: u16) {
        let value = self.read_register(Register::A);
        self.write_byte(address, value);
    }

    pub fn ld_rr_nn(&mut self, reg_pair: RegisterPair, value: u16) {
        self.write_register_pair(reg_pair, value);
    }

    pub fn ld_sp_hl(&mut self) {
        self.sp = self.get_hl();
    }

    pub fn ld_sp_ix(&mut self) {
        self.sp = self.ix;
    }

    pub fn ld_sp_iy(&mut self) {
        self.sp = self.iy;
    }

    pub fn push_rr(&mut self, reg_pair: RegisterPair) {
        let value = self.read_register_pair(reg_pair);
        self.push(value);
    }

    pub fn push_ix(&mut self) {
        self.push(self.ix);
    }

    pub fn push_iy(&mut self) {
        self.push(self.iy);
    }

    pub fn pop_rr(&mut self, reg_pair: RegisterPair) {
        let value = self.pop();
        self.write_register_pair(reg_pair, value);
    }

    pub fn pop_ix(&mut self) {
        self.ix = self.pop();
    }

    pub fn pop_iy(&mut self) {
        self.iy = self.pop();
    }

    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, value);
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        value
    }

    /// Load SP into memory at address nn
    pub fn ld_nn_sp(&mut self, address: u16) {
        self.write_word(address, self.sp);
    }

    /// Load HL into memory at address nn
    pub fn ld_nn_hl(&mut self, address: u16) {
        self.write_word(address, self.get_hl());
    }

    /// Load IX into memory at address nn
    pub fn ld_nn_ix(&mut self, address: u16) {
        self.write_word(address, self.ix);
    }

    /// Load IY into memory at address nn
    pub fn ld_nn_iy(&mut self, address: u16) {
        self.write_word(address, self.iy);
    }

    /// Exchange HL with the value at the top of the stack
    pub fn ex_sp_hl(&mut self) {
        let temp = self.read_word(self.sp);
        self.write_word(self.sp, self.get_hl());
        self.set_hl(temp);
    }

    /// Exchange IX with the value at the top of the stack
    pub fn ex_sp_ix(&mut self) {
        let temp = self.read_word(self.sp);
        self.write_word(self.sp, self.ix);
        self.ix = temp;
    }

    /// Exchange IY with the value at the top of the stack
    pub fn ex_sp_iy(&mut self) {
        let temp = self.read_word(self.sp);
        self.write_word(self.sp, self.iy);
        self.iy = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_r_r() {
        let mut cpu = Cpu::new();
        cpu.write_register(Register::B, 0x42);
        cpu.ld_r_r(Register::C, Register::B);
        assert_eq!(cpu.read_register(Register::C), 0x42);
    }

    #[test]
    fn test_ld_r_n() {
        let mut cpu = Cpu::new();
        cpu.ld_r_n(Register::A, 0xFF);
        assert_eq!(cpu.read_register(Register::A), 0xFF);
    }

    #[test]
    fn test_ld_hl_r() {
        let mut cpu = Cpu::new();
        cpu.write_register_pair(RegisterPair::HL, 0x1000);
        cpu.write_register(Register::B, 0x42);
        cpu.ld_hl_r(Register::B);
        assert_eq!(cpu.read_byte(0x1000), 0x42);
    }

    // Add more tests for other load instructions...

    #[test]
    fn test_ld_nn_sp() {
        let mut cpu = Cpu::new();
        cpu.sp = 0x1234;
        cpu.ld_nn_sp(0x2000);
        assert_eq!(cpu.read_word(0x2000), 0x1234);
    }

    #[test]
    fn test_ld_nn_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x5678);
        cpu.ld_nn_hl(0x2000);
        assert_eq!(cpu.read_word(0x2000), 0x5678);
    }

    #[test]
    fn test_ld_nn_ix() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x9ABC;
        cpu.ld_nn_ix(0x2000);
        assert_eq!(cpu.read_word(0x2000), 0x9ABC);
    }

    #[test]
    fn test_ld_nn_iy() {
        let mut cpu = Cpu::new();
        cpu.iy = 0xDEF0;
        cpu.ld_nn_iy(0x2000);
        assert_eq!(cpu.read_word(0x2000), 0xDEF0);
    }

    #[test]
    fn test_ex_sp_hl() {
        let mut cpu = Cpu::new();
        cpu.set_hl(0x1234);
        cpu.sp = 0x2000;
        cpu.write_word(cpu.sp, 0x5678);
        cpu.ex_sp_hl();
        assert_eq!(cpu.get_hl(), 0x5678);
        assert_eq!(cpu.read_word(cpu.sp), 0x1234);
    }

    #[test]
    fn test_ex_sp_ix() {
        let mut cpu = Cpu::new();
        cpu.ix = 0x1234;
        cpu.sp = 0x2000;
        cpu.write_word(cpu.sp, 0x5678);
        cpu.ex_sp_ix();
        assert_eq!(cpu.ix, 0x5678);
        assert_eq!(cpu.read_word(cpu.sp), 0x1234);
    }

    #[test]
    fn test_ex_sp_iy() {
        let mut cpu = Cpu::new();
        cpu.iy = 0x1234;
        cpu.sp = 0x2000;
        cpu.write_word(cpu.sp, 0x5678);
        cpu.ex_sp_iy();
        assert_eq!(cpu.iy, 0x5678);
        assert_eq!(cpu.read_word(cpu.sp), 0x1234);
    }
}
