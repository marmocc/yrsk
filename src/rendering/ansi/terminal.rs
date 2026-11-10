use crate::common::etchel::Etchel;
use crate::common::position::Position;
use crate::common::size::Size;
use crate::rendering::ansi::writer::Writer;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::io;

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

const DEFAULT_SIZE: Size = Size::new(80, 40);

pub struct Terminal {
    writer: Writer,
    output: PlatformOutput,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Self {
            writer: Writer::new(DEFAULT_SIZE),
            output: platform_output()?,
        };

        terminal.writer.push_cursor_hide();
        terminal.writer.push_alternate_enter();
        terminal.writer.push_clear();
        terminal.writer.write_to(&mut terminal.output)?;

        Ok(terminal)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.writer.clear(); // Discard half-built frame
        self.writer.push_reset();
        self.writer.push_cursor_show();
        self.writer.push_alternate_leave();
        let _ = self.writer.write_to(&mut self.output);
    }
}

impl Canvas for Terminal {
    #[inline]
    fn size(&self) -> Size {
        Size::new(80, 40)
    }

    #[inline]
    fn plot(&mut self, position: Position, etchel: Etchel) {
        self.writer.push_move(position);
        self.writer.push_foreground(etchel.foreground);
        self.writer.push_background(etchel.background);
        self.writer.push_glyph_u8(etchel.glyph);
    }
}

impl Renderer for Terminal {
    #[inline]
    fn present(&mut self) -> io::Result<()> {
        self.writer.write_to(&mut self.output)
    }
}
