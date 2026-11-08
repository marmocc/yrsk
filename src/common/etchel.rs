use crate::common::color::Color;

#[derive(Clone, Copy)]
pub struct Etchel {
    pub foreground: Color,
    pub background: Color,
    pub glyph: u8,
}

impl Etchel {
    pub fn solid(color: Color) -> Self {
        Self {
            foreground: Color::black(),
            background: color,
            glyph: b' ',
        }
    }
}
