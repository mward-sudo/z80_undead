//! Decoder module handles Z80 instruction decoding and prefix handling

use super::instruction::{Instruction, InstructionType};
use crate::Result;

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
        }
    }

    /// Detects and handles instruction prefixes
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

    /// Decodes a single instruction
    pub fn decode(&mut self, opcode: u8) -> Result<Instruction> {
        if self.handle_prefix(opcode) {
            // Return early if this byte was a prefix
            return Err(crate::EmulatorError::SystemError(
                "Incomplete instruction (prefix only)".to_string(),
            ));
        }

        let instruction = self.lookup_instruction(self.current_prefix, opcode)?;
        self.current_prefix = Prefix::None;
        Ok(instruction)
    }

    /// Looks up the instruction implementation based on prefix and opcode
    fn lookup_instruction(&self, prefix: Prefix, opcode: u8) -> Result<Instruction> {
        // For now, just handle NOP (0x00) with no prefix
        match (prefix, opcode) {
            (Prefix::None, 0x00) => Ok(Instruction::new(
                1,                        // length
                super::instruction::nop,  // execute fn
                "NOP",                    // mnemonic
                InstructionType::Special, // instruction type
                4,                        // t_states
            )),
            _ => Err(crate::EmulatorError::InvalidOpcode(opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_detection() {
        let mut decoder = Decoder::new();

        // Test CB prefix
        assert!(decoder.handle_prefix(0xCB));
        assert_eq!(decoder.current_prefix, Prefix::Cb);

        // Reset and test DD prefix
        decoder.current_prefix = Prefix::None;
        assert!(decoder.handle_prefix(0xDD));
        assert_eq!(decoder.current_prefix, Prefix::Dd);

        // Test DDCB prefix sequence
        assert!(decoder.handle_prefix(0xCB));
        assert_eq!(decoder.current_prefix, Prefix::DdCb);
    }

    #[test]
    fn test_nop_decoding() {
        let mut decoder = Decoder::new();
        let instruction = decoder.decode(0x00).unwrap();

        assert_eq!(instruction.length, 1);
        assert_eq!(instruction.mnemonic, "NOP");
        assert_eq!(instruction.instruction_type, InstructionType::Special);
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
}
