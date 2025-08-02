// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{EulerRot, Mat3, Vec3, Vec4};
use strum::{Display, EnumIter, EnumString};

use crate::{
    textures::{GPUGrade, Grade},
    DualDevice,
};

#[derive(
    Debug,
    Default,
    Display,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum ProceduralTextureType {
    #[default]
    None,
    Grade,
    Checkerboard,
    FBMNoise,
    TurbulenceNoise,
    // VoronoiNoise,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUProceduralTexture {
    flags: u32,
    texture_type: u32,
    octaves: u32,
    lacunarity: f32,
    scale: Vec4,
    low_frequency_scale: Vec4,
    high_frequency_scale: Vec4,
    low_frequency_translation: Vec4,
    high_frequency_translation: Vec4,
    hue_rotation: Mat3,
    amplitude_gain: f32,
    grade: GPUGrade,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProceduralTexture {
    pub texture_type: ProceduralTextureType,
    pub scale: Vec4,
    pub grade: Grade,
    pub octaves: u32,
    pub lacunarity: f32,
    pub amplitude_gain: f32,
    pub low_frequency_scale: Vec4,
    pub high_frequency_scale: Vec4,
    pub low_frequency_translation: Vec4,
    pub high_frequency_translation: Vec4,
    pub hue_rotation_angles: Vec3,
    pub use_trap_colour: bool,
}

impl Default for ProceduralTexture {
    fn default() -> Self {
        Self {
            texture_type: ProceduralTextureType::None,
            scale: Vec4::ONE,
            grade: Grade::default(),
            octaves: 10,
            lacunarity: 2.,
            amplitude_gain: 0.75,
            low_frequency_scale: Vec4::ONE,
            high_frequency_scale: Vec4::ONE,
            low_frequency_translation: Vec4::ZERO,
            high_frequency_translation: Vec4::ZERO,
            hue_rotation_angles: Vec3::ZERO,
            use_trap_colour: false,
        }
    }
}

impl ProceduralTexture {}

impl DualDevice<GPUProceduralTexture, Std430GPUProceduralTexture> for ProceduralTexture {
    fn to_gpu(&self) -> GPUProceduralTexture {
        let radian_hue_rotation: Vec3 = self.hue_rotation_angles * std::f32::consts::PI / 180.;
        GPUProceduralTexture {
            texture_type: self.texture_type as u32,
            scale: self.scale,
            grade: self.grade.to_gpu(),
            octaves: self.octaves.max(1),
            lacunarity: self.lacunarity,
            amplitude_gain: self.amplitude_gain,
            low_frequency_scale: self.low_frequency_scale,
            high_frequency_scale: self.high_frequency_scale,
            low_frequency_translation: self.low_frequency_translation,
            high_frequency_translation: self.high_frequency_translation,
            hue_rotation: Mat3::from_euler(
                EulerRot::XYZ,
                radian_hue_rotation.x,
                radian_hue_rotation.y,
                radian_hue_rotation.z,
            ),
            flags: self.use_trap_colour as u32,
        }
    }
}
