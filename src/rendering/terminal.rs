use crate::{
    common::{Color, Digit3, Position},
    rendering::{context::Canvas, etchel::Etchel},
};
use std::io::Write;

pub struct Terminal {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
    buffer: Vec<u8>,
}

enum Layer {
    Foreground,
    Background,
}

impl Terminal {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cursor: None,
            foreground: None,
            background: None,
            buffer: Vec::with_capacity(capacity),
        }
    }

    pub fn new() -> Self {
        Self::with_capacity(4096)
    }

    fn push_move(&mut self, position: Position) {
        if self.cursor == Some(position) {
            return;
        }

        let row = position.y + 1;
        let col = position.x + 1;

        write!(self.buffer, "\x1b[{};{}H", row, col).unwrap();
        self.cursor = Some(position);
    }

    fn bump_cursor(&mut self) {
        self.cursor = self.cursor.map(|p| p + (1, 0));
    }

    fn push_color(&mut self, layer: Layer, color: Color) {
        let r_digits = Digit3::new(color.r);
        let g_digits = Digit3::new(color.g);
        let b_digits = Digit3::new(color.b);

        self.buffer.extend_from_slice(b"\x1b[");
        self.buffer.push(match layer {
            Layer::Foreground => b'3',
            Layer::Background => b'4',
        });
        self.buffer.extend_from_slice(b"8;2;");
        self.buffer.extend_from_slice(&r_digits.as_bytes());
        self.buffer.push(b';');
        self.buffer.extend_from_slice(&g_digits.as_bytes());
        self.buffer.push(b';');
        self.buffer.extend_from_slice(&b_digits.as_bytes());
        self.buffer.push(b'm');
    }

    fn push_foreground(&mut self, color: Color) {
        if self.foreground == Some(color) {
            return;
        }

        self.push_color(Layer::Foreground, color);
        self.foreground = Some(color);
    }

    fn push_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

        self.push_color(Layer::Background, color);
        self.background = Some(color);
    }

    fn push_byte(&mut self, value: u8) {
        self.buffer.push(value);
        self.bump_cursor();
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let mut out = std::io::stdout().lock();
        out.write_all(&self.buffer)?;
        out.flush()?;
        self.buffer.clear();
        Ok(())
    }
}

impl Canvas for Terminal {
    fn plot(&mut self, position: Position, etchel: Etchel) {
        self.push_move(position);
        self.push_foreground(etchel.foreground);
        self.push_background(etchel.background);
        self.push_byte(etchel.glyph);
        self.bump_cursor();
    }

    fn fullscreen_blit(&mut self, etchels: &[Etchel]) {
        self.push_move(Position::zero());
        for etchel in etchels {
            self.push_foreground(etchel.foreground);
            self.push_background(etchel.background);
            self.push_byte(etchel.glyph);
        }
        self.cursor = None;
    }
}
