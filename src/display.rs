#[derive(Debug)]
pub struct Tile {
    pixels: [[u8; 8]; 8],
}

/// The display encodes how all the graphics hardware is tied together. The
/// viewport is the actual visible screen while the display itself captures
/// pixels being rendered off screen too. Only 10 sprites can be used at a time
/// but 40 can be represented in memory. Tiles can overlap each other, as the
/// full screen is only 144 x 160 pixels.
#[derive(Debug)]
pub struct Display {
    tiles: [Tile; 384],
    // the (x, y) position of what can be shown
    view_port: (u8, u8),
    // aka OAM (Object Attribut Memory)
    sprites: [Sprite; 40],
}

pub struct ObjectAttributeMap {
    x: u8,
    y: u8,
    tile_number: u8,
    priority: u8,
    flip_x: bool,
    flip_y: bool,
    palette: bool,
}

/// There are two types of sprites that can be used, one that is 8 x 8 pixels
/// and one that is 8 x 16 pixels
#[derive(Debug)]
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
pub enum Register {
    LCD = 0xFF40,
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
