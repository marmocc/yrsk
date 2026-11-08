use crate::common::{color::Color, digit3::Digit3, position::Position};
use std::io::Write;

pub struct Sequence<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> Sequence<N> {
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self { bytes, len: N }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    #[inline]
    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_bytes());
    }
}

pub const ENTER_ALTERNATE: Sequence<8> = Sequence::new(*b"\x1b[?1049h");
pub const LEAVE_ALTERNATE: Sequence<8> = Sequence::new(*b"\x1b[?1049l");
pub const HIDE_CURSOR: Sequence<6> = Sequence::new(*b"\x1b[?25l");
pub const SHOW_CURSOR: Sequence<6> = Sequence::new(*b"\x1b[?25h");
pub const CLEAR: Sequence<4> = Sequence::new(*b"\x1b[2J");
pub const RESET: Sequence<4> = Sequence::new(*b"\x1b[0m");

#[inline]
pub fn go(position: Position) -> Sequence<24> {
    let mut bytes = [0u8; 24];
    let mut rest = &mut bytes[..];
    write!(rest, "\x1b[{};{}H", position.y + 1, position.x + 1).unwrap();
    let len = 24 - rest.len();
    Sequence { bytes, len }
}

#[inline]
pub fn foreground(color: Color) -> Sequence<19> {
    let r = Digit3::new(color.r).as_bytes();
    let g = Digit3::new(color.g).as_bytes();
    let b = Digit3::new(color.b).as_bytes();

    #[rustfmt::skip]
    let sequence: [u8; 19] = [
        b'\x1b', b'[', b'3', //3 for Foreground
        b'8', b';', b'2', b';',
        r[0], r[1], r[2], b';',
        g[0], g[1], g[2], b';',
        b[0], b[1], b[2], b'm',
    ];

    Sequence {
        bytes: sequence,
        len: 19,
    }
}

#[inline]
pub fn background(color: Color) -> Sequence<19> {
    let r = Digit3::new(color.r).as_bytes();
    let g = Digit3::new(color.g).as_bytes();
    let b = Digit3::new(color.b).as_bytes();

    #[rustfmt::skip]
    let sequence: [u8; 19] = [
        b'\x1b', b'[', b'4', // 4 for Background
        b'8', b';', b'2', b';',
        r[0], r[1], r[2], b';',
        g[0], g[1], g[2], b';',
        b[0], b[1], b[2], b'm',
    ];

    Sequence {
        bytes: sequence,
        len: 19,
    }
}
