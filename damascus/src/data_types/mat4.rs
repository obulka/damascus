// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt::Debug;

use glam;
use num_traits::{AsPrimitive, FromPrimitive};

use super::{Vec4, Vec4f};

pub trait Mat4<T>: Default
where
    T: FromPrimitive + Copy + Debug + 'static,
    f64: AsPrimitive<T> + From<T>,
{
    fn new(
        m00: T,
        m01: T,
        m02: T,
        m03: T,
        m10: T,
        m11: T,
        m12: T,
        m13: T,
        m20: T,
        m21: T,
        m22: T,
        m23: T,
        m30: T,
        m31: T,
        m32: T,
        m33: T,
    ) -> Self;

    fn x_axis(&self) -> &impl Vec4<T>;
    fn y_axis(&self) -> &impl Vec4<T>;
    fn z_axis(&self) -> &impl Vec4<T>;
    fn w_axis(&self) -> &impl Vec4<T>;

    fn x_axis_mut(&mut self) -> &mut impl Vec4<T>;
    fn y_axis_mut(&mut self) -> &mut impl Vec4<T>;
    fn z_axis_mut(&mut self) -> &mut impl Vec4<T>;
    fn w_axis_mut(&mut self) -> &mut impl Vec4<T>;

    fn value(&self, column: usize, row: usize) -> T {
        match column {
            0 => self.x_axis().value(row),
            1 => self.y_axis().value(row),
            2 => self.z_axis().value(row),
            3 => self.w_axis().value(row),
            _ => panic!("index out of bounds"),
        }
    }

    fn set_expression(&mut self, column: usize, row: usize, expression: String) {
        match column {
            0 => self.x_axis_mut().set_expression(row, expression),
            1 => self.y_axis_mut().set_expression(row, expression),
            2 => self.z_axis_mut().set_expression(row, expression),
            3 => self.w_axis_mut().set_expression(row, expression),
            _ => {}
        }
    }

    fn with_expression(mut self, column: usize, row: usize, expression: String) -> Self {
        self.set_expression(column, row, expression);
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mat4f {
    pub x_axis: Vec4f,
    pub y_axis: Vec4f,
    pub z_axis: Vec4f,
    pub w_axis: Vec4f,
}

impl From<glam::Mat4> for Mat4f {
    fn from(value: glam::Mat4) -> Self {
        Self::new(
            value.x_axis.x,
            value.x_axis.y,
            value.x_axis.z,
            value.x_axis.w,
            value.y_axis.x,
            value.y_axis.y,
            value.y_axis.z,
            value.y_axis.w,
            value.z_axis.x,
            value.z_axis.y,
            value.z_axis.z,
            value.z_axis.w,
            value.w_axis.x,
            value.w_axis.y,
            value.w_axis.z,
            value.w_axis.w,
        )
    }
}

impl From<Mat4f> for glam::Mat4 {
    fn from(value: Mat4f) -> Self {
        glam::Mat4::from_cols(
            value.x_axis.into(),
            value.y_axis.into(),
            value.z_axis.into(),
            value.w_axis.into(),
        )
    }
}

impl Mat4<f32> for Mat4f {
    fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Self {
        Self {
            x_axis: Vec4f::new(m00, m01, m02, m03),
            y_axis: Vec4f::new(m10, m11, m12, m13),
            z_axis: Vec4f::new(m20, m21, m22, m23),
            w_axis: Vec4f::new(m30, m31, m32, m33),
        }
    }

    fn x_axis(&self) -> &impl Vec4<f32> {
        &self.x_axis
    }

    fn y_axis(&self) -> &impl Vec4<f32> {
        &self.y_axis
    }

    fn z_axis(&self) -> &impl Vec4<f32> {
        &self.z_axis
    }

    fn w_axis(&self) -> &impl Vec4<f32> {
        &self.w_axis
    }

    fn x_axis_mut(&mut self) -> &mut impl Vec4<f32> {
        &mut self.x_axis
    }

    fn y_axis_mut(&mut self) -> &mut impl Vec4<f32> {
        &mut self.y_axis
    }

    fn z_axis_mut(&mut self) -> &mut impl Vec4<f32> {
        &mut self.z_axis
    }

    fn w_axis_mut(&mut self) -> &mut impl Vec4<f32> {
        &mut self.w_axis
    }
}
