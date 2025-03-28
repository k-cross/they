use crate::cpu::CPU;
use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
    pub pixels: [[Pixel; 8]; 8],
}

impl Tile {
    pub fn new(color: Pixel) -> Tile {
        Tile {
            pixels: [[color; 8]; 8],
        }
    }
}

/// The display encodes how all the graphics hardware is tied together. The
/// viewport is the actual visible screen while the display itself captures
/// pixels being rendered off screen too. Only 10 sprites can be used at a time
/// but 40 can be represented in memory. Tiles can overlap each other, as the
/// full screen is only 144 x 160 pixels.
#[derive(Debug)]
pub struct Display {
    pub tiles: [Tile; 384],
    // the (x, y) position of what can be shown
    pub view_port: (u8, u8),
    // aka OAM (Object Attribut Memory)
    pub sprites: [Sprite; 40],
    pub tile_map: [[u8; 512]; 2],
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyx: u8,
    pub wy: u8,
    pub wx: u8,
    pub stat: u8,
}

impl Display {
    /// The sprite_size argument defaults to false or small
    pub fn new(sprite_size: bool) -> Display {
        Display {
            tiles: [Tile::new(Pixel::Black); 384],
            tile_map: [[0u8; 512]; 2],
            view_port: (0, 0),
            sprites: [if sprite_size {
                Sprite::Big([[0; 8]; 16])
            } else {
                Sprite::Little([[0; 8]; 8])
            }; 40],
            scy: 0,
            scx: 0,
            ly: 0,
            lyx: 0,
            wy: 0,
            wx: 0,
            stat: 0,
        }
    }

    pub fn set_lcdc(&mut self, cpu: &mut CPU, flags: Vec<LCDC>, cond: bool) -> u8 {
        if cond {
            let v = cpu.memory.read_byte(Register::LCDC as u16)
                | flags.into_iter().fold(0u8, |acc, f| (acc | f as u8));
            cpu.memory.write_byte(Register::LCDC as u16, v);
            v
        } else {
            let v = cpu.memory.read_byte(Register::LCDC as u16)
                & !flags.into_iter().fold(0u8, |acc, f| (acc | f as u8));
            cpu.memory.write_byte(Register::LCDC as u16, v);
            v
        }
    }

    pub fn check_lcdc(&mut self, cpu: &mut CPU, flags: Vec<LCDC>) -> bool {
        let f = flags.into_iter().fold(0u8, |acc, f| (acc | f as u8));
        cpu.memory.read_byte(Register::LCDC as u16) & f == f
    }

    /// Blocks can be sequentially loaded in memory, there indexing pattern will be the same across the two different addressing schemes.
    /// * Block 0: 0-127 (LCDC Bit 4: 1)
    /// * Block 1: 128-255
    /// * Block 2: 0-127 (LCDC Bit 4: 0)
    pub fn load_tiles(&mut self, cpu: &mut CPU, block: u8) {
        match block {
            0 => self.load_tile_range(cpu, 0x8000..0x8800),
            1 => self.load_tile_range(cpu, 0x8800..0x9000),
            2 => self.load_tile_range(cpu, 0x9000..0x9800),
            _ => self.load_tile_range(cpu, 0x8000..0x9800),
        }
    }

    fn load_tile_range(&mut self, cpu: &mut CPU, range: Range<u16>) {
        for (i, addr) in range.step_by(16).enumerate() {
            let mut tile = self.tiles[i];
            for j in 0..8 {
                let (mut lb, mut hb) = (
                    cpu.memory.read_byte(addr + (j * 2) as u16),
                    cpu.memory.read_byte(addr + 1 + (j * 2) as u16),
                );
                let mut pixel_row = [Pixel::Black; 8];
                for k in 0..8 {
                    pixel_row[k] = match (hb & 0x80, lb & 0x80) {
                        (0x80, 0x80) => Pixel::Black,
                        (0x0, 0x0) => Pixel::White,
                        (0x0, 0x80) => Pixel::Grey,
                        (0x80, 0x0) => Pixel::DarkGrey,
                        _ => Pixel::Black,
                    };
                    hb <<= 1;
                    lb <<= 1;
                }
                tile.pixels[j] = pixel_row;
            }
            self.tiles[i] = tile;
        }
    }

    /// There are two maps which contain how each tile should be placed
    /// $9800-$9BFF and $9C00-$9FFF and which one is used is guided by register
    /// values.
    pub fn load_tile_map(&mut self, cpu: &mut CPU) {
        for (i, addr) in (0x9800..0x9C00).enumerate() {
            self.tile_map[0][i] = cpu.memory.ram[addr];
            self.tile_map[1][i] = cpu.memory.ram[addr + 1024];
        }
    }

    /// Return the index of the desired tile.
    pub fn get_tile(&mut self) -> usize {
        0
    }
}

pub struct ObjectAttributeMap {
    pub x: u8,
    pub y: u8,
    pub tile_idx: u8,
    pub priority: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub dmg_palette: bool,
    // bank is for CGB
    pub bank: bool,
    // palette is for CGB
    pub cgb_palette: u8,
}

