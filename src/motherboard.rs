use crate::{cpu::CPU, display::Display, sound::Voices};

/// The Motherboard controls all the coordination involved between the disparate
/// hardware components
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
