// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::Vec2;

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

    pub fn half_size(&self) -> Vec2 {
        0.5 * self.size
    }

    pub fn top_left(&self) -> Vec2 {
        let half_size: Vec2 = self.half_size();
        Vec2::new(self.center.x - half_size.x, self.center.y + half_size.y)
    }

    pub fn contains(&self, point: Vec2) -> bool {
        let half_size: Vec2 = 0.5 * self.size;

        point.x >= self.center.x - half_size.x
            && point.x <= self.center.x + half_size.x
            && point.y >= self.center.y - half_size.y
            && point.y <= self.center.y + half_size.y
    }
}
