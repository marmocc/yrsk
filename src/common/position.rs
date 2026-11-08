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
