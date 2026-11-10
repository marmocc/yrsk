use std::time::Instant;

use crate::{
    common::{color::Color, etchel::Etchel, position::Position},
    rendering::canvas::Canvas,
};

#[inline]
pub fn chaos(origin: Instant, canvas: &mut impl Canvas) {
    let size = canvas.size();
    let seed = origin.elapsed().as_micros() as u32;

    for y in 0..size.height {
        for x in 0..size.width {
            let mut h = x
                .wrapping_mul(374_761_393)
                .wrapping_add(y.wrapping_mul(668_265_263))
                .wrapping_add(seed.wrapping_mul(2_246_822_519));
            h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
            h ^= h >> 16;

            let color = Color::new(h as u8, (h >> 8) as u8, (h >> 16) as u8);
            canvas.plot(Position::new(x as i32, y as i32), Etchel::solid(color));
        }
    }
}
