use super::{ALUFlag, CPU};
use crate::cpu::instructions::{Reg, read_reg, write_reg};

pub fn operation(_c: &mut CPU, opcode: u8) -> u8 {
    match opcode {
        0x0 => todo!(),
        _ => {
            eprintln!("Prefix Opcode is not implemented: {}", opcode);
            1
        }
    }
}
