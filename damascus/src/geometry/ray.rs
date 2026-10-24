// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{USizeVec2, Vec2};

use super::rectangle::Rectangle;

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ray2d {
    origin: Vec2,
    direction: Vec2,
    reciprocal_direction: Vec2,
    sign: USizeVec2,
}

impl Ray2d {
    pub fn new(origin: Vec2, mut direction: Vec2) -> Self {
        direction = direction.normalize();
        let reciprocal_direction: Vec2 = direction.recip();
        Self {
            origin,
            direction,
            reciprocal_direction,
            sign: USizeVec2::new(
                (reciprocal_direction.x < 0.) as usize,
                (reciprocal_direction.y < 0.) as usize,
            ),
        }
    }

    pub fn origin(&self) -> &Vec2 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec2 {
        &self.direction
    }

    pub fn set_direction(&mut self, new_direction: Vec2) {
        if let Some(direction) = new_direction.try_normalize() {
            self.direction = direction;
            self.reciprocal_direction = self.direction.recip();
            self.sign.x = (self.reciprocal_direction.x < 0.) as usize;
            self.sign.y = (self.reciprocal_direction.y < 0.) as usize;
        }
    }

    pub fn advance(&mut self, distance: f32) {
        self.origin = self.position_at_distance(distance);
    }

    pub fn advance_at_angle(&mut self, distance: f32, angle: f32) {
        self.origin += distance * self.direction.rotate_angle(angle);
    }

    pub fn advance_in_direction(&mut self, distance: f32, direction: Vec2) {
        self.origin += distance * direction;
    }

    pub fn perpendicular_direction_ccw(&self) -> Vec2 {
        Vec2::new(-self.direction.y, self.direction.x)
    }

    pub fn perpendicular_direction_cw(&self) -> Vec2 {
        Vec2::new(self.direction.y, -self.direction.x)
    }

    pub fn position_at_distance(&self, distance: f32) -> Vec2 {
        self.origin + distance * self.direction
    }

    pub fn intersect_rectangle(&self, rectangle: &Rectangle) -> Option<Vec2> {
        let bounds: &[Vec2; 2] = &[rectangle.bottom_left(), rectangle.top_right()];

        let mut tmin: f32 = (bounds[self.sign.x].x - self.origin.x) * self.reciprocal_direction.x;
        let mut tmax: f32 =
            (bounds[1 - self.sign.x].x - self.origin.x) * self.reciprocal_direction.x;
        let tymin: f32 = (bounds[self.sign.y].y - self.origin.y) * self.reciprocal_direction.y;
        let tymax: f32 = (bounds[1 - self.sign.y].y - self.origin.y) * self.reciprocal_direction.y;

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        return Some(Vec2::new(tmin, tmax));
    }

    pub fn rotate(&mut self, angle: f32) {
        self.set_direction(self.direction.rotate_angle(angle));
    }
}
