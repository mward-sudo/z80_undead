use log::info;
use z80_undead::{system::System, Result};

fn main() -> Result<()> {
    env_logger::init();
    info!("Z80 Undead Emulator Starting...");

    let mut system = System::new();

    // Example program: just a NOP instruction
    let program = [0x00];
    system.load_program(&program)?;

    // Execute one instruction
    system.tick()?;

    info!("Emulation completed successfully");
    Ok(())
}
