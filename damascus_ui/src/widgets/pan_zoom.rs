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
pub enum PanZoomMessage {
    Zoom(f32),
    BeginPan,
    EndPan,
    MoveCursor(Vec2),
    Resize(Rectangle),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PanZoom {
    pub panning: bool,
    pub zoom: f32,
    pub pan: Vec2,
    pub global_cursor_position: Vec2,
    pub local_cursor_position: Vec2,
    pub bounds: Rectangle,
}

impl Default for PanZoom {
    fn default() -> Self {
        Self {
            panning: false,
            zoom: 1.0,
            pan: Vec2::ZERO,
            global_cursor_position: Vec2::ZERO,
            local_cursor_position: Vec2::ZERO,
            bounds: Rectangle::default(),
        }
    }
}

impl PanZoom {
    pub fn global_to_local(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x - self.bounds.center.x,
            self.bounds.center.y - position.y,
        )
    }

    pub fn local_to_global(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x + self.bounds.center.x,
            self.bounds.center.y - position.y,
        )
    }

    pub fn local_to_canvas(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x + 0.5 * self.bounds.size.x,
            0.5 * self.bounds.size.y - position.y,
        )
    }

    pub fn canvas_to_local(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x - 0.5 * self.bounds.size.x,
            0.5 * self.bounds.size.y - position.y,
        )
    }
}

impl Widget<PanZoomMessage> for PanZoom {
    fn update(
        &mut self,
        _context: &mut Context,
        message: PanZoomMessage,
    ) -> iced::Task<PanZoomMessage> {
        match message {
            PanZoomMessage::Zoom(zoom) => {
                let cursor_position_before: Vec2 =
                    self.local_cursor_position * self.zoom - self.pan;

                self.zoom /= zoom.exp();

                let cursor_position: Vec2 = self.local_cursor_position * self.zoom - self.pan;

                self.pan += cursor_position - cursor_position_before;
            }
            PanZoomMessage::BeginPan => {
                if self.bounds.contains(self.global_cursor_position) {
                    self.panning = true;
                }
            }
            PanZoomMessage::EndPan => self.panning = false,
            PanZoomMessage::MoveCursor(global_cursor_position) => {
                self.global_cursor_position = global_cursor_position;

                let local_cursor_position: Vec2 = self.global_to_local(global_cursor_position);

                if self.panning {
                    let drag_delta: Vec2 =
                        (local_cursor_position - self.local_cursor_position) * self.zoom;
                    self.pan += drag_delta;
                }

                self.local_cursor_position = local_cursor_position;
            }
            PanZoomMessage::Resize(bounds) => self.bounds = bounds,
        }
        iced::Task::none()
    }
}
