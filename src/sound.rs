/// Sound contains 4 distinct voices with
pub struct Voices {
    pulse_a: (
        Register::Control,
        Register::Frequency,
        Register::Volume,
        Register::Length,
        Register::Sweep,
    ),
    pulse_b: (
        Register::Control,
        Register::Frequency,
        Register::Volume,
        Register::Length,
        Register::Sweep,
    ),
    wave: (
        Register::Control,
        Register::Frequency,
        Register::Volume,
        Register::Length,
        Register::Sweep,
    ),
    noise: (
        Register::Control,
        Register::Frequency,
        Register::Volume,
        Register::Length,
        Register::Sweep,
    ),
}

pub enum Register {
    Control(u8),
    Frequency(u8),
    Volume(u8),
    Length(u8),
    Sweep(u8),
}
