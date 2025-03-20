/// The joypad is represented in the memory address 0xFF00 and contains an interesting byte pattern:
///   | 7 | 6 | 5        | 4         | 3          | 2         | 1      | 0       |
///   | 1 | 1 | standard | direction | down/start | up/select | left/B | right/A |
///
/// For example `start` is represented as:
///   | 1 | 1 | 0 | 1 | 0 | 1 | 1 | 1 |
pub enum Joypad {
    Down = 0b11100111,
    Up = 0b11101011,
    Left = 0b11101101,
    Right = 0b11101110,
    Start = 0b11010111,
    Select = 0b11011011,
    ButtonB = 0b11011101,
    ButtonA = 0b11011110,
}
