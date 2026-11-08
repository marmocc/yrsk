use crate::common::{etchel::Etchel, position::Position, size::Size};
use std::time::Duration;

pub trait Canvas {
    fn elapsed(&self) -> Duration;
    fn size(&self) -> Size;
    fn plot(&mut self, position: Position, etchel: Etchel);
    fn fullscreen_blit(&mut self, etchels: &[Etchel]);
}
