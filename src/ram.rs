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

#[repr(u16)]
pub enum MemoryRegisters {
    JOYP = 0xFF00,
    SB = 0xFF01,
    SC = 0xFF02,
    DIV = 0xFF04,
    TIMA = 0xFF05,
    TMA = 0xFF06,
    TAC = 0xFF07,
    IF = 0xFF0F,
    NR10 = 0xFF10,
    NR11 = 0xFF11,
    NR12 = 0xFF12,
    NR13 = 0xFF13,
    NR14 = 0xFF14,
    NR21 = 0xFF16,
    NR22 = 0xFF17,
    NR23 = 0xFF18,
    NR24 = 0xFF19,
    NR30 = 0xFF1A,
    NR31 = 0xFF1B,
    NR32 = 0xFF1C,
    NR33 = 0xFF1D,
    NR34 = 0xFF1E,
    NR41 = 0xFF20,
    NR42 = 0xFF21,
    NR43 = 0xFF22,
    NR44 = 0xFF23,
    NR50 = 0xFF24,
    NR51 = 0xFF25,
    NR52 = 0xFF26,
    WaveRAMStart = 0xFF30,
    LCDC = 0xFF40,
    STAT = 0xFF41,
    SCY = 0xFF42,
    SCX = 0xFF43,
    LY = 0xFF44,
    LYC = 0xFF45,
    DMA = 0xFF46,
    BGP = 0xFF47,
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    WY = 0xFF4A,
    WX = 0xFF4B,
    KEY1 = 0xFF4D,
    VBK = 0xFF4F,
    HDMA1 = 0xFF51,
    HDMA2 = 0xFF52,
    HDMA3 = 0xFF53,
    HDMA4 = 0xFF54,
    HDMA5 = 0xFF55,
    RP = 0xFF56,
    BCPS = 0xFF68,
    BCPD = 0xFF69,
    OCPS = 0xFF6A,
    OCPD = 0xFF6B,
    OPRI = 0xFF6C,
    SVBK = 0xFF70,
    PCM12 = 0xFF76,
    PCM34 = 0xFF77,
    IE = 0xFFFF,
}

#[derive(Debug)]
pub struct Memory {
    pub ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory { ram: [0; RAM_SIZE] };
        m.initialize();
        m
    }

    fn initialize(&mut self) {
        self.write_byte(MemoryRegisters::NR10 as u16, 0x80);
        self.write_byte(MemoryRegisters::NR11 as u16, 0xBF);
        self.write_byte(MemoryRegisters::NR12 as u16, 0xF3);
        self.write_byte(MemoryRegisters::NR14 as u16, 0xBF);
        self.write_byte(MemoryRegisters::NR21 as u16, 0x3F);
        self.write_byte(MemoryRegisters::NR24 as u16, 0xBF);
        self.write_byte(MemoryRegisters::NR30 as u16, 0x7F);
        self.write_byte(MemoryRegisters::NR31 as u16, 0xFF);
        self.write_byte(MemoryRegisters::NR32 as u16, 0x9F);
        self.write_byte(MemoryRegisters::NR34 as u16, 0xFF);
        self.write_byte(MemoryRegisters::NR41 as u16, 0xFF);
        self.write_byte(MemoryRegisters::NR44 as u16, 0xBF);
        self.write_byte(MemoryRegisters::NR50 as u16, 0x77);
        self.write_byte(MemoryRegisters::NR51 as u16, 0xF3);
        self.write_byte(MemoryRegisters::NR52 as u16, 0xF1);
        self.write_byte(MemoryRegisters::LCDC as u16, 0x91);
        self.write_byte(MemoryRegisters::BGP as u16, 0xFC);
        self.write_byte(MemoryRegisters::OBP0 as u16, 0xFF);
        self.write_byte(MemoryRegisters::OBP1 as u16, 0xFF);
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
