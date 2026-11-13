use std::time::Instant;

use crate::common::{color::Color, etchel::Etchel, position::Position};
use crate::rendering::{canvas::Canvas, painter::Painter};

pub struct Chaos {
    origin: Instant,
}

impl Chaos {
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for Chaos {
    fn default() -> Self {
        Self::new()
    }
}

impl Painter for Chaos {
    fn paint(&mut self, canvas: &mut impl Canvas) {
        let size = canvas.size();
        let seed = self.origin.elapsed().as_micros() as u32;

        for y in 0..size.height() {
            for x in 0..size.width() {
                let (xh, yh) = (u32::from(x), u32::from(y));
                let mut h = xh
                    .wrapping_mul(374_761_393)
                    .wrapping_add(yh.wrapping_mul(668_265_263))
                    .wrapping_add(seed.wrapping_mul(2_246_822_519));
                h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
                h ^= h >> 16;

                let color = Color::new(h as u8, (h >> 8) as u8, (h >> 16) as u8);
                canvas.plot(Position::new(x, y), Etchel::solid(color));
            }
        }
    }
}
