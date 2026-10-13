// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::Vec2;
use iced;

use damascus::geometry::rectangle::Rectangle;

use crate::app::Context;

use super::Widget;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SelectionBoxMessage {
    BeginSelection(Vec2),
    EndSelection,
    MoveCursor(Vec2),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SelectionBox {
    pub selecting: bool,
    pub initial_cursor_position: Vec2,
    pub current_cursor_position: Vec2,
}

impl Default for SelectionBox {
    fn default() -> Self {
        Self {
            selecting: false,
            initial_cursor_position: Vec2::ZERO,
            current_cursor_position: Vec2::ZERO,
        }
    }
}

impl SelectionBox {
    pub fn bounds(&self) -> Rectangle {
        Rectangle::from_corners(self.initial_cursor_position, self.current_cursor_position)
    }

    pub fn contains(&self, point: Vec2) -> bool {
        if self.selecting {
            self.bounds().contains(point)
        } else {
            false
        }
    }
}

impl Widget<SelectionBoxMessage> for SelectionBox {
    fn update(
        &mut self,
        _context: &mut Context,
        message: SelectionBoxMessage,
    ) -> iced::Task<SelectionBoxMessage> {
        match message {
            SelectionBoxMessage::BeginSelection(cursor_position) => {
                self.selecting = true;
                self.initial_cursor_position = cursor_position;
                self.current_cursor_position = cursor_position;
            }
            SelectionBoxMessage::EndSelection => self.selecting = false,
            SelectionBoxMessage::MoveCursor(cursor_position) => {
                self.current_cursor_position = cursor_position;
            }
        }
        iced::Task::none()
    }
}
