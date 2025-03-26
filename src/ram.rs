use std::ops::Range;
const RAM_SIZE: usize = 0xFFFF;
const INTERRUPT_ADDR: u16 = 0xFFFF;
const INTERNAL2_RAM_ADDR: u16 = 0xFE80;
const IO_PORTS_ADDR: u16 = 0xFF00;
const SPRITES_ADDR: u16 = 0xFE00;
const ECHO_RAM_ADDR: u16 = 0xE000;
const INTERNAL_RAM_ADDR: u16 = 0xC000;
const SWITCHABLE_RAM_ADDR: u16 = 0xA000;
const SWITCHABLE_ROM_ADDR: u16 = 0x4000;
const ROM_ADDR: u16 = 0x0000;
// ranges
const TOTAL_VRAM_ADDR: Range<u16> = 0x8000..0x97FF;

#[derive(Debug)]
pub struct Memory {
    pub ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; RAM_SIZE] }
    }

    pub fn initialize(&mut self) {
        self.write_byte(0xFF10, 0x80);
        self.write_byte(0xFF11, 0xBF);
        self.write_byte(0xFF12, 0xF3);
        self.write_byte(0xFF14, 0xBF);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF19, 0xBF);
        self.write_byte(0xFF1A, 0x7F);
        self.write_byte(0xFF1B, 0xFF);
        self.write_byte(0xFF1C, 0x9F);
        self.write_byte(0xFF1E, 0xFF);
        self.write_byte(0xFF20, 0xFF);
        self.write_byte(0xFF23, 0xBF);
        self.write_byte(0xFF24, 0x77);
        self.write_byte(0xFF25, 0xF3);
        self.write_byte(0xFF26, 0xF1);
        self.write_byte(0xFF40, 0x91);
        self.write_byte(0xFF47, 0xFC);
        self.write_byte(0xFF48, 0xFF);
        self.write_byte(0xFF49, 0xFF);
    }

    pub(crate) fn read_byte(&mut self, addr: u16) -> u8 {
        //TODO: match on ranges for memory protection
        self.ram[addr as usize]
    }

    pub(crate) fn read_word(&mut self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) << 8 | self.read_byte(addr + 1) as u16
    }

    pub(crate) fn write_byte(&mut self, addr: u16, val: u8) {
        //TODO: match on ranges for memory protection
        self.ram[addr as usize] = val;
    }

    pub(crate) fn write_word(&mut self, addr: u16, val: u16) {
        let v1: u8 = (val >> 8) as u8;
        let v2: u8 = val as u8;
        self.write_byte(addr, v1);
        self.write_byte(addr + 1, v2);
    }
}
