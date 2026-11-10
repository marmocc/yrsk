use crate::common::{etchel::Etchel, position::Position, size::Size};

pub trait Canvas {
    fn size(&self) -> Size;
    fn plot(&mut self, position: Position, etchel: Etchel);
}
