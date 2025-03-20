use super::CPU;

// load functions
fn ld_bc_n16(c: &mut CPU) {
    let value = c.get_word();
    c.registers.b = (value >> 8) as u8;
    c.registers.c = (value & 0x00FF) as u8;
}

pub(crate) fn operations(c: &mut CPU, opcode: u8) {
    match opcode {
        0x0 => (),
        0x1 => ld_bc_n16(c),
        _ => eprintln!("OpCode is not implemented: {}", opcode),
    }
}
