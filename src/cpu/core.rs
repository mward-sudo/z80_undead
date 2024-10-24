pub struct Cpu {
    // 8-bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub i: u8,
    pub l: u8,
    pub f: u8, // Flag register

    // 16-bit registers
    pub pc: u16, // Program Counter
    pub sp: u16, // Stack Pointer
    pub ix: u16,
    pub iy: u16,
    pub di: u16,

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

    pub halted: bool,

    pub interrupt_mode: u8,

    // Timing state
    pub t_states: u64,        // Total executed T-states (clock cycles)
    pub m_cycles: u64,        // Total executed M-cycles (machine cycles)
    pub current_t_states: u8, // T-states for the current instruction
    pub current_m_cycles: u8, // M-cycles for the current instruction
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
            i: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            di: 0,
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
            halted: false,
            interrupt_mode: 0,
            t_states: 0,
            m_cycles: 0,
            current_t_states: 0,
            current_m_cycles: 0,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.add_t_states(3); // Memory read requires 3 T-states
        self.add_m_cycle(); // Memory access constitutes one M-cycle
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.add_t_states(3); // Memory write requires 3 T-states
        self.add_m_cycle(); // Memory access constitutes one M-cycle
        self.memory[address as usize] = value;
    }

    pub fn increment_pc(&mut self, amount: u16) {
        self.pc = self.pc.wrapping_add(amount);
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn step(&mut self) {
        if self.halted {
            self.add_t_states(4); // Halted state consumes 4 T-states per cycle
            return;
        }

        self.reset_instruction_timing();
        let opcode = self.fetch_byte();
        self.execute(opcode);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.nop(),
            0x01 => {
                let value = self.fetch_word();
                self.ld_bc(value);
            }
            // ... implement other opcodes
            0x76 => self.halt(),
            _ => panic!("Unimplemented opcode: 0x{:02X}", opcode),
        }
    }

    fn fetch_word(&mut self) -> u16 {
        let low = self.fetch_byte() as u16;
        let high = self.fetch_byte() as u16;
        (high << 8) | low
    }

    fn ld_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    // Timing management methods
    pub fn add_t_states(&mut self, states: u8) {
        self.current_t_states += states;
        self.t_states += states as u64;
    }

    pub fn add_m_cycle(&mut self) {
        self.current_m_cycles += 1;
        self.m_cycles += 1;
    }

    pub fn reset_instruction_timing(&mut self) {
        self.current_t_states = 0;
        self.current_m_cycles = 0;
    }

    /// Returns current timing information as (total_t_states, total_m_cycles, current_t_states, current_m_cycles)
    pub fn get_timing_info(&self) -> (u64, u64, u8, u8) {
        (
            self.t_states,
            self.m_cycles,
            self.current_t_states,
            self.current_m_cycles,
        )
    }

    /// Waits until the specified number of T-states have elapsed
    pub fn wait_states(&mut self, states: u8) {
        self.add_t_states(states);
    }

    /// Synchronizes the CPU with external hardware by consuming remaining T-states in current M-cycle
    pub fn sync_m_cycle(&mut self) {
        let remaining = 4 - (self.current_t_states % 4);
        if remaining < 4 {
            self.add_t_states(remaining);
        }
    }

    /// Returns the number of T-states elapsed in the current M-cycle
    pub fn t_states_in_m_cycle(&self) -> u8 {
        self.current_t_states % 4
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

    #[test]
    fn test_timing_basic_memory_operations() {
        let mut cpu = Cpu::new();

        // Test memory read timing
        cpu.read_byte(0x1000);
        assert_eq!(cpu.current_t_states, 3);
        assert_eq!(cpu.current_m_cycles, 1);

        // Test memory write timing
        cpu.reset_instruction_timing();
        cpu.write_byte(0x1000, 0x42);
        assert_eq!(cpu.current_t_states, 3);
        assert_eq!(cpu.current_m_cycles, 1);
    }

    #[test]
    fn test_timing_fetch_operations() {
        let mut cpu = Cpu::new();

        // Test byte fetch timing
        cpu.fetch_byte();
        assert_eq!(cpu.current_t_states, 3);
        assert_eq!(cpu.current_m_cycles, 1);

        // Test word fetch timing (should be 6 T-states, 2 M-cycles)
        cpu.reset_instruction_timing();
        cpu.fetch_word();
        assert_eq!(cpu.current_t_states, 6);
        assert_eq!(cpu.current_m_cycles, 2);
    }

    #[test]
    fn test_timing_counters() {
        let mut cpu = Cpu::new();

        // Test accumulation of T-states and M-cycles
        cpu.add_t_states(4);
        cpu.add_m_cycle();
        assert_eq!(cpu.t_states, 4);
        assert_eq!(cpu.m_cycles, 1);

        cpu.add_t_states(3);
        cpu.add_m_cycle();
        assert_eq!(cpu.t_states, 7);
        assert_eq!(cpu.m_cycles, 2);
    }

    #[test]
    fn test_timing_reset() {
        let mut cpu = Cpu::new();

        cpu.add_t_states(4);
        cpu.add_m_cycle();
        cpu.reset_instruction_timing();

        assert_eq!(cpu.current_t_states, 0);
        assert_eq!(cpu.current_m_cycles, 0);
        // Total counts should remain unchanged
        assert_eq!(cpu.t_states, 4);
        assert_eq!(cpu.m_cycles, 1);
    }

    #[test]
    fn test_wait_states() {
        let mut cpu = Cpu::new();
        cpu.wait_states(2);
        assert_eq!(cpu.current_t_states, 2);
        assert_eq!(cpu.t_states, 2);
    }

    #[test]
    fn test_sync_m_cycle() {
        let mut cpu = Cpu::new();

        // Add 1 T-state
        cpu.add_t_states(1);
        cpu.sync_m_cycle();
        assert_eq!(cpu.current_t_states, 4);

        // Add 2 T-states
        cpu.reset_instruction_timing();
        cpu.add_t_states(2);
        cpu.sync_m_cycle();
        assert_eq!(cpu.current_t_states, 4);

        // Add 3 T-states
        cpu.reset_instruction_timing();
        cpu.add_t_states(3);
        cpu.sync_m_cycle();
        assert_eq!(cpu.current_t_states, 4);

        // Add 4 T-states (already aligned)
        cpu.reset_instruction_timing();
        cpu.add_t_states(4);
        cpu.sync_m_cycle();
        assert_eq!(cpu.current_t_states, 4);
    }

    #[test]
    fn test_t_states_in_m_cycle() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.t_states_in_m_cycle(), 0);

        cpu.add_t_states(1);
        assert_eq!(cpu.t_states_in_m_cycle(), 1);

        cpu.add_t_states(2);
        assert_eq!(cpu.t_states_in_m_cycle(), 3);

        cpu.add_t_states(2);
        assert_eq!(cpu.t_states_in_m_cycle(), 1);
    }
}
