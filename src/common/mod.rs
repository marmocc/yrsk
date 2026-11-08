use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<i32> for Position {
    type Output = Position;
    fn add(self, rhs: i32) -> Self::Output {
        Position::new(self.x + rhs, self.y + rhs)
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
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
