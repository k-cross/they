use std::fmt;

/// The LR35902 CPU Registers
///
/// The accumulator register is mostly used for storing results. The flags
/// register contains 4 bits used for ALU operations where the low bits remain
/// free and are represeted as:
///   * Z - bit 7 - math operation resulted in zero
///   * N - bit 6 - math operation is subtraction
///   * H - bit 5 - math operation resulted in half-carry
///   * C - bit 4 - math operation resulted in carry
///
/// Memory addresses are 16-bits so `high` and `low` are both for indirect
/// memory access.
///
/// The rest of B, D, C, and E are all general purpose
#[derive(Debug)]
pub struct Registers {
    // high registers
    pub acc: u8,
    pub b: u8,
    pub d: u8,
    pub high: u8,
    // low registers
    pub flags: u8,
    pub c: u8,
    pub e: u8,
    pub low: u8,

    // stack pointer
    pub sp: u16,
    // program counter
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            acc: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            flags: 0,
            high: 0,
            low: 0,
            sp: 0,
            pc: 0,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "acc:{:2x} flags:{:2x} b:{:2x} c:{:2x} d:{:2x} e:{:2x} \
                high:{:2x} low:{:2x} pc:{:4x} sp:{:4x}",
            self.acc,
            self.b,
            self.c,
            self.d,
            self.e,
            self.flags,
            self.high,
            self.low,
            self.pc,
            self.sp
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ALUFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

pub enum OpCode {
    Add = 0x00,
    Load = 0x01,
}

pub enum Interrupt {
    // LCD has drawn a frame
    VBlank,
    // LCD controller changed
    LCDController,
    // serial transfer completed
    Serial,
    // timer countdown
    Timer,
    // user pressed a button
    HiToLo,
    // break out of a cpu powersaving halt
    Halt,
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    /// Initializes all the values for a new CPU to be used with the Motherboard
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    /// Tick performs 1 cpu operation per cycle given the internal state of the
    /// CPU.
    pub fn tick() {
        todo!();
    }

    fn operation(opcode: u16) -> u8 {
        match opcode {
            0x00 => 1,
            op => {
                println!("warning! opcode: {}, not implemented", op);
                0
            }
        }
    }
}
