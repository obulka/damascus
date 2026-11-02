// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Vec2, Vec4};

use damascus::geometry::rectangle::Rectangle;

use crate::{type_conversion::ToPath, widgets::pan_zoom::PanZoom};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeContext {
    pub label: String,
    pub shape: Rectangle,
    pub colour: Vec4,
}

impl NodeContext {
    pub fn new(label: String) -> Self {
        Self {
            label: label,
            shape: Rectangle::default().size(Vec2::new(100.0, 33.0)),
            ..Default::default()
        }
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.shape.center = position;
        self
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

    pub fn local_to_canvas_path(&self, pan_zoom: &PanZoom) -> iced::widget::canvas::Path {
        pan_zoom.local_to_canvas_rectangle(&self.shape).to_path()
    }

    pub fn local_path(&self) -> iced::widget::canvas::Path {
        self.shape.to_path()
    }

    pub fn input_position(&self, input_index: f32, input_count: f32) -> Vec2 {
        let node_top_left: Vec2 = self.shape.top_left();

        let step_size: f32 = self.shape.size.x / input_count;
        let half_step_size: f32 = 0.5 * step_size;

        Vec2::new(
            node_top_left.x + input_index * step_size + half_step_size,
            node_top_left.y,
        )
    }

    pub fn output_position(&self, output_index: f32, output_count: f32) -> Vec2 {
        let node_bottom_left: Vec2 = self.shape.bottom_left();

        let step_size: f32 = self.shape.size.x / output_count;
        let half_step_size: f32 = 0.5 * step_size;

        Vec2::new(
            node_bottom_left.x + output_index * step_size + half_step_size,
            node_bottom_left.y,
        )
    }
}
