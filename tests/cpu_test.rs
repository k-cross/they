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
    cpu.memory.ram[0x0202] = 0xAA;
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
fn test_inc_sp() {
    let mut cpu = setup(0x33);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.sp, 0x1);
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

#[test]
fn test_dec_r8() {
    // specific opcode for register b
    let mut cpu = setup(0x5);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x5);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.b, 0x1);
    assert_eq!(cpu.registers.flags, 0x0);

    // check flags on overflow
    cpu.registers.pc = 0;
    cpu.registers.b = 0x0;
    cpu.exec();
    assert_eq!(cpu.registers.b, 0x0);
    assert_eq!(cpu.registers.flags, (ALUFlag::C as u8 | ALUFlag::N as u8));
}

#[test]
fn test_ld_r8_n8() {
    // specific opcode for register b
    let mut cpu = setup(0x6);
    cpu.memory.ram[0x1] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x6);
    assert_eq!(cpu.memory.ram[0x1], 0xFF);
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.b, 0xFF);
}

#[test]
fn test_rlca() {
    // specific opcode for register a
    let mut cpu = setup(0x7);
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x7);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0b10);
}

#[test]
fn test_rla() {
    // specific opcode for register a
    let mut cpu = setup(0x17);
    cpu.registers.acc = 0xFF;
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x0], 0x17);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0b11111110);
    assert_eq!(cpu.registers.flags, ALUFlag::C as u8);
}

#[test]
fn test_ld_a16m_sp() {
    // specific opcode for register sp
    let mut cpu = setup(0x8);
    cpu.registers.sp = 0xAAFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x3);
    assert_eq!(cpu.registers.sp, 0xAAFF);
    assert_eq!(cpu.memory.ram[0x0], 0xAA);
    assert_eq!(cpu.memory.ram[0x1], 0xFF);
}

#[test]
fn test_add_hl_r16() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x9);
    cpu.registers.high = 0xFF;
    cpu.registers.b = 0x0;
    cpu.registers.c = 0x1;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0xFF);
    assert_eq!(cpu.registers.low, 0x01);
    assert_eq!(cpu.registers.flags, 0x0);

    // with overflow
    cpu.registers.pc = 0;
    cpu.registers.high = 0xFF;
    cpu.registers.low = 0xFF;
    cpu.registers.b = 0x0;
    cpu.registers.c = 0x1;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0x0);
    assert_eq!(cpu.registers.low, 0x0);
    assert_eq!(cpu.registers.flags, ALUFlag::C as u8);
}

#[test]
fn test_ld_a_r16m() {
    // opcode where r16 is registers(b,c)
    let mut cpu = setup(0xA);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xAA);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x2);
    assert_eq!(cpu.memory.ram[0x0202], 0xAA);
}

#[test]
fn test_dec_r16() {
    // specific opcode for register b
    let mut cpu = setup(0xB);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x1);
}

#[test]
fn test_rrca() {
    // specific opcode for register a
    let mut cpu = setup(0xF);
    cpu.registers.acc = 0b100;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0b10);
    assert_eq!(cpu.registers.flags, 0);

    // with flags
    cpu.registers.pc = 0;
    cpu.registers.acc = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0b0111_1111);
    assert_eq!(cpu.registers.flags, ALUFlag::C as u8);
}

#[test]
fn test_jr_e8() {
    let i: i8 = -5;
    let mut cpu = setup(0x18);
    cpu.memory.ram[7] = 0x18;
    cpu.registers.pc = 7;
    cpu.memory.ram[8] = i as u8;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x4);
}

#[test]
fn test_rra() {
    // specific opcode for register a
    let mut cpu = setup(0x1F);
    cpu.registers.acc = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0b01111111);
    assert_eq!(cpu.registers.flags, ALUFlag::C as u8);

    // with carry pre-set to rotate in
    cpu.registers.acc = 0b1111_1110;
    cpu.registers.pc = 0;
    cpu.exec();
    assert_eq!(cpu.registers.acc, 0xFF);
    assert_eq!(cpu.registers.flags, 0);
}

