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

// 9 bit shift
fn sla_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let v = rv << 1;
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, rv & 0x80 == 0x80);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 9 bit shift
fn sla_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let v = rv << 1;
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, rv & 0x80 == 0x80);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

// 8 bit shift
fn sra_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let v = rv >> 1 | (rv & 0x80);
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, rv & 1 == 1);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 8 bit shift
fn sra_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let v = rv >> 1 | (rv & 0x80);
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, rv & 1 == 1);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

// 9 bit shift
fn srl_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let v = rv >> 1;
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::C, rv & 1 == 1);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

// 9 bit shift
fn srl_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let v = rv >> 1;
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::C, rv & 1 == 1);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

fn swap_r8(c: &mut CPU, r: Reg) -> u8 {
    let rv = read_reg(c, &r);
    let tmp = rv >> 4;
    let v = rv << 4 | tmp;
    write_reg(c, &r, v);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    2
}

fn swap_hlm(c: &mut CPU) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let rv = c.memory.read_byte(addr);
    let tmp = rv >> 4;
    let v = rv << 4 | tmp;
    c.memory.write_byte(addr, v);
    c.set_flag(ALUFlag::Z, v == 0);
    c.set_flag(ALUFlag::C, false);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, false);
    4
}

fn bit_r8(c: &mut CPU, mask: u8, r: Reg) -> u8 {
    let z = read_reg(c, &r) & mask == 0;
    c.set_flag(ALUFlag::Z, z);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, true);
    2
}

fn bit_hlm(c: &mut CPU, mask: u8) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let z = c.memory.read_byte(addr) & mask == 0;
    c.set_flag(ALUFlag::Z, z);
    c.set_flag(ALUFlag::N, false);
    c.set_flag(ALUFlag::H, true);
    3
}

fn res_r8(c: &mut CPU, mask: u8, r: Reg) -> u8 {
    let v = read_reg(c, &r) & !mask;
    write_reg(c, &r, v);
    2
}

fn res_hlm(c: &mut CPU, mask: u8) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = c.memory.read_byte(addr) & !mask;
    c.memory.write_byte(addr, v);
    4
}

fn set_r8(c: &mut CPU, mask: u8, r: Reg) -> u8 {
    let v = read_reg(c, &r) | mask;
    write_reg(c, &r, v);
    2
}

