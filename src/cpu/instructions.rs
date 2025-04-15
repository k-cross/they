use super::{ALUFlag, CPU, prefix_instructions};

// private to make functions more generic since a lot of the handling will be
// the same.
pub(crate) enum Reg {
    A,
    B,
    C,
    D,
    E,
    FLAGS,
    H,
    L,
}

// return values of instructions are machine cycles
fn ld_r8_n8(c: &mut CPU, r: Reg) -> u8 {
    let v = c.get_instr();
    write_reg(c, &r, v);
    2
}

fn ld_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r2);
    write_reg(c, &r1, v);
    //std::thread::sleep(std::time::Duration::from_millis(1));
    c.memory.print_serial();
    1
}

fn ld_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = c.memory.read_byte(addr);
    write_reg(c, &r1, v);
    2
}

fn ld_r16m_n8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = c.get_instr();
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.memory.write_byte(addr, v);
    2
}

fn ld_r16_n16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let val = c.get_word_instr();
    write_reg(c, &r1, (val >> 8) as u8);
    write_reg(c, &r2, val as u8);
    3
}

fn ld_sp_n16(c: &mut CPU) -> u8 {
    c.registers.sp = c.get_word_instr();
    3
}

fn ld_r16m_r8(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    let v = read_reg(c, &r3);
    c.memory.write_byte(addr, v);
    2
}

fn ld_r16m_a(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.memory.write_byte(addr, c.registers.acc);
    2
}

fn ld_a16m_sp(c: &mut CPU) -> u8 {
    let addr = c.get_word_instr();
    c.memory.write_word(addr, c.registers.sp);
    3
}

fn ld_a8m_a(c: &mut CPU) -> u8 {
    let addr = 0xFF00 | c.get_instr() as u16;
    c.memory.write_byte(addr, c.registers.acc);
    3
}

fn ld_a_a8m(c: &mut CPU) -> u8 {
    let addr = 0xFF00 | c.get_instr() as u16;
    c.registers.acc = c.memory.read_byte(addr);
    3
}

fn ld_a_r16m(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.registers.acc = c.memory.read_byte(addr);
    2
}

fn ld_hl_spe8(c: &mut CPU) -> u8 {
    let mut hl = c.registers.sp;
    let b = c.get_instr() as i16 as u16;
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::Z, false);
    c.set_flag(ALUFlag::H, (hl & 0x000F) + (b & 0x000F) > 0x000F);
    c.set_flag(ALUFlag::C, (hl & 0x00FF) + (b & 0x00FF) > 0x00FF);
    hl = hl.wrapping_add(b);
    c.registers.high = (hl >> 8) as u8;
    c.registers.low = hl as u8;
    3
}

fn ld_sp_hl(c: &mut CPU) -> u8 {
    c.registers.sp = (c.registers.high as u16) << 8 | c.registers.low as u16;
    2
}

fn ld_hlim_a(c: &mut CPU) -> u8 {
    let mut hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    c.memory.write_byte(hl, c.registers.acc);
    hl += 1;
    c.registers.high = (hl >> 8) as u8;
    c.registers.low = hl as u8;
    2
}

fn ld_hldm_a(c: &mut CPU) -> u8 {
    let mut hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    c.memory.write_byte(hl, c.registers.acc);
    hl -= 1;
    c.registers.high = (hl >> 8) as u8;
    c.registers.low = hl as u8;
    2
}

fn ld_a_hlim(c: &mut CPU) -> u8 {
    let mut hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = c.memory.read_byte(hl);
    hl = hl.wrapping_add(1);
    c.registers.high = (hl >> 8) as u8;
    c.registers.low = hl as u8;
    c.registers.acc = v;
    2
}

fn ld_a_hldm(c: &mut CPU) -> u8 {
    let mut hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = c.memory.read_byte(hl);
    hl = hl.wrapping_sub(1);
    c.registers.high = (hl >> 8) as u8;
    c.registers.low = hl as u8;
    c.registers.acc = v;
    2
}

fn ld_a16m_a(c: &mut CPU) -> u8 {
    let addr = c.get_word_instr();
    c.memory.write_byte(addr, c.registers.acc);
    4
}

fn ld_a_a16m(c: &mut CPU) -> u8 {
    let addr = c.get_word_instr();
    c.registers.acc = c.memory.read_byte(addr);
    4
}

fn ld_cm_a(c: &mut CPU) -> u8 {
    c.memory
        .write_byte(0xFF00 | c.registers.c as u16, c.registers.acc);
    2
}

fn ld_a_cm(c: &mut CPU) -> u8 {
    c.registers.acc = c.memory.read_byte(0xFF00 | c.registers.c as u16);
    2
}

