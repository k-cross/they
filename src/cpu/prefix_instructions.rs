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
        _ => {
            eprintln!("Prefix Opcode is not implemented: {}", opcode);
            1
        }
    }
}
