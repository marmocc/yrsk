use crate::common::{color::Color, digit3::Digit3, position::Position};
use std::io::Write;

pub struct Writer {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
    buffer: Vec<u8>,
}

impl Writer {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cursor: None,
            foreground: None,
            background: None,
            buffer: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    pub fn flush<T: Write>(&mut self, out: &mut T) {
        out.write_all(&self.buffer);
        out.flush();
        self.clear();
    }

    #[inline]
    pub fn forget_cursor(&mut self) {
        self.cursor = None;
    }

    #[inline]
    fn bump_cursor(&mut self) {
        self.cursor = self.cursor.map(|c| c + (1, 0))
    }

    #[inline]
    pub fn push_move(&mut self, position: Position) {
        if self.cursor == Some(position) {
            return;
        }

        let row = position.y + 1;
        let col = position.x + 1;

        write!(self.buffer, "\x1b[{};{}H", row, col).unwrap();
        self.cursor = Some(position);
    }

    #[inline]
    pub fn push_foreground(&mut self, color: Color) {
        if self.foreground == Some(color) {
            return;
        }

        let r = Digit3::new(color.r).as_bytes();
        let g = Digit3::new(color.g).as_bytes();
        let b = Digit3::new(color.b).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 19] = [
            b'\x1b', b'[', b'3', //3 for Foreground
            b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2], b'm',
        ];

        self.buffer.extend_from_slice(&sequence);
        self.foreground = Some(color);
    }

    #[inline]
    pub fn push_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

        let r = Digit3::new(color.r).as_bytes();
        let g = Digit3::new(color.g).as_bytes();
        let b = Digit3::new(color.b).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 19] = [
            b'\x1b', b'[', b'4', // 4 for Background
            b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2], b'm',
        ];

        self.buffer.extend_from_slice(&sequence);
        self.background = Some(color);
    }

    #[inline]
    pub fn push_glyph_u8(&mut self, glyph: u8) {
        self.buffer.push(glyph);
        self.bump_cursor();
    }

    #[inline]
    pub fn push_alternate_enter(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049h");
    }

    #[inline]
    pub fn push_alternate_leave(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049l");
    }

    #[inline]
    pub fn push_cursor_hide(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    #[inline]
    pub fn push_cursor_show(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    #[inline]
    pub fn push_clear(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[2J");
    }

    #[inline]
    pub fn push_reset(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
    }
}