#[test]
fn test_jr_nz_e8() {
    let i: i8 = -5;
    // specific opcode for register a
    let mut cpu = setup(0x20);
    cpu.memory.ram[7] = 0x20;
    cpu.registers.pc = 7;
    cpu.memory.ram[8] = i as u8;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x4);

    cpu.memory.ram[7] = 0x20;
    cpu.registers.pc = 7;
    cpu.registers.flags = ALUFlag::Z as u8;
    cpu.memory.ram[8] = i as u8;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x9);
}

#[test]
fn test_ld_hlim_a() {
    let mut cpu = setup(0x22);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert_eq!(cpu.registers.high, 0x01);
    assert_eq!(cpu.registers.low, 0x01);
    assert_eq!(cpu.memory.ram[0x0100], 0x1);
}

#[test]
fn test_ld_a_hlim() {
    let mut cpu = setup(0x2A);
    cpu.memory.ram[0x0100] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFF);
    assert_eq!(cpu.registers.high, 0x01);
    assert_eq!(cpu.registers.low, 0x01);
    assert_eq!(cpu.memory.ram[0x0100], 0xFF);
}

#[test]
fn test_ld_a_hldm() {
    let mut cpu = setup(0x3A);
    cpu.memory.ram[0x0100] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFF);
    assert_eq!(cpu.registers.high, 0x00);
    assert_eq!(cpu.registers.low, 0xFF);
    assert_eq!(cpu.memory.ram[0x0100], 0xFF);
}

#[test]
fn test_ld_hldm_a() {
    let mut cpu = setup(0x32);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert_eq!(cpu.registers.high, 0x00);
    assert_eq!(cpu.registers.low, 0xFF);
    assert_eq!(cpu.memory.ram[0x0100], 0x1);
}

#[test]
fn test_daa() {
    let mut cpu = setup(0x27);

    // decimal adjustment accumulator with C and H flag set
    cpu.set_flag(ALUFlag::C, true);
    cpu.set_flag(ALUFlag::H, true);
    cpu.registers.acc = 0xFF;
    cpu.registers.pc = 0;
    cpu.exec();

    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, (0xFF - 0x66));
    assert!(cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::N));

    // decimal adjustment accumulator with N flag set
    cpu.set_flag(ALUFlag::N, true);
    cpu.registers.acc = 0xA0;
    cpu.registers.pc = 0;
    cpu.exec();

    let mut acc: u8 = 0xA0;
    acc = acc.wrapping_add(0x60);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, acc);
    assert!(cpu.check_flag(ALUFlag::C));
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_set_and_check_alu_flags() {
    let mut cpu = setup(0x0);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::H));

    cpu.set_flag(ALUFlag::Z, true);
    cpu.set_flag(ALUFlag::C, true);
    cpu.set_flag(ALUFlag::N, true);
    cpu.set_flag(ALUFlag::H, true);

    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(cpu.check_flag(ALUFlag::C));
    assert!(cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::H));
}

#[test]
fn test_jr_z_e8() {
    let i: i8 = -5;
    // specific opcode for register a
    let mut cpu = setup(0x28);
    cpu.memory.ram[7] = 0x28;
    cpu.registers.pc = 7;
    cpu.memory.ram[8] = i as u8;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x9);

    cpu.memory.ram[7] = 0x28;
    cpu.registers.pc = 7;
    cpu.registers.flags = ALUFlag::Z as u8;
    cpu.memory.ram[8] = i as u8;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x4);
}

#[test]
fn test_ld_sp_n16() {
    // opcode where r16 is registers(b,c)
    let mut cpu = setup(0x31);
    cpu.memory.ram[0x1] = 0xFF;
    cpu.memory.ram[0x2] = 0xEE;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x3);
    assert_eq!(cpu.registers.sp, 0xFFEE);
}

