use super::*;

#[test]
fn test_decoder_prefix_state() {
    let mut fixture = CpuTestFixture::new();

    // Test CB prefix state
    fixture.load_and_run(&[0xCB, 0x00]).unwrap();
    assert_eq!(fixture.cpu.decoder.current_prefix, Prefix::Cb);

    // Test DD prefix state
    let mut fixture = CpuTestFixture::new();
    fixture.load_and_run(&[0xDD, 0x21]).unwrap();
    assert_eq!(fixture.cpu.decoder.current_prefix, Prefix::Dd);

    // Test nested DDCB prefix state
    let mut fixture = CpuTestFixture::new();
    fixture.load_and_run(&[0xDD, 0xCB, 0x01]).unwrap();
    assert_eq!(fixture.cpu.decoder.current_prefix, Prefix::DdCb);
}

#[test]
fn test_decoder_reset() {
    let mut fixture = CpuTestFixture::new();

    // Set prefix and verify reset
    fixture.cpu.decoder.current_prefix = Prefix::Cb;
    fixture.cpu.decoder.reset();
    assert_eq!(fixture.cpu.decoder.current_prefix, Prefix::None);
}
