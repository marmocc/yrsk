use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    #[inline]
    pub fn flatten(self, stride: i32) -> i32 {
        self.y * stride + self.x
    }

    #[inline]
    pub fn with_x(self, x: i32) -> Self {
        Self { x, y: self.y }
    }

    #[inline]
    pub fn with_y(self, y: i32) -> Self {
        Self { x: self.x, y }
    }

    #[inline]
    pub fn add_x(self, x: i32) -> Self {
        Self {
            x: x + self.x,
            y: self.y,
        }
    }

    #[inline]
    pub fn add_y(self, y: i32) -> Self {
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

impl Add<(i32, i32)> for Position {
    type Output = Position;
    #[inline]
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}
