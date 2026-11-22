//
// The following widget was adapted from iced's TextInput:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/text_input/value.rs
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

use meval;
use unicode_segmentation::UnicodeSegmentation;

/// The value of a [`TextInput`].
///
/// [`TextInput`]: super::TextInput
// TODO: Reduce allocations, cache results (?)
#[derive(Debug, Clone)]
pub struct Value {
    graphemes: Vec<String>,
    evaluation: f64,
}

impl Value {
    /// Creates a new [`Value`] from a string slice.
    pub fn new(value: f64) -> Self {
        Self {
            graphemes: format!("{:}", value)
                .graphemes(true)
                .map(String::from)
                .collect(),
            evaluation: value,
        }
    }

    /// Creates a new [`Value`] from a string slice.
    pub fn from_expression(string: &str, default_value: f64) -> Self {
        let graphemes = UnicodeSegmentation::graphemes(string, true)
            .map(String::from)
            .collect();
        match string.parse::<meval::Expr>() {
            Ok(expression) => match expression.eval() {
                Ok(value) => Self {
                    graphemes: graphemes,
                    evaluation: value,
                },
                Err(_error) => Self {
                    graphemes: graphemes,
                    evaluation: default_value,
                },
            },
            Err(_error) => Self {
                graphemes: graphemes,
                evaluation: default_value,
            },
        }
    }

    pub fn evaluation(&self) -> f64 {
        self.evaluation
    }

    pub fn reevaluate(&mut self) {
        match self.to_string().parse::<meval::Expr>() {
            Ok(expression) => match expression.eval() {
                Ok(value) => {
                    self.evaluation = value;
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Returns whether the [`Value`] is empty or not.
    ///
    /// A [`Value`] is empty when it contains no graphemes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total amount of graphemes in the [`Value`].
    pub fn len(&self) -> usize {
        self.graphemes.len()
    }

    /// Returns the position of the previous start of a word from the given
    /// grapheme `index`.
    pub fn previous_start_of_word(&self, index: usize) -> usize {
        let previous_string = &self.graphemes[..index.min(self.graphemes.len())].concat();

        UnicodeSegmentation::split_word_bound_indices(previous_string as &str)
            .filter(|(_, word)| !word.trim_start().is_empty())
            .next_back()
            .map(|(i, previous_word)| {
                index
                    - UnicodeSegmentation::graphemes(previous_word, true).count()
                    - UnicodeSegmentation::graphemes(
                        &previous_string[i + previous_word.len()..] as &str,
                        true,
                    )
                    .count()
            })
            .unwrap_or(0)
    }

    /// Returns the position of the next end of a word from the given grapheme
    /// `index`.
    pub fn next_end_of_word(&self, index: usize) -> usize {
        let next_string = &self.graphemes[index..].concat();

        UnicodeSegmentation::split_word_bound_indices(next_string as &str)
            .find(|(_, word)| !word.trim_start().is_empty())
            .map(|(i, next_word)| {
                index
                    + UnicodeSegmentation::graphemes(next_word, true).count()
                    + UnicodeSegmentation::graphemes(&next_string[..i] as &str, true).count()
            })
            .unwrap_or(self.len())
    }

    /// Returns a new [`Value`] containing the graphemes from `start` until the
    /// given `end`.
    pub fn select(&self, start: usize, end: usize) -> Self {
        let graphemes = self.graphemes[start.min(self.len())..end.min(self.len())].to_vec();

        Self {
            graphemes,
            evaluation: self.evaluation,
        }
    }

    /// Returns a new [`Value`] containing the graphemes until the given
    /// `index`.
    pub fn until(&self, index: usize) -> Self {
        let graphemes = self.graphemes[..index.min(self.len())].to_vec();

        Self {
            graphemes,
            evaluation: self.evaluation,
        }
    }

    /// Inserts a new `char` at the given grapheme `index`.
    pub fn insert(&mut self, index: usize, c: char) {
        self.graphemes.insert(index, c.to_string());

        self.graphemes = UnicodeSegmentation::graphemes(&self.to_string() as &str, true)
            .map(String::from)
            .collect();

        self.reevaluate();
    }

    /// Inserts a bunch of graphemes at the given grapheme `index`.
    pub fn insert_many(&mut self, index: usize, mut value: Value) {
        let _ = self
            .graphemes
            .splice(index..index, value.graphemes.drain(..));

        self.reevaluate();
    }

    /// Removes the grapheme at the given `index`.
    pub fn remove(&mut self, index: usize) {
        let _ = self.graphemes.remove(index);

        self.reevaluate();
    }

    /// Removes the graphemes from `start` to `end`.
    pub fn remove_many(&mut self, start: usize, end: usize) {
        let _ = self.graphemes.splice(start..end, std::iter::empty());

        self.reevaluate();
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.graphemes.concat())
    }
}
