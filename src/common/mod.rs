use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<i32> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Position::new(self.x + rhs, self.y + rhs)
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Digit3 {
    pub hundreds: u8,
    pub tens: u8,
    pub ones: u8,
}

impl Digit3 {
    #[inline]
    pub fn new(value: u8) -> Self {
        Self {
            hundreds: b'0' + value / 100,
            tens: b'0' + value / 10 % 10,
            ones: b'0' + value % 10,
        }
    }

    #[inline]
    pub fn as_bytes(self) -> [u8; 3] {
        [self.hundreds, self.tens, self.ones]
    }
}
