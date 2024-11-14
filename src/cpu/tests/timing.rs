use super::*;

#[test]
fn test_instruction_timing() {
    let mut fixture = CpuTestFixture::new();
    
    // Test NOP timing (4 T-states)
    fixture.load_and_run(&[0x00]).unwrap();
    assert_eq!(fixture.cpu.cycles, 4);
    
    // Test LD A,n timing (7 T-states)
    let mut fixture = CpuTestFixture::new();
    fixture.load_and_run(&[0x3E, 0x42]).unwrap();
    assert_eq!(fixture.cpu.cycles, 7);
}

#[test]
fn test_memory_timing() {
    let mut fixture = CpuTestFixture::new();
    
    // Test LD (HL),n timing (10 T-states)
    fixture.cpu.registers.set_hl(0x1000);
    fixture.load_and_run(&[0x36, 0x42]).unwrap();
    assert_eq!(fixture.cpu.cycles, 10);
} 