fn inc_sp(c: &mut CPU) -> u8 {
    // no flags are set for overflows
    c.registers.sp = c.registers.sp.wrapping_add(1);
    2
}

fn inc_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    // no flags are set for overflows
    let val = ((read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16).wrapping_add(1);
    write_reg(c, &r1, (val >> 8) as u8);
    write_reg(c, &r2, val as u8);
    2
}

fn inc_r8(c: &mut CPU, r: Reg) -> u8 {
    // cycles 1
    let v = read_reg(c, &r);
    let vv = v.wrapping_add(1);
    write_reg(c, &r, vv);
    c.set_flag(ALUFlag::Z, vv == 0);
    c.set_flag(ALUFlag::H, (0x18 & v == 0x08) && (0x10 & vv != 0));
    c.set_flag(ALUFlag::N, false);
    1
}

fn dec_r16m(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as usize) << 8 | read_reg(c, &r2) as usize;
    let v = c.memory.ram[addr];
    let vv = v.wrapping_sub(1);
    c.memory.ram[addr] = vv;
    if vv & 0b0000_1000 != 0 && v & 0b0001_0000 != 0 && v & 0b0000_1000 == 0 {
        c.set_flag(ALUFlag::H, true);
    };
    if vv == 0 {
        c.set_flag(ALUFlag::Z, true);
    }
    c.set_flag(ALUFlag::N, true);
    3
}

fn inc_r16m(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as usize) << 8 | read_reg(c, &r2) as usize;
    let v = c.memory.ram[addr];
    let vv = v.wrapping_add(1);
    c.memory.ram[addr] = vv;
    if vv & 0b0001_0000 != 0 && v & 0b0001_0000 == 0 {
        c.set_flag(ALUFlag::H, true);
    }
    if vv == 0 {
        c.set_flag(ALUFlag::Z, true);
    }
    c.set_flag(ALUFlag::N, false);
    3
}

fn dec_r8(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    let vv = v.wrapping_sub(1);
    write_reg(c, &r, vv);
    c.set_flag(ALUFlag::Z, vv == 0);
    c.set_flag(ALUFlag::H, (0x18 & v == 0x10) && (0x08 & vv != 0));
    c.set_flag(ALUFlag::N, true);
    1
}

fn dec_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = ((read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16).wrapping_sub(1);
    write_reg(c, &r1, (v >> 8) as u8);
    write_reg(c, &r2, v as u8);
    2
}

fn dec_sp(c: &mut CPU) -> u8 {
    c.registers.sp = c.registers.sp.wrapping_sub(1);
    2
}

fn rlca(c: &mut CPU) -> u8 {
    let rv = c.registers.acc;
    let carry = rv & 0x80 == 0x80;
    let v = (rv << 1) | if carry { 1 } else { 0 };
    c.registers.acc = v;
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    1
}

fn rla(c: &mut CPU) -> u8 {
    let rv = c.registers.acc;
    let v = (rv << 1) | if c.check_flag(ALUFlag::C) { 1 } else { 0 };
    c.registers.acc = v;
    c.set_flag(ALUFlag::C, rv & 0x80 == 0x80);
    c.set_flag(ALUFlag::Z, false);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    1
}

fn rrca(c: &mut CPU) -> u8 {
    let rv = c.registers.acc;
    let carry = rv & 1 == 1;
    let v = rv >> 1 | if carry { 0x80 } else { 0 };
    c.registers.acc = v;
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, false);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    1
}

fn rra(c: &mut CPU) -> u8 {
    let v = c.registers.acc;
    c.registers.acc = v >> 1 | if c.check_flag(ALUFlag::C) { 0x80 } else { 0 };
    c.set_flag(ALUFlag::C, v & 0x01 == 0x01);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::Z, false);
    1
}

/// Generically do the ALU add operation for 8 bit values and set the proper
/// flags
fn add_u8(c: &mut CPU, v1: u8, v2: u8) -> u8 {
    let overflow_result = v1.wrapping_add(v2);
    match v1.checked_add(v2) {
        Some(_) => c.set_flag(ALUFlag::C, false),
        None => c.set_flag(ALUFlag::C, true),
    }
    c.set_flag(ALUFlag::H, (0x0F & v1) + (0x0F & v2) > 0x0F);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::Z, overflow_result == 0);
    overflow_result
}

/// Generically do the ALU add operation for 16 bit values and set the proper
/// flags
fn add_u16(c: &mut CPU, v1: u16, v2: u16) -> u16 {
    let overflow_result = v1.wrapping_add(v2);
    match v1.checked_add(v2) {
        Some(_) => c.set_flag(ALUFlag::C, false),
        None => c.set_flag(ALUFlag::C, true),
    }
    c.set_flag(ALUFlag::H, (0x00FF & v1) + (0x00FF & v2) > 0x00FF);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::Z, overflow_result == 0);
    overflow_result
}

