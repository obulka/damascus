// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt::Debug;

use glam;
use num_traits::{AsPrimitive, FromPrimitive};

use super::{ExpressionData, Float};

pub trait Vec4<T>: Default
where
    T: FromPrimitive + Copy + Debug + 'static,
    f64: AsPrimitive<T> + From<T>,
{
    fn new(x: T, y: T, z: T, w: T) -> Self;

    fn x(&self) -> &impl ExpressionData<T>;
    fn y(&self) -> &impl ExpressionData<T>;
    fn z(&self) -> &impl ExpressionData<T>;
    fn w(&self) -> &impl ExpressionData<T>;

    fn x_mut(&mut self) -> &mut impl ExpressionData<T>;
    fn y_mut(&mut self) -> &mut impl ExpressionData<T>;
    fn z_mut(&mut self) -> &mut impl ExpressionData<T>;
    fn w_mut(&mut self) -> &mut impl ExpressionData<T>;
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vec4f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl From<glam::Vec4> for Vec4f {
    fn from(value: glam::Vec4) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}

impl From<Vec4f> for glam::Vec4 {
    fn from(value: Vec4f) -> Self {
        glam::Vec4::new(
            value.x.value(),
            value.y.value(),
            value.z.value(),
            value.w.value(),
        )
    }
}

impl Vec4<f32> for Vec4f {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    fn x(&self) -> &impl ExpressionData<f32> {
        &self.x
    }

    fn y(&self) -> &impl ExpressionData<f32> {
        &self.y
    }

    fn z(&self) -> &impl ExpressionData<f32> {
        &self.z
    }

    fn w(&self) -> &impl ExpressionData<f32> {
        &self.w
    }

    fn x_mut(&mut self) -> &mut impl ExpressionData<f32> {
        &mut self.x
    }

    fn y_mut(&mut self) -> &mut impl ExpressionData<f32> {
        &mut self.y
    }

    fn z_mut(&mut self) -> &mut impl ExpressionData<f32> {
        &mut self.z
    }

    fn w_mut(&mut self) -> &mut impl ExpressionData<f32> {
        &mut self.w
    }
}
