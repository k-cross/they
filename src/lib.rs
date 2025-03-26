pub mod cartridge;
pub mod cpu;
pub mod display;
pub mod motherboard;
pub mod ram;
pub mod sound;

// timing of hardware components
pub const CPU_HZ: u32 = 4_194_304;
pub const RAM_HZ: u32 = 1_048_576;
pub const PPU_HZ: u32 = 4_194_304;
pub const VRAM_HZ: u32 = 2_097_152;
