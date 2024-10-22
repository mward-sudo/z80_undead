# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-03-XX

### Added
- Implemented Z80 CPU core architecture
- Added CPU instruction modules:
  - Core instruction set implementation
  - Extended (ED prefix) instructions
  - Control flow instructions
  - Miscellaneous instructions
  - Undocumented instructions support
- Interrupt handling system implementation
- CPU core testing framework
- Memory management system
- Register management system
- Flag handling implementation

### Changed
- Restructured project to use modular architecture
- Improved code organization with separate modules for different instruction types

### Deprecated
- N/A

### Removed
- Placeholder files from initial setup

### Fixed
- N/A

### Security
- N/A

## [0.0.1] - 2023-04-XX

### Added
- Initial project structure
- README.md with project overview, goals, and planned features
- CHANGELOG.md file
- LICENSE file (MIT License)
- Basic project configuration files (Cargo.toml, mix.exs)
- Placeholder files for core components:
  - src/cpu.rs
  - src/cpu/cpu_tests.rs
  - src/cpu_tests.rs
  - src/main.rs
  - lib/z80/cpu.ex

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

[Unreleased]: https://github.com/yourusername/z80-emulator/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/z80-emulator/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/yourusername/z80-emulator/releases/tag/v0.0.1
