use super::*;

#[test]
fn test_instruction_flow() {
    let mut fixture = CpuTestFixture::new();

    // Test basic instruction sequence
    let program = [
        0x3E, 0x42, // LD A, 0x42
        0x06, 0x10, // LD B, 0x10
        0x80, // ADD A, B
    ];

    fixture.load_and_run(&program).unwrap();

    assert_eq!(fixture.cpu.registers.a, 0x52);
    assert_eq!(fixture.cpu.registers.b, 0x10);
    assert_eq!(fixture.cpu.get_pc(), 5);
}

#[test]
fn test_flag_updates() {
    let mut fixture = CpuTestFixture::new();

    // Test flag updates after arithmetic
    let program = [
        0x3E, 0xFF, // LD A, 0xFF
        0x3C, // INC A
    ];

    fixture.load_and_run(&program).unwrap();

    assert_eq!(fixture.cpu.registers.a, 0x00);
    assert!(fixture.cpu.flags.zero);
    assert!(fixture.cpu.flags.half_carry);
}
