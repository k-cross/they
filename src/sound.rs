/// Sound contains 4 distinct voices with
#[derive(Debug)]
pub struct Voices {
    pub pulse_a: (Register, Register, Register, Register, Register),
    pub pulse_b: (Register, Register, Register, Register, Register),
    pub wave: (Register, Register, Register, Register, Register),
    pub noise: (Register, Register, Register, Register, Register),
}

impl Voices {
    pub fn new() -> Voices {
        Voices {
            pulse_a: (
                Register::Control(0),
                Register::Frequency(0),
                Register::Volume(0),
                Register::Length(0),
                Register::Sweep(0),
            ),
            pulse_b: (
                Register::Control(0),
                Register::Frequency(0),
                Register::Volume(0),
                Register::Length(0),
                Register::Sweep(0),
            ),
            wave: (
                Register::Control(0),
                Register::Frequency(0),
                Register::Volume(0),
                Register::Length(0),
                Register::Sweep(0),
            ),
            noise: (
                Register::Control(0),
                Register::Frequency(0),
                Register::Volume(0),
                Register::Length(0),
                Register::Sweep(0),
            ),
        }
    }
}

#[derive(Debug)]
pub enum Register {
    Control(u8),
    Frequency(u8),
    Volume(u8),
    Length(u8),
    Sweep(u8),
}
