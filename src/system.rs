use crate::{BootParameters, cartridge::Cartridge, cpu::CPU, ppu::Display, sound::Voices};

/// The _system_ controls all the coordination involved between the disparate
/// hardware components.
///
/// Bringup needs to happen in a particular order:
///   1. CPU - Set Registers
///   2. RAM - Set Hardware Adress Registers
///   3. PPU - These are all set by reading from RAM
#[derive(Debug)]
pub struct System {
    pub cpu: CPU,
    pub display: Display,
    pub sound: Voices,
    pub cartridge: Cartridge,
}

impl System {
    pub fn new(boot_params: BootParameters) -> System {
        System {
            cpu: CPU::new(),
            display: Display::new(false),
            sound: Voices::new(),
            cartridge: Cartridge::new(&boot_params.rom_path),
        }
    }
}
