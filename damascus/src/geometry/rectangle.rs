// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Square {
    pub center: Vec2,
    pub width: f32,
}

impl Square {
    pub fn center(mut self, center: Vec2) -> Self {
        self.center = center;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Rectangle {
    pub center: Vec2,
    pub size: Vec2,
}

impl Rectangle {
    pub fn center(mut self, center: Vec2) -> Self {
        self.center = center;
        self
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}
