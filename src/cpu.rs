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
}

// Flag bit positions
const FLAG_C: u8 = 0x01; // Carry Flag
const FLAG_N: u8 = 0x02; // Add/Subtract
const FLAG_PV: u8 = 0x04; // Parity/Overflow
const FLAG_H: u8 = 0x10; // Half Carry
const FLAG_Z: u8 = 0x40; // Zero
const FLAG_S: u8 = 0x80; // Sign

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
}

// Add this at the end of the file, after the existing tests module
#[cfg(test)]
mod cpu_tests;
