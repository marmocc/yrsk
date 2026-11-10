use crate::{
    common::{color::Color, etchel::Etchel, position::Position},
    rendering::command::Command,
};

pub fn chaos(x: u32, y: u32, seed: u32) -> Command {
    let mut h = x
        .wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263))
        .wrapping_add(seed.wrapping_mul(2_246_822_519));
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^= h >> 16;

    let color = Color::new(h as u8, (h >> 8) as u8, (h >> 16) as u8);

    Command::Plot {
        position: Position::new(x as i32, y as i32),
        etchel: Etchel::solid(color),
    }
}
