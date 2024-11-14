use super::*;

#[test]
fn test_cb_prefix_handling() {
    let mut fixture = CpuTestFixture::new();
    
    // Test RLC B (CB 00) - 8 T-states
    fixture.load_and_run(&[0xCB, 0x00]).unwrap();
    assert_eq!(fixture.cpu.cycles, 8);
}

#[test]
fn test_dd_prefix_handling() {
    let mut fixture = CpuTestFixture::new();
    
    // Test LD IX,nn (DD 21 nn nn) - 14 T-states
    fixture.load_and_run(&[0xDD, 0x21, 0x34, 0x12]).unwrap();
    assert_eq!(fixture.cpu.cycles, 14);
}

#[test]
fn test_nested_prefix_handling() {
    let mut fixture = CpuTestFixture::new();
    
    // Test RLC (IX+d) (DD CB d 06) - 23 T-states
    fixture.load_and_run(&[0xDD, 0xCB, 0x01, 0x06]).unwrap();
    assert_eq!(fixture.cpu.cycles, 23);
} 