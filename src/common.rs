#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Digit3 {
    pub hundreds: u8,
    pub tens: u8,
    pub ones: u8,
}

impl Digit3 {
    pub fn new(value: u8) -> Self {
        Self {
            hundreds: b'0' + value / 100,
            tens: b'0' + value / 10 % 10,
            ones: b'0' + value % 10,
        }
    }

    pub fn as_bytes(self) -> [u8; 3] {
        [self.hundreds, self.tens, self.ones]
    }
}
