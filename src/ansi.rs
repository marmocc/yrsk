use crate::common::{Color, Digit3, Position};
use std::io::Write;

pub struct Terminal {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
    buffer: Vec<u8>,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            cursor: None,
            foreground: None,
            background: None,
            buffer: Vec::new(),
        }
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

    fn push_color(&mut self, toggle: bool, color: Color) {
        let r_digits = Digit3::new(color.r);
        let g_digits = Digit3::new(color.g);
        let b_digits = Digit3::new(color.b);

        self.buffer.extend_from_slice(b"\x1b[");
        self.buffer.push(if toggle { b'3' } else { b'4' });
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

        self.push_color(true, color);
        self.foreground = Some(color);
    }

    pub fn push_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

        self.push_color(false, color);
        self.background = Some(color);
    }
}
