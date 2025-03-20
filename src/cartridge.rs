pub struct Cartridge {
    // memory bank controller
    pub mbc: Vec<u16>,
    pub rom: Vec<u16>,
    pub ram: Vec<u16>,
}
