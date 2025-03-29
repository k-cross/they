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
    pub rom_path: String,
    // false 8x8, true 8x16
    pub sprite_size: bool,
}

impl BootParameters {
    pub fn new() -> BootParameters {
        BootParameters {
            rom_path: "./rom_tests/blarggs-test-roms/cpu_instrs/cpu_instrs.gb".to_owned(),
            sprite_size: false,
        }
    }
}
