use they::cpu::{ALUFlag, CPU};

fn setup(opcode: u8) -> CPU {
    let mut cpu = CPU::new();

    cpu.memory.ram[0x0] = opcode;
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
    cpu.memory.ram[0x0202] = 1;
    cpu
}

#[test]
fn test_nop() {
    // opcode for nop
    let mut cpu = setup(0x0);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x2);
    assert_eq!(cpu.registers.d, 0x2);
    assert_eq!(cpu.registers.e, 0x2);
    assert_eq!(cpu.registers.flags, 0x0);
    assert_eq!(cpu.registers.high, 0x1);
    assert_eq!(cpu.registers.low, 0x0);
    assert_eq!(cpu.memory.ram[0x0], 0x0);
}

#[test]
fn test_ld_r16_n16() {
    // opcode where r16 is registers(b,c)
    let mut cpu = setup(0x1);
    cpu.memory.ram[0x1] = 0xFF;
    cpu.memory.ram[0x2] = 0xEE;
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x1);
    assert_eq!(cpu.registers.pc, 0x3);
    assert_eq!(cpu.registers.b, 0xFF);
    assert_eq!(cpu.registers.c, 0xEE);
}

#[test]
fn test_ld_r16m_a() {
    // opcode where r16 is registers(b,c)
    let mut cpu = setup(0x2);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x2);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x2);
    assert_eq!(cpu.memory.ram[0x0202], 0x1);
}

#[test]
fn test_inc_r16() {
    // specific opcode for registers (b, c)
    let mut cpu = setup(0x3);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x3);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x3);
    assert_eq!(cpu.registers.flags, 0x0);
}

#[test]
fn test_inc_r8() {
    // specific opcode for register b
    let mut cpu = setup(0x4);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x4);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.b, 0x3);
    assert_eq!(cpu.registers.flags, 0x0);

    // check flags on overflow
    cpu.registers.pc = 0;
    cpu.registers.b = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x0);
    assert_eq!(cpu.registers.flags, ALUFlag::C as u8);
}
