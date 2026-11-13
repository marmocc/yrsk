use crate::common::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Etchel {
    pub foreground: Color,
    pub background: Color,
    pub glyph: u8,
}

impl Etchel {
    #[inline]
    pub const fn solid(color: Color) -> Self {
        Self {
            foreground: Color::black(),
            background: color,
            glyph: b' ',
        }
    }
}
