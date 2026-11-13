use crate::common::{color::Color, digits::Digits, position::Position, size::Size};

const DEFAULT_SEQUENCE_LENGTH: usize = 32;

pub struct Encoder {
    buffer: Vec<u8>,
}

impl Encoder {
    #[inline]
    pub fn new(size: Size) -> Self {
        Self {
            buffer: Vec::with_capacity(
                size.width() as usize * size.height() as usize * DEFAULT_SEQUENCE_LENGTH,
            ),
        }
    }

    #[inline]
    pub fn written(&self) -> &[u8] {
        &self.buffer
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    pub fn encode_move(&mut self, position: Position) {
        let row = Digits::from_u16(position.y.saturating_add(1)).as_bytes();
        let col = Digits::from_u16(position.x.saturating_add(1)).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 14] = [
            b'\x1b', b'[',
            row[0], row[1], row[2], row[3], row[4],
            b';',
            col[0], col[1], col[2], col[3], col[4],
            b'H',
        ];

        self.buffer.extend_from_slice(&sequence);
    }

    #[inline]
    pub fn encode_foreground(&mut self, color: Color) {
        let r = Digits::look_u8(color.r).as_bytes();
        let g = Digits::look_u8(color.g).as_bytes();
        let b = Digits::look_u8(color.b).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 19] = [
            b'\x1b', b'[', b'3', // 3 for Foreground
            b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2], b'm',
        ];

        self.buffer.extend_from_slice(&sequence);
    }

    #[inline]
    pub fn encode_background(&mut self, color: Color) {
        let r = Digits::look_u8(color.r).as_bytes();
        let g = Digits::look_u8(color.g).as_bytes();
        let b = Digits::look_u8(color.b).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 19] = [
            b'\x1b', b'[', b'4', // 4 for Background
            b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2], b'm',
        ];

        self.buffer.extend_from_slice(&sequence);
    }

    #[inline]
    pub fn encode_glyph_u8(&mut self, glyph: u8) {
        self.buffer.push(glyph);
    }

    #[inline]
    pub fn encode_glyph_char(&mut self, glyph: char) {
        let mut scratch = [0u8; 4];
        let encoded = glyph.encode_utf8(&mut scratch).as_bytes();
        self.buffer.extend_from_slice(encoded);
    }

    #[inline]
    pub fn encode_alternate_enter(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049h");
    }

    #[inline]
    pub fn encode_alternate_leave(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049l");
    }

    #[inline]
    pub fn encode_cursor_hide(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    #[inline]
    pub fn encode_cursor_show(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    #[inline]
    pub fn encode_clear(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[2J");
    }

    #[inline]
    pub fn encode_reset(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encoder() -> Encoder {
        Encoder::new(Size::try_new(80, 40).unwrap())
    }

    // --- cursor ---------------------------------------------------------

    #[test]
    fn move_is_one_based_row_then_column() {
        let mut e = encoder();
        e.encode_move(Position::new(0, 0));
        assert_eq!(e.buffer, b"\x1b[00001;00001H");
    }

    #[test]
    fn move_uses_zero_padded_five_digit_coordinates() {
        let mut e = encoder();
        e.encode_move(Position::new(79, 39));
        assert_eq!(e.buffer, b"\x1b[00040;00080H");
    }

    #[test]
    fn move_saturates_at_max_coordinates() {
        let mut e = encoder();
        e.encode_move(Position::new(u16::MAX, u16::MAX));
        assert_eq!(e.buffer, b"\x1b[65535;65535H");
    }

    #[test]
    fn repeated_move_is_always_emitted() {
        let mut e = encoder();
        e.encode_move(Position::new(4, 2));
        e.encode_move(Position::new(4, 2));
        assert_eq!(e.buffer, b"\x1b[00003;00005H\x1b[00003;00005H");
    }

    // --- colours --------------------------------------------------------

    #[test]
    fn foreground_sequence_is_zero_padded_truecolor() {
        let mut e = encoder();
        e.encode_foreground(Color::new(1, 20, 255));
        assert_eq!(e.buffer, b"\x1b[38;2;001;020;255m");
    }

    #[test]
    fn background_sequence_is_zero_padded_truecolor() {
        let mut e = encoder();
        e.encode_background(Color::new(0, 128, 7));
        assert_eq!(e.buffer, b"\x1b[48;2;000;128;007m");
    }

    #[test]
    fn foreground_and_background_use_distinct_sequences() {
        let mut e = encoder();
        e.encode_foreground(Color::new(5, 5, 5));
        e.encode_background(Color::new(5, 5, 5));
        assert_eq!(
            e.buffer,
            b"\x1b[38;2;005;005;005m\x1b[48;2;005;005;005m".as_slice()
        );
    }

    #[test]
    fn colour_boundaries_are_encoded_correctly() {
        let mut e = encoder();
        e.encode_foreground(Color::new(9, 10, 99));
        e.encode_background(Color::new(100, 199, 200));
        assert_eq!(
            e.buffer,
            b"\x1b[38;2;009;010;099m\x1b[48;2;100;199;200m".as_slice()
        );
    }

    // --- glyphs ---------------------------------------------------------

    #[test]
    fn byte_glyph_is_written_verbatim() {
        let mut e = encoder();
        e.encode_glyph_u8(b'@');
        assert_eq!(e.buffer, b"@");
    }

    #[test]
    fn ascii_char_is_encoded() {
        let mut e = encoder();
        e.encode_glyph_char('a');
        assert_eq!(e.buffer, b"a");
    }

    #[test]
    fn two_byte_char_is_utf8_encoded() {
        let mut e = encoder();
        e.encode_glyph_char('é');
        assert_eq!(e.buffer, "é".as_bytes());
    }

    #[test]
    fn four_byte_char_is_utf8_encoded() {
        let mut e = encoder();
        e.encode_glyph_char('\u{1f600}');
        assert_eq!(e.buffer, "\u{1f600}".as_bytes());
    }

    // --- buffer ---------------------------------------------------------

    #[test]
    fn clear_buffer_discards_written_bytes() {
        let mut e = encoder();
        e.encode_glyph_u8(b'a');
        e.clear();
        assert!(e.written().is_empty());
    }

    // --- fixed sequences ------------------------------------------------

    #[test]
    fn fixed_sequences_have_expected_bytes() {
        let mut e = encoder();
        e.encode_alternate_enter();
        assert_eq!(e.buffer, b"\x1b[?1049h");
        e.clear();
        e.encode_alternate_leave();
        assert_eq!(e.buffer, b"\x1b[?1049l");
        e.clear();
        e.encode_cursor_hide();
        assert_eq!(e.buffer, b"\x1b[?25l");
        e.clear();
        e.encode_cursor_show();
        assert_eq!(e.buffer, b"\x1b[?25h");
        e.clear();
        e.encode_clear();
        assert_eq!(e.buffer, b"\x1b[2J");
        e.clear();
        e.encode_reset();
        assert_eq!(e.buffer, b"\x1b[0m");
    }
}
