#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub pixels: [[u8; 8]; 8],
}

impl Tile {
    fn new() -> Tile {
        Tile {
            pixels: [[0; 8]; 8],
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
            tiles: [Tile::new(); 384],
            view_port: (0, 0),
            sprites: [if sprite_size {
                Sprite::Big([[0; 8]; 16])
            } else {
                Sprite::Little([[0; 8]; 8])
            }; 40],
        }
    }

    pub fn set_lcdc(lcdc_val: u8, flags: Vec<LCDC>) -> u8 {
        lcdc_val | flags.into_iter().fold(0u8, |acc, f| (acc | f as u8))
    }

    pub fn check_lcdc(lcdc_val: &u8, flags: Vec<LCDC>) -> bool {
        let f = flags.into_iter().fold(0u8, |acc, f| (acc | f as u8));
        lcdc_val & f == f
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
/// display properly on a modern computers.
#[repr(u32)]
pub enum RGB {
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
