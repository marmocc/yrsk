use crate::common::etchel::Etchel;
use crate::common::position::Position;
use crate::common::size::Size;
use crate::rendering::ansi::writer::Writer;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::fs::File;
use std::io::{Result, Write};

#[cfg(unix)]
pub fn get_raw_stdout() -> Result<File> {
    use std::os::fd::AsFd;
    let fd = std::io::stdout().as_fd().try_clone_to_owned()?;
    Ok(std::fs::File::from(fd))
}

#[cfg(windows)]
pub fn get_raw_stdout() -> Result<File> {
    use std::os::windows::io::AsHandle;
    let handle = std::io::stdout().as_handle().try_clone_to_owned()?;
    Ok(std::fs::File::from(handle))
}

const DEFAULT_SIZE: Size = Size::new(80, 40);

pub struct Terminal {
    writer: Writer,
    output: File,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut terminal = Self {
            writer: Writer::new(DEFAULT_SIZE),
            output: get_raw_stdout()?,
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
    fn present(&mut self) -> Result<()> {
        self.output.write_all(self.writer.written())?;
        self.writer.clear();
        Ok(())
    }
}
