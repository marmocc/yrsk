use crate::common::arena::Arena;
use crate::common::position::Position;
use crate::rendering::ansi::writer::Writer;
use crate::rendering::command::Command;
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

pub struct Terminal {
    writer: Writer,
    output: PlatformOutput,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let mut terminal = Self {
            writer: Writer::with_capacity(8192),
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

impl Renderer for Terminal {
    fn present(&mut self, commands: &[Command], assets: Arena) -> io::Result<()> {
        for command in commands {
            match command {
                Command::Plot { position, etchel } => {
                    self.writer.push_move(*position);
                    self.writer.push_foreground(etchel.foreground);
                    self.writer.push_background(etchel.background);
                    self.writer.push_glyph_u8(etchel.glyph);
                }
                Command::FullscreenBlit { handle } => {
                    let etchels = assets.get(*handle);
                    self.writer.push_move(Position::zero());
                    for etchel in etchels {
                        self.writer.push_foreground(etchel.foreground);
                        self.writer.push_background(etchel.background);
                        self.writer.push_glyph_u8(etchel.glyph);
                    }
                    self.writer.forget_cursor();
                }
                Command::Blit { position, handle } => todo!(),
            }
        }

        self.writer.write_to(&mut self.output)
    }
}
