const RAM_SIZE: usize = 0x10000;

#[repr(u16)]
pub enum MemoryRegister {
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
    serial_out: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut m = Memory {
            serial_out: Vec::new(),
            ram: [0; RAM_SIZE],
        };
        m.initialize();
        m
    }

    pub fn register_read(&mut self, r: MemoryRegister) -> u8 {
        self.read_byte(r as u16)
    }

    pub fn register_write(&mut self, r: MemoryRegister, v: u8) {
        self.write_byte(r as u16, v);
    }

    // These could change depending on which GB version
    fn initialize(&mut self) {
        self.write_byte(MemoryRegister::NR10 as u16, 0x80);
        self.write_byte(MemoryRegister::NR11 as u16, 0xBF);
        self.write_byte(MemoryRegister::NR12 as u16, 0xF3);
        self.write_byte(MemoryRegister::NR14 as u16, 0xBF);
        self.write_byte(MemoryRegister::NR21 as u16, 0x3F);
        self.write_byte(MemoryRegister::NR24 as u16, 0xBF);
        self.write_byte(MemoryRegister::NR30 as u16, 0x7F);
        self.write_byte(MemoryRegister::NR31 as u16, 0xFF);
        self.write_byte(MemoryRegister::NR32 as u16, 0x9F);
        self.write_byte(MemoryRegister::NR33 as u16, 0xFF);
        self.write_byte(MemoryRegister::NR34 as u16, 0xBF);
        self.write_byte(MemoryRegister::NR41 as u16, 0xFF);
        self.write_byte(MemoryRegister::NR44 as u16, 0xBF);
        self.write_byte(MemoryRegister::NR50 as u16, 0x77);
        self.write_byte(MemoryRegister::NR51 as u16, 0xF3);
        self.write_byte(MemoryRegister::NR52 as u16, 0xF1);
        self.write_byte(MemoryRegister::LCDC as u16, 0x91);
        self.write_byte(MemoryRegister::BGP as u16, 0xFC);
        self.write_byte(MemoryRegister::OBP0 as u16, 0xFF);
        self.write_byte(MemoryRegister::OBP1 as u16, 0xFF);
        self.write_byte(MemoryRegister::JOYP as u16, 0xCF);
        self.write_byte(MemoryRegister::DIV as u16, 0x18);
        self.write_byte(MemoryRegister::TAC as u16, 0xF8);
        self.write_byte(MemoryRegister::IF as u16, 0xE1);
    }

    pub(crate) fn read_byte(&mut self, addr: u16) -> u8 {
        //TODO: match on ranges for memory protection
        self.ram[addr as usize]
    }

    pub(crate) fn read_word(&mut self, addr: u16) -> u16 {
        self.read_byte(addr) as u16 | (self.read_byte(addr + 1) as u16) << 8
    }

    pub(crate) fn write_byte(&mut self, addr: u16, val: u8) {
        //TODO: match on ranges for memory protection
        match addr {
            0xFF01 => self.serial_out.push(val),
            _ => (),
        }
        self.ram[addr as usize] = val;
    }

    pub(crate) fn write_word(&mut self, addr: u16, val: u16) {
        //lower nibble
        let v1: u8 = val as u8;
        //upper nibble
        let v2: u8 = (val >> 8) as u8;
        self.write_byte(addr, v1);
        self.write_byte(addr + 1, v2);
    }

    pub(crate) fn print_serial(&mut self) {
        println!("{}", String::from_utf8_lossy(&self.serial_out));
    }
}
