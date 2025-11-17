// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{Mat3, Vec3};
use macro_rules_attribute::derive;

use crate::EnumHashTraits;

pub mod primitives;
pub mod vertex;

#[repr(C)]
#[derive(
    Debug, Default, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize,
)]
#[serde(default)]
pub struct Transform {
    pub translation: Vec3,
    pub uniform_scale: f32,
    pub inverse_rotation: Mat3,
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum BlendType {
    #[default]
    Union,
    Subtraction,
    Intersection,
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum Repetition {
    #[default]
    None,
    Finite,
    Infinite,
}
