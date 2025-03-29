use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Cartridge {
    // memory bank controller
    pub mbc: Vec<u8>,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom_path: &str) -> Cartridge {
        let mut rom: Vec<u8> = Vec::new();
        File::open(rom_path)
            .and_then(|mut f| f.read_to_end(&mut rom))
            .ok();

        Cartridge {
            rom,
            ram: Vec::new(),
            mbc: Vec::new(),
        }
    }
}
