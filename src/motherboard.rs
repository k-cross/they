use crate::{cpu::CPU, ppu::Display, sound::Voices};

/// The Motherboard controls all the coordination involved between the disparate
/// hardware components.
///
/// Bringup needs to happen in a particular order:
///   1. CPU - Set Registers
///   2. RAM - Set Hardware Adress Registers
///   3. PPU - These are all set by reading from RAM
#[derive(Debug)]
pub struct Motherboard {
    pub cpu: CPU,
    pub display: Display,
    pub sound: Voices,
}

impl Motherboard {
    pub fn new() -> Motherboard {
        Motherboard {
            cpu: CPU::new(),
            display: Display::new(false),
            sound: Voices::new(),
        }
    }
}
