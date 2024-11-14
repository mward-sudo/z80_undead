//! Instruction tables for the Z80 CPU

use super::instruction::create_nop;
use super::instruction::{ExecuteFn, Instruction, InstructionType};
use std::collections::HashMap;

// Static string tables for instruction mnemonics
const BIT_MNEMONICS: [[&str; 8]; 8] = [
    [
        "BIT 0, B",
        "BIT 0, C",
        "BIT 0, D",
        "BIT 0, E",
        "BIT 0, H",
        "BIT 0, L",
        "BIT 0, (HL)",
        "BIT 0, A",
    ],
    [
        "BIT 1, B",
        "BIT 1, C",
        "BIT 1, D",
        "BIT 1, E",
        "BIT 1, H",
        "BIT 1, L",
        "BIT 1, (HL)",
        "BIT 1, A",
    ],
    [
        "BIT 2, B",
        "BIT 2, C",
        "BIT 2, D",
        "BIT 2, E",
        "BIT 2, H",
        "BIT 2, L",
        "BIT 2, (HL)",
        "BIT 2, A",
    ],
    [
        "BIT 3, B",
        "BIT 3, C",
        "BIT 3, D",
        "BIT 3, E",
        "BIT 3, H",
        "BIT 3, L",
        "BIT 3, (HL)",
        "BIT 3, A",
    ],
    [
        "BIT 4, B",
        "BIT 4, C",
        "BIT 4, D",
        "BIT 4, E",
        "BIT 4, H",
        "BIT 4, L",
        "BIT 4, (HL)",
        "BIT 4, A",
    ],
    [
        "BIT 5, B",
        "BIT 5, C",
        "BIT 5, D",
        "BIT 5, E",
        "BIT 5, H",
        "BIT 5, L",
        "BIT 5, (HL)",
        "BIT 5, A",
    ],
    [
        "BIT 6, B",
        "BIT 6, C",
        "BIT 6, D",
        "BIT 6, E",
        "BIT 6, H",
        "BIT 6, L",
        "BIT 6, (HL)",
        "BIT 6, A",
    ],
    [
        "BIT 7, B",
        "BIT 7, C",
        "BIT 7, D",
        "BIT 7, E",
        "BIT 7, H",
        "BIT 7, L",
        "BIT 7, (HL)",
        "BIT 7, A",
    ],
];

// Add these new constant tables for IX/IY instructions
const IX_LOAD_MNEMONICS: [&str; 8] = [
    "LD B, (IX+d)",
    "LD C, (IX+d)",
    "LD D, (IX+d)",
    "LD E, (IX+d)",
    "LD H, (IX+d)",
    "LD L, (IX+d)",
    "LD (IX+d), r", // Placeholder for store
    "LD A, (IX+d)",
];

const IY_LOAD_MNEMONICS: [&str; 8] = [
    "LD B, (IY+d)",
    "LD C, (IY+d)",
    "LD D, (IY+d)",
    "LD E, (IY+d)",
    "LD H, (IY+d)",
    "LD L, (IY+d)",
    "LD (IY+d), r", // Placeholder for store
    "LD A, (IY+d)",
];

// Add these new constant tables at the top with the other tables
const IX_ARITHMETIC_MNEMONICS: [[&str; 4]; 4] = [
    ["ADD IX, BC", "ADD IX, DE", "ADD IX, IX", "ADD IX, SP"],
    ["ADC IX, BC", "ADC IX, DE", "ADC IX, IX", "ADC IX, SP"],
    ["SUB IX, BC", "SUB IX, DE", "SUB IX, IX", "SUB IX, SP"],
    ["SBC IX, BC", "SBC IX, DE", "SBC IX, IX", "SBC IX, SP"],
];

const IY_ARITHMETIC_MNEMONICS: [[&str; 4]; 4] = [
    ["ADD IY, BC", "ADD IY, DE", "ADD IY, IY", "ADD IY, SP"],
    ["ADC IY, BC", "ADC IY, DE", "ADC IY, IY", "ADC IY, SP"],
    ["SUB IY, BC", "SUB IY, DE", "SUB IY, IY", "SUB IY, SP"],
    ["SBC IY, BC", "SBC IY, DE", "SBC IY, IY", "SBC IY, SP"],
];

