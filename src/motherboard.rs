use crate::cpu::CPU;

/// The Motherboard controls all the coordination involved between the disparate
/// hardware components
#[derive(Debug)]
pub struct Motherboard {
    pub cpu: CPU,
}

impl Motherboard {
    pub fn new() -> Motherboard {
        Motherboard { cpu: CPU::new() }
    }
}
