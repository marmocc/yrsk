use crate::common::{arena::Handle, etchel::Etchel, position::Position};

#[derive(Clone, Copy)]
pub enum Command {
    Plot {
        position: Position,
        etchel: Etchel,
    },
    Blit {
        position: Position,
        handle: Handle<Etchel>,
    },
    FullscreenBlit {
        handle: Handle<Etchel>,
    },
}
