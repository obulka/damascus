// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{Mat3, Vec3};
use strum::{Display, EnumCount, EnumIter, EnumString};
use wgpu;

use super::DualDevice;

pub mod camera;
pub mod primitive;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsStd430, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Transform {
    translation: Vec3,
    inverse_rotation: Mat3,
    uniform_scale: f32,
}

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BlendType {
    #[default]
    Union,
    Subtraction,
    Intersection,
}

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Repetition {
    #[default]
    None,
    Finite,
    Infinite,
}

pub trait Vertex<G: Copy + Clone + AsStd430<Output = S>, S>: DualDevice<G, S> {
    fn attr_array() -> [wgpu::VertexAttribute];
}
