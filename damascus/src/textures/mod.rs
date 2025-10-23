// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::f32::consts::PI;

use crevice::std430::{self, AsStd430};
use glam::{EulerRot, Mat3, UVec2, Vec3, Vec4};
use slotmap::SlotMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{DualDevice, Enumerator};

pub mod generators;
pub mod processors;

slotmap::new_key_type! { pub struct TextureId; }

#[derive(
    Debug,
    Default,
    Display,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum AOVs {
    #[default]
    Beauty,
    WorldPosition,
    LocalPosition,
    Normals,
    Depth,
    Cryptomatte,
    Stats,
}

impl Enumerator for AOVs {}

#[derive(
    Debug,
    Default,
    Display,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Texture {
    #[default]
    None,
    Grade,
    Checkerboard,
    Noise,
    Sampled,
}

impl Enumerator for Texture {}

impl DualDevice<UVec2, std430::UVec2> for Texture {
    fn to_gpu(&self) -> UVec2 {
        UVec2::new(*self as u32, 0)
    }
}

pub type Textures = SlotMap<TextureId, Texture>;
