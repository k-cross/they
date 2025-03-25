use they::cpu::{ALUFlag, CPU};

fn setup(opcode: u8) -> CPU {
    let mut cpu = CPU::new();

    cpu.memory.ram[0x0] = 0xCB;
    cpu.memory.ram[0x1] = opcode;
    cpu.registers.acc = 1;
    cpu.registers.b = 2;
    cpu.registers.c = 2;
    cpu.registers.d = 2;
    cpu.registers.e = 2;
    cpu.registers.flags = 0;
    cpu.registers.high = 1;
    cpu.registers.low = 0;
    cpu.registers.pc = 0;
    cpu.registers.sp = 0;
    cpu
}

#[test]
fn test_rl_r8() {
    let mut cpu = setup(0x10);
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x4);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rl_hlm() {
    let mut cpu = setup(0x16);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0xE1);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rr_r8() {
    let mut cpu = setup(0x18);
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x1);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rr_hlm() {
    let mut cpu = setup(0x1E);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0xF8);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rlc_r8() {
    let mut cpu = setup(0x0);
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x4);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rlc_hlm() {
    let mut cpu = setup(0x6);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0xE1);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rrc_r8() {
    let mut cpu = setup(0x8);
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x1);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_rrc_hlm() {
    let mut cpu = setup(0xE);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0x78);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_sla_r8() {
    let mut cpu = setup(0x20);
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x4);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_sla_hlm() {
    let mut cpu = setup(0x26);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0xE0);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_sra_r8() {
    let mut cpu = setup(0x28);
    cpu.registers.b = 0x8F;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0xC7);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_sra_hlm() {
    let mut cpu = setup(0x2E);
    cpu.memory.ram[0x0100] = 0x0F;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0x7);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_srl_r8() {
    let mut cpu = setup(0x38);
    cpu.registers.b = 0x8F;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x47);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_srl_hlm() {
    let mut cpu = setup(0x3E);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0x78);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_swap_r8() {
    let mut cpu = setup(0x30);
    cpu.registers.b = 0x8F;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0xF8);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_swap_hlm() {
    let mut cpu = setup(0x36);
    cpu.memory.ram[0x0100] = 0xF0;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0x0F);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_bit_r8() {
    let mut cpu = setup(0x40);
    cpu.registers.b = 0x0;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x0);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_bit_hlm() {
    let mut cpu = setup(0x46);
    cpu.memory.ram[0x0100] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0100], 0xFF);
    assert_eq!(cpu.registers.pc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}
