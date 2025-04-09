use std::path::{Path, PathBuf};

pub mod cartridge;
pub mod cpu;
pub mod interface;
pub mod ppu;
pub mod ram;
pub mod sound;
pub mod system;

// timing of hardware components
pub const CPU_HZ: u32 = 4_194_304;
pub const RAM_HZ: u32 = 1_048_576;
pub const PPU_HZ: u32 = 4_194_304;
pub const VRAM_HZ: u32 = 2_097_152;

pub struct BootParameters {
    pub rom_path: PathBuf,
    // false 8x8, true 8x16
    pub sprite_size: bool,
}

impl BootParameters {
    pub fn new(p: Option<&str>) -> BootParameters {
        let p = match p {
            Some(v) => Path::new(v).to_path_buf(),
            None => Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("rom_tests/blarggs-test-roms/cpu_instrs/individual/01-special.gb"),
            //.join("rom_tests/blarggs-test-roms/cpu_instrs/individual/06-ld r,r.gb"),
            //.join("rom_tests/blarggs-test-roms/cpu_instrs/individual/04-op r,imm.gb"),
            //.join("rom_tests/blarggs-test-roms/cpu_instrs/individual/10-bit ops.gb"),
        };
        BootParameters {
            rom_path: p,
            sprite_size: false,
        }
    }
}