/// Generically do the ALU adc operation for 8 bit values and set the proper
/// flags
fn adc_u8(c: &mut CPU, v1: u8, v2: u8) -> u8 {
    let carry = if c.check_flag(ALUFlag::C) { 1 } else { 0 };
    let overflow_result = v1.wrapping_add(v2).wrapping_add(carry);
    match v1.checked_add(v2).and_then(|res| res.checked_add(carry)) {
        Some(_) => c.set_flag(ALUFlag::C, false),
        None => c.set_flag(ALUFlag::C, true),
    }
    c.set_flag(ALUFlag::H, (0x0F & v1) + (0x0F & v2) + carry > 0x0F);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::Z, overflow_result == 0);
    overflow_result
}

/// Generically do the ALU sub operation for 8 bit values and set the proper
/// flags
fn sub_u8(c: &mut CPU, v1: u8, v2: u8) -> u8 {
    let result = v1.wrapping_sub(v2);
    c.set_flag(ALUFlag::C, v2 > v1);
    c.set_flag(
        ALUFlag::H,
        ((0xF0 & v1).wrapping_sub(0xF0 & v2) ^ 0x0F) < 0x0F,
    );
    c.set_flag(ALUFlag::Z, result == 0);
    c.set_flag(ALUFlag::N, true);
    result
}

/// Generically do the ALU sbc operation for 8 bit values and set the proper
/// flags
fn sbc_u8(c: &mut CPU, v1: u8, v2: u8) -> u8 {
    let carry = if c.check_flag(ALUFlag::C) { 1 } else { 0 };
    let result = v1.wrapping_sub(v2).wrapping_sub(carry);
    c.set_flag(ALUFlag::C, (v2 + carry) > v1);
    c.set_flag(
        ALUFlag::H,
        ((0xF0 & v1).wrapping_sub(0xF0 & v2).wrapping_sub(carry) ^ 0x0F) < 0x0F,
    );
    c.set_flag(ALUFlag::Z, result == 0);
    c.set_flag(ALUFlag::N, true);
    result
}

/// Only sets flags, no return value needed
fn cp_u8(c: &mut CPU, v1: u8, v2: u8) {
    let result = v1.wrapping_sub(v2);
    c.set_flag(ALUFlag::C, v2 > v1);
    c.set_flag(
        ALUFlag::H,
        (0xF0 & v1).wrapping_sub(0xF0 & v2) ^ 0x0F < 0x0F,
    );
    c.set_flag(ALUFlag::Z, result == 0);
    c.set_flag(ALUFlag::N, true);
}

fn add_hl_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    let result = add_u16(c, hl, v);
    c.registers.high = (result >> 8) as u8;
    c.registers.low = result as u8;
    2
}

fn adc_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let result = adc_u8(c, v, v2);
    write_reg(c, &r1, result);
    1
}

fn adc_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let result = adc_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn add_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let result = add_u8(c, v, v2);
    write_reg(c, &r1, result);
    1
}

fn add_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let result = add_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn add_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let result = add_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn adc_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let result = adc_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn add_r16_sp(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let sp = c.registers.sp;
    let v = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    let result = add_u16(c, v, sp);
    c.registers.high = (result >> 8) as u8;
    c.registers.low = result as u8;
    2
}

fn add_sp_e8(c: &mut CPU) -> u8 {
    let a = c.registers.sp;
    let b = c.get_instr() as i8 as i16 as u16;
    c.registers.sp = add_u16(c, a, b);
    c.set_flag(ALUFlag::Z, false);
    4
}

fn sub_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let result = sub_u8(c, v, v2);
    write_reg(c, &r1, result);
    1
}

fn sub_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let result = sub_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn sbc_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let result = sbc_u8(c, v, v2);
    write_reg(c, &r1, result);
    1
}

fn sbc_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let result = sbc_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn sub_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let result = sub_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn sbc_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let result = sbc_u8(c, v, v2);
    write_reg(c, &r1, result);
    2
}

fn and_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let res = v & v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, true);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn and_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let res = v & v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, true);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    2
}

fn and_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let res = v & v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, true);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn xor_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let res = v ^ v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn xor_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let res = v ^ v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn xor_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let res = v ^ v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn or_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    let res = v | v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn or_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    let res = v | v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    2
}

fn or_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    let res = v | v2;
    write_reg(c, &r1, res);
    c.set_flag(ALUFlag::Z, res == 0);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn cp_r8_r8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = read_reg(c, &r2);
    cp_u8(c, v, v2);
    1
}

