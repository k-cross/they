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

// return values of instructions are machine cycles
fn ld_r8_n8(c: &mut CPU, r: Reg) -> u8 {
    let v = c.get_instr();
    write_reg(c, &r, v);
    2
}

fn ld_r16m_n8(c: &mut CPU, r1: Reg, r2: Reg) -> u8 {
    let v = c.get_instr();
    let addr = (read_reg(c, &r1) as usize) << 8 | read_reg(c, &r2) as usize;
    c.memory.ram[addr] = v;
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
        c.registers.flags = 0;
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
        c.registers.flags = 0;
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
        c.registers.flags = 0;
    } else {
        write_reg(c, &r, v >> 1);
        c.registers.flags = ALUFlag::C as u8;
    }
    2
}

fn rra(c: &mut CPU) -> u8 {
    let _ = rr(c, Reg::A);
    1
}

fn rr(c: &mut CPU, r: Reg) -> u8 {
    let v = read_reg(c, &r);
    let carry = c.registers.flags & ALUFlag::C as u8 != 0;
    if 0b0000_0001 & v == 0 {
        if carry {
            write_reg(c, &r, (v >> 1) | 0b1000_0000);
        } else {
            write_reg(c, &r, v >> 1);
        }
        c.registers.flags = 0;
    } else {
        if carry {
            write_reg(c, &r, (v >> 1) | 0b1000_0000);
            c.registers.flags = ALUFlag::C as u8;
        } else {
            write_reg(c, &r, v >> 1);
            c.registers.flags = ALUFlag::C as u8;
        }
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

fn scf(c: &mut CPU) -> u8 {
    c.set_flag(ALUFlag::C, true);
    1
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

fn jr_nz_e8(c: &mut CPU) -> u8 {
    if c.registers.flags & ALUFlag::Z as u8 == 0 {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as i32) + offset as i32) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}

fn jr_z_e8(c: &mut CPU) -> u8 {
    if c.check_flag(ALUFlag::Z) {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as i32) + offset as i32) as u16;
        3
    } else {
        c.registers.pc += 1;
        2
    }
}

fn jr_nc_e8(c: &mut CPU) -> u8 {
    if c.registers.flags & ALUFlag::C as u8 == 0 {
        let offset = c.get_instr() as i8;
        c.registers.pc = ((c.registers.pc as i32) + offset as i32) as u16;
        3
    } else {
        c.registers.pc += 1;
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
        c.set_flag(ALUFlag::N, false);
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

// Helpers
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

/// Each instruction returns _machine cycles_ (M-cycles) instead of _clock
/// ticks_ (T-states). To convert M-cycles to T-states:
///   t_states = m_cycles * 4
pub(crate) fn operations(c: &mut CPU, opcode: u8) -> u8 {
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
        0x3A => ld_a_hldm(c),
        _ => {
            eprintln!("OpCode is not implemented: {}", opcode);
            1
        }
    }
}
