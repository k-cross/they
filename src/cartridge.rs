use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Cartridge {
    // memory bank controller
    pub mbc: Vec<u8>,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub title: String,
}

impl Cartridge {
    pub fn new(rom_path: &PathBuf) -> Cartridge {
        let mut rom: Vec<u8> = Vec::new();
        File::open(rom_path)
            .and_then(|mut f| f.read_to_end(&mut rom))
            .expect("GB Rom file did not open");
        let title: String = std::str::from_utf8(&rom[0x134..0x13E]).unwrap().to_owned();

        Cartridge {
            rom,
            title,
            ram: Vec::new(),
            mbc: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BootParameters;

    use super::*;

    fn setup() -> Cartridge {
        let bp = BootParameters::new(None);
        Cartridge::new(&bp.rom_path)
    }

    #[test]
    fn test_initialization() {
        let c = setup();
        assert_eq!(c.title.as_str(), "CPU_INSTRS");
    }
}