fn cp_r8_n8(c: &mut CPU, r1: Reg) -> u8 {
    let v = read_reg(c, &r1);
    let v2 = c.get_instr();
    cp_u8(c, v, v2);
    1
}

fn cp_r8_r16m(c: &mut CPU, r1: Reg, r2: Reg, r3: Reg) -> u8 {
    let addr = (read_reg(c, &r2) as u16) << 8 | read_reg(c, &r3) as u16;
    let v = read_reg(c, &r1);
    let v2 = c.memory.read_byte(addr);
    cp_u8(c, v, v2);
    2
}

fn scf(c: &mut CPU) -> u8 {
    c.set_flag(ALUFlag::C, true);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::N, false);
    1
}

fn stop_n8(c: &mut CPU) -> u8 {
    // mostly used to switch speeds, ignoring for now
    c.stop = true;
    4
}

fn jr_e8(c: &mut CPU) -> u8 {
    let offset = c.get_instr() as i8;
    // convert to u32 to expand the bit range before converting to i32, so sign is not affected
    c.registers.pc = ((c.registers.pc as u32 as i32) + (offset as i32)) as u16;
    c.memory.print_serial();
    //std::thread::sleep(std::time::Duration::from_millis(100));
    3
}

fn jr_nz_e8(c: &mut CPU) -> u8 {
    if c.registers.flags & ALUFlag::Z as u8 == 0 {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (offset as i32)) as u16;
        3
    } else {
        c.registers.pc = c.registers.pc.wrapping_add(1);
        2
    }
}

fn jr_z_e8(c: &mut CPU) -> u8 {
    if c.check_flag(ALUFlag::Z) {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (offset as i32)) as u16;
        3
    } else {
        c.registers.pc = c.registers.pc.wrapping_add(1);
        2
    }
}

fn jr_nc_e8(c: &mut CPU) -> u8 {
    if !c.check_flag(ALUFlag::C) {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (offset as i32)) as u16;
        3
    } else {
        c.registers.pc = c.registers.pc.wrapping_add(1);
        2
    }
}

fn jr_c_e8(c: &mut CPU) -> u8 {
    if c.check_flag(ALUFlag::C) {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as u32 as i32) + (offset as i32)) as u16;
        3
    } else {
        c.registers.pc = c.registers.pc.wrapping_add(1);
        2
    }
}

fn daa(c: &mut CPU) -> u8 {
    let mut acc = c.registers.acc;
    let mut adjust = if c.check_flag(ALUFlag::C) { 0x60 } else { 0x00 };

    if c.check_flag(ALUFlag::H) {
        adjust |= 0x06;
    };

    if c.check_flag(ALUFlag::N) {
        if acc & 0x0F > 0x09 {
            adjust |= 0x06;
        };
        if acc > 0x99 {
            adjust |= 0x60;
        };
        acc = acc.wrapping_add(adjust);
    } else {
        acc = acc.wrapping_sub(adjust);
    }

    c.set_flag(ALUFlag::C, adjust >= 0x60);
    c.set_flag(ALUFlag::H, false);
    c.set_flag(ALUFlag::Z, acc == 0);
    c.registers.acc = acc;
    1
}

fn cpl(c: &mut CPU) -> u8 {
    c.registers.acc = !c.registers.acc;
    c.set_flag(ALUFlag::N, true);
    c.set_flag(ALUFlag::H, true);
    1
}

fn ccf(c: &mut CPU) -> u8 {
    c.registers.flags = c.registers.flags & 0b1001_1111;
    if c.check_flag(ALUFlag::C) {
        c.set_flag(ALUFlag::C, false);
    } else {
        c.set_flag(ALUFlag::C, true);
    }
    1
}

fn halt(c: &mut CPU) -> u8 {
    c.halt = true;
    1
}

fn ret(c: &mut CPU) -> u8 {
    let v = c.memory.read_word(c.registers.sp);
    c.registers.pc = v;
    c.registers.sp += 2;
    4
}

fn ret_cc(c: &mut CPU, flag: ALUFlag, set: bool) -> u8 {
    if c.check_flag(flag) == set {
        let v = c.memory.read_word(c.registers.sp);
        c.registers.pc = v;
        c.registers.sp += 2;
        5
    } else {
        2
    }
}

fn jp_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    c.registers.pc = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    4
}

fn jp_a16(c: &mut CPU) -> u8 {
    c.registers.pc = c.get_word_instr();
    3
}

fn jp_a16_cc(c: &mut CPU, flag: ALUFlag, set: bool) -> u8 {
    if c.check_flag(flag) == set {
        c.registers.pc = c.get_word_instr();
        4
    } else {
        c.registers.pc = c.registers.pc.wrapping_add(2);
        3
    }
}

