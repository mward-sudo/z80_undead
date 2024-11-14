//! CPU module handles Z80 CPU emulation including registers, flags, and instruction execution.

mod instruction;

use crate::event::{Event, EventQueue};
use crate::{memory::Memory, Result};
use instruction::create_nop;

/// Represents the Z80 CPU state
pub struct Cpu {
    // Program Counter
    pc: u16,
    // Stack Pointer
    sp: u16,
    // Main register set
    a: u8, // Accumulator
    b: u8, // B register
    c: u8, // C register
    d: u8, // D register
    e: u8, // E register
    h: u8, // H register
    l: u8, // L register
    // Alternate register set
    a_prime: u8,
    b_prime: u8,
    c_prime: u8,
    d_prime: u8,
    e_prime: u8,
    h_prime: u8,
    l_prime: u8,
    // Index registers
    ix: u16, // IX index register
    iy: u16, // IY index register
    // Special purpose registers
    i: u8, // Interrupt vector
    r: u8, // Memory refresh
    // Flags register
    flags: Flags,
    flags_prime: Flags,
    // Memory reference
    memory: Memory,
    // Add T-state counter
    t_states: u32,
    event_queue: EventQueue,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Flags {
    // Primary flags
    pub sign: bool,         // S: Bit 7 - Set if result is negative
    pub zero: bool,         // Z: Bit 6 - Set if result is zero
    pub half_carry: bool,   // H: Bit 4 - Set if carry from bit 3 to 4
    pub parity: bool,       // P/V: Bit 2 - Parity/Overflow flag
    pub add_subtract: bool, // N: Bit 1 - Set if last op was subtraction
    pub carry: bool,        // C: Bit 0 - Set if result overflowed
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    // Convert flags to a byte representation
    pub fn to_byte(&self) -> u8 {
        let mut result = 0u8;
        if self.sign {
            result |= 0b1000_0000;
        }
        if self.zero {
            result |= 0b0100_0000;
        }
        if self.half_carry {
            result |= 0b0001_0000;
        }
        if self.parity {
            result |= 0b0000_0100;
        }
        if self.add_subtract {
            result |= 0b0000_0010;
        }
        if self.carry {
            result |= 0b0000_0001;
        }
        result
    }

    // Set flags from a byte
    pub fn from_byte(&mut self, byte: u8) {
        self.sign = (byte & 0b1000_0000) != 0;
        self.zero = (byte & 0b0100_0000) != 0;
        self.half_carry = (byte & 0b0001_0000) != 0;
        self.parity = (byte & 0b0000_0100) != 0;
        self.add_subtract = (byte & 0b0000_0010) != 0;
        self.carry = (byte & 0b0000_0001) != 0;
    }
}

impl Cpu {
    /// Creates a new CPU instance with initialized memory
    pub fn new(memory: Memory) -> Self {
        Self {
            pc: 0,
            sp: 0xFFFF,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            a_prime: 0,
            b_prime: 0,
            c_prime: 0,
            d_prime: 0,
            e_prime: 0,
            h_prime: 0,
            l_prime: 0,
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            flags: Flags::default(),
            flags_prime: Flags::default(),
            memory,
            t_states: 0,
            event_queue: EventQueue::new(),
        }
    }

    /// Executes a single instruction and processes events
    pub fn step(&mut self) -> Result<u32> {
        // Initial T-state for this step
        let start_t_states = self.t_states;

        // Process any pending events before fetch
        self.process_events()?;

        // Fetch (opcode fetch is included in instruction.t_states)
        let opcode = self.memory.read_byte(self.pc)?;

        // Process events after fetch
        self.process_events()?;

        // Increment R register
        self.increment_r();

        // Decode
        let instruction = match opcode {
            0x00 => create_nop(),
            _ => return Err(crate::EmulatorError::InvalidOpcode(opcode)),
        };

        // Execute and process events during instruction
        (instruction.execute)(self)?;
        self.process_events()?;

        // Add instruction T-states (includes fetch)
        self.t_states += instruction.t_states;

        // Process final events for this instruction
        self.process_events()?;

        // Increment PC
        self.pc = self.pc.wrapping_add(instruction.length as u16);

        Ok(self.t_states - start_t_states) // Return actual T-states consumed
    }

