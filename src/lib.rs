pub mod cpu;
pub mod event;
pub mod memory;
pub mod system;
pub mod timing;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("Memory access error at address {0:#04x}")]
    MemoryError(u16),
    #[error("Invalid opcode {0:#02x}")]
    InvalidOpcode(u8),
    #[error("System error: {0}")]
    SystemError(String),
    #[error("Event processing error: {0}")]
    EventError(String),
}

pub type Result<T> = std::result::Result<T, EmulatorError>;
