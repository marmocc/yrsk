use crate::common::position::Position;
use crate::{Error, Result};
use std::num::NonZeroU16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: NonZeroU16,
    pub height: NonZeroU16,
}

impl Size {
    #[inline]
    pub const fn new(width: NonZeroU16, height: NonZeroU16) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn try_new(width: u16, height: u16) -> Option<Self> {
        match (NonZeroU16::new(width), NonZeroU16::new(height)) {
            (Some(width), Some(height)) => Some(Self { width, height }),
            _ => None,
        }
    }

    #[inline]
    pub const fn width(self) -> u16 {
        self.width.get()
    }

    #[inline]
    pub const fn height(self) -> u16 {
        self.height.get()
    }

    #[inline]
    pub const fn area(self) -> u32 {
        self.width.get() as u32 * self.height.get() as u32
    }

    #[inline]
    pub const fn contains(&self, position: Position) -> bool {
        position.x < self.width.get() && position.y < self.height.get()
    }

    #[inline]
    pub const fn last(self) -> Position {
        Position::new(self.width.get() - 1, self.height.get() - 1)
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

impl TryFrom<(u16, u16)> for Size {
    type Error = crate::Error;

    fn try_from((width, height): (u16, u16)) -> Result<Self> {
        match Self::try_new(width, height) {
            Some(size) => Ok(size),
            None if width == 0 => Err(Error::ZeroWidth),
            None => Err(Error::ZeroHeight),
        }
    }
}

impl From<Size> for (u16, u16) {
    fn from(size: Size) -> Self {
        (size.width(), size.height())
    }
}

impl From<(NonZeroU16, NonZeroU16)> for Size {
    fn from((width, height): (NonZeroU16, NonZeroU16)) -> Self {
        Self::new(width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_rejects_zero_dimensions() {
        assert_eq!(Size::try_new(0, 40), None);
        assert_eq!(Size::try_new(80, 0), None);
        assert_eq!(Size::try_new(0, 0), None);
    }

    #[test]
    fn try_new_can_be_unwrapped_in_a_constant() {
        const SIZE: Size = Size::try_new(80, 40).unwrap();

        assert_eq!(SIZE.width(), 80);
        assert_eq!(SIZE.height(), 40);
    }

    #[test]
    fn try_new_accepts_non_zero_dimensions() {
        let size = Size::try_new(80, 40).unwrap();

        assert_eq!(size.width(), 80);
        assert_eq!(size.height(), 40);
        assert_eq!(size.area(), 3200);
    }

    #[test]
    fn try_from_tuple_reports_zero_dimension() {
        assert!(matches!(Size::try_from((0, 40)), Err(Error::ZeroWidth)));
        assert!(matches!(Size::try_from((80, 0)), Err(Error::ZeroHeight)));
        assert!(Size::try_from((80, 40)).is_ok());
    }

    #[test]
    fn converts_to_and_from_raw_pair() {
        let size = Size::try_new(80, 40).unwrap();
        let non_zero = (NonZeroU16::new(80).unwrap(), NonZeroU16::new(40).unwrap());

        assert_eq!(Size::from(non_zero), size);

        let pair: (u16, u16) = size.into();
        assert_eq!(pair, (80, 40));
    }

    #[test]
    fn last_is_zero_based() {
        assert_eq!(Size::try_new(80, 40).unwrap().last(), Position::new(79, 39));
    }

    #[test]
    fn contains_checks_bounds() {
        let size = Size::try_new(80, 40).unwrap();

        assert!(size.contains(Position::new(0, 0)));
        assert!(size.contains(Position::new(79, 39)));
        assert!(!size.contains(Position::new(80, 39)));
        assert!(!size.contains(Position::new(79, 40)));
    }

    #[test]
    fn one_by_one_does_not_underflow() {
        let size = Size::try_new(1, 1).unwrap();

        assert_eq!(size.last(), Position::new(0, 0));
        assert_eq!(size.advance(Position::new(0, 0)), Position::new(0, 0));
    }

    #[test]
    fn advance_wraps_to_next_row() {
        let size = Size::try_new(80, 40).unwrap();

        assert_eq!(size.advance(Position::new(79, 0)), Position::new(0, 1));
    }

    #[test]
    fn advance_clamps_at_last_position() {
        let size = Size::try_new(80, 40).unwrap();

        assert_eq!(size.advance(Position::new(79, 39)), Position::new(79, 39));
    }
}
