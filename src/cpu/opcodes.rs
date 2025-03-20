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

fn ld_r16_n16(c: &mut CPU, r1: Reg, r2: Reg) {
    // cycles 3
    let val = c.get_word_instr();
    write_reg(c, &r1, (val >> 8) as u8);
    write_reg(c, &r2, val as u8);
}

fn ld_r16m_a(c: &mut CPU, r1: Reg, r2: Reg) {
    // cycles 2
    let addr = (read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16;
    c.memory.write_byte(addr, c.registers.acc);
}

fn inc_r16(c: &mut CPU, r1: Reg, r2: Reg) {
    // cycles 2
    // no flags are set for overflows
    let val = ((read_reg(c, &r1) as u16) << 8 | read_reg(c, &r2) as u16).wrapping_add(1);
    write_reg(c, &r1, (val >> 8) as u8);
    write_reg(c, &r2, val as u8);
}

fn inc_r8(c: &mut CPU, r: Reg) {
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
        0x0 => {
            // cycles 1
            ()
        }
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
        _ => eprintln!("OpCode is not implemented: {}", opcode),
    }
}
