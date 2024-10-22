pub struct Cpu {
    // 8-bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8, // Flag register

    // 16-bit registers
    pub pc: u16, // Program Counter
    pub sp: u16, // Stack Pointer
    pub ix: u16,
    pub iy: u16,

    // Alternate registers
    pub a_alt: u8,
    pub b_alt: u8,
    pub c_alt: u8,
    pub d_alt: u8,
    pub e_alt: u8,
    pub h_alt: u8,
    pub l_alt: u8,
    pub f_alt: u8,

    // Interrupt flip-flops
    pub iff1: bool,
    pub iff2: bool,

    // Interrupt mode
    pub im: u8,

    // Memory (we'll use a Vec<u8> to represent the full 64KB addressable memory)
    pub memory: Vec<u8>,
}

impl Cpu {
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

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn increment_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_word_operations() {
        let mut cpu = Cpu::new();
        cpu.write_word(0x1000, 0x1234);
        assert_eq!(cpu.read_word(0x1000), 0x1234);
        assert_eq!(cpu.read_byte(0x1000), 0x34);
        assert_eq!(cpu.read_byte(0x1001), 0x12);
    }
}
