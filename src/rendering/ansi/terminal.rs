use crate::common::etchel::Etchel;
use crate::common::{color::Color, position::Position, size::Size};
use crate::rendering::ansi::sequence;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::io::{self, Write};
use std::time::Instant;

#[cfg(windows)]
type PlatformOutput = std::fs::File;
#[cfg(windows)]
fn platform_output() -> io::Result<PlatformOutput> {
    use std::os::windows::io::AsHandle;
    let handle = std::io::stdout().as_handle().try_clone_to_owned()?;
    Ok(std::fs::File::from(handle))
}

#[cfg(not(windows))]
type PlatformOutput = std::io::Stdout;
#[cfg(not(windows))]
fn platform_output() -> io::Result<PlatformOutput> {
    Ok(std::io::stdout())
}

pub struct Terminal {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
    origin: Instant,
    buffer: Vec<u8>,
    output: PlatformOutput,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Self {
            cursor: None,
            foreground: None,
            background: None,
            origin: Instant::now(),
            buffer: Vec::with_capacity(8192),
            output: platform_output()?,
        };

        sequence::HIDE_CURSOR.write_to(&mut terminal.buffer);
        sequence::ENTER_ALTERNATE.write_to(&mut terminal.buffer);
        sequence::CLEAR.write_to(&mut terminal.buffer);
        terminal.present()?;

        Ok(terminal)
    }

    #[inline]
    fn set_cursor(&mut self, position: Position) {
        if self.cursor == Some(position) {
            return;
        }

        let sequence = sequence::go(position);
        self.buffer.extend_from_slice(sequence.as_bytes());
        self.cursor = Some(position);
    }

    #[inline]
    fn set_foreground(&mut self, color: Color) {
        if self.foreground == Some(color) {
            return;
        }

        let sequence = sequence::foreground(color);
        self.buffer.extend_from_slice(sequence.as_bytes());
        self.foreground = Some(color);
    }

    #[inline]
    fn set_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

        let sequence = sequence::background(color);
        self.buffer.extend_from_slice(sequence.as_bytes());
        self.background = Some(color);
    }

    #[inline]
    fn bump_cursor(&mut self) {
        self.cursor = self.cursor.map(|c| c + (1, 0))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.buffer.clear(); // Discard half-built frame
        sequence::RESET.write_to(&mut self.buffer);
        sequence::SHOW_CURSOR.write_to(&mut self.buffer);
        sequence::LEAVE_ALTERNATE.write_to(&mut self.buffer);
        let _ = self.present();
    }
}

impl Canvas for Terminal {
    #[inline]
    fn size(&self) -> Size {
        Size::new(80, 40)
    }

    #[inline]
    fn elapsed(&self) -> std::time::Duration {
        self.origin.elapsed()
    }

    #[inline]
    fn plot(&mut self, position: Position, etchel: Etchel) {
        self.set_cursor(position);
        self.set_foreground(etchel.foreground);
        self.set_background(etchel.background);
        self.buffer.push(etchel.glyph);
        self.bump_cursor();
    }

    #[inline]
    fn fullscreen_blit(&mut self, etchels: &[Etchel]) {
        self.set_cursor(Position::zero());
        for etchel in etchels {
            self.set_foreground(etchel.foreground);
            self.set_background(etchel.background);
            self.buffer.push(etchel.glyph);
            self.bump_cursor();
        }
        self.cursor = None;
    }
}

impl Renderer for Terminal {
    fn present(&mut self) -> std::io::Result<()> {
        self.output.write_all(&self.buffer)?;
        self.output.flush()?;
        self.buffer.clear();
        Ok(())
    }
}