fn set_hlm(c: &mut CPU, mask: u8) -> u8 {
    let addr = (c.registers.high as u16) << 8 | c.registers.low as u16;
    let v = c.memory.read_byte(addr) | mask;
    c.memory.write_byte(addr, v);
    4
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
        0x20 => sla_r8(c, Reg::B),
        0x21 => sla_r8(c, Reg::C),
        0x22 => sla_r8(c, Reg::D),
        0x23 => sla_r8(c, Reg::E),
        0x24 => sla_r8(c, Reg::H),
        0x25 => sla_r8(c, Reg::L),
        0x26 => sla_hlm(c),
        0x27 => sla_r8(c, Reg::A),
        0x28 => sra_r8(c, Reg::B),
        0x29 => sra_r8(c, Reg::C),
        0x2A => sra_r8(c, Reg::D),
        0x2B => sra_r8(c, Reg::E),
        0x2C => sra_r8(c, Reg::H),
        0x2D => sra_r8(c, Reg::L),
        0x2E => sra_hlm(c),
        0x2F => sra_r8(c, Reg::A),
        0x30 => swap_r8(c, Reg::B),
        0x31 => swap_r8(c, Reg::C),
        0x32 => swap_r8(c, Reg::D),
        0x33 => swap_r8(c, Reg::E),
        0x34 => swap_r8(c, Reg::H),
        0x35 => swap_r8(c, Reg::L),
        0x36 => swap_hlm(c),
        0x37 => swap_r8(c, Reg::A),
        0x38 => srl_r8(c, Reg::B),
        0x39 => srl_r8(c, Reg::C),
        0x3A => srl_r8(c, Reg::D),
        0x3B => srl_r8(c, Reg::E),
        0x3C => srl_r8(c, Reg::H),
        0x3D => srl_r8(c, Reg::L),
        0x3E => srl_hlm(c),
        0x3F => srl_r8(c, Reg::A),
        0x40 => bit_r8(c, 0b0000_0001, Reg::B),
        0x41 => bit_r8(c, 0b0000_0001, Reg::C),
        0x42 => bit_r8(c, 0b0000_0001, Reg::D),
        0x43 => bit_r8(c, 0b0000_0001, Reg::E),
        0x44 => bit_r8(c, 0b0000_0001, Reg::H),
        0x45 => bit_r8(c, 0b0000_0001, Reg::L),
        0x46 => bit_hlm(c, 0b0000_0001),
        0x47 => bit_r8(c, 0b0000_0001, Reg::A),
        0x48 => bit_r8(c, 0b0000_0010, Reg::B),
        0x49 => bit_r8(c, 0b0000_0010, Reg::C),
        0x4A => bit_r8(c, 0b0000_0010, Reg::D),
        0x4B => bit_r8(c, 0b0000_0010, Reg::E),
        0x4C => bit_r8(c, 0b0000_0010, Reg::H),
        0x4D => bit_r8(c, 0b0000_0010, Reg::L),
        0x4E => bit_hlm(c, 0b0000_0010),
        0x4F => bit_r8(c, 0b0000_0010, Reg::A),
        0x50 => bit_r8(c, 0b0000_0100, Reg::B),
        0x51 => bit_r8(c, 0b0000_0100, Reg::C),
        0x52 => bit_r8(c, 0b0000_0100, Reg::D),
        0x53 => bit_r8(c, 0b0000_0100, Reg::E),
        0x54 => bit_r8(c, 0b0000_0100, Reg::H),
        0x55 => bit_r8(c, 0b0000_0100, Reg::L),
        0x56 => bit_hlm(c, 0b0000_0100),
        0x57 => bit_r8(c, 0b0000_0100, Reg::A),
        0x58 => bit_r8(c, 0b0000_1000, Reg::B),
        0x59 => bit_r8(c, 0b0000_1000, Reg::C),
        0x5A => bit_r8(c, 0b0000_1000, Reg::D),
        0x5B => bit_r8(c, 0b0000_1000, Reg::E),
        0x5C => bit_r8(c, 0b0000_1000, Reg::H),
        0x5D => bit_r8(c, 0b0000_1000, Reg::L),
        0x5E => bit_hlm(c, 0b0000_1000),
        0x5F => bit_r8(c, 0b0000_1000, Reg::A),
        0x60 => bit_r8(c, 0b0001_0000, Reg::B),
        0x61 => bit_r8(c, 0b0001_0000, Reg::C),
        0x62 => bit_r8(c, 0b0001_0000, Reg::D),
        0x63 => bit_r8(c, 0b0001_0000, Reg::E),
        0x64 => bit_r8(c, 0b0001_0000, Reg::H),
        0x65 => bit_r8(c, 0b0001_0000, Reg::L),
        0x66 => bit_hlm(c, 0b0001_0000),
        0x67 => bit_r8(c, 0b0001_0000, Reg::A),
        0x68 => bit_r8(c, 0b0010_0000, Reg::B),
        0x69 => bit_r8(c, 0b0010_0000, Reg::C),
        0x6A => bit_r8(c, 0b0010_0000, Reg::D),
        0x6B => bit_r8(c, 0b0010_0000, Reg::E),
        0x6C => bit_r8(c, 0b0010_0000, Reg::H),
        0x6D => bit_r8(c, 0b0010_0000, Reg::L),
        0x6E => bit_hlm(c, 0b0010_0000),
        0x6F => bit_r8(c, 0b0010_0000, Reg::A),
        0x70 => bit_r8(c, 0b0100_0000, Reg::B),
        0x71 => bit_r8(c, 0b0100_0000, Reg::C),
        0x72 => bit_r8(c, 0b0100_0000, Reg::D),
        0x73 => bit_r8(c, 0b0100_0000, Reg::E),
        0x74 => bit_r8(c, 0b0100_0000, Reg::H),
        0x75 => bit_r8(c, 0b0100_0000, Reg::L),
        0x76 => bit_hlm(c, 0b0100_0000),
        0x77 => bit_r8(c, 0b0100_0000, Reg::A),
        0x78 => bit_r8(c, 0b1000_0000, Reg::B),
        0x79 => bit_r8(c, 0b1000_0000, Reg::C),
        0x7A => bit_r8(c, 0b1000_0000, Reg::D),
        0x7B => bit_r8(c, 0b1000_0000, Reg::E),
        0x7C => bit_r8(c, 0b1000_0000, Reg::H),
        0x7D => bit_r8(c, 0b1000_0000, Reg::L),
        0x7E => bit_hlm(c, 0b1000_0000),
        0x7F => bit_r8(c, 0b1000_0000, Reg::A),
        0x80 => res_r8(c, 0b0000_0001, Reg::B),
        0x81 => res_r8(c, 0b0000_0001, Reg::C),
        0x82 => res_r8(c, 0b0000_0001, Reg::D),
        0x83 => res_r8(c, 0b0000_0001, Reg::E),
        0x84 => res_r8(c, 0b0000_0001, Reg::H),
        0x85 => res_r8(c, 0b0000_0001, Reg::L),
        0x86 => res_hlm(c, 0b0000_0001),
        0x87 => res_r8(c, 0b0000_0001, Reg::A),
        0x88 => res_r8(c, 0b0000_0010, Reg::B),
        0x89 => res_r8(c, 0b0000_0010, Reg::C),
        0x8A => res_r8(c, 0b0000_0010, Reg::D),
        0x8B => res_r8(c, 0b0000_0010, Reg::E),
        0x8C => res_r8(c, 0b0000_0010, Reg::H),
        0x8D => res_r8(c, 0b0000_0010, Reg::L),
        0x8E => res_hlm(c, 0b0000_0010),
        0x8F => res_r8(c, 0b0000_0010, Reg::A),
        0x90 => res_r8(c, 0b0000_0100, Reg::B),
        0x91 => res_r8(c, 0b0000_0100, Reg::C),
        0x92 => res_r8(c, 0b0000_0100, Reg::D),
        0x93 => res_r8(c, 0b0000_0100, Reg::E),
        0x94 => res_r8(c, 0b0000_0100, Reg::H),
        0x95 => res_r8(c, 0b0000_0100, Reg::L),
        0x96 => res_hlm(c, 0b0000_0100),
        0x97 => res_r8(c, 0b0000_0100, Reg::A),
        0x98 => res_r8(c, 0b0000_1000, Reg::B),
        0x99 => res_r8(c, 0b0000_1000, Reg::C),
        0x9A => res_r8(c, 0b0000_1000, Reg::D),
        0x9B => res_r8(c, 0b0000_1000, Reg::E),
        0x9C => res_r8(c, 0b0000_1000, Reg::H),
        0x9D => res_r8(c, 0b0000_1000, Reg::L),
        0x9E => res_hlm(c, 0b0000_1000),
        0x9F => res_r8(c, 0b0000_1000, Reg::A),
        0xA0 => res_r8(c, 0b0001_0000, Reg::B),
        0xA1 => res_r8(c, 0b0001_0000, Reg::C),
        0xA2 => res_r8(c, 0b0001_0000, Reg::D),
        0xA3 => res_r8(c, 0b0001_0000, Reg::E),
        0xA4 => res_r8(c, 0b0001_0000, Reg::H),
        0xA5 => res_r8(c, 0b0001_0000, Reg::L),
        0xA6 => res_hlm(c, 0b0001_0000),
        0xA7 => res_r8(c, 0b0001_0000, Reg::A),
        0xA8 => res_r8(c, 0b0010_0000, Reg::B),
        0xA9 => res_r8(c, 0b0010_0000, Reg::C),
        0xAA => res_r8(c, 0b0010_0000, Reg::D),
        0xAB => res_r8(c, 0b0010_0000, Reg::E),
        0xAC => res_r8(c, 0b0010_0000, Reg::H),
        0xAD => res_r8(c, 0b0010_0000, Reg::L),
        0xAE => res_hlm(c, 0b0010_0000),
        0xAF => res_r8(c, 0b0010_0000, Reg::A),
        0xB0 => res_r8(c, 0b0100_0000, Reg::B),
        0xB1 => res_r8(c, 0b0100_0000, Reg::C),
        0xB2 => res_r8(c, 0b0100_0000, Reg::D),
        0xB3 => res_r8(c, 0b0100_0000, Reg::E),
        0xB4 => res_r8(c, 0b0100_0000, Reg::H),
        0xB5 => res_r8(c, 0b0100_0000, Reg::L),
        0xB6 => res_hlm(c, 0b0100_0000),
        0xB7 => res_r8(c, 0b0100_0000, Reg::A),
        0xB8 => res_r8(c, 0b1000_0000, Reg::B),
        0xB9 => res_r8(c, 0b1000_0000, Reg::C),
        0xBA => res_r8(c, 0b1000_0000, Reg::D),
        0xBB => res_r8(c, 0b1000_0000, Reg::E),
        0xBC => res_r8(c, 0b1000_0000, Reg::H),
        0xBD => res_r8(c, 0b1000_0000, Reg::L),
        0xBE => res_hlm(c, 0b1000_0000),
        0xBF => res_r8(c, 0b1000_0000, Reg::A),
        //set
        0xC0 => set_r8(c, 0b0000_0001, Reg::B),
        0xC1 => set_r8(c, 0b0000_0001, Reg::C),
        0xC2 => set_r8(c, 0b0000_0001, Reg::D),
        0xC3 => set_r8(c, 0b0000_0001, Reg::E),
        0xC4 => set_r8(c, 0b0000_0001, Reg::H),
        0xC5 => set_r8(c, 0b0000_0001, Reg::L),
        0xC6 => set_hlm(c, 0b0000_0001),
        0xC7 => set_r8(c, 0b0000_0001, Reg::A),
        0xC8 => set_r8(c, 0b0000_0010, Reg::B),
        0xC9 => set_r8(c, 0b0000_0010, Reg::C),
        0xCA => set_r8(c, 0b0000_0010, Reg::D),
        0xCB => set_r8(c, 0b0000_0010, Reg::E),
        0xCC => set_r8(c, 0b0000_0010, Reg::H),
        0xCD => set_r8(c, 0b0000_0010, Reg::L),
        0xCE => set_hlm(c, 0b0000_0010),
        0xCF => set_r8(c, 0b0000_0010, Reg::A),
        0xD0 => set_r8(c, 0b0000_0100, Reg::B),
        0xD1 => set_r8(c, 0b0000_0100, Reg::C),
        0xD2 => set_r8(c, 0b0000_0100, Reg::D),
        0xD3 => set_r8(c, 0b0000_0100, Reg::E),
        0xD4 => set_r8(c, 0b0000_0100, Reg::H),
        0xD5 => set_r8(c, 0b0000_0100, Reg::L),
        0xD6 => set_hlm(c, 0b0000_0100),
        0xD7 => set_r8(c, 0b0000_0100, Reg::A),
        0xD8 => set_r8(c, 0b0000_1000, Reg::B),
        0xD9 => set_r8(c, 0b0000_1000, Reg::C),
        0xDA => set_r8(c, 0b0000_1000, Reg::D),
        0xDB => set_r8(c, 0b0000_1000, Reg::E),
        0xDC => set_r8(c, 0b0000_1000, Reg::H),
        0xDD => set_r8(c, 0b0000_1000, Reg::L),
        0xDE => set_hlm(c, 0b0000_1000),
        0xDF => set_r8(c, 0b0000_1000, Reg::A),
        0xE0 => set_r8(c, 0b0001_0000, Reg::B),
        0xE1 => set_r8(c, 0b0001_0000, Reg::C),
        0xE2 => set_r8(c, 0b0001_0000, Reg::D),
        0xE3 => set_r8(c, 0b0001_0000, Reg::E),
        0xE4 => set_r8(c, 0b0001_0000, Reg::H),
        0xE5 => set_r8(c, 0b0001_0000, Reg::L),
        0xE6 => set_hlm(c, 0b0001_0000),
        0xE7 => set_r8(c, 0b0001_0000, Reg::A),
        0xE8 => set_r8(c, 0b0010_0000, Reg::B),
        0xE9 => set_r8(c, 0b0010_0000, Reg::C),
        0xEA => set_r8(c, 0b0010_0000, Reg::D),
        0xEB => set_r8(c, 0b0010_0000, Reg::E),
        0xEC => set_r8(c, 0b0010_0000, Reg::H),
        0xED => set_r8(c, 0b0010_0000, Reg::L),
        0xEE => set_hlm(c, 0b0010_0000),
        0xEF => set_r8(c, 0b0010_0000, Reg::A),
        0xF0 => set_r8(c, 0b0100_0000, Reg::B),
        0xF1 => set_r8(c, 0b0100_0000, Reg::C),
        0xF2 => set_r8(c, 0b0100_0000, Reg::D),
        0xF3 => set_r8(c, 0b0100_0000, Reg::E),
        0xF4 => set_r8(c, 0b0100_0000, Reg::H),
        0xF5 => set_r8(c, 0b0100_0000, Reg::L),
        0xF6 => set_hlm(c, 0b0100_0000),
        0xF7 => set_r8(c, 0b0100_0000, Reg::A),
        0xF8 => set_r8(c, 0b1000_0000, Reg::B),
        0xF9 => set_r8(c, 0b1000_0000, Reg::C),
        0xFA => set_r8(c, 0b1000_0000, Reg::D),
        0xFB => set_r8(c, 0b1000_0000, Reg::E),
        0xFC => set_r8(c, 0b1000_0000, Reg::H),
        0xFD => set_r8(c, 0b1000_0000, Reg::L),
        0xFE => set_hlm(c, 0b1000_0000),
        0xFF => set_r8(c, 0b1000_0000, Reg::A),
    }
}
