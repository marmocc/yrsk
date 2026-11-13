use crate::Result;
use crate::common::etchel::Etchel;
use crate::common::position::Position;
use crate::common::size::Size;
use crate::rendering::ansi::platform::prepare_and_get_stdout;
use crate::rendering::ansi::writer::Writer;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU16;

const DEFAULT_SIZE: Size = Size::new(NonZeroU16::new(80).unwrap(), NonZeroU16::new(40).unwrap());

pub struct Terminal {
    writer: Writer,
    output: File,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut terminal = Self {
            writer: Writer::new(DEFAULT_SIZE),
            output: prepare_and_get_stdout()?,
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
        DEFAULT_SIZE
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
