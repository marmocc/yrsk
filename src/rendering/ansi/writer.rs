use crate::common::{color::Color, digits::Digits, position::Position, size::Size};
const DEFAULT_SEQUENCE_LENGTH: usize = 32;

pub struct Writer {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
    buffer: Vec<u8>,
    size: Size,
}

impl Writer {
    #[inline]
    pub fn new(size: Size) -> Self {
        Self {
            size,
            cursor: None,
            foreground: None,
            background: None,
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
    pub fn forget_cursor(&mut self) {
        self.cursor = None;
    }

    #[inline]
    pub fn forget_foreground(&mut self) {
        self.foreground = None;
    }

    #[inline]
    pub fn forget_background(&mut self) {
        self.background = None;
    }

    #[inline]
    pub fn bump_cursor(&mut self) {
        self.cursor = self.cursor.map(|p| self.size.advance(p));
    }

    #[inline]
    pub fn push_move(&mut self, position: Position) {
        if self.cursor == Some(position) {
            return;
        }

        let row = Digits::from_u16(position.y + 1).as_bytes();
        let col = Digits::from_u16(position.x + 1).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 14] = [
            b'\x1b', b'[',
            row[0], row[1], row[2], row[3], row[4],
            b';',
            col[0], col[1], col[2], col[3], col[4],
            b'H',
        ];

        self.buffer.extend_from_slice(&sequence);
        self.cursor = Some(position);
    }

    #[inline]
    pub fn push_foreground(&mut self, color: Color) {
        if self.foreground == Some(color) {
            return;
        }

        let r = Digits::look_u8(color.r).as_bytes();
        let g = Digits::look_u8(color.g).as_bytes();
        let b = Digits::look_u8(color.b).as_bytes();

        #[rustfmt::skip]
        let sequence: [u8; 19] = [
            b'\x1b', b'[', b'3', //3 for Foreground
            b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2], b'm',
        ];

        self.buffer.extend_from_slice(&sequence);
        self.foreground = Some(color);
    }

    #[inline]
    pub fn push_background(&mut self, color: Color) {
        if self.background == Some(color) {
            return;
        }

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
        self.background = Some(color);
    }

    #[inline]
    pub fn push_glyph_u8(&mut self, glyph: u8) {
        self.buffer.push(glyph);
        self.bump_cursor();
    }

    #[inline]
    pub fn push_glyph_char(&mut self, glyph: char) {
        let mut scratch = [0u8; 4];
        let encoded = glyph.encode_utf8(&mut scratch).as_bytes();
        self.buffer.extend_from_slice(encoded);
        self.bump_cursor();
    }

    #[inline]
    pub fn push_alternate_enter(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049h");
        self.forget_cursor();
    }

    #[inline]
    pub fn push_alternate_leave(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?1049l");
        self.forget_cursor();
    }

    #[inline]
    pub fn push_cursor_hide(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25l");
    }

