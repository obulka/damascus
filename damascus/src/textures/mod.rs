// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::f32::consts::PI;

use crevice::std430::{self, AsStd430};
use glam::{EulerRot, Mat3, UVec2, Vec3, Vec4};
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
pub enum TextureType {
    #[default]
    None,
    Grade,
    Checkerboard,
    Noise,
    Sampled,
}

impl Enumerator for TextureType {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUTexture {
    flags: u32,
    hue_rotation: Mat3,
    texture_type: u32,
    texture_index: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Texture {
    pub texture_type: TextureType,
    pub hue_rotation_angles: Vec3,
    pub use_trap_colour: bool,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture_type: TextureType::None,
            hue_rotation_angles: Vec3::ZERO,
            use_trap_colour: false,
        }
    }
}

impl Texture {}

impl DualDevice<UVec2, std430::UVec2> for Texture {
    fn to_gpu(&self) -> UVec2 {
        // let radian_hue_rotation: Vec3 = self.hue_rotation_angles * PI / 180.;
        // GPUTexture {
        //     flags: self.use_trap_colour as u32,
        //     texture_type: self.texture_type as u32,
        //     hue_rotation: Mat3::from_euler(
        //         EulerRot::XYZ,
        //         radian_hue_rotation.x,
        //         radian_hue_rotation.y,
        //         radian_hue_rotation.z,
        //     ),
        // }
        UVec2::new(self.texture_type as u32, 0)
    }
}
