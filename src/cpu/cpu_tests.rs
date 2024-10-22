use super::Cpu;
use super::*;

#[test]
fn test_cpu_initialization() {
    let cpu = Cpu::new();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.b, 0);
    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.d, 0);
    assert_eq!(cpu.e, 0);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.f, 0);
    assert_eq!(cpu.pc, 0);
    assert_eq!(cpu.sp, 0);
    assert_eq!(cpu.ix, 0);
    assert_eq!(cpu.iy, 0);
    assert_eq!(cpu.a_alt, 0);
    assert_eq!(cpu.f_alt, 0);
    assert_eq!(cpu.iff1, false);
    assert_eq!(cpu.iff2, false);
    assert_eq!(cpu.im, 0);
    assert_eq!(cpu.memory.len(), 65536);
}

#[test]
fn test_memory_operations() {
    let mut cpu = Cpu::new();

    // Test writing and reading a single byte
    cpu.write_byte(0x1000, 0x42);
    assert_eq!(cpu.read_byte(0x1000), 0x42);

    // Test writing and reading at memory boundaries
    cpu.write_byte(0x0000, 0xFF);
    cpu.write_byte(0xFFFF, 0xAA);
    assert_eq!(cpu.read_byte(0x0000), 0xFF);
    assert_eq!(cpu.read_byte(0xFFFF), 0xAA);

    // Test overwriting
    cpu.write_byte(0x1000, 0x24);
    assert_eq!(cpu.read_byte(0x1000), 0x24);
}

#[test]
fn test_pc_operations() {
    let mut cpu = Cpu::new();

    cpu.increment_pc(1);
    assert_eq!(cpu.pc, 1);

    cpu.increment_pc(10);
    assert_eq!(cpu.pc, 11);

    // Test wrapping
    cpu.pc = 0xFFFE;
    cpu.increment_pc(3);
    assert_eq!(cpu.pc, 1);
}

#[test]
fn test_flag_operations() {
    let mut cpu = Cpu::new();

    // Test setting and getting individual flags
    cpu.set_flag(FLAG_Z, true);
    assert!(cpu.get_flag(FLAG_Z));
    cpu.set_flag(FLAG_Z, false);
    assert!(!cpu.get_flag(FLAG_Z));

    // Test multiple flags
    cpu.set_flag(FLAG_C, true);
    cpu.set_flag(FLAG_PV, true);
    cpu.set_flag(FLAG_S, true);
    assert!(cpu.get_flag(FLAG_C));
    assert!(cpu.get_flag(FLAG_PV));
    assert!(cpu.get_flag(FLAG_S));
    assert!(!cpu.get_flag(FLAG_Z));
    assert!(!cpu.get_flag(FLAG_H));
    assert!(!cpu.get_flag(FLAG_N));

    // Test clearing flags
    cpu.set_flag(FLAG_C, false);
    cpu.set_flag(FLAG_PV, false);
    assert!(!cpu.get_flag(FLAG_C));
    assert!(!cpu.get_flag(FLAG_PV));
    assert!(cpu.get_flag(FLAG_S));
}

#[test]
fn test_add_a() {
    let mut cpu = Cpu::new();

    // Test basic addition
    cpu.a = 5;
    cpu.add_a(3);
    assert_eq!(cpu.a, 8);
    assert!(!cpu.get_flag(FLAG_Z));
    assert!(!cpu.get_flag(FLAG_S));
    assert!(!cpu.get_flag(FLAG_C));
    assert!(!cpu.get_flag(FLAG_PV));

    // Test addition resulting in zero
    cpu.a = 0;
    cpu.add_a(0);
    assert_eq!(cpu.a, 0);
    assert!(cpu.get_flag(FLAG_Z));

    // Test addition with carry
    cpu.a = 255;
    cpu.add_a(1);
    assert_eq!(cpu.a, 0);
    assert!(cpu.get_flag(FLAG_Z));
    assert!(cpu.get_flag(FLAG_C));

    // Test addition causing half-carry
    cpu.a = 0x0F;
    cpu.add_a(1);
    assert_eq!(cpu.a, 0x10);
    assert!(cpu.get_flag(FLAG_H));

    // Test addition causing overflow
    cpu.a = 127;
    cpu.add_a(1);
    assert_eq!(cpu.a, 128);
    assert!(cpu.get_flag(FLAG_PV));
    assert!(cpu.get_flag(FLAG_S));

    // Test addition with negative result
    cpu.a = 250;
    cpu.add_a(10);
    assert_eq!(cpu.a, 4);
    assert!(cpu.get_flag(FLAG_C));
    assert!(!cpu.get_flag(FLAG_S));
}

#[test]
fn test_add_a_r() {
    let mut cpu = Cpu::new();
    cpu.a = 5;
    cpu.b = 3;
    cpu.add_a_r(cpu.b);
    assert_eq!(cpu.a, 8);
}

#[test]
fn test_add_a_n() {
    let mut cpu = Cpu::new();
    cpu.a = 5;
    cpu.add_a_n(3);
    assert_eq!(cpu.a, 8);
}

#[test]
fn test_add_a_hl() {
    let mut cpu = Cpu::new();
    cpu.a = 5;
    cpu.h = 0x10;
    cpu.l = 0x00;
    cpu.write_byte(0x1000, 3);
    cpu.add_a_hl();
    assert_eq!(cpu.a, 8);
}
