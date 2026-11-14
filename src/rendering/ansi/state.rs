use crate::common::{color::Color, position::Position, size::Size};

pub struct State {
    cursor: Option<Position>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl State {
    #[inline]
    pub const fn new() -> Self {
        Self {
            cursor: None,
            foreground: None,
            background: None,
        }
    }

    #[inline]
    pub fn ensure_cursor(&mut self, position: Position, emit: impl FnOnce(Position)) {
        Self::emit_if_changed(&mut self.cursor, position, emit);
    }

    #[inline]
    pub fn ensure_foreground(&mut self, color: Color, emit: impl FnOnce(Color)) {
        Self::emit_if_changed(&mut self.foreground, color, emit);
    }

    #[inline]
    pub fn ensure_background(&mut self, color: Color, emit: impl FnOnce(Color)) {
        Self::emit_if_changed(&mut self.background, color, emit);
    }

    #[inline]
    pub fn advance_cursor(&mut self, size: Size) {
        self.cursor = self.cursor.map(|position| {
            let max_position = size.last();
            if position.x < max_position.x {
                position.add_x(1)
            } else if position.y < max_position.y {
                position.add_y(1).with_x(0)
            } else {
                max_position
            }
        });
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
    fn emit_if_changed<T: Copy + PartialEq>(slot: &mut Option<T>, value: T, emit: impl FnOnce(T)) {
        if *slot == Some(value) {
            return;
        }

        emit(value);
        *slot = Some(value);
    }
}

impl Default for State {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- cursor ---------------------------------------------------------

    #[test]
    fn first_cursor_is_always_emitted() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(0, 0), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(0, 0)]);
    }

    #[test]
    fn repeated_cursor_is_skipped() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(4, 2), |position| emitted.push(position));
        state.ensure_cursor(Position::new(4, 2), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(4, 2)]);
    }

    #[test]
    fn changed_cursor_is_emitted() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(4, 2), |position| emitted.push(position));
        state.ensure_cursor(Position::new(5, 2), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(4, 2), Position::new(5, 2)]);
    }

    #[test]
    fn forgotten_cursor_is_emitted_again() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(2, 2), |position| emitted.push(position));
        state.forget_cursor();
        state.ensure_cursor(Position::new(2, 2), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(2, 2), Position::new(2, 2)]);
    }

    // --- cursor advancement ---------------------------------------------

    fn size() -> Size {
        Size::try_new(80, 40).unwrap()
    }

    #[test]
    fn advance_matches_the_next_cell() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(0, 0), |position| emitted.push(position));
        state.advance_cursor(size());
        state.ensure_cursor(Position::new(1, 0), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(0, 0)]);
    }

    #[test]
    fn advance_wraps_to_the_next_row() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(79, 0), |position| emitted.push(position));
        state.advance_cursor(size());
        state.ensure_cursor(Position::new(0, 1), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(79, 0)]);
    }

    #[test]
    fn advance_clamps_at_the_last_cell() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_cursor(Position::new(79, 39), |position| emitted.push(position));
        state.advance_cursor(size());
        state.ensure_cursor(Position::new(79, 39), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(79, 39)]);
    }

    #[test]
    fn advancement_with_unknown_cursor_keeps_it_unknown() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.advance_cursor(size());
        state.ensure_cursor(Position::new(0, 0), |position| emitted.push(position));

        assert_eq!(emitted, [Position::new(0, 0)]);
    }

    // --- colours --------------------------------------------------------

    #[test]
    fn repeated_foreground_is_skipped() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_foreground(Color::new(8, 8, 8), |color| emitted.push(color));
        state.ensure_foreground(Color::new(8, 8, 8), |color| emitted.push(color));

        assert_eq!(emitted, [Color::new(8, 8, 8)]);
    }

    #[test]
    fn changed_foreground_is_emitted() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_foreground(Color::new(1, 1, 1), |color| emitted.push(color));
        state.ensure_foreground(Color::new(1, 1, 2), |color| emitted.push(color));

        assert_eq!(emitted, [Color::new(1, 1, 1), Color::new(1, 1, 2)]);
    }

    #[test]
    fn forgotten_foreground_is_emitted_again() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_foreground(Color::new(1, 2, 3), |color| emitted.push(color));
        state.forget_foreground();
        state.ensure_foreground(Color::new(1, 2, 3), |color| emitted.push(color));

        assert_eq!(emitted, [Color::new(1, 2, 3), Color::new(1, 2, 3)]);
    }

    #[test]
    fn background_is_tracked_independently() {
        let mut state = State::new();
        let mut emitted = Vec::new();

        state.ensure_background(Color::new(4, 5, 6), |color| emitted.push(color));
        state.ensure_foreground(Color::new(4, 5, 6), |color| emitted.push(color));

        assert_eq!(emitted, [Color::new(4, 5, 6), Color::new(4, 5, 6)]);
    }
}