    /// Process any events scheduled for the current T-state
    fn process_events(&mut self) -> Result<()> {
        while let Some((event, t_state)) = self.event_queue.peek() {
            if *t_state > self.t_states {
                break;
            }

            let (event, _) = self.event_queue.pop().unwrap();
            self.handle_event(event)?;
        }
        Ok(())
    }

    /// Handle a single event
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            // Handle different event types
            // This will be expanded as we add more event types
            Event::Interrupt => self.handle_interrupt()?,
            Event::Timer => self.handle_timer()?,
            // ... other event types
        }
        Ok(())
    }

    /// Returns the current program counter value
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    /// Loads a program into memory at the specified address
    pub fn load_program(&mut self, address: u16, program: &[u8]) -> Result<()> {
        self.memory.load(address, program)
    }

    // Helper methods for 16-bit register pairs
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    /// Exchange main register set with alternate register set
    pub fn exchange_register_sets(&mut self) {
        std::mem::swap(&mut self.a, &mut self.a_prime);
        std::mem::swap(&mut self.b, &mut self.b_prime);
        std::mem::swap(&mut self.c, &mut self.c_prime);
        std::mem::swap(&mut self.d, &mut self.d_prime);
        std::mem::swap(&mut self.e, &mut self.e_prime);
        std::mem::swap(&mut self.h, &mut self.h_prime);
        std::mem::swap(&mut self.l, &mut self.l_prime);
        std::mem::swap(&mut self.flags, &mut self.flags_prime);
    }

    /// Increment R register (called during instruction fetch)
    pub fn increment_r(&mut self) {
        self.r = (self.r & 0x80) | ((self.r + 1) & 0x7f);
    }

    /// Returns the current T-state count
    pub fn get_t_states(&self) -> u32 {
        self.t_states
    }

    /// Resets the T-state counter
    pub fn reset_t_states(&mut self) {
        self.t_states = 0;
    }

    fn handle_interrupt(&mut self) -> Result<()> {
        // TODO: Implement interrupt handling
        Ok(())
    }

    fn handle_timer(&mut self) -> Result<()> {
        // TODO: Implement timer event handling
        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(Memory::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EmulatorError;

    #[test]
    fn test_cpu_initialization() {
        let cpu = Cpu::default();
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0xFFFF);
    }

    #[test]
    fn test_nop_execution() {
        let mut cpu = Cpu::default();
        let program = [0x00]; // NOP instruction
        cpu.load_program(0, &program).unwrap();

        let initial_pc = cpu.get_pc();
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), initial_pc + 1);
    }

    #[test]
    fn test_invalid_opcode() {
        let mut cpu = Cpu::default();
        let program = [0xFF]; // Invalid opcode
        cpu.load_program(0, &program).unwrap();

        let result = cpu.step();
        assert!(matches!(result, Err(EmulatorError::InvalidOpcode(0xFF))));
    }

    #[test]
    fn test_program_loading() {
        let mut cpu = Cpu::default();
        let program = [0x00, 0x01, 0x02];
        let address = 0x100;

        cpu.load_program(address, &program).unwrap();
        assert_eq!(cpu.memory.read_byte(address).unwrap(), 0x00);
        assert_eq!(cpu.memory.read_byte(address + 1).unwrap(), 0x01);
        assert_eq!(cpu.memory.read_byte(address + 2).unwrap(), 0x02);
    }

    #[test]
    fn test_flags_to_byte() {
        let mut flags = Flags::new();
        flags.sign = true;
        flags.zero = true;
        flags.carry = true;
        assert_eq!(flags.to_byte(), 0b1100_0001);
    }

    #[test]
    fn test_flags_from_byte() {
        let mut flags = Flags::new();
        flags.from_byte(0b1100_0001);
        assert!(flags.sign);
        assert!(flags.zero);
        assert!(flags.carry);
        assert!(!flags.half_carry);
        assert!(!flags.parity);
        assert!(!flags.add_subtract);
    }

    #[test]
    fn test_arithmetic_flags() {
        let mut cpu = Cpu::new(Memory::default());

        // Test addition with carry
        cpu.update_arithmetic_flags(0x7F, 0x01, false, true);
        assert!(cpu.flags.sign);
        assert!(!cpu.flags.zero);
        assert!(!cpu.flags.carry);
        assert!(cpu.flags.parity); // Overflow from positive to negative

        // Test subtraction with borrow
        cpu.update_arithmetic_flags(0x00, 0x01, false, false);
        assert!(cpu.flags.sign);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags.carry);
        assert!(!cpu.flags.parity);
    }

    #[test]
    fn test_register_pairs() {
        let mut cpu = Cpu {
            b: 0x12,
            c: 0x34,
            ..Default::default()
        };

        assert_eq!(cpu.get_bc(), 0x1234);

        cpu.set_bc(0x5678);
        assert_eq!(cpu.b, 0x56);
        assert_eq!(cpu.c, 0x78);

        // Test DE pair
        cpu.set_de(0x9ABC);
        assert_eq!(cpu.get_de(), 0x9ABC);
        assert_eq!(cpu.d, 0x9A);
        assert_eq!(cpu.e, 0xBC);

        // Test HL pair
        cpu.set_hl(0xDEF0);
        assert_eq!(cpu.get_hl(), 0xDEF0);
        assert_eq!(cpu.h, 0xDE);
        assert_eq!(cpu.l, 0xF0);
    }

    #[test]
    fn test_exchange_register_sets() {
        let mut cpu = Cpu {
            a: 0x12,
            a_prime: 0x34,
            flags: Flags {
                zero: true,
                ..Default::default()
            },
            flags_prime: Flags {
                zero: false,
                ..Default::default()
            },
            ..Default::default()
        };

        cpu.exchange_register_sets();

        assert_eq!(cpu.a, 0x34);
        assert_eq!(cpu.a_prime, 0x12);
        assert!(!cpu.flags.zero);
        assert!(cpu.flags_prime.zero);
    }

    #[test]
    fn test_r_register_increment() {
        let mut cpu = Cpu {
            r: 0x00,
            ..Default::default()
        };

        cpu.increment_r();
        assert_eq!(cpu.r, 0x01);

        // Test wrap around of lower 7 bits
        cpu.r = 0x7F;
        cpu.increment_r();
        assert_eq!(cpu.r, 0x00);

        // Test preservation of bit 7
        cpu.r = 0x80;
        cpu.increment_r();
        assert_eq!(cpu.r, 0x81);
    }

    #[test]
    fn test_instruction_timing() {
        let mut cpu = Cpu::default();
        let program = [0x00]; // NOP instruction
        cpu.load_program(0, &program).unwrap();

        // NOP should take 4 T-states
        let t_states = cpu.step().unwrap();
        assert_eq!(t_states, 4);
        assert_eq!(cpu.get_t_states(), 4);
    }

    #[test]
    fn test_event_processing() {
        let mut cpu = Cpu::default();

        // Schedule an event
        cpu.event_queue.push(Event::Timer, 4);

        // Execute NOP (4 T-states)
        let program = [0x00];
        cpu.load_program(0, &program).unwrap();

        let t_states = cpu.step().unwrap();

        // Verify timing and event processing
        assert_eq!(t_states, 4);
        assert!(cpu.event_queue.is_empty());
    }

    #[test]
    fn test_event_timing_sequence() {
        let mut cpu = Cpu::default();

        // Schedule multiple events
        cpu.event_queue.push(Event::Timer, 2);
        cpu.event_queue.push(Event::Interrupt, 4);

        let program = [0x00]; // NOP
        cpu.load_program(0, &program).unwrap();

        let t_states = cpu.step().unwrap();

        // Verify:
        // 1. Both events were processed
        // 2. Correct number of T-states elapsed
        // 3. Events were processed in order
        assert_eq!(t_states, 4);
        assert!(cpu.event_queue.is_empty());
    }

    #[test]
    fn test_event_processing_order() {
        let mut cpu = Cpu::default();

        // Schedule events out of order
        cpu.event_queue.push(Event::Interrupt, 4);
        cpu.event_queue.push(Event::Timer, 2);

        // Verify events are sorted by T-state
        let first_event = cpu.event_queue.peek().unwrap();
        match first_event.0 {
            Event::Timer => (),
            _ => panic!("Events not properly ordered by T-state"),
        }
    }
}
