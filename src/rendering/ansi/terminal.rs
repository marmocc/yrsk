use crate::common::etchel::Etchel;
use crate::common::{position::Position, size::Size};
use crate::rendering::ansi::writer::Writer;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::io;
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
    origin: Instant,
    writer: Writer,
    output: PlatformOutput,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Self {
            origin: Instant::now(),
            writer: Writer::with_capacity(8192),
            output: platform_output()?,
        };

        terminal.writer.push_cursor_hide();
        terminal.writer.push_alternate_enter();
        terminal.writer.push_clear();
        terminal.present()?;

        Ok(terminal)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.writer.clear(); // Discard half-built frame
        self.writer.push_reset();
        self.writer.push_cursor_show();
        self.writer.push_alternate_leave();
        let _ = self.present();
    }
}

impl Canvas for Terminal {
    #[inline]
    fn size(&self) -> Size {
        Size::new(80, 40) // Placeholder
    }

    #[inline]
    fn elapsed(&self) -> std::time::Duration {
        self.origin.elapsed()
    }

    #[inline]
    fn plot(&mut self, position: Position, etchel: Etchel) {
        self.writer.push_move(position);
        self.writer.push_foreground(etchel.foreground);
        self.writer.push_background(etchel.background);
        self.writer.push_glyph_u8(etchel.glyph);
    }

    #[inline]
    fn fullscreen_blit(&mut self, etchels: &[Etchel]) {
        self.writer.push_move(Position::zero());
        for etchel in etchels {
            self.writer.push_foreground(etchel.foreground);
            self.writer.push_background(etchel.background);
            self.writer.push_glyph_u8(etchel.glyph);
        }
        self.writer.forget_cursor();
    }
}

impl Renderer for Terminal {
    fn present(&mut self) -> io::Result<()> {
        self.writer.flush(&mut self.output)
    }
}