#[test]
fn test_inc_r16m() {
    let mut cpu = setup(0x34);
    cpu.registers.high = 0x10;
    cpu.registers.low = 0x10;
    cpu.memory.ram[0x1010] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0x10);
    assert_eq!(cpu.registers.low, 0x10);
    assert_eq!(cpu.memory.ram[0x1010], 0);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_dec_r16m() {
    let mut cpu = setup(0x35);
    cpu.registers.high = 0x10;
    cpu.registers.low = 0x10;
    cpu.memory.ram[0x1010] = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0x10);
    assert_eq!(cpu.registers.low, 0x10);
    assert_eq!(cpu.memory.ram[0x1010], 0xFE);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_ld_r16m_n8() {
    let mut cpu = setup(0x36);
    cpu.registers.high = 0x10;
    cpu.registers.low = 0x10;
    cpu.memory.ram[0x1] = 0x66;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.high, 0x10);
    assert_eq!(cpu.registers.low, 0x10);
    assert_eq!(cpu.memory.ram[0x1010], 0x66);
}

#[test]
fn test_scf() {
    let mut cpu = setup(0x37);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_cpl() {
    let mut cpu = setup(0x2F);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFE);
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_add_r16_sp() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x39);
    cpu.registers.high = 0x00;
    cpu.registers.low = 0xAA;
    cpu.registers.sp = 0xFF00;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0xFF);
    assert_eq!(cpu.registers.low, 0xAA);
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(!cpu.check_flag(ALUFlag::C));

    // with overflow
    cpu.registers.flags = 0;
    cpu.registers.pc = 0;
    cpu.registers.high = 0xFF;
    cpu.registers.low = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.high, 0x0);
    assert_eq!(cpu.registers.low, 0x0);
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
    assert!(cpu.check_flag(ALUFlag::C));
}

#[test]
fn test_ld_r8_r8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x40);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.b, 2);
}

#[test]
fn test_ld_r16m_r8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x70);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.memory.ram[0x0100], cpu.registers.b);
}

#[test]
fn test_add_r8_r8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x80);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x3);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_adc_r8_r8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x88);
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x4);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_add_r8_r16m() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x86);
    cpu.memory.ram[0x0100] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xB);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_adc_r8_r16m() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x8E);
    cpu.memory.ram[0x0100] = 0xA;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xC);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_sub_r8_r8() {
    // specific opcode for registers a, b
    let mut cpu = setup(0x90);
    cpu.registers.acc = 0x3;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_sub_r8_r16m() {
    // specific opcode for registers b,c
    let mut cpu = setup(0x96);
    cpu.memory.ram[0x0100] = 0xA;
    cpu.registers.acc = 0xB;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_sbc_r8_r8() {
    // specific opcode for registers a, b
    let mut cpu = setup(0x98);
    cpu.registers.acc = 0xFF;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFC);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_sbc_r8_r16m() {
    let mut cpu = setup(0x9E);
    cpu.registers.acc = 0xFF;
    cpu.memory.ram[0x0100] = 0xFE;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x0);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_and_r8_r8() {
    let mut cpu = setup(0xA0);
    cpu.registers.acc = 0xFF;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_and_r8_r16m() {
    let mut cpu = setup(0xA6);
    cpu.registers.acc = 0xFF;
    cpu.memory.ram[0x0100] = 0xFE;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFE);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_xor_r8_r8() {
    let mut cpu = setup(0xA8);
    cpu.registers.acc = 0xFF;
    cpu.registers.b = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_xor_r8_r16m() {
    let mut cpu = setup(0xAE);
    cpu.registers.acc = 0xFF;
    cpu.memory.ram[0x0100] = 0xFE;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x1);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_or_r8_r8() {
    let mut cpu = setup(0xB0);
    cpu.registers.acc = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFF);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_or_r8_r16m() {
    let mut cpu = setup(0xB6);
    cpu.registers.acc = 0xFF;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xFF);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_cp_r8_r8() {
    let mut cpu = setup(0xB8);
    cpu.registers.acc = 0x2;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0x2);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_cp_r8_r16m() {
    let mut cpu = setup(0xBE);
    cpu.memory.ram[0x0100] = 0xA;
    cpu.registers.acc = 0xB;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.registers.acc, 0xB);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_ret_cc() {
    let mut cpu = setup(0xC0);
    cpu.registers.sp = 0x0100;
    cpu.memory.ram[0x0100] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x102);
    assert_eq!(cpu.registers.pc, 0xA00);
}

#[test]
fn test_pop_r16() {
    let mut cpu = setup(0xC1);
    cpu.registers.sp = 0x0100;
    cpu.memory.ram[0x0100] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x102);
    assert_eq!(cpu.registers.b, 0xA);
    assert_eq!(cpu.registers.c, 0x0);
}

#[test]
fn test_jp_a16_cc() {
    let mut cpu = setup(0xC2);
    cpu.memory.ram[0x01] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x0A00);

    cpu.registers.pc = 0;
    cpu.set_flag(ALUFlag::Z, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x03);
}

#[test]
fn test_adc_r8_n8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0xCE);
    cpu.memory.ram[0x01] = 0xA;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0xC);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_add_r8_n8() {
    // specific opcode for registers b,c
    let mut cpu = setup(0xC6);
    cpu.memory.ram[0x01] = 0xA;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0xB);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_call_a16_cc() {
    // test positive condition
    let mut cpu = setup(0xC4);
    cpu.registers.sp = 0x0102;
    cpu.memory.ram[0x1] = 0xAA;
    cpu.memory.ram[0x100] = 0xA;
    cpu.memory.ram[0x102] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x100);
    assert_eq!(cpu.registers.pc, 0xAA00);
    assert_eq!(cpu.memory.ram[0x100], 0x0);
    assert_eq!(cpu.memory.ram[0x101], 0x03);

    // test false condition
    cpu.registers.sp = 0x0102;
    cpu.registers.pc = 0x0;
    cpu.set_flag(ALUFlag::Z, true);
    cpu.memory.ram[0x1] = 0xAA;
    cpu.memory.ram[0x100] = 0xA;
    cpu.memory.ram[0x102] = 0xA;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x102);
    assert_eq!(cpu.registers.pc, 0x1);
    assert_eq!(cpu.memory.ram[0x100], 0xA);
    assert_eq!(cpu.memory.ram[0x102], 0xA);
}