fn call_a16(c: &mut CPU) -> u8 {
    c.registers.sp -= 2;
    c.memory.write_word(c.registers.sp, c.registers.pc + 2);
    c.registers.pc = c.get_word_instr();
    6
}

fn call_a16_cc(c: &mut CPU, flag: ALUFlag, set: bool) -> u8 {
    if c.check_flag(flag) == set {
        c.registers.sp -= 2;
        c.memory.write_word(c.registers.sp, c.registers.pc + 2);
        c.registers.pc = c.get_word_instr();
        6
    } else {
        3
    }
}

fn pop_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = c.memory.read_word(c.registers.sp);
    write_reg(c, &r1, (v >> 8) as u8);
    write_reg(c, &r2, v as u8);
    c.registers.sp += 2;
    3
}

fn push_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    c.registers.sp -= 2;
    let v = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.memory.write_word(c.registers.sp, v);
    4
}

fn rst(c: &mut CPU, val: u16) -> u8 {
    c.registers.sp = c.registers.sp.wrapping_sub(2);
    c.memory.write_word(c.registers.sp, c.registers.pc);
    c.registers.pc = val;
    4
}

fn prefix(c: &mut CPU) -> u8 {
    let opcode = c.get_instr();
    1 + prefix_instructions::operation(c, opcode)
}

fn reti(c: &mut CPU) -> u8 {
    c.registers.pc = c.memory.read_word(c.registers.sp);
    c.registers.sp += 2;
    c.ei = true;
    4
}

fn di(c: &mut CPU) -> u8 {
    c.di = true;
    1
}

fn ei(c: &mut CPU) -> u8 {
    c.ei = true;
    1
}

// Helpers
pub(crate) fn read_reg(c: &mut CPU, r: &Reg) -> u8 {
    match r {
        Reg::A => c.registers.acc,
        Reg::B => c.registers.b,
        Reg::C => c.registers.c,
        Reg::D => c.registers.d,
        Reg::E => c.registers.e,
        Reg::H => c.registers.high,
        Reg::L => c.registers.low,
        Reg::FLAGS => c.registers.flags,
    }
}

pub(crate) fn write_reg(c: &mut CPU, r: &Reg, v: u8) {
    match r {
        Reg::A => c.registers.acc = v,
        Reg::B => c.registers.b = v,
        Reg::C => c.registers.c = v,
        Reg::D => c.registers.d = v,
        Reg::E => c.registers.e = v,
        Reg::H => c.registers.high = v,
        Reg::L => c.registers.low = v,
        Reg::FLAGS => c.registers.flags = v,
    }
}

