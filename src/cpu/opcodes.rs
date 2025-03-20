use super::{ALUFlag, CPU};

// load functions
fn ld_bc_n16(c: &mut CPU) {
    let val = c.get_word_instr();
    c.registers.b = (val >> 8) as u8;
    c.registers.c = val as u8;
}

fn ld_bc_a(c: &mut CPU) {
    let addr = ((c.registers.b as u16) << 8) | c.registers.c as u16;
    c.memory.write_byte(addr, c.registers.acc);
}

fn inc_bc(c: &mut CPU) {
    // no flags
    let val = (((c.registers.b as u16) << 8) | c.registers.c as u16).wrapping_add(1);
    c.registers.b = (val >> 8) as u8;
    c.registers.c = val as u8;
}

pub(crate) fn operations(c: &mut CPU, opcode: u8) {
    match opcode {
        0x0 => {
            // cycles 1
            ()
        }
        0x1 => {
            ld_bc_n16(c);
            // cycles 3
        }
        0x2 => {
            ld_bc_a(c);
            // cycles 2
        }
        0x3 => {
            inc_bc(c);
            // cycles 2
        }
        _ => eprintln!("OpCode is not implemented: {}", opcode),
    }
}
