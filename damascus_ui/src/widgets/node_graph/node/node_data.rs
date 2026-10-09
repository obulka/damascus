// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Vec2, Vec4};

use damascus::geometry::rectangle::Rectangle;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeData {
    pub label: String,
    pub shape: Rectangle,
    pub colour: Vec4,
}

impl NodeData {
    pub fn new(label: String) -> Self {
        Self {
            label: label,
            ..Default::default()
        }
    }

    pub fn shape(mut self, shape: Rectangle) -> Self {
        self.shape = shape;
        self
    }

    pub fn colour(mut self, colour: Vec4) -> Self {
        self.colour = colour;
        self
    }

    pub fn top_left(&self) -> Vec2 {
        self.shape.top_left()
    }

    pub fn canvas_space_center(&self, bounds: iced::Rectangle) -> iced::Point {
        iced::Point::new(
            self.shape.center.x + 0.5 * bounds.width,
            0.5 * bounds.height - self.shape.center.y,
        )
    }

    pub fn canvas_space_top_left(&self, bounds: iced::Rectangle) -> iced::Point {
        iced::Point::new(
            self.shape.center.x + 0.5 * (bounds.width - self.shape.size.x),
            0.5 * (bounds.height - self.shape.size.y) - self.shape.center.y,
        )
    }
}
