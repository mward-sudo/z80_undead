# Z80 Emulator

## Project Overview

This project implements an accurate emulator for the Zilog Z80 processor. The Z80 was a popular 8-bit microprocessor used in many classic computers and gaming systems. This emulator serves as both a learning tool and a foundation for future retro-computing projects.

## Current Version

**Alpha v0.1.0**

## Current State

### Implemented Features

1. **Complete Z80 Instruction Set**
   - All documented Z80 instructions
   - All undocumented Z80 instructions
   - Full IX/IY register support
   - Comprehensive flag handling

2. **CPU Core**
   - Complete register set including alternates
   - All addressing modes
   - Full interrupt handling (IM 0, 1, 2)
   - Non-maskable interrupt support

3. **Memory System**
   - Full 64KB addressable memory
   - Memory read/write interface
   - Basic I/O port simulation

4. **Instruction Categories**
   - 8-bit and 16-bit load operations
   - 8-bit and 16-bit arithmetic
   - Bit manipulation instructions
   - Block transfer and search operations
   - Jump, call, and return instructions
   - I/O operations
   - Exchange, block transfer, and search instructions
   - Rotate and shift instructions

5. **Testing**
   - Comprehensive test suite for all instructions
   - Flag behavior verification
   - Edge case testing
   - Interrupt handling tests

### Next Development Phase

1. **Timing Implementation**
   - Add cycle-accurate timing
   - Implement T-state counting
   - Add memory wait states

2. **Instruction Decoder**
   - Implement full instruction decoder
   - Add opcode fetch cycle
   - Support all instruction prefixes

3. **System Integration**
   - Memory-mapped I/O support
   - Peripheral device interface
   - System bus emulation

## Project Structure

## Goals

1. **Accurate Z80 Core Emulation**
   - Implement the complete Z80 instruction set
   - Emulate all Z80 registers and flag behaviors
   - Support all addressing modes and interrupt handling

2. **Cycle-Accurate Timing**
   - Ensure proper instruction timing
   - Implement adjustable clock speeds

3. **Flexible Memory Interface**
   - Support full 64KB addressable memory
   - Provide hooks for memory-mapped I/O

4. **Debugging Capabilities**
   - Step-through execution
   - Register and memory inspection
   - Breakpoint support

5. **Extensibility**
   - Design with future full system emulation in mind
   - Prepare interfaces for peripherals and I/O

6. **Testing and Validation**
   - Comprehensive unit and integration testing
   - Comparison with known-good Z80 implementations

## Planned Features

- Complete Z80 instruction set implementation
- Accurate register set emulation, including alternate registers
- Support for all Z80 interrupt modes
- Flexible memory read/write interface
- Basic I/O system for future expansion
- Debugger interface for step-by-step execution
- Configurable initial state and binary loading
- Comprehensive test suite

## Future Expansion

- Full Z80-based computer system emulation
- Performance optimizations
- Potential GUI for easier interaction and visualization

## Contributing

This project is currently in its early stages. Contributions, suggestions, and feedback are welcome! Please feel free to open issues or submit pull requests.

## Version Control

This project uses semantic versioning. The version number format is MAJOR.MINOR.PATCH.
- Alpha releases are denoted by the PATCH version number.
- The latest stable release can be found on the `main` branch.
- Development work is done on feature branches and merged into `develop`.

To access a specific version, you can use git tags. For example:

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project draws inspiration from various Z80 emulation projects and resources. Specific acknowledgements will be added as the project progresses.
