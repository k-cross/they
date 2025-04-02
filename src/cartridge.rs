use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// The Cartridge's ROM gets stored in memory banks of 16Kb sizes each. The
/// system contains two 16Kb banks so any game of 32Kb or smaller doesn't rely
/// on banking. RAM gets banked in 8Kb chuncks and the system has a RAM size of
/// 8Kb onboard already.
#[derive(Debug, Clone)]
pub struct Cartridge {
    // memory bank controller
    pub mbc: Vec<u8>,
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub title: String,
    pub model: Model,
    pub rom_size: u32,
    pub ram_size: Option<u32>,
}

impl Cartridge {
    pub fn new(rom_path: &PathBuf) -> Cartridge {
        let mut rom: Vec<u8> = Vec::new();
        File::open(rom_path)
            .and_then(|mut f| f.read_to_end(&mut rom))
            .expect("GB Rom file did not open");
        let title: String = std::str::from_utf8(&rom[0x134..0x13E]).unwrap().to_owned();
        let model: Model = rom[0x147].into();
        let rom_size: u32 = 1 << (15 + rom[0x148]);
        let ram_size: Option<u32> = match rom[0x149] {
            0 => Some(0),
            1 => None,
            2 => Some(1 << 13),
            3 => Some(1 << 15),
            4 => Some(1 << 17),
            5 => Some(1 << 16),
            _ => None,
        };

        Cartridge {
            rom,
            title,
            model,
            rom_size,
            ram_size,
            ram: Vec::new(),
            mbc: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Model {
    ROM = 0x00,
    MBC1 = 0x01,
    MBC1RAM = 0x02,
    MBC1RAMBATTERY = 0x03,
    MBC2 = 0x05,
    MBC2BATTERY = 0x06,
    ROMRAM = 0x08,
    ROMRAMBATTERY = 0x09,
    MMM01 = 0x0B,
    MMM01RAM = 0x0C,
    MMM01RAMBATTERY = 0x0D,
    MBC3TIMERBATTERY = 0x0F,
    MBC3TIMERRAMBATTERY = 0x10,
    MBC3 = 0x11,
    MBC3RAM = 0x12,
    MBC3RAMBATTERY = 0x13,
    MBC5 = 0x19,
    MBC5RAM = 0x1A,
    MBC5RAMBATTERY = 0x1B,
    MBC5RUMBLE = 0x1C,
    MBC5RUMBLERAM = 0x1D,
    MBC5RUMBLERAMBATTERY = 0x1E,
    MBC6 = 0x20,
    MBC7SENSORRUMBLERAMBATTERY = 0x22,
    POCKET = 0xFC,
    BANDAI = 0xFD,
    HuC3 = 0xFE,
    HuC1RAMBATTERY = 0xFF,
    // Not Applicable aka no known mapping
    NA = 0xAA,
}

impl From<u8> for Model {
    fn from(n: u8) -> Self {
        match n {
            0x00 => Model::ROM,
            0x01 => Model::MBC1,
            0x02 => Model::MBC1RAM,
            0x03 => Model::MBC1RAMBATTERY,
            0x05 => Model::MBC2,
            0x06 => Model::MBC2BATTERY,
            0x08 => Model::ROMRAM,
            0x09 => Model::ROMRAMBATTERY,
            0x0B => Model::MMM01,
            0x0C => Model::MMM01RAM,
            0x0D => Model::MMM01RAMBATTERY,
            0x0F => Model::MBC3TIMERBATTERY,
            0x10 => Model::MBC3TIMERRAMBATTERY,
            0x11 => Model::MBC3,
            0x12 => Model::MBC3RAM,
            0x13 => Model::MBC3RAMBATTERY,
            0x19 => Model::MBC5,
            0x1A => Model::MBC5RAM,
            0x1B => Model::MBC5RAMBATTERY,
            0x1C => Model::MBC5RUMBLE,
            0x1D => Model::MBC5RUMBLERAM,
            0x1E => Model::MBC5RUMBLERAMBATTERY,
            0x20 => Model::MBC6,
            0x22 => Model::MBC7SENSORRUMBLERAMBATTERY,
            0xFC => Model::POCKET,
            0xFD => Model::BANDAI,
            0xFE => Model::HuC3,
            0xFF => Model::HuC1RAMBATTERY,
            _ => Model::NA,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BootParameters;

    use super::*;

    fn setup() -> Cartridge {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("rom_tests/blarggs-test-roms/cpu_instrs/cpu_instrs.gb");
        let bp = BootParameters::new(p.to_str());
        Cartridge::new(&bp.rom_path)
    }

    #[test]
    fn test_initialization() {
        let c = setup();
        assert_eq!(c.title.as_str(), "CPU_INSTRS");
        assert_eq!(c.ram_size, Some(0));
        assert_eq!(c.rom_size, 1 << 16);
    }
}
