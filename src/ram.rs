const RAM_SIZE: usize = 0xFFFF;
const INTERRUPT_ADDR: u16 = 0xFFFF;
const INTERNAL2_RAM_ADDR: u16 = 0xFE80;
const IO_PORTS_ADDR: u16 = 0xFF00;
const SPRITES_ADDR: u16 = 0xFE00;
const ECHO_RAM_ADDR: u16 = 0xE000;
const INTERNAL_RAM_ADDR: u16 = 0xC000;
const SWITCHABLE_RAM_ADDR: u16 = 0xA000;
const VIDEO_RAM_ADDR: u16 = 0x8000;
const SWITCHABLE_ROM_ADDR: u16 = 0x4000;
const ROM_ADDR: u16 = 0x0000;

#[derive(Debug)]
pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { ram: [0; RAM_SIZE] }
    }

    pub(crate) fn read_byte(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub(crate) fn read_word(&mut self, addr: u16) -> u16 {
        self.read_byte(addr) as u16 | (self.read_byte(addr + 1) as u16) << 8
    }
}
