// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Vec2, Vec3, Vec4};

use damascus::geometry::rectangle::Rectangle;

pub trait FromGlam<GlamType> {
    fn to_glam(&self) -> GlamType;
    fn from_glam(glam_type: &GlamType) -> Self;
}

impl FromGlam<Vec2> for iced::Point {
    #[inline]
    fn to_glam(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    #[inline]
    fn from_glam(glam_type: &Vec2) -> Self {
        Self::new(glam_type.x, glam_type.y)
    }
}

impl FromGlam<Vec2> for iced::Size {
    #[inline]
    fn to_glam(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    #[inline]
    fn from_glam(glam_type: &Vec2) -> Self {
        Self::new(glam_type.x, glam_type.y)
    }
}

impl FromGlam<Vec3> for iced::Color {
    #[inline]
    fn to_glam(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    #[inline]
    fn from_glam(glam_type: &Vec3) -> Self {
        Self::from_rgb(glam_type.x, glam_type.y, glam_type.z)
    }
}

impl FromGlam<Vec4> for iced::Color {
    #[inline]
    fn to_glam(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    #[inline]
    fn from_glam(glam_type: &Vec4) -> Self {
        Self::from_rgba(glam_type.x, glam_type.y, glam_type.z, glam_type.w)
    }
}

pub trait ToPath {
    fn to_path(&self) -> iced::widget::canvas::Path;
}

impl ToPath for Rectangle {
    #[inline]
    fn to_path(&self) -> iced::widget::canvas::Path {
        iced::widget::canvas::Path::rectangle(
            iced::Point::from_glam(&self.bottom_left()),
            iced::Size::from_glam(&self.size),
        )
    }
}