/// Each instruction returns _machine cycles_ (M-cycles) instead of _clock
/// ticks_ (T-states). To convert M-cycles to T-states:
///   t_states = m_cycles * 4
pub(crate) fn operations(c: &mut CPU, opcode: u8) -> u8 {
    println!("opcode: {:2x}", opcode);
    match opcode {
        0x0 => 1,
        0x1 => ld_r16_n16(c, Reg::B, Reg::C),
        0x2 => ld_r16m_a(c, Reg::B, Reg::C),
        0x3 => inc_r16(c, Reg::B, Reg::C),
        0x4 => inc_r8(c, Reg::B),
        0x5 => dec_r8(c, Reg::B),
        0x6 => ld_r8_n8(c, Reg::B),
        0x7 => rlca(c),
        0x8 => ld_a16m_sp(c),
        0x9 => add_hl_r16(c, Reg::B, Reg::C),
        0xA => ld_a_r16m(c, Reg::B, Reg::C),
        0xB => dec_r16(c, Reg::B, Reg::C),
        0xC => inc_r8(c, Reg::C),
        0xD => dec_r8(c, Reg::C),
        0xE => ld_r8_n8(c, Reg::C),
        0xF => rrca(c),
        0x10 => stop_n8(c),
        0x11 => ld_r16_n16(c, Reg::D, Reg::E),
        0x12 => ld_r16m_a(c, Reg::D, Reg::E),
        0x13 => inc_r16(c, Reg::D, Reg::E),
        0x14 => inc_r8(c, Reg::D),
        0x15 => dec_r8(c, Reg::D),
        0x16 => ld_r8_n8(c, Reg::D),
        0x17 => rla(c),
        0x18 => jr_e8(c),
        0x19 => add_hl_r16(c, Reg::D, Reg::E),
        0x1A => ld_a_r16m(c, Reg::D, Reg::E),
        0x1B => dec_r16(c, Reg::D, Reg::E),
        0x1C => inc_r8(c, Reg::E),
        0x1D => dec_r8(c, Reg::E),
        0x1E => ld_r8_n8(c, Reg::E),
        0x1F => rra(c),
        0x20 => jr_nz_e8(c),
        0x21 => ld_r16_n16(c, Reg::H, Reg::L),
        0x22 => ld_hlim_a(c),
        0x23 => inc_r16(c, Reg::H, Reg::L),
        0x24 => inc_r8(c, Reg::H),
        0x25 => dec_r8(c, Reg::H),
        0x26 => ld_r8_n8(c, Reg::H),
        0x27 => daa(c),
        0x28 => jr_z_e8(c),
        0x29 => add_hl_r16(c, Reg::H, Reg::L),
        0x2A => ld_a_hlim(c),
        0x2B => dec_r16(c, Reg::H, Reg::L),
        0x2C => inc_r8(c, Reg::L),
        0x2D => dec_r8(c, Reg::L),
        0x2E => ld_r8_n8(c, Reg::L),
        0x2F => cpl(c),
        0x30 => jr_nc_e8(c),
        0x31 => ld_sp_n16(c),
        0x32 => ld_hldm_a(c),
        0x33 => inc_sp(c),
        0x34 => inc_r16m(c, Reg::H, Reg::L),
        0x35 => dec_r16m(c, Reg::H, Reg::L),
        0x36 => ld_r16m_n8(c, Reg::H, Reg::L),
        0x37 => scf(c),
        0x38 => jr_c_e8(c),
        0x39 => add_r16_sp(c, Reg::H, Reg::L),
        0x3A => ld_a_hldm(c),
        0x3B => dec_sp(c),
        0x3C => inc_r8(c, Reg::A),
        0x3D => dec_r8(c, Reg::A),
        0x3E => ld_r8_n8(c, Reg::A),
        0x3F => ccf(c),
        0x40 => ld_r8_r8(c, Reg::B, Reg::B),
        0x41 => ld_r8_r8(c, Reg::B, Reg::C),
        0x42 => ld_r8_r8(c, Reg::B, Reg::D),
        0x43 => ld_r8_r8(c, Reg::B, Reg::E),
        0x44 => ld_r8_r8(c, Reg::B, Reg::H),
        0x45 => ld_r8_r8(c, Reg::B, Reg::L),
        0x46 => ld_r8_r16m(c, Reg::B, Reg::H, Reg::L),
        0x47 => ld_r8_r8(c, Reg::B, Reg::A),
        0x48 => ld_r8_r8(c, Reg::C, Reg::B),
        0x49 => ld_r8_r8(c, Reg::C, Reg::C),
        0x4A => ld_r8_r8(c, Reg::C, Reg::D),
        0x4B => ld_r8_r8(c, Reg::C, Reg::E),
        0x4C => ld_r8_r8(c, Reg::C, Reg::H),
        0x4D => ld_r8_r8(c, Reg::C, Reg::L),
        0x4E => ld_r8_r16m(c, Reg::C, Reg::H, Reg::L),
        0x4F => ld_r8_r8(c, Reg::C, Reg::A),
        0x50 => ld_r8_r8(c, Reg::D, Reg::B),
        0x51 => ld_r8_r8(c, Reg::D, Reg::C),
        0x52 => ld_r8_r8(c, Reg::D, Reg::D),
        0x53 => ld_r8_r8(c, Reg::D, Reg::E),
        0x54 => ld_r8_r8(c, Reg::D, Reg::H),
        0x55 => ld_r8_r8(c, Reg::D, Reg::L),
        0x56 => ld_r8_r16m(c, Reg::D, Reg::H, Reg::L),
        0x57 => ld_r8_r8(c, Reg::D, Reg::A),
        0x58 => ld_r8_r8(c, Reg::E, Reg::B),
        0x59 => ld_r8_r8(c, Reg::E, Reg::C),
        0x5A => ld_r8_r8(c, Reg::E, Reg::D),
        0x5B => ld_r8_r8(c, Reg::E, Reg::E),
        0x5C => ld_r8_r8(c, Reg::E, Reg::H),
        0x5D => ld_r8_r8(c, Reg::E, Reg::L),
        0x5E => ld_r8_r16m(c, Reg::E, Reg::H, Reg::L),
        0x5F => ld_r8_r8(c, Reg::E, Reg::A),
        0x60 => ld_r8_r8(c, Reg::H, Reg::B),
        0x61 => ld_r8_r8(c, Reg::H, Reg::C),
        0x62 => ld_r8_r8(c, Reg::H, Reg::D),
        0x63 => ld_r8_r8(c, Reg::H, Reg::E),
        0x64 => ld_r8_r8(c, Reg::H, Reg::H),
        0x65 => ld_r8_r8(c, Reg::H, Reg::L),
        0x66 => ld_r8_r16m(c, Reg::H, Reg::H, Reg::L),
        0x67 => ld_r8_r8(c, Reg::H, Reg::A),
        0x68 => ld_r8_r8(c, Reg::L, Reg::B),
        0x69 => ld_r8_r8(c, Reg::L, Reg::C),
        0x6A => ld_r8_r8(c, Reg::L, Reg::D),
        0x6B => ld_r8_r8(c, Reg::L, Reg::E),
        0x6C => ld_r8_r8(c, Reg::L, Reg::H),
        0x6D => ld_r8_r8(c, Reg::L, Reg::L),
        0x6E => ld_r8_r16m(c, Reg::L, Reg::H, Reg::L),
        0x6F => ld_r8_r8(c, Reg::L, Reg::A),
        0x70 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::B),
        0x71 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::C),
        0x72 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::D),
        0x73 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::E),
        0x74 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::H),
        0x75 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::L),
        0x76 => halt(c),
        0x77 => ld_r16m_r8(c, Reg::H, Reg::L, Reg::A),
        0x78 => ld_r8_r8(c, Reg::A, Reg::B),
        0x79 => ld_r8_r8(c, Reg::A, Reg::C),
        0x7A => ld_r8_r8(c, Reg::A, Reg::D),
        0x7B => ld_r8_r8(c, Reg::A, Reg::E),
        0x7C => ld_r8_r8(c, Reg::A, Reg::H),
        0x7D => ld_r8_r8(c, Reg::A, Reg::L),
        0x7E => ld_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0x7F => ld_r8_r8(c, Reg::A, Reg::A),
        0x80 => add_r8_r8(c, Reg::A, Reg::B),
        0x81 => add_r8_r8(c, Reg::A, Reg::C),
        0x82 => add_r8_r8(c, Reg::A, Reg::D),
        0x83 => add_r8_r8(c, Reg::A, Reg::E),
        0x84 => add_r8_r8(c, Reg::A, Reg::H),
        0x85 => add_r8_r8(c, Reg::A, Reg::L),
        0x86 => add_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0x87 => add_r8_r8(c, Reg::A, Reg::A),
        0x88 => adc_r8_r8(c, Reg::A, Reg::B),
        0x89 => adc_r8_r8(c, Reg::A, Reg::C),
        0x8A => adc_r8_r8(c, Reg::A, Reg::D),
        0x8B => adc_r8_r8(c, Reg::A, Reg::E),
        0x8C => adc_r8_r8(c, Reg::A, Reg::H),
        0x8D => adc_r8_r8(c, Reg::A, Reg::L),
        0x8E => adc_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0x8F => adc_r8_r8(c, Reg::A, Reg::A),
        0x90 => sub_r8_r8(c, Reg::A, Reg::B),
        0x91 => sub_r8_r8(c, Reg::A, Reg::C),
        0x92 => sub_r8_r8(c, Reg::A, Reg::D),
        0x93 => sub_r8_r8(c, Reg::A, Reg::E),
        0x94 => sub_r8_r8(c, Reg::A, Reg::H),
        0x95 => sub_r8_r8(c, Reg::A, Reg::L),
        0x96 => sub_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0x97 => sub_r8_r8(c, Reg::A, Reg::A),
        0x98 => sbc_r8_r8(c, Reg::A, Reg::B),
        0x99 => sbc_r8_r8(c, Reg::A, Reg::C),
        0x9A => sbc_r8_r8(c, Reg::A, Reg::D),
        0x9B => sbc_r8_r8(c, Reg::A, Reg::E),
        0x9C => sbc_r8_r8(c, Reg::A, Reg::H),
        0x9D => sbc_r8_r8(c, Reg::A, Reg::L),
        0x9E => sbc_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0x9F => sbc_r8_r8(c, Reg::A, Reg::A),
        0xA0 => and_r8_r8(c, Reg::A, Reg::B),
        0xA1 => and_r8_r8(c, Reg::A, Reg::C),
        0xA2 => and_r8_r8(c, Reg::A, Reg::D),
        0xA3 => and_r8_r8(c, Reg::A, Reg::E),
        0xA4 => and_r8_r8(c, Reg::A, Reg::H),
        0xA5 => and_r8_r8(c, Reg::A, Reg::L),
        0xA6 => and_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0xA7 => and_r8_r8(c, Reg::A, Reg::A),
        0xA8 => xor_r8_r8(c, Reg::A, Reg::B),
        0xA9 => xor_r8_r8(c, Reg::A, Reg::C),
        0xAA => xor_r8_r8(c, Reg::A, Reg::D),
        0xAB => xor_r8_r8(c, Reg::A, Reg::E),
        0xAC => xor_r8_r8(c, Reg::A, Reg::H),
        0xAD => xor_r8_r8(c, Reg::A, Reg::L),
        0xAE => xor_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0xAF => xor_r8_r8(c, Reg::A, Reg::A),
        0xB0 => or_r8_r8(c, Reg::A, Reg::B),
        0xB1 => or_r8_r8(c, Reg::A, Reg::C),
        0xB2 => or_r8_r8(c, Reg::A, Reg::D),
        0xB3 => or_r8_r8(c, Reg::A, Reg::E),
        0xB4 => or_r8_r8(c, Reg::A, Reg::H),
        0xB5 => or_r8_r8(c, Reg::A, Reg::L),
        0xB6 => or_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0xB7 => or_r8_r8(c, Reg::A, Reg::A),
        0xB8 => cp_r8_r8(c, Reg::A, Reg::B),
        0xB9 => cp_r8_r8(c, Reg::A, Reg::C),
        0xBA => cp_r8_r8(c, Reg::A, Reg::D),
        0xBB => cp_r8_r8(c, Reg::A, Reg::E),
        0xBC => cp_r8_r8(c, Reg::A, Reg::H),
        0xBD => cp_r8_r8(c, Reg::A, Reg::L),
        0xBE => cp_r8_r16m(c, Reg::A, Reg::H, Reg::L),
        0xBF => cp_r8_r8(c, Reg::A, Reg::A),
        0xC0 => ret_cc(c, ALUFlag::Z, false),
        0xC1 => pop_r16(c, Reg::B, Reg::C),
        0xC2 => jp_a16_cc(c, ALUFlag::Z, false),
        0xC3 => jp_a16(c),
        0xC4 => call_a16_cc(c, ALUFlag::Z, false),
        0xC5 => push_r16(c, Reg::B, Reg::C),
        0xC6 => add_r8_n8(c, Reg::A),
        0xC7 => rst(c, 0x0),
        0xC8 => ret_cc(c, ALUFlag::Z, true),
        0xC9 => ret(c),
        0xCA => jp_a16_cc(c, ALUFlag::Z, true),
        0xCB => prefix(c),
        0xCC => call_a16_cc(c, ALUFlag::Z, true),
        0xCD => call_a16(c),
        0xCE => adc_r8_n8(c, Reg::A),
        0xCF => rst(c, 0x8),
        0xD0 => ret_cc(c, ALUFlag::C, false),
        0xD1 => pop_r16(c, Reg::D, Reg::E),
        0xD2 => jp_a16_cc(c, ALUFlag::C, false),
        // no D3
        0xD4 => call_a16_cc(c, ALUFlag::C, false),
        0xD5 => push_r16(c, Reg::D, Reg::E),
        0xD6 => sub_r8_n8(c, Reg::A),
        0xD7 => rst(c, 0x10),
        0xD8 => ret_cc(c, ALUFlag::C, true),
        0xD9 => reti(c),
        0xDA => jp_a16_cc(c, ALUFlag::C, true),
        // no DB
        0xDC => call_a16_cc(c, ALUFlag::C, true),
        // no DD
        0xDE => sbc_r8_n8(c, Reg::A),
        0xDF => rst(c, 0x18),
        0xE0 => ld_a8m_a(c),
        0xE1 => pop_r16(c, Reg::H, Reg::L),
        0xE2 => ld_cm_a(c),
        // no E3
        // no E4
        0xE5 => push_r16(c, Reg::H, Reg::L),
        0xE6 => and_r8_n8(c, Reg::A),
        0xE7 => rst(c, 0x20),
        0xE8 => add_sp_e8(c),
        0xE9 => jp_r16(c, Reg::H, Reg::L),
        0xEA => ld_a16m_a(c),
        // no EB
        // no EC
        // no ED
        0xEE => xor_r8_n8(c, Reg::A),
        0xEF => rst(c, 0x28),
        0xF0 => ld_a_a8m(c),
        0xF1 => pop_r16(c, Reg::A, Reg::FLAGS),
        0xF2 => ld_a_cm(c),
        0xF3 => di(c),
        // no F4
        0xF5 => push_r16(c, Reg::A, Reg::FLAGS),
        0xF6 => or_r8_n8(c, Reg::A),
        0xF7 => rst(c, 0x30),
        0xF8 => ld_hl_spe8(c),
        0xF9 => ld_sp_hl(c),
        0xFA => ld_a_a16m(c),
        0xFB => ei(c),
        // no FC
        // no FD
        0xFE => cp_r8_n8(c, Reg::A),
        0xFF => rst(c, 0x38),
        _ => {
            eprintln!("OpCode is not implemented: {}", opcode);
            1
        }
    }
}
