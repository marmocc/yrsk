use crate::common::{Color, Position};
use crate::rendering::etchel::Etchel;
use crate::rendering::painter::Painter;

pub struct Chaos;
impl Painter for Chaos {
    fn paint(&mut self, canvas: &mut impl super::canvas::Canvas) {
        let size = canvas.size();
        let time = canvas.elapsed().as_nanos() as u32;

        for y in 0..size.height {
            for x in 0..size.width {
                let mut h = x
                    .wrapping_mul(374_761_393)
                    .wrapping_add(y.wrapping_mul(668_265_263))
                    .wrapping_add(time.wrapping_mul(2_246_822_519));
                h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
                h ^= h >> 16;

                let color = Color::new(h as u8, (h >> 8) as u8, (h >> 16) as u8);
                canvas.plot(Position::new(x as i32, y as i32), Etchel::solid(color));
            }
        }
    }
}