#[test]
fn test_push_r16() {
    let mut cpu = setup(0xC5);
    cpu.registers.sp = 0x0102;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x100);
    assert_eq!(cpu.memory.ram[0x100], 0x2);
    assert_eq!(cpu.memory.ram[0x101], 0x2);
    assert_eq!(cpu.registers.b, 0x2);
    assert_eq!(cpu.registers.c, 0x2);
}

#[test]
fn test_sub_r8_n8() {
    let mut cpu = setup(0xD6);
    cpu.memory.ram[0x01] = 0xA;
    cpu.registers.acc = 0xC;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0x2);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_sbc_r8_n8() {
    let mut cpu = setup(0xDE);
    cpu.memory.ram[0x01] = 0xA;
    cpu.registers.acc = 0xC;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0x1);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_and_r8_n8() {
    let mut cpu = setup(0xE6);
    cpu.registers.acc = 0xFF;
    cpu.memory.ram[0x01] = 0xA;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0xA);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_or_r8_n8() {
    let mut cpu = setup(0xF6);
    cpu.registers.acc = 0xFF;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0xFF);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_xor_r8_n8() {
    let mut cpu = setup(0xEE);
    cpu.registers.acc = 0xF0;
    cpu.memory.ram[0x01] = 0xFA;
    cpu.set_flag(ALUFlag::C, true);
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0xA);
    assert!(!cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(!cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_cp_r8_n8() {
    let mut cpu = setup(0xFE);
    cpu.registers.acc = 0x2;
    cpu.memory.ram[0x01] = 0x02;
    cpu.exec();
    assert_eq!(cpu.registers.pc, 0x2);
    assert_eq!(cpu.registers.acc, 0x2);
    assert!(cpu.check_flag(ALUFlag::Z));
    assert!(!cpu.check_flag(ALUFlag::C));
    assert!(!cpu.check_flag(ALUFlag::H));
    assert!(cpu.check_flag(ALUFlag::N));
}

#[test]
fn test_reti_cc() {
    let mut cpu = setup(0xD9);
    cpu.registers.sp = 0x0100;
    cpu.memory.ram[0x0100] = 0xA;
    cpu.ei = false;
    cpu.exec();
    assert_eq!(cpu.registers.sp, 0x102);
    assert_eq!(cpu.registers.pc, 0xA00);
    assert!(cpu.ei);
}
