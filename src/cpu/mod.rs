/// Represents the Z80 CPU, including registers and flags

pub struct Cpu {
    // 8-bit registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8, // Flag register

    // 16-bit registers
    pc: u16, // Program Counter
    sp: u16, // Stack Pointer
    ix: u16,
    iy: u16,

    // Alternate registers
    a_alt: u8,
    b_alt: u8,
    c_alt: u8,
    d_alt: u8,
    e_alt: u8,
    h_alt: u8,
    l_alt: u8,
    f_alt: u8,

    // Interrupt flip-flops
    iff1: bool,
    iff2: bool,

    // Interrupt mode
    im: u8,

    // Memory (we'll use a Vec<u8> to represent the full 64KB addressable memory)
    memory: Vec<u8>,
}

impl Cpu {
    /// Creates a new CPU instance with default values
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            ix: 0,
            iy: 0,
            a_alt: 0,
            b_alt: 0,
            c_alt: 0,
            d_alt: 0,
            e_alt: 0,
            h_alt: 0,
            l_alt: 0,
            f_alt: 0,
            iff1: false,
            iff2: false,
            im: 0,
            memory: vec![0; 65536], // Initialize 64KB of memory
        }
    }

    /// Reads a byte from memory at the given address
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Writes a byte to memory at the given address
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    /// Increments the program counter
    pub fn increment_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }

    // Helper functions for flag operations
    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.f |= flag;
        } else {
            self.f &= !flag;
        }
    }

    fn get_flag(&self, flag: u8) -> bool {
        (self.f & flag) != 0
    }

    /// Performs the ADD A, r operation
    ///
    /// This function implements the ADD instruction for the Z80 CPU.
    /// It adds the value to the accumulator (A register) and updates the flags accordingly.
    ///
    /// Flags affected:
    /// - S is set if result is negative; otherwise, it is reset.
    /// - Z is set if result is 0; otherwise, it is reset.
    /// - H is set if carry from bit 3; otherwise, it is reset.
    /// - P/V is set if overflow; otherwise, it is reset.
    /// - N is reset.
    /// - C is set if carry from bit 7; otherwise, it is reset.
    pub fn add_a(&mut self, value: u8) {
        let a = self.a;
        let result = a.wrapping_add(value);

        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (a & 0x0F) + (value & 0x0F) > 0x0F);
        self.set_flag(FLAG_PV, (a ^ value ^ 0x80) & (a ^ result) & 0x80 != 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (a as u16) + (value as u16) > 0xFF);

        self.a = result;
    }

    /// ADD A, r
    pub fn add_a_r(&mut self, r: u8) {
        self.add_a(r);
    }

    /// ADD A, n
    pub fn add_a_n(&mut self, n: u8) {
        self.add_a(n);
    }

    /// ADD A, (HL)
    pub fn add_a_hl(&mut self) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.add_a(value);
    }

    /// Performs the SUB A, r operation
    ///
    /// This function implements the SUB instruction for the Z80 CPU.
    /// It subtracts the value from the accumulator (A register) and updates the flags accordingly.
    ///
    /// Flags affected:
    /// - S is set if result is negative; otherwise, it is reset.
    /// - Z is set if result is 0; otherwise, it is reset.
    /// - H is set if borrow from bit 4; otherwise, it is reset.
    /// - P/V is set if overflow; otherwise, it is reset.
    /// - N is set.
    /// - C is set if borrow; otherwise, it is reset.
    pub fn sub_a(&mut self, value: u8) {
        let a = self.a;
        let result = a.wrapping_sub(value);

        self.set_flag(FLAG_S, result & 0x80 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (a & 0x0F) < (value & 0x0F));
        self.set_flag(FLAG_PV, (a ^ value) & (a ^ result) & 0x80 != 0);
        self.set_flag(FLAG_N, true);
        self.set_flag(FLAG_C, a < value);

        self.a = result;
    }

    /// SUB A, r
    pub fn sub_a_r(&mut self, r: u8) {
        self.sub_a(r);
    }

    /// SUB A, n
    pub fn sub_a_n(&mut self, n: u8) {
        self.sub_a(n);
    }

    /// SUB A, (HL)
    pub fn sub_a_hl(&mut self) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.sub_a(value);
    }

    /// Performs the AND A, r operation
    ///
    /// This function implements the AND instruction for the Z80 CPU.
    /// It performs a bitwise AND between the accumulator (A register) and the value,
    /// storing the result in the accumulator.
    ///
    /// Flags affected:
    /// - S is set if result is negative; otherwise, it is reset.
    /// - Z is set if result is 0; otherwise, it is reset.
    /// - H is set.
    /// - P/V is set if parity even; otherwise, it is reset.
    /// - N is reset.
    /// - C is reset.
    pub fn and_a(&mut self, value: u8) {
        self.a &= value;

        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_H, true);
        self.set_flag(FLAG_PV, self.a.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, false);
    }

    /// Performs the OR A, r operation
    ///
    /// This function implements the OR instruction for the Z80 CPU.
    /// It performs a bitwise OR between the accumulator (A register) and the value,
    /// storing the result in the accumulator.
    ///
    /// Flags affected:
    /// - S is set if result is negative; otherwise, it is reset.
    /// - Z is set if result is 0; otherwise, it is reset.
    /// - H is reset.
    /// - P/V is set if parity even; otherwise, it is reset.
    /// - N is reset.
    /// - C is reset.
    pub fn or_a(&mut self, value: u8) {
        self.a |= value;

        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, self.a.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, false);
    }

    /// Performs the XOR A, r operation
    ///
    /// This function implements the XOR instruction for the Z80 CPU.
    /// It performs a bitwise XOR between the accumulator (A register) and the value,
    /// storing the result in the accumulator.
    ///
    /// Flags affected:
    /// - S is set if result is negative; otherwise, it is reset.
    /// - Z is set if result is 0; otherwise, it is reset.
    /// - H is reset.
    /// - P/V is set if parity even; otherwise, it is reset.
    /// - N is reset.
    /// - C is reset.
    pub fn xor_a(&mut self, value: u8) {
        self.a ^= value;

        self.set_flag(FLAG_S, self.a & 0x80 != 0);
        self.set_flag(FLAG_Z, self.a == 0);
        self.set_flag(FLAG_H, false);
        self.set_flag(FLAG_PV, self.a.count_ones() % 2 == 0);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, false);
    }

    // Implement specific versions for each operation
    pub fn and_a_r(&mut self, r: u8) {
        self.and_a(r);
    }

    pub fn and_a_n(&mut self, n: u8) {
        self.and_a(n);
    }

    pub fn and_a_hl(&mut self) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.and_a(value);
    }

    pub fn or_a_r(&mut self, r: u8) {
        self.or_a(r);
    }

    pub fn or_a_n(&mut self, n: u8) {
        self.or_a(n);
    }

    pub fn or_a_hl(&mut self) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.or_a(value);
    }

    pub fn xor_a_r(&mut self, r: u8) {
        self.xor_a(r);
    }

    pub fn xor_a_n(&mut self, n: u8) {
        self.xor_a(n);
    }

    pub fn xor_a_hl(&mut self) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.xor_a(value);
    }

    // Helper method to get the value of HL register pair
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    /// LD r, r' - Load register r with value from register r'
    pub fn ld_r_r(&mut self, dest: Register, src: Register) {
        let value = self.read_register(src);
        self.write_register(dest, value);
    }

    /// LD r, n - Load register r with value n
    pub fn ld_r_n(&mut self, dest: Register, n: u8) {
        self.write_register(dest, n);
    }

    /// LD r, (HL) - Load register r with value from memory address (HL)
    pub fn ld_r_hl(&mut self, dest: Register) {
        let address = self.get_hl();
        let value = self.read_byte(address);
        self.write_register(dest, value);
    }

    /// LD (HL), r - Store value from register r into memory address (HL)
    pub fn ld_hl_r(&mut self, src: Register) {
        let value = self.read_register(src);
        let address = self.get_hl();
        self.write_byte(address, value);
    }

    /// LD (HL), n - Store value n into memory address (HL)
    pub fn ld_hl_n(&mut self, n: u8) {
        let address = self.get_hl();
        self.write_byte(address, n);
    }

    /// LD A, (BC)/(DE)/(nn) - Load A with value from memory address in BC, DE, or nn
    pub fn ld_a_bc_de_nn(&mut self, address: u16) {
        self.a = self.read_byte(address);
    }

    /// LD (BC)/(DE)/(nn), A - Store value from A into memory address in BC, DE, or nn
    pub fn ld_bc_de_nn_a(&mut self, address: u16) {
        self.write_byte(address, self.a);
    }

    /// LD dd, nn - Load 16-bit register pair dd with value nn
    pub fn ld_dd_nn(&mut self, dest: RegisterPair, nn: u16) {
        self.write_register_pair(dest, nn);
    }

    /// LD IX/IY, nn - Load IX or IY with value nn
    pub fn ld_ix_iy_nn(&mut self, dest: IndexRegister, nn: u16) {
        match dest {
            IndexRegister::IX => self.ix = nn,
            IndexRegister::IY => self.iy = nn,
        }
    }

    /// LD SP, HL/IX/IY - Load SP with value from HL, IX, or IY
    pub fn ld_sp_hl_ix_iy(&mut self, src: RegisterPair) {
        self.sp = self.read_register_pair(src);
    }

    /// PUSH qq - Push register pair qq onto stack
    pub fn push_qq(&mut self, src: RegisterPair) {
        let value = self.read_register_pair(src);
        self.sp = self.sp.wrapping_sub(2);
        self.write_word(self.sp, value);
    }

    /// POP qq - Pop value from stack into register pair qq
    pub fn pop_qq(&mut self, dest: RegisterPair) {
        let value = self.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        self.write_register_pair(dest, value);
    }

    // Helper method to write a 16-bit word to memory
    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }

    // Helper method to read a 16-bit word from memory
    fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    // Helper methods for register operations
    fn read_register(&self, reg: Register) -> u8 {
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

    fn write_register(&mut self, reg: Register, value: u8) {
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

    // Helper methods for register pair operations
    fn read_register_pair(&self, pair: RegisterPair) -> u16 {
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

    fn write_register_pair(&mut self, pair: RegisterPair, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;
        match pair {
            RegisterPair::BC => {
                self.b = high;
                self.c = low;
            }
            RegisterPair::DE => {
                self.d = high;
                self.e = low;
            }
            RegisterPair::HL => {
                self.h = high;
                self.l = low;
            }
            RegisterPair::AF => {
                self.a = high;
                self.f = low;
            }
            RegisterPair::SP => self.sp = value,
            RegisterPair::IX => self.ix = value,
            RegisterPair::IY => self.iy = value,
        }
    }
}

// Flag bit positions
const FLAG_C: u8 = 0x01; // Carry Flag
const FLAG_N: u8 = 0x02; // Add/Subtract
const FLAG_PV: u8 = 0x04; // Parity/Overflow
const FLAG_H: u8 = 0x10; // Half Carry
const FLAG_Z: u8 = 0x40; // Zero
const FLAG_S: u8 = 0x80; // Sign

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(Clone, Copy)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    AF,
    SP,
    IX,
    IY,
}

#[derive(Clone, Copy)]
pub enum IndexRegister {
    IX,
    IY,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::new();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.f, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.memory.len(), 65536);
    }

    #[test]
    fn test_memory_operations() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x1000, 0x42);
        assert_eq!(cpu.read_byte(0x1000), 0x42);
    }

    #[test]
    fn test_pc_increment() {
        let mut cpu = Cpu::new();
        cpu.increment_pc(5);
        assert_eq!(cpu.pc, 5);
        cpu.increment_pc(65535);
        assert_eq!(cpu.pc, 4); // Should wrap around
    }

    #[test]
    fn test_flag_operations() {
        let mut cpu = Cpu::new();
        cpu.set_flag(FLAG_Z, true);
        assert!(cpu.get_flag(FLAG_Z));
        cpu.set_flag(FLAG_Z, false);
        assert!(!cpu.get_flag(FLAG_Z));
    }

    #[test]
    fn test_add_a() {
        let mut cpu = Cpu::new();

        // Test basic addition
        cpu.a = 5;
        cpu.add_a(3);
        assert_eq!(cpu.a, 8);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_PV));

        // Test addition resulting in zero
        cpu.a = 0;
        cpu.add_a(0);
        assert_eq!(cpu.a, 0);
        assert!(cpu.get_flag(FLAG_Z));

        // Test addition with carry
        cpu.a = 255;
        cpu.add_a(1);
        assert_eq!(cpu.a, 0);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_C));

        // Test addition causing half-carry
        cpu.a = 0x0F;
        cpu.add_a(1);
        assert_eq!(cpu.a, 0x10);
        assert!(cpu.get_flag(FLAG_H));

        // Test addition causing overflow
        cpu.a = 127;
        cpu.add_a(1);
        assert_eq!(cpu.a, 128);
        assert!(cpu.get_flag(FLAG_PV));
        assert!(cpu.get_flag(FLAG_S));

        // Test addition with negative result
        cpu.a = 250;
        cpu.add_a(10);
        assert_eq!(cpu.a, 4);
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_S));
    }

    #[test]
    fn test_add_a_r() {
        let mut cpu = Cpu::new();
        cpu.a = 5;
        cpu.b = 3;
        cpu.add_a_r(cpu.b);
        assert_eq!(cpu.a, 8);
    }

    #[test]
    fn test_add_a_n() {
        let mut cpu = Cpu::new();
        cpu.a = 5;
        cpu.add_a_n(3);
        assert_eq!(cpu.a, 8);
    }

    #[test]
    fn test_add_a_hl() {
        let mut cpu = Cpu::new();
        cpu.a = 5;
        cpu.h = 0x10;
        cpu.l = 0x00;
        cpu.write_byte(0x1000, 3);
        cpu.add_a_hl();
        assert_eq!(cpu.a, 8);
    }

    #[test]
    fn test_sub_a() {
        let mut cpu = Cpu::new();

        // Test basic subtraction
        cpu.a = 10;
        cpu.sub_a(3);
        assert_eq!(cpu.a, 7);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_PV));
        assert!(cpu.get_flag(FLAG_N));

        // Test subtraction resulting in zero
        cpu.a = 5;
        cpu.sub_a(5);
        assert_eq!(cpu.a, 0);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_N));

        // Test subtraction with borrow
        cpu.a = 3;
        cpu.sub_a(5);
        assert_eq!(cpu.a, 254);
        assert!(cpu.get_flag(FLAG_S));
        assert!(cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_N));

        // Test subtraction causing half-borrow
        cpu.a = 0x10;
        cpu.sub_a(1);
        assert_eq!(cpu.a, 0x0F);
        assert!(cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_N));

        // Test subtraction causing overflow
        cpu.a = 127;
        cpu.sub_a(255);
        assert_eq!(cpu.a, 128);
        assert!(cpu.get_flag(FLAG_PV));
        assert!(cpu.get_flag(FLAG_S));
        assert!(cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_sub_a_r() {
        let mut cpu = Cpu::new();
        cpu.a = 10;
        cpu.b = 3;
        cpu.sub_a_r(cpu.b);
        assert_eq!(cpu.a, 7);
        assert!(cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_sub_a_n() {
        let mut cpu = Cpu::new();
        cpu.a = 10;
        cpu.sub_a_n(3);
        assert_eq!(cpu.a, 7);
        assert!(cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_sub_a_hl() {
        let mut cpu = Cpu::new();
        cpu.a = 10;
        cpu.h = 0x10;
        cpu.l = 0x00;
        cpu.write_byte(0x1000, 3);
        cpu.sub_a_hl();
        assert_eq!(cpu.a, 7);
        assert!(cpu.get_flag(FLAG_N));
    }

    #[test]
    fn test_register_operations() {
        let mut cpu = Cpu::new();

        // Test 8-bit registers
        cpu.a = 0xAA;
        assert_eq!(cpu.a, 0xAA);

        // Test 16-bit registers
        cpu.ix = 0xBBCC;
        assert_eq!(cpu.ix, 0xBBCC);

        // Test alternate registers
        cpu.a_alt = 0xDD;
        assert_eq!(cpu.a_alt, 0xDD);
    }

    #[test]
    fn test_flag_operations_extended() {
        let mut cpu = Cpu::new();

        // Test all flags
        for &flag in &[FLAG_C, FLAG_N, FLAG_PV, FLAG_H, FLAG_Z, FLAG_S] {
            cpu.set_flag(flag, true);
            assert!(cpu.get_flag(flag));
            cpu.set_flag(flag, false);
            assert!(!cpu.get_flag(flag));
        }

        // Test combination of flags
        cpu.set_flag(FLAG_Z | FLAG_C, true);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_C));
        assert!(!cpu.get_flag(FLAG_S));
    }

    #[test]
    fn test_memory_operations_extended() {
        let mut cpu = Cpu::new();

        // Test edge addresses
        cpu.write_byte(0x0000, 0x42);
        assert_eq!(cpu.read_byte(0x0000), 0x42);

        cpu.write_byte(0xFFFF, 0x24);
        assert_eq!(cpu.read_byte(0xFFFF), 0x24);
    }

    #[test]
    fn test_interrupt_related() {
        let mut cpu = Cpu::new();

        cpu.iff1 = true;
        assert!(cpu.iff1);

        cpu.iff2 = false;
        assert!(!cpu.iff2);

        cpu.im = 2;
        assert_eq!(cpu.im, 2);
    }

    #[test]
    fn test_add_a_extended() {
        let mut cpu = Cpu::new();

        // Test ADD A with IX high byte
        cpu.a = 0x10;
        cpu.ix = 0x2030;
        cpu.add_a((cpu.ix >> 8) as u8);
        assert_eq!(cpu.a, 0x30);

        // Test ADD A with overflow and all flags
        cpu.a = 0x80;
        cpu.add_a(0x80);
        assert_eq!(cpu.a, 0);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_C));
        assert!(cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_H)); // Changed this assertion
        assert!(!cpu.get_flag(FLAG_N));

        // Test half-carry
        cpu.a = 0x0F;
        cpu.add_a(0x01);
        assert_eq!(cpu.a, 0x10);
        assert!(cpu.get_flag(FLAG_H));
        assert!(!cpu.get_flag(FLAG_C));
    }

    #[test]
    fn test_and_a() {
        let mut cpu = Cpu::new();

        cpu.a = 0b11001100;
        cpu.and_a(0b10101010);
        assert_eq!(cpu.a, 0b10001000);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_S)); // Changed: The result is negative (bit 7 is set)
        assert!(cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_PV)); // Changed: The result has even parity
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));

        cpu.a = 0xFF;
        cpu.and_a(0x00);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_S)); // The result is zero, so S flag should be reset
        assert!(cpu.get_flag(FLAG_PV));
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
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));

        cpu.a = 0x00;
        cpu.or_a(0x00);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_xor_a() {
        let mut cpu = Cpu::new();

        cpu.a = 0b11001100;
        cpu.xor_a(0b10101010);
        assert_eq!(cpu.a, 0b01100110);
        assert!(!cpu.get_flag(FLAG_Z));
        assert!(!cpu.get_flag(FLAG_S));
        assert!(!cpu.get_flag(FLAG_H));
        assert!(cpu.get_flag(FLAG_PV));
        assert!(!cpu.get_flag(FLAG_N));
        assert!(!cpu.get_flag(FLAG_C));

        cpu.a = 0xFF;
        cpu.xor_a(0xFF);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(FLAG_Z));
        assert!(cpu.get_flag(FLAG_PV));
    }

    #[test]
    fn test_load_instructions() {
        let mut cpu = Cpu::new();

        // Test LD r, r'
        cpu.b = 0x12;
        cpu.ld_r_r(Register::C, Register::B);
        assert_eq!(cpu.c, 0x12);

        // Test LD r, n
        cpu.ld_r_n(Register::D, 0x34);
        assert_eq!(cpu.d, 0x34);

        // Test LD r, (HL)
        cpu.h = 0x56;
        cpu.l = 0x78;
        cpu.write_byte(0x5678, 0x9A);
        cpu.ld_r_hl(Register::E);
        assert_eq!(cpu.e, 0x9A);

        // Test LD (HL), r
        cpu.a = 0xBC;
        cpu.ld_hl_r(Register::A);
        assert_eq!(cpu.read_byte(0x5678), 0xBC);

        // Test LD (HL), n
        cpu.ld_hl_n(0xDE);
        assert_eq!(cpu.read_byte(0x5678), 0xDE);

        // Test LD A, (BC)
        cpu.b = 0x12;
        cpu.c = 0x34;
        cpu.write_byte(0x1234, 0xEF);
        cpu.ld_a_bc_de_nn(0x1234);
        assert_eq!(cpu.a, 0xEF);

        // Test LD (DE), A
        cpu.d = 0x56;
        cpu.e = 0x78;
        cpu.a = 0xAB;
        cpu.ld_bc_de_nn_a(0x5678);
        assert_eq!(cpu.read_byte(0x5678), 0xAB);

        // Test LD dd, nn
        cpu.ld_dd_nn(RegisterPair::BC, 0xCDEF);
        assert_eq!(cpu.b, 0xCD);
        assert_eq!(cpu.c, 0xEF);

        // Test LD IX, nn
        cpu.ld_ix_iy_nn(IndexRegister::IX, 0x1122);
        assert_eq!(cpu.ix, 0x1122);

        // Test LD SP, HL
        cpu.h = 0x33;
        cpu.l = 0x44;
        cpu.ld_sp_hl_ix_iy(RegisterPair::HL);
        assert_eq!(cpu.sp, 0x3344);

        // Test PUSH qq
        cpu.sp = 0xFFFF;
        cpu.ld_dd_nn(RegisterPair::BC, 0x5566);
        cpu.push_qq(RegisterPair::BC);
        assert_eq!(cpu.sp, 0xFFFD);
        assert_eq!(cpu.read_word(0xFFFD), 0x5566);

        // Test POP qq
        cpu.pop_qq(RegisterPair::DE);
        assert_eq!(cpu.d, 0x55);
        assert_eq!(cpu.e, 0x66);
        assert_eq!(cpu.sp, 0xFFFF);
    }

    // Add more tests for and_a_r, and_a_n, and_a_hl, or_a_r, or_a_n, or_a_hl, xor_a_r, xor_a_n, xor_a_hl
}
