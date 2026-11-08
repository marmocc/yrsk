#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
