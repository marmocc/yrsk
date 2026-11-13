use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    #[inline]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    #[inline]
    pub const fn flatten(self, stride: u16) -> u32 {
        self.y as u32 * stride as u32 + self.x as u32
    }

    #[inline]
    pub const fn with_x(self, x: u16) -> Self {
        Self { x, y: self.y }
    }

    #[inline]
    pub const fn with_y(self, y: u16) -> Self {
        Self { x: self.x, y }
    }

    #[inline]
    pub const fn add_x(self, x: u16) -> Self {
        Self {
            x: x + self.x,
            y: self.y,
        }
    }

    #[inline]
    pub const fn add_y(self, y: u16) -> Self {
        Self {
            x: self.x,
            y: y + self.y,
        }
    }
}

impl Add for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<(u16, u16)> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: (u16, u16)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}