// Add these new constant tables for ED prefix instructions
const ED_BLOCK_MNEMONICS: [&str; 16] = [
    "LDI", "CPI", "INI", "OUTI", "LDD", "CPD", "IND", "OUTD", "LDIR", "CPIR", "INIR", "OTIR",
    "LDDR", "CPDR", "INDR", "OTDR",
];

const ED_IO_MNEMONICS: [[&str; 8]; 2] = [
    [
        "IN B,(C)", "IN C,(C)", "IN D,(C)", "IN E,(C)", "IN H,(C)", "IN L,(C)", "IN (C)",
        "IN A,(C)",
    ],
    [
        "OUT (C),B",
        "OUT (C),C",
        "OUT (C),D",
        "OUT (C),E",
        "OUT (C),H",
        "OUT (C),L",
        "OUT (C),0",
        "OUT (C),A",
    ],
];

/// Represents different instruction tables for the Z80
#[derive(Debug, Default)]
pub struct InstructionTables {
    /// Main instruction set (unprefixed)
    main: HashMap<u8, Instruction>,
    /// CB-prefixed instructions (bit operations)
    cb: HashMap<u8, Instruction>,
    /// DD/FD-prefixed instructions (IX/IY instructions)
    ddfd: HashMap<u8, Instruction>,
    /// ED-prefixed instructions (extended instructions)
    ed: HashMap<u8, Instruction>,
}

impl InstructionTables {
    pub fn new() -> Self {
        let mut tables = Self::default();
        tables.initialize();
        tables
    }

    fn initialize(&mut self) {
        self.init_main_table();
        self.init_cb_table();
        self.init_ddfd_table();
        self.init_ed_table();
    }

    fn init_main_table(&mut self) {
        // Start with NOP (0x00)
        self.main.insert(
            0x00,
            Instruction::new("NOP", 1, 4, InstructionType::Control, create_nop()),
        );
    }

    fn init_cb_table(&mut self) {
        // RLC instructions (CB 00-07)
        for i in 0..8 {
            self.cb.insert(
                i,
                Instruction::new(
                    match i {
                        0 => "RLC B",
                        1 => "RLC C",
                        2 => "RLC D",
                        3 => "RLC E",
                        4 => "RLC H",
                        5 => "RLC L",
                        6 => "RLC (HL)",
                        7 => "RLC A",
                        _ => unreachable!(),
                    },
                    2,                           // CB prefix + opcode
                    if i == 6 { 15 } else { 8 }, // (HL) takes more T-states
                    InstructionType::Rotate,
                    create_nop(), // Placeholder until we implement actual operations
                ),
            );
        }

        // RRC instructions (CB 08-0F)
        for i in 0x08..0x10 {
            self.cb.insert(
                i,
                Instruction::new(
                    match i & 0x07 {
                        0 => "RRC B",
                        1 => "RRC C",
                        2 => "RRC D",
                        3 => "RRC E",
                        4 => "RRC H",
                        5 => "RRC L",
                        6 => "RRC (HL)",
                        7 => "RRC A",
                        _ => unreachable!(),
                    },
                    2,
                    if (i & 0x07) == 6 { 15 } else { 8 },
                    InstructionType::Rotate,
                    create_nop(),
                ),
            );
        }

        // BIT instructions (CB 40-7F)
        for bit in 0..8 {
            for reg in 0..8 {
                let opcode = 0x40 | (bit << 3) | reg;
                self.cb.insert(
                    opcode,
                    Instruction::new(
                        BIT_MNEMONICS[bit as usize][reg as usize],
                        2,
                        if reg == 6 { 12 } else { 8 },
                        InstructionType::BitManip,
                        create_nop(),
                    ),
                );
            }
        }
    }

