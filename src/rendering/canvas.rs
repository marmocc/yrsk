use std::time::Duration;

use crate::{
    common::{Position, Size},
    rendering::etchel::Etchel,
};

pub trait Canvas {
    fn elapsed(&self) -> Duration;
    fn size(&self) -> Size;
    fn plot(&mut self, position: Position, etchel: Etchel);
    fn fullscreen_blit(&mut self, etchels: &[Etchel]);
}
