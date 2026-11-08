use crate::{common::Position, rendering::etchel::Etchel};

pub trait Canvas {
    fn plot(&mut self, position: Position, etchel: Etchel);
    fn fullscreen_blit(&mut self, etchels: &[Etchel]);
}
