use crate::common::{Color, Digit3, Position};
use std::io::Write;

#[derive(Default)]
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_move(&mut self, position: Position) {
        if self.cursor == Some(position) {
            return;
        }

        let row = position.y + 1;
        let col = position.x + 1;

        write!(self.buffer, "\x1b[{};{}H", row, col).unwrap();
        self.cursor = Some(position);
    }

    fn push_color(&mut self, layer: Layer, color: Color) {
        let r_digits = Digit3::new(color.r);
        let g_digits = Digit3::new(color.g);
        let b_digits = Digit3::new(color.b);

        self.buffer.extend_from_slice(b"\x1b[");
        match layer {
            Layer::Foreground => self.buffer.push(b'3'),
            Layer::Background => self.buffer.push(b'4'),
        }
        self.buffer.extend_from_slice(b"8;2;");
        self.buffer.extend_from_slice(&r_digits.as_bytes());
        self.buffer.push(b';');
        self.buffer.extend_from_slice(&g_digits.as_bytes());
        self.buffer.push(b';');
        self.buffer.extend_from_slice(&b_digits.as_bytes());
        self.buffer.push(b'm');
    }

    pub fn push_foreground(&mut self, color: Color) {
        if self.foreground == Some(color) {
            return;
        }

        self.push_color(Layer::Foreground, color);
        self.foreground = Some(color);
    }

    pub fn push_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

        self.push_color(Layer::Background, color);
        self.background = Some(color);
    }

    pub fn push_byte(&mut self, value: u8) {
        self.buffer.push(value);
        self.cursor = self.cursor.map(|p| p.add(1, 0));
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let mut out = std::io::stdout().lock();
        out.write_all(&self.buffer)?;
        out.flush()?;
        self.buffer.clear();
        Ok(())
    }
}
