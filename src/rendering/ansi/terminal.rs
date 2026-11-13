use crate::Result;
use crate::common::etchel::Etchel;
use crate::common::position::Position;
use crate::common::size::Size;
use crate::rendering::ansi::encoder::Encoder;
use crate::rendering::ansi::platform::prepare_and_get_stdout;
use crate::rendering::ansi::state::State;
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use std::fs::File;
use std::io::Write;

const DEFAULT_SIZE: Size = Size::try_new(80, 40).unwrap();

pub struct Terminal {
    state: State,
    encoder: Encoder,
    output: File,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let output = prepare_and_get_stdout()?;
        let mut terminal = Self {
            state: State::new(),
            encoder: Encoder::new(DEFAULT_SIZE),
            output,
        };

        terminal.encoder.encode_cursor_hide();
        terminal.encoder.encode_alternate_enter();
        terminal.encoder.encode_clear();
        terminal.present()?;

        Ok(terminal)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.encoder.clear(); // Discard half-built frame
        self.encoder.encode_reset();
        self.encoder.encode_cursor_show();
        self.encoder.encode_alternate_leave();
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
        let encoder = &mut self.encoder;
        let state = &mut self.state;

        state.ensure_cursor(position, |p| encoder.encode_move(p));
        state.ensure_foreground(etchel.foreground, |c| encoder.encode_foreground(c));
        state.ensure_background(etchel.background, |c| encoder.encode_background(c));
        encoder.encode_glyph_u8(etchel.glyph);
        state.advance();
    }
}

impl Renderer for Terminal {
    #[inline]
    fn present(&mut self) -> Result<()> {
        self.output.write_all(self.encoder.written())?;
        self.encoder.clear();
        Ok(())
    }
}