    fn init_ddfd_table(&mut self) {
        // Load instructions using IX/IY (DD/FD 46-7E)
        for reg in 0..8 {
            let opcode = 0x46 | (reg << 3);

            // DD prefix (IX) instructions - store in lower half of address space
            self.ddfd.insert(
                opcode,
                Instruction::new(
                    IX_LOAD_MNEMONICS[reg as usize],
                    3,
                    19,
                    InstructionType::Load,
                    create_nop(),
                ),
            );

            // FD prefix (IY) instructions - store in upper half of address space
            self.ddfd.insert(
                opcode | 0x80, // Use bit 7 to differentiate IY instructions
                Instruction::new(
                    IY_LOAD_MNEMONICS[reg as usize], // Use IY mnemonics
                    3,
                    19,
                    InstructionType::Load,
                    create_nop(),
                ),
            );
        }

        // Updated arithmetic instructions implementation
        let arithmetic_ops = [(0x84, 0), (0x8C, 1), (0x94, 2), (0x9C, 3)];

        for (base_op, op_idx) in arithmetic_ops {
            for reg_pair in 0..4 {
                let opcode = base_op | (reg_pair << 4);

                // DD prefix version (IX)
                self.ddfd.insert(
                    opcode,
                    Instruction::new(
                        IX_ARITHMETIC_MNEMONICS[op_idx][reg_pair as usize],
                        2,  // Prefix + opcode
                        15, // Arithmetic takes 15 T-states
                        InstructionType::Arithmetic,
                        create_nop(),
                    ),
                );

                // FD prefix version (IY)
                self.ddfd.insert(
                    opcode | 0x20,
                    Instruction::new(
                        IY_ARITHMETIC_MNEMONICS[op_idx][reg_pair as usize],
                        2,
                        15,
                        InstructionType::Arithmetic,
                        create_nop(),
                    ),
                );
            }
        }
    }

    fn init_ed_table(&mut self) {
        // Block transfer and search instructions (ED A0-BF)
        let block_instructions = [
            (0xA0, "LDI", 16),
            (0xA1, "CPI", 16),
            (0xA2, "INI", 16),
            (0xA3, "OUTI", 16),
            (0xA8, "LDD", 16),
            (0xA9, "CPD", 16),
            (0xAA, "IND", 16),
            (0xAB, "OUTD", 16),
            (0xB0, "LDIR", 21), // Explicitly define LDIR
            (0xB1, "CPIR", 21),
            (0xB2, "INIR", 21),
            (0xB3, "OTIR", 21),
            (0xB8, "LDDR", 21),
            (0xB9, "CPDR", 21),
            (0xBA, "INDR", 21),
            (0xBB, "OTDR", 21),
        ];

        for &(opcode, mnemonic, t_states) in &block_instructions {
            self.ed.insert(
                opcode,
                Instruction::new(mnemonic, 2, t_states, InstructionType::Block, create_nop()),
            );
        }

        // I/O instructions (ED 40-7F)
        let io_instructions = [
            (0x40, "IN B,(C)"),
            (0x41, "OUT (C),B"),
            (0x48, "IN C,(C)"),
            (0x49, "OUT (C),C"),
            (0x50, "IN D,(C)"),
            (0x51, "OUT (C),D"),
            (0x58, "IN E,(C)"),
            (0x59, "OUT (C),E"),
            (0x60, "IN H,(C)"),
            (0x61, "OUT (C),H"),
            (0x68, "IN L,(C)"),
            (0x69, "OUT (C),L"),
            (0x70, "IN (C)"),
            (0x71, "OUT (C),0"),
            (0x78, "IN A,(C)"),
            (0x79, "OUT (C),A"), // Explicitly define OUT (C),A
        ];

        for &(opcode, mnemonic) in &io_instructions {
            self.ed.insert(
                opcode,
                Instruction::new(mnemonic, 2, 12, InstructionType::IO, create_nop()),
            );
        }

        // Extended arithmetic instructions
        let arithmetic_ops = [
            (0x4C, "NEG", 8),  // Negate accumulator
            (0x67, "RRD", 18), // Rotate right decimal
            (0x6F, "RLD", 18), // Rotate left decimal
            (0x44, "IM 0", 8), // Set interrupt mode 0
            (0x54, "IM 1", 8), // Set interrupt mode 1
            (0x5E, "IM 2", 8), // Set interrupt mode 2
        ];

        for (opcode, mnemonic, t_states) in arithmetic_ops {
            self.ed.insert(
                opcode,
                Instruction::new(
                    mnemonic,
                    2,
                    t_states,
                    InstructionType::Arithmetic,
                    create_nop(),
                ),
            );
        }
    }

