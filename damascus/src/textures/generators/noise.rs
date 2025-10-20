// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{EulerRot, Mat3, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    DualDevice, Enumerator,
};

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
pub enum NoiseType {
    #[default]
    None,
    FBMNoise,
    TurbulenceNoise,
    // VoronoiNoise,
}

impl Enumerator for NoiseType {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUNoise {
    flags: u32,
    noise_type: u32,
    octaves: u32,
    lacunarity: f32,
    scale: Vec4,
    low_frequency_scale: Vec4,
    high_frequency_scale: Vec4,
    low_frequency_translation: Vec4,
    high_frequency_translation: Vec4,
    amplitude_gain: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Noise {
    pub noise_type: NoiseType,
    pub octaves: u32,
    pub lacunarity: f32,
    pub amplitude_gain: f32,
    pub scale: Vec4,
    pub low_frequency_scale: Vec4,
    pub high_frequency_scale: Vec4,
    pub low_frequency_translation: Vec4,
    pub high_frequency_translation: Vec4,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            noise_type: NoiseType::None,
            octaves: 10,
            lacunarity: 2.,
            amplitude_gain: 0.75,
            scale: Vec4::ONE,
            low_frequency_scale: Vec4::ONE,
            high_frequency_scale: Vec4::ONE,
            low_frequency_translation: Vec4::ZERO,
            high_frequency_translation: Vec4::ZERO,
        }
    }
}

impl Noise {}

impl DualDevice<GPUNoise, Std430GPUNoise> for Noise {
    fn to_gpu(&self) -> GPUNoise {
        GPUNoise {
            flags: 0,
            noise_type: self.noise_type as u32,
            octaves: self.octaves.max(1),
            lacunarity: self.lacunarity,
            amplitude_gain: self.amplitude_gain,
            scale: self.scale,
            low_frequency_scale: self.low_frequency_scale,
            high_frequency_scale: self.high_frequency_scale,
            low_frequency_translation: self.low_frequency_translation,
            high_frequency_translation: self.high_frequency_translation,
        }
    }
}
