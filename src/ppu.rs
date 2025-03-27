use crate::cpu::CPU;

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
}

impl Display {
    /// The sprite_size argument defaults to false or small
    pub fn new(sprite_size: bool) -> Display {
        Display {
            tiles: [Tile::new(Pixel::Black); 384],
            view_port: (0, 0),
            sprites: [if sprite_size {
                Sprite::Big([[0; 8]; 16])
            } else {
                Sprite::Little([[0; 8]; 8])
            }; 40],
        }
    }

    pub fn set_lcdc(&mut self, lcdc_val: u8, flags: Vec<LCDC>) -> u8 {
        lcdc_val | flags.into_iter().fold(0u8, |acc, f| (acc | f as u8))
    }

    pub fn check_lcdc(&mut self, lcdc_val: &u8, flags: Vec<LCDC>) -> bool {
        let f = flags.into_iter().fold(0u8, |acc, f| (acc | f as u8));
        lcdc_val & f == f
    }

    pub fn load_tiles(&mut self, cpu: &mut CPU, block: u8) {
        for (i, addr) in (0x8000..0x8800).step_by(16).enumerate() {
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

        //Load Blocks 1 and 2
        //
        //let lcdc = cpu.memory.read_byte(Register::LCDC as u16);
        //let offset = 0x400;
        //if self.check_lcdc(&lcdc, vec![LCDC::WindowDataArea]) {
        //    for (i, addr) in (0x8800..0x9800).step_by(2).enumerate() {
        //        let pixels = cpu.memory.read_word(addr);
        //    }
        //}
    }
}

pub struct ObjectAttributeMap {
    pub x: u8,
    pub y: u8,
    pub tile_number: u8,
    pub priority: u8,
    pub flip_x: bool,
    pub flip_y: bool,
    pub palette: bool,
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
#[derive(Debug)]
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
}