    #[inline]
    pub fn push_cursor_show(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[?25h");
    }

    #[inline]
    pub fn push_clear(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[2J");
        self.forget_cursor();
    }

    #[inline]
    pub fn push_reset(&mut self) {
        self.buffer.extend_from_slice(b"\x1b[0m");
        self.forget_foreground();
        self.forget_background();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn writer() -> Writer {
        Writer::new(Size::try_new(80, 40).unwrap())
    }

    // --- cursor ---------------------------------------------------------

    #[test]
    fn first_move_is_always_emitted() {
        let mut w = writer();
        w.push_move(Position::new(0, 0));
        assert_eq!(w.buffer, b"\x1b[00001;00001H");
    }

    #[test]
    fn move_uses_one_based_row_then_column() {
        let mut w = writer();
        w.push_move(Position::new(79, 39));
        assert_eq!(w.buffer, b"\x1b[00040;00080H");
    }

    #[test]
    fn repeated_move_to_same_position_is_skipped() {
        let mut w = writer();
        w.push_move(Position::new(4, 2));
        w.push_move(Position::new(4, 2));
        assert_eq!(w.buffer, b"\x1b[00003;00005H");
    }

    #[test]
    fn glyph_advances_tracked_cursor_so_adjacent_move_is_skipped() {
        let mut w = writer();
        w.push_move(Position::new(0, 0));
        w.push_glyph_u8(b'a');
        w.push_move(Position::new(1, 0));
        w.push_glyph_u8(b'b');
        assert_eq!(w.buffer, b"\x1b[00001;00001Hab");
    }

    #[test]
    fn non_adjacent_move_is_emitted() {
        let mut w = writer();
        w.push_move(Position::new(0, 0));
        w.push_glyph_u8(b'a');
        w.push_move(Position::new(5, 0));
        assert_eq!(w.buffer, b"\x1b[00001;00001Ha\x1b[00001;00006H");
    }

    #[test]
    fn glyph_without_known_cursor_keeps_cursor_unknown() {
        let mut w = writer();
        w.push_glyph_u8(b'a');
        w.push_move(Position::new(1, 0));
        assert_eq!(w.buffer, b"a\x1b[00001;00002H");
    }

    #[test]
    fn forget_cursor_forces_next_move() {
        let mut w = writer();
        w.push_move(Position::new(2, 2));
        w.forget_cursor();
        w.push_move(Position::new(2, 2));
        assert_eq!(w.buffer, b"\x1b[00003;00003H\x1b[00003;00003H");
    }

    #[test]
    fn bump_cursor_into_new_line() {
        let mut w = writer();
        w.push_move(Position::new(78, 0));
        w.push_glyph_u8(b'a');
        assert_eq!(w.cursor, Some(Position::new(79, 0)));
        w.push_glyph_u8(b'a');
        assert_eq!(w.cursor, Some(Position::new(0, 1)));
    }

    #[test]
    fn ascii_char_is_encoded() {
        let mut w = writer();
        w.push_glyph_char('a');
        assert_eq!(w.buffer, b"a");
    }

    #[test]
    fn two_byte_char_is_utf8_encoded() {
        let mut w = writer();
        w.push_glyph_char('é');
        assert_eq!(w.buffer, "é".as_bytes());
    }

    #[test]
    fn four_byte_char_is_utf8_encoded() {
        let mut w = writer();
        w.push_glyph_char('\u{1f600}');
        assert_eq!(w.buffer, "\u{1f600}".as_bytes());
    }

    #[test]
    fn char_advances_tracked_cursor_once() {
        let mut w = writer();
        w.push_move(Position::new(0, 0));
        w.push_glyph_char('é');
        w.push_move(Position::new(1, 0));
        assert_eq!(w.buffer, "\x1b[00001;00001Hé".as_bytes());
    }

    #[test]
    fn repeated_char_does_not_emit_cursor_moves() {
        let mut w = writer();
        w.push_move(Position::new(0, 0));
        w.push_glyph_char('a');
        w.push_move(Position::new(1, 0));
        w.push_glyph_char('b');
        w.push_move(Position::new(2, 0));
        assert_eq!(w.buffer, b"\x1b[00001;00001Hab");
    }

    // --- colours --------------------------------------------------------

    #[test]
    fn foreground_sequence_is_zero_padded_truecolor() {
        let mut w = writer();
        w.push_foreground(Color::new(1, 20, 255));
        assert_eq!(w.buffer, b"\x1b[38;2;001;020;255m");
    }

    #[test]
    fn background_sequence_is_zero_padded_truecolor() {
        let mut w = writer();
        w.push_background(Color::new(0, 128, 7));
        assert_eq!(w.buffer, b"\x1b[48;2;000;128;007m");
    }

    #[test]
    fn same_foreground_twice_is_emitted_once() {
        let mut w = writer();
        w.push_foreground(Color::new(8, 8, 8));
        w.push_foreground(Color::new(8, 8, 8));
        assert_eq!(w.buffer.len(), 19);
    }

    #[test]
    fn same_background_twice_is_emitted_once() {
        let mut w = writer();
        w.push_background(Color::new(8, 8, 8));
        w.push_background(Color::new(8, 8, 8));
        assert_eq!(w.buffer.len(), 19);
    }

    #[test]
    fn changed_colour_is_emitted_again() {
        let mut w = writer();
        w.push_foreground(Color::new(1, 1, 1));
        w.push_foreground(Color::new(1, 1, 2));
        assert_eq!(w.buffer.len(), 38);
    }

    #[test]
    fn foreground_and_background_are_tracked_independently() {
        let mut w = writer();
        w.push_foreground(Color::new(5, 5, 5));
        w.push_background(Color::new(5, 5, 5));
        assert_eq!(w.buffer.len(), 38);
    }

    #[test]
    fn colour_boundaries_are_encoded_correctly() {
        let mut w = writer();
        w.push_foreground(Color::new(9, 10, 99));
        w.push_background(Color::new(100, 199, 200));
        assert_eq!(
            w.buffer,
            b"\x1b[38;2;009;010;099m\x1b[48;2;100;199;200m".as_slice()
        );
    }

    // --- state invalidation --------------------------------------------

    #[test]
    fn reset_forgets_both_colours() {
        let mut w = writer();
        w.push_foreground(Color::new(1, 2, 3));
        w.push_background(Color::new(4, 5, 6));
        w.push_reset();
        w.clear();
        w.push_foreground(Color::new(1, 2, 3));
        w.push_background(Color::new(4, 5, 6));
        assert_eq!(w.buffer.len(), 38);
    }

    #[test]
    fn reset_does_not_forget_cursor() {
        let mut w = writer();
        w.push_move(Position::new(3, 3));
        w.push_reset();
        w.clear();
        w.push_move(Position::new(3, 3));
        assert!(w.buffer.is_empty());
    }

    #[test]
    fn clear_forgets_cursor() {
        let mut w = writer();
        w.push_move(Position::new(3, 3));
        w.push_clear();
        w.clear();
        w.push_move(Position::new(3, 3));
        assert_eq!(w.buffer, b"\x1b[00004;00004H");
    }

    #[test]
    fn alternate_screen_switches_forget_cursor() {
        let mut w = writer();
        w.push_move(Position::new(1, 1));
        w.push_alternate_enter();
        w.clear();
        w.push_move(Position::new(1, 1));
        assert_eq!(w.buffer, b"\x1b[00002;00002H");

        w.clear();
        w.push_alternate_leave();
        w.clear();
        w.push_move(Position::new(1, 1));
        assert_eq!(w.buffer, b"\x1b[00002;00002H");
    }

    #[test]
    fn cursor_visibility_does_not_forget_cursor() {
        let mut w = writer();
        w.push_move(Position::new(1, 1));
        w.push_cursor_hide();
        w.push_cursor_show();
        w.clear();
        w.push_move(Position::new(1, 1));
        assert!(w.buffer.is_empty());
    }

    // --- fixed sequences ------------------------------------------------

    #[test]
    fn fixed_sequences_have_expected_bytes() {
        let mut w = writer();
        w.push_alternate_enter();
        assert_eq!(w.buffer, b"\x1b[?1049h");
        w.clear();
        w.push_alternate_leave();
        assert_eq!(w.buffer, b"\x1b[?1049l");
        w.clear();
        w.push_cursor_hide();
        assert_eq!(w.buffer, b"\x1b[?25l");
        w.clear();
        w.push_cursor_show();
        assert_eq!(w.buffer, b"\x1b[?25h");
        w.clear();
        w.push_clear();
        assert_eq!(w.buffer, b"\x1b[2J");
        w.clear();
        w.push_reset();
        assert_eq!(w.buffer, b"\x1b[0m");
    }
}
