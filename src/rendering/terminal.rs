use crate::{
    common::{Color, Digit3, Position, Size},
    rendering::{canvas::Canvas, etchel::Etchel, renderer::Renderer},
};
use std::{io::Write, time::Instant};

pub struct Terminal {
    size: Size,
    origin: Instant,
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
    pub fn with_size(size: Size) -> Self {
        let capacity = (size.width * size.height * 32) as usize;
        Self {
            size,
            origin: Instant::now(),
            cursor: None,
            foreground: None,
            background: None,
            buffer: Vec::with_capacity(capacity),
        }
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

    fn push_glyph_u8(&mut self, value: u8) {
        self.buffer.push(value);
        self.bump_cursor();
    }
}

impl Renderer for Terminal {
    fn present(&mut self) -> std::io::Result<()> {
        let mut out = std::io::stdout();
        out.write_all(&self.buffer)?;
        out.flush()?;
        self.buffer.clear();
        Ok(())
    }
}

impl Canvas for Terminal {
    fn size(&self) -> Size {
        self.size
    }

    fn elapsed(&self) -> std::time::Duration {
        self.origin.elapsed()
    }

    fn plot(&mut self, position: Position, etchel: Etchel) {
        self.push_move(position);
        self.push_foreground(etchel.foreground);
        self.push_background(etchel.background);
        self.push_glyph_u8(etchel.glyph);
    }

    fn fullscreen_blit(&mut self, etchels: &[Etchel]) {
        self.push_move(Position::zero());
        for etchel in etchels {
            self.push_foreground(etchel.foreground);
            self.push_background(etchel.background);
            self.push_glyph_u8(etchel.glyph);
        }
        self.cursor = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_plots_emit_one_move() {
        let mut t = Terminal::with_size(Size {
            width: 10,
            height: 2,
        });
        for x in 0..5 {
            t.plot(Position::new(x, 0), Etchel::solid(Color::new(1, 2, 3)));
        }
        let sequences = t.buffer.iter().filter(|&&b| b == b'\x1b').count();
        assert_eq!(sequences, 3); // 3 because of first-time Foreground, Background and Move
    }

    #[test]
    fn two_push_move_emit_one() {
        let mut t = Terminal::with_size(Size {
            width: 10,
            height: 2,
        });

        t.push_move(Position::new(4, 0));
        t.push_move(Position::new(4, 0));

        let sequences = t.buffer.iter().filter(|&&b| b == b'\x1b').count();
        assert_eq!(sequences, 1);
    }

    #[test]
    fn two_push_foreground_emit_one() {
        let mut t = Terminal::with_size(Size {
            width: 10,
            height: 2,
        });

        t.push_foreground(Color::new(8, 8, 8));
        t.push_foreground(Color::new(8, 8, 8));

        let sequences = t.buffer.iter().filter(|&&b| b == b'\x1b').count();
        assert_eq!(sequences, 1);
    }

    #[test]
    fn two_push_background_emit_one() {
        let mut t = Terminal::with_size(Size {
            width: 10,
            height: 2,
        });

        t.push_background(Color::new(8, 8, 8));
        t.push_background(Color::new(8, 8, 8));

        let sequences = t.buffer.iter().filter(|&&b| b == b'\x1b').count();
        assert_eq!(sequences, 1);
    }
}