impl ObjectAttributeMap {
    /// Byte 0: y position
    /// Byte 1: x position
    /// Byte 2: tile index
    /// Byte 3: flags
    pub fn new(v: u32) -> ObjectAttributeMap {
        // Verify if this is the right byte order
        let (b0, b1, b2, b3) = (v as u8, (v >> 8) as u8, (v >> 16) as u8, (v >> 24) as u8);
        ObjectAttributeMap {
            y: b0,
            x: b1,
            tile_idx: b2,
            priority: 0b1000_0000 & b3 != 0,
            flip_y: 0b0100_0000 & b3 != 0,
            flip_x: 0b0010_0000 & b3 != 0,
            // use OBP0 (false) or OBP1 (true)
            dmg_palette: 0b0001_0000 & b3 != 0,
            bank: 0b0000_1000 & b3 != 0,
            cgb_palette: 0b0000_0111 & b3,
        }
    }
}

/// There are two types of sprites that can be used, one that is 8 x 8 pixels
/// and one that is 8 x 16 pixels
#[derive(Debug, Copy, Clone)]
pub enum Sprite {
    Big([[u8; 8]; 16]),
    Little([[u8; 8]; 8]),
}

/// Pixels are represented with 2 bits but need to be converted to RGB to
/// display properly on a modern computers. There values can be transcoded as 2
/// bit value codes to RGB:
///   * White = 0x00
///   * Grey = 0x01
///   * DarkGrey = 0x10
///   * Black = 0x11
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pixel {
    Grey = 0x808080,
    DarkGrey = 0x696969,
    Black = 0x000000,
    White = 0xFFFFFF,
}

/// Registers are actually 16 bit memory addresses to 8 bit storage, this enum
/// only encodes their memory location
#[derive(Debug)]
pub enum Register {
    LCDC = 0xFF40,
    STAT = 0xFF41,
    SCY = 0xFF42,
    SCX = 0xFF43,
    LY = 0xFF44,
    LYC = 0xFF45,
    BGP = 0xFF46,
    OBP0 = 0xFF47,
    OBP1 = 0xFF48,
    WY = 0xFF49,
    WX = 0xFF4A,
}

/// LCD & PPU enable: 0 = Off; 1 = On
/// Window tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
/// Window enable: 0 = Off; 1 = On
/// BG & Window tile data area: 0 = 8800–97FF; 1 = 8000–8FFF
/// BG tile map area: 0 = 9800–9BFF; 1 = 9C00–9FFF
/// OBJ size: 0 = 8×8; 1 = 8×16
/// OBJ enable: 0 = Off; 1 = On
/// BG & Window enable / priority [Different meaning in CGB Mode]: 0 = Off; 1 = On
#[derive(Debug, Clone, Copy)]
pub enum LCDC {
    PPUEnabled = 0b1000_0000,
    WindowTileMapArea = 0b0100_0000,
    WindowEnabled = 0b0010_0000,
    WindowDataArea = 0b0001_0000,
    BgTileMapArea = 0b0000_1000,
    ObjSize = 0b0000_0100,
    ObjEnabled = 0b0000_0010,
    BgWindowPriority = 0b0000_0001,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (CPU, Display) {
        (CPU::new(), Display::new(false))
    }

    #[test]
    fn load_pixel_test() {
        let (mut cpu, mut disp) = setup();
        disp.load_tiles(&mut cpu, 0);
        let white_tile = Tile::new(Pixel::White);
        for tile in &disp.tiles[0..128] {
            assert_eq!(tile, &white_tile);
        }
    }

    #[test]
    fn set_lcdc_test() {
        let (mut cpu, mut disp) = setup();
        let flags = vec![
            LCDC::WindowTileMapArea,
            LCDC::WindowEnabled,
            LCDC::WindowDataArea,
            LCDC::PPUEnabled,
            LCDC::ObjSize,
            LCDC::BgTileMapArea,
            LCDC::BgWindowPriority,
            LCDC::ObjEnabled,
        ];
        disp.set_lcdc(&mut cpu, flags.clone(), true);
        assert!(disp.check_lcdc(&mut cpu, flags.clone()));
        assert_eq!(cpu.memory.read_byte(Register::LCDC as u16), 0xFF);
        disp.set_lcdc(&mut cpu, flags, false);
        assert_eq!(cpu.memory.read_byte(Register::LCDC as u16), 0);
    }

    #[test]
    fn oam_test() {
        let oam = ObjectAttributeMap::new(0xFFFFFFFF);
        assert_eq!(oam.cgb_palette, 7);
        assert_eq!(oam.x, 0xFF);
        assert_eq!(oam.y, 0xFF);
        assert_eq!(oam.tile_idx, 0xFF);
        assert!(oam.bank);
        assert!(oam.flip_x);
        assert!(oam.flip_y);
        assert!(oam.priority);
        assert!(oam.dmg_palette);
    }
}
