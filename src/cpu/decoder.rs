//! Decoder module handles Z80 instruction decoding and prefix handling

use super::instruction::{create_nop, Instruction, InstructionType};
use super::tables::InstructionTables;
use crate::Result;
use std::sync::LazyLock;

/// Represents Z80 instruction prefixes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefix {
    None,
    Cb,   // Bit operations, rotates and shifts
    Dd,   // IX instructions
    Fd,   // IY instructions
    Ed,   // Extended instructions
    DdCb, // IX bit operations
    FdCb, // IY bit operations
}

/// The main instruction decoder
pub struct Decoder {
    current_prefix: Prefix,
    tables: InstructionTables,
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            current_prefix: Prefix::None,
            tables: InstructionTables::new(),
        }
    }

    /// Handles prefix bytes and updates decoder state
    pub fn handle_prefix(&mut self, opcode: u8) -> bool {
        match (self.current_prefix, opcode) {
            (Prefix::None, 0xCB) => {
                self.current_prefix = Prefix::Cb;
                true
            }
            (Prefix::None, 0xDD) => {
                self.current_prefix = Prefix::Dd;
                true
            }
            (Prefix::None, 0xFD) => {
                self.current_prefix = Prefix::Fd;
                true
            }
            (Prefix::None, 0xED) => {
                self.current_prefix = Prefix::Ed;
                true
            }
            (Prefix::Dd, 0xCB) => {
                self.current_prefix = Prefix::DdCb;
                true
            }
            (Prefix::Fd, 0xCB) => {
                self.current_prefix = Prefix::FdCb;
                true
            }
            _ => false,
        }
    }

    /// Decodes an opcode based on the current prefix state
    pub fn decode(&mut self, opcode: u8) -> Result<Instruction> {
        // First check if this is a prefix byte
        if self.handle_prefix(opcode) {
            return Ok(PREFIX_INSTRUCTION.clone());
        }

        // Look up the instruction based on current prefix
        match self.current_prefix {
            Prefix::None => self
                .tables
                .lookup_main(opcode)
                .cloned()
                .ok_or(crate::EmulatorError::InvalidOpcode(opcode)),
            Prefix::Cb => self
                .tables
                .lookup_cb(opcode)
                .cloned()
                .ok_or(crate::EmulatorError::InvalidOpcode(opcode)),
            Prefix::Dd | Prefix::Fd => self
                .tables
                .lookup_ddfd(opcode)
                .cloned()
                .ok_or(crate::EmulatorError::InvalidOpcode(opcode)),
            Prefix::Ed => self
                .tables
                .lookup_ed(opcode)
                .cloned()
                .ok_or(crate::EmulatorError::InvalidOpcode(opcode)),
            _ => Err(crate::EmulatorError::InvalidOpcode(opcode)),
        }
    }
}

static PREFIX_INSTRUCTION: LazyLock<Instruction> =
    LazyLock::new(|| Instruction::new("PREFIX", 1, 4, InstructionType::Special, create_nop()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_decoding() {
        let mut decoder = Decoder::new();
        let instruction = decoder.decode(0x00).unwrap();

        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.mnemonic, "NOP");
        assert_eq!(instruction.instruction_type, InstructionType::Control);
        assert_eq!(instruction.t_states, 4);
    }

    #[test]
    fn test_invalid_opcode() {
        let mut decoder = Decoder::new();
        let result = decoder.decode(0xFF);
        assert!(matches!(
            result,
            Err(crate::EmulatorError::InvalidOpcode(0xFF))
        ));
    }

    #[test]
    fn test_prefix_handling() {
        let mut decoder = Decoder::new();

        assert!(decoder.handle_prefix(0xCB));
        assert_eq!(decoder.current_prefix, Prefix::Cb);

        decoder.current_prefix = Prefix::None;
        assert!(decoder.handle_prefix(0xDD));
        assert_eq!(decoder.current_prefix, Prefix::Dd);

        assert!(decoder.handle_prefix(0xCB));
        assert_eq!(decoder.current_prefix, Prefix::DdCb);
    }
}
