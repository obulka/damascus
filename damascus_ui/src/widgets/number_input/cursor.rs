//
// The following widget was adapted from iced's TextInput:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/text_input/cursor.rs
//

// Copyright 2019 Héctor Ramón, Iced contributors

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Track the cursor of a text input.
use super::value::Value;

/// The cursor of a text input.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor {
    state: State,
}

/// The state of a [`Cursor`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /// Cursor without a selection
    Index(usize),

    /// Cursor selecting a range of text
    Selection {
        /// The start of the selection
        start: usize,
        /// The end of the selection
        end: usize,
    },
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            state: State::Index(0),
        }
    }
}

impl Cursor {
    /// Returns the [`State`] of the [`Cursor`].
    pub fn state(&self, value: &Value) -> State {
        match self.state {
            State::Index(index) => State::Index(index.min(value.len())),
            State::Selection { start, end } => {
                let start = start.min(value.len());
                let end = end.min(value.len());

                if start == end {
                    State::Index(start)
                } else {
                    State::Selection { start, end }
                }
            }
        }
    }

    /// Returns the current selection of the [`Cursor`] for the given [`Value`].
    ///
    /// `start` is guaranteed to be <= than `end`.
    pub fn selection(&self, value: &Value) -> Option<(usize, usize)> {
        match self.state(value) {
            State::Selection { start, end } => Some((start.min(end), start.max(end))),
            State::Index(_) => None,
        }
    }

    pub(crate) fn move_to(&mut self, position: usize) {
        self.state = State::Index(position);
    }

    pub(crate) fn move_right(&mut self, value: &Value) {
        self.move_right_by_amount(value, 1);
    }

    pub(crate) fn move_right_by_words(&mut self, value: &Value) {
        self.move_to(value.next_end_of_word(self.right(value)));
    }

    pub(crate) fn move_right_by_amount(&mut self, value: &Value, amount: usize) {
        match self.state(value) {
            State::Index(index) => {
                self.move_to(index.saturating_add(amount).min(value.len()));
            }
            State::Selection { start, end } => self.move_to(end.max(start)),
        }
    }

    pub(crate) fn move_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => self.move_to(index - 1),
            State::Selection { start, end } => self.move_to(start.min(end)),
            State::Index(_) => self.move_to(0),
        }
    }

    pub(crate) fn move_left_by_words(&mut self, value: &Value) {
        self.move_to(value.previous_start_of_word(self.left(value)));
    }

    pub(crate) fn select_range(&mut self, start: usize, end: usize) {
        if start == end {
            self.state = State::Index(start);
        } else {
            self.state = State::Selection { start, end };
        }
    }

    pub(crate) fn select_left(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index > 0 => {
                self.select_range(index, index - 1);
            }
            State::Selection { start, end } if end > 0 => {
                self.select_range(start, end - 1);
            }
            _ => {}
        }
    }

    pub(crate) fn select_right(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) if index < value.len() => {
                self.select_range(index, index + 1);
            }
            State::Selection { start, end } if end < value.len() => {
                self.select_range(start, end + 1);
            }
            _ => {}
        }
    }

    pub(crate) fn select_left_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.previous_start_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.previous_start_of_word(end));
            }
        }
    }

    pub(crate) fn select_right_by_words(&mut self, value: &Value) {
        match self.state(value) {
            State::Index(index) => {
                self.select_range(index, value.next_end_of_word(index));
            }
            State::Selection { start, end } => {
                self.select_range(start, value.next_end_of_word(end));
            }
        }
    }

    pub(crate) fn select_all(&mut self, value: &Value) {
        self.select_range(0, value.len());
    }

    pub(crate) fn start(&self, value: &Value) -> usize {
        let start = match self.state {
            State::Index(index) => index,
            State::Selection { start, .. } => start,
        };

        start.min(value.len())
    }

    pub(crate) fn end(&self, value: &Value) -> usize {
        let end = match self.state {
            State::Index(index) => index,
            State::Selection { end, .. } => end,
        };

        end.min(value.len())
    }

    fn left(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.min(end),
        }
    }

    fn right(&self, value: &Value) -> usize {
        match self.state(value) {
            State::Index(index) => index,
            State::Selection { start, end } => start.max(end),
        }
    }
}
