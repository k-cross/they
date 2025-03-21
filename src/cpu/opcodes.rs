use super::{ALUFlag, CPU};

// private to make functions more generic since a lot of the handling will be
// the same.
enum Reg {
    A,
    B,
    C,
    D,
    E,
    FLAGS,
    H,
    L,
}

// return values of instructions are cycles or ticks
fn ld_r8_n8(c: &mut CPU, r: Reg) -> u8 {
    let v = c.get_instr();
    write_reg(c, &r, v);
    2
}

fn ld_r16_n16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let val = c.get_word_instr();
    write_reg(c, &r1, (val >> 8) as u8);
    write_reg(c, &r2, val as u8);
    3
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

fn ld_a_r16m(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.registers.acc = c.memory.read_byte(addr);
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
    match v.checked_add(1) {
        Some(vv) => {
            write_reg(c, &r, vv);
            if (0b00001000 & v == 0) && (0b00001000 & vv != 0) {
                c.registers.flags = ALUFlag::H as u8;
            }
        }
        None => {
            write_reg(c, &r, 0);
            c.registers.flags = ALUFlag::C as u8;
        }
    }
    1
}

fn dec_r8(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    match v.checked_sub(1) {
        Some(vv) => {
            write_reg(c, &r, vv);
            if (0b00001000 & v == 0) && (0b00001000 & vv != 0) {
                c.registers.flags = ALUFlag::H as u8 | ALUFlag::N as u8;
            }
            if vv == 0 {
                c.registers.flags = c.registers.flags | ALUFlag::Z as u8;
            }
        }
        None => {
            write_reg(c, &r, 0);
            c.registers.flags = ALUFlag::C as u8 | ALUFlag::N as u8;
        }
    }
    1
}

fn dec_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = ((read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16).wrapping_sub(1);
    write_reg(c, &r1, (v >> 8) as u8);
    write_reg(c, &r2, v as u8);
    2
}

fn rlca(c: &mut CPU) -> u8 {
    let _ = rlc(c, Reg::A);
    1
}

fn rlc(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    if 0b1000000 & v == 0 {
        write_reg(c, &r, v << 1);
    } else {
        write_reg(c, &r, v << 1);
        c.registers.flags = ALUFlag::C as u8;
    }
    2
}

fn rla(c: &mut CPU) -> u8 {
    let _ = rl(c, Reg::A);
    1
}

fn rl(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    let carry = c.registers.flags & ALUFlag::C as u8 != 0;
    if 0b1000000 & v == 0 {
        if carry {
            write_reg(c, &r, (v << 1) | 0b1);
        } else {
            write_reg(c, &r, v << 1);
        }
    } else {
        if carry {
            write_reg(c, &r, (v << 1) | 0b1);
            c.registers.flags = ALUFlag::C as u8;
        } else {
            write_reg(c, &r, v << 1);
            c.registers.flags = ALUFlag::C as u8;
        }
    }
    2
}

fn rrca(c: &mut CPU) -> u8 {
    let _ = rrc(c, Reg::A);
    1
}

fn rrc(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    if 0b01 & v == 0 {
        write_reg(c, &r, v >> 1);
    } else {
        write_reg(c, &r, v >> 1);
        c.registers.flags = ALUFlag::C as u8;
    }
    2
}

fn add_hl_r16(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let hl = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    match v.checked_add(hl) {
        Some(vv) => {
            if (0b00001000_00000000 & hl == 0) && (0b00001000_00000000 & vv != 0) {
                c.registers.flags = ALUFlag::H as u8;
            }
            c.registers.high = (vv >> 8) as u8;
            c.registers.low = vv as u8;
        }
        None => {
            c.registers.high = 0;
            c.registers.low = 0;
            c.registers.flags = ALUFlag::C as u8;
        }
    }
    2
}

fn stop_n8(_c: &mut CPU) -> u8 {
    // mostly used to switch speeds, ignoring for now
    todo!()
}

fn jr_e8(c: &mut CPU) -> u8 {
    let offset = c.get_instr() as i8;
    c.registers.pc = ((c.registers.pc as i32) + (offset as i32)) as u16;
    3
}

fn read_reg(c: &mut CPU, r: &Reg) -> u8 {
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

fn write_reg(c: &mut CPU, r: &Reg, v: u8) {
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

pub(crate) fn operations(c: &mut CPU, opcode: u8) {
    match opcode {
        0x0 => (),
        0x1 => {
            ld_r16_n16(c, Reg::B, Reg::C);
        }
        0x2 => {
            ld_r16m_a(c, Reg::B, Reg::C);
        }
        0x3 => {
            inc_r16(c, Reg::B, Reg::C);
        }
        0x4 => {
            inc_r8(c, Reg::B);
        }
        0x5 => {
            dec_r8(c, Reg::B);
        }
        0x6 => {
            ld_r8_n8(c, Reg::B);
        }
        0x7 => {
            rlca(c);
        }
        0x8 => {
            ld_a16m_sp(c);
        }
        0x9 => {
            add_hl_r16(c, Reg::B, Reg::C);
        }
        0xA => {
            ld_a_r16m(c, Reg::B, Reg::C);
        }
        0xB => {
            dec_r16(c, Reg::B, Reg::C);
        }
        0xC => {
            inc_r8(c, Reg::C);
        }
        0xD => {
            dec_r8(c, Reg::C);
        }
        0xE => {
            ld_r8_n8(c, Reg::C);
        }
        0xF => {
            rrca(c);
        }
        0x10 => {
            stop_n8(c);
        }
        0x11 => {
            ld_r16_n16(c, Reg::D, Reg::E);
        }
        0x12 => {
            ld_r16m_a(c, Reg::D, Reg::E);
        }
        0x13 => {
            inc_r16(c, Reg::D, Reg::E);
        }
        0x14 => {
            inc_r8(c, Reg::D);
        }
        0x15 => {
            dec_r8(c, Reg::D);
        }
        0x16 => {
            ld_r8_n8(c, Reg::D);
        }
        0x17 => {
            rla(c);
        }
        0x18 => {
            jr_e8(c);
        }
        0x19 => {
            add_hl_r16(c, Reg::D, Reg::E);
        }
        0x1A => {
            ld_a_r16m(c, Reg::D, Reg::E);
        }
        _ => eprintln!("OpCode is not implemented: {}", opcode),
    }
}
