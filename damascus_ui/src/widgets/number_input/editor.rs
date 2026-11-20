//
// The following widget was adapted from iced's TextInput:
// https://github.com/iced-rs/iced/blob/0.14.0/widget/src/text_input/editor.rs
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

use super::{cursor::Cursor, value::Value};

pub struct Editor<'a> {
    value: &'a mut Value,
    cursor: &'a mut Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(value: &'a mut Value, cursor: &'a mut Cursor) -> Editor<'a> {
        Editor { value, cursor }
    }

    pub fn contents(&self) -> String {
        self.value.to_string()
    }

    pub fn insert(&mut self, character: char) {
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        self.value.insert(self.cursor.end(self.value), character);
        self.cursor.move_right(self.value);
    }

    pub fn paste(&mut self, content: Value) {
        let length = content.len();
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        self.value.insert_many(self.cursor.end(self.value), content);

        self.cursor.move_right_by_amount(self.value, length);
    }

    pub fn backspace(&mut self) {
        match self.cursor.selection(self.value) {
            Some((start, end)) => {
                self.cursor.move_left(self.value);
                self.value.remove_many(start, end);
            }
            None => {
                let start = self.cursor.start(self.value);

                if start > 0 {
                    self.cursor.move_left(self.value);
                    self.value.remove(start - 1);
                }
            }
        }
    }

    pub fn delete(&mut self) {
        match self.cursor.selection(self.value) {
            Some(_) => {
                self.backspace();
            }
            None => {
                let end = self.cursor.end(self.value);

                if end < self.value.len() {
                    self.value.remove(end);
                }
            }
        }
    }
}
