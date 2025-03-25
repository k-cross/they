use super::{ALUFlag, CPU};
use crate::cpu::instructions::{Reg, read_reg, write_reg};

// 9 bit rotate
fn rl_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let v = (rv << 1) | if c.check_flag(ALUFlag::C) { 1 } else { 0 };
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, v & 0x80 == 0x80);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 9 bit rotate
fn rl_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let v = (rv << 1) | if c.check_flag(ALUFlag::C) { 1 } else { 0 };
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, v & 0x80 == 0x80);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

// 9 bit rotate
fn rr_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let v = (rv >> 1) | if c.check_flag(ALUFlag::C) { 0x80 } else { 0 };
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, v & 0x01 == 0x01);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 9 bit rotate
fn rr_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let v = (rv >> 1) | if c.check_flag(ALUFlag::C) { 0x80 } else { 0 };
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, v & 0x01 == 0x01);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

// 8 bit rotate
fn rlc_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let carry = rv & 0x80 == 0x80;
    let v = (rv << 1) | if carry { 1 } else { 0 };
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 8 bit rotate
fn rlc_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let carry = rv & 0x80 == 0x80;
    let v = (rv << 1) | if carry { 1 } else { 0 };
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

// 8 bit rotate
fn rrc_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let carry = rv & 1 == 1;
    let v = rv >> 1 | if carry { 0x80 } else { 0 };
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 8 bit rotate
fn rrc_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let carry = rv & 1 == 1;
    let v = rv >> 1 | if carry { 0x80 } else { 0 };
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, carry);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

fn sla_r8(c: &mut CPU, r: Reg) -> u8 {
    8
}

pub(crate) fn operation(c: &mut CPU, opcode: u8) -> u8 {
    match opcode {
        0x00 => rlc_r8(c, Reg::B),
        0x01 => rlc_r8(c, Reg::C),
        0x02 => rlc_r8(c, Reg::D),
        0x03 => rlc_r8(c, Reg::E),
        0x04 => rlc_r8(c, Reg::H),
        0x05 => rlc_r8(c, Reg::L),
        0x06 => rlc_hlm(c),
        0x07 => rlc_r8(c, Reg::A),
        0x08 => rrc_r8(c, Reg::B),
        0x09 => rrc_r8(c, Reg::C),
        0x0A => rrc_r8(c, Reg::D),
        0x0B => rrc_r8(c, Reg::E),
        0x0C => rrc_r8(c, Reg::H),
        0x0D => rrc_r8(c, Reg::L),
        0x0E => rrc_hlm(c),
        0x0F => rrc_r8(c, Reg::A),
        0x10 => rl_r8(c, Reg::B),
        0x11 => rl_r8(c, Reg::C),
        0x12 => rl_r8(c, Reg::D),
        0x13 => rl_r8(c, Reg::E),
        0x14 => rl_r8(c, Reg::H),
        0x15 => rl_r8(c, Reg::L),
        0x16 => rl_hlm(c),
        0x17 => rl_r8(c, Reg::A),
        0x18 => rr_r8(c, Reg::B),
        0x19 => rr_r8(c, Reg::C),
        0x1A => rr_r8(c, Reg::D),
        0x1B => rr_r8(c, Reg::E),
        0x1C => rr_r8(c, Reg::H),
        0x1D => rr_r8(c, Reg::L),
        0x1E => rr_hlm(c),
        0x1F => rr_r8(c, Reg::A),
        0x20 => sla_r8(c, Reg::A),
        _ => {
            eprintln!("Prefix Opcode is not implemented: {}", opcode);
            1
        }
    }
}