    /// Looks up an instruction in the main table
    pub fn lookup_main(&self, opcode: u8) -> Option<&Instruction> {
        self.main.get(&opcode)
    }

    /// Looks up a CB-prefixed instruction
    pub fn lookup_cb(&self, opcode: u8) -> Option<&Instruction> {
        self.cb.get(&opcode)
    }

    /// Looks up a DD/FD-prefixed instruction
    pub fn lookup_ddfd(&self, opcode: u8) -> Option<&Instruction> {
        self.ddfd.get(&opcode)
    }

    /// Looks up an ED-prefixed instruction
    pub fn lookup_ed(&self, opcode: u8) -> Option<&Instruction> {
        self.ed.get(&opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_instruction() {
        let tables = InstructionTables::new();
        let nop = tables
            .lookup_main(0x00)
            .expect("NOP instruction should exist");

        assert_eq!(nop.mnemonic, "NOP");
        assert_eq!(nop.length, 1);
        assert_eq!(nop.t_states, 4);
        assert_eq!(nop.instruction_type, InstructionType::Control);
    }

    #[test]
    fn test_invalid_opcode() {
        let tables = InstructionTables::new();
        assert!(tables.lookup_main(0xFF).is_none());
    }

    #[test]
    fn test_cb_prefix_instructions() {
        let tables = InstructionTables::new();

        // Test RLC B instruction
        let rlc_b = tables
            .lookup_cb(0x00)
            .expect("RLC B instruction should exist");
        assert_eq!(rlc_b.mnemonic, "RLC B");
        assert_eq!(rlc_b.length, 2);
        assert_eq!(rlc_b.t_states, 8);
        assert_eq!(rlc_b.instruction_type, InstructionType::Rotate);

        // Test RLC (HL) instruction
        let rlc_hl = tables
            .lookup_cb(0x06)
            .expect("RLC (HL) instruction should exist");
        assert_eq!(rlc_hl.mnemonic, "RLC (HL)");
        assert_eq!(rlc_hl.length, 2);
        assert_eq!(rlc_hl.t_states, 15);
        assert_eq!(rlc_hl.instruction_type, InstructionType::Rotate);

        // Test BIT instruction
        let bit_7_a = tables
            .lookup_cb(0x7F)
            .expect("BIT 7,A instruction should exist");
        assert_eq!(bit_7_a.mnemonic, "BIT 7, A");
        assert_eq!(bit_7_a.length, 2);
        assert_eq!(bit_7_a.t_states, 8);
        assert_eq!(bit_7_a.instruction_type, InstructionType::BitManip);
    }

    #[test]
    fn test_bit_instructions() {
        let tables = InstructionTables::new();

        // Test BIT 0,B
        let bit_0_b = tables
            .lookup_cb(0x40)
            .expect("BIT 0,B instruction should exist");
        assert_eq!(bit_0_b.mnemonic, "BIT 0, B");
        assert_eq!(bit_0_b.length, 2);
        assert_eq!(bit_0_b.t_states, 8);
        assert_eq!(bit_0_b.instruction_type, InstructionType::BitManip);

        // Test BIT 7,A
        let bit_7_a = tables
            .lookup_cb(0x7F)
            .expect("BIT 7,A instruction should exist");
        assert_eq!(bit_7_a.mnemonic, "BIT 7, A");
        assert_eq!(bit_7_a.length, 2);
        assert_eq!(bit_7_a.t_states, 8);
        assert_eq!(bit_7_a.instruction_type, InstructionType::BitManip);

        // Test BIT 3,(HL)
        let bit_3_hl = tables
            .lookup_cb(0x5E)
            .expect("BIT 3,(HL) instruction should exist");
        assert_eq!(bit_3_hl.mnemonic, "BIT 3, (HL)");
        assert_eq!(bit_3_hl.length, 2);
        assert_eq!(bit_3_hl.t_states, 12);
        assert_eq!(bit_3_hl.instruction_type, InstructionType::BitManip);
    }

    #[test]
    fn test_ix_load_instructions() {
        let tables = InstructionTables::new();

        // Test LD B,(IX+d)
        let load_b_ix = tables.lookup_ddfd(0x46).expect("LD B,(IX+d) should exist");
        assert_eq!(load_b_ix.mnemonic, "LD B, (IX+d)");
        assert_eq!(load_b_ix.length, 3);
        assert_eq!(load_b_ix.t_states, 19);
        assert_eq!(load_b_ix.instruction_type, InstructionType::Load);

        // Test LD A,(IX+d)
        let load_a_ix = tables.lookup_ddfd(0x7E).expect("LD A,(IX+d) should exist");
        assert_eq!(load_a_ix.mnemonic, "LD A, (IX+d)");
        assert_eq!(load_a_ix.length, 3);
        assert_eq!(load_a_ix.t_states, 19);
        assert_eq!(load_a_ix.instruction_type, InstructionType::Load);
    }

    #[test]
    fn test_iy_load_instructions() {
        let tables = InstructionTables::new();

        // Test LD B,(IY+d) - opcode 0x46 with bit 7 set for IY
        let load_b_iy = tables
            .lookup_ddfd(0x46 | 0x80)
            .expect("LD B,(IY+d) should exist");
        assert_eq!(load_b_iy.mnemonic, "LD B, (IY+d)");
        assert_eq!(load_b_iy.length, 3);
        assert_eq!(load_b_iy.t_states, 19);
        assert_eq!(load_b_iy.instruction_type, InstructionType::Load);
    }

    #[test]
    fn test_ix_arithmetic() {
        let tables = InstructionTables::new();

        // Test ADD IX,BC
        let add_ix_bc = tables.lookup_ddfd(0x84).expect("ADD IX,BC should exist");
        assert_eq!(add_ix_bc.mnemonic, "ADD IX, BC");
        assert_eq!(add_ix_bc.length, 2);
        assert_eq!(add_ix_bc.t_states, 15);
        assert_eq!(add_ix_bc.instruction_type, InstructionType::Arithmetic);
    }

    #[test]
    fn test_ed_block_instructions() {
        let tables = InstructionTables::new();

        // Test LDI instruction
        let ldi = tables
            .lookup_ed(0xA0)
            .expect("LDI instruction should exist");
        assert_eq!(ldi.mnemonic, "LDI");
        assert_eq!(ldi.length, 2);
        assert_eq!(ldi.t_states, 16);
        assert_eq!(ldi.instruction_type, InstructionType::Block);

        // Test LDIR instruction
        let ldir = tables
            .lookup_ed(0xB0)
            .expect("LDIR instruction should exist");
        assert_eq!(ldir.mnemonic, "LDIR");
        assert_eq!(ldir.length, 2);
        assert_eq!(ldir.t_states, 21);
        assert_eq!(ldir.instruction_type, InstructionType::Block);
    }

    #[test]
    fn test_ed_io_instructions() {
        let tables = InstructionTables::new();

        // Test IN B,(C)
        let in_b = tables.lookup_ed(0x40).expect("IN B,(C) should exist");
        assert_eq!(in_b.mnemonic, "IN B,(C)");
        assert_eq!(in_b.length, 2);
        assert_eq!(in_b.t_states, 12);
        assert_eq!(in_b.instruction_type, InstructionType::IO);

        // Test OUT (C),A
        let out_a = tables.lookup_ed(0x79).expect("OUT (C),A should exist");
        assert_eq!(out_a.mnemonic, "OUT (C),A");
        assert_eq!(out_a.length, 2);
        assert_eq!(out_a.t_states, 12);
        assert_eq!(out_a.instruction_type, InstructionType::IO);
    }

    #[test]
    fn test_ed_arithmetic_instructions() {
        let tables = InstructionTables::new();

        // Test NEG
        let neg = tables
            .lookup_ed(0x4C)
            .expect("NEG instruction should exist");
        assert_eq!(neg.mnemonic, "NEG");
        assert_eq!(neg.length, 2);
        assert_eq!(neg.t_states, 8);
        assert_eq!(neg.instruction_type, InstructionType::Arithmetic);

        // Test RLD
        let rld = tables
            .lookup_ed(0x6F)
            .expect("RLD instruction should exist");
        assert_eq!(rld.mnemonic, "RLD");
        assert_eq!(rld.length, 2);
        assert_eq!(rld.t_states, 18);
        assert_eq!(rld.instruction_type, InstructionType::Arithmetic);
    }
}
