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

    /// Starts the instruction loop.
    pub fn run(&mut self) {
        loop {
            //println!("{}", self.cpu.registers);
            self.cpu.exec();
            let c = self.cpu.memory.read_byte(0xFF02);
            print!(" -- Char: {}:{} -- ", c, c as char);
        }
    }

    /// Initialize the system like loading the game's cartridge rom into system memory.
    pub fn initialize(&mut self) {
        self.load_rom_bank(0, 1 << 15);
    }

    fn load_rom_bank(&mut self, low: u16, high: u16) {
        for (mem_idx, rom_idx) in (low..high).enumerate() {
            self.cpu.memory.ram[mem_idx] = self.cartridge.rom[rom_idx as usize];
        }
    }
}
