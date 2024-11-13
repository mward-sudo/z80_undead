pub mod cpu;
pub mod memory;
pub mod system;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Memory access error at address {0:#04x}")]
    MemoryError(u16),
    #[error("Invalid opcode {0:#02x}")]
    InvalidOpcode(u8),
    #[error("System error: {0}")]
    SystemError(String),
}

pub type Result<T> = std::result::Result<T, EmulatorError>;
