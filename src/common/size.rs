use crate::common::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    #[inline]
    pub const fn new(width: u16, height: u16) -> Self {
        assert!(width != 0 && height != 0);
        Self { width, height }
    }

    #[inline]
    pub const fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    #[inline]
    pub const fn last(&self) -> Position {
        Position::new(self.width - 1, self.height - 1)
    }

    #[inline]
    pub const fn advance(&self, position: Position) -> Position {
        let max_position = self.last();
        if position.x < max_position.x {
            position.add_x(1)
        } else if position.y < max_position.y {
            position.add_y(1).with_x(0)
        } else {
            max_position
        }
    }
}
