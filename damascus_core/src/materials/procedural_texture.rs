// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::Vec4;
use strum::{Display, EnumIter, EnumString};

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
    texture_type: u32,
    scale: Vec4,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    octaves: u32,
    lacunarity: f32,
    amplitude_gain: f32,
    gamma: f32,
    low_frequency_scale: Vec4,
    high_frequency_scale: Vec4,
    low_frequency_translation: Vec4,
    high_frequency_translation: Vec4,
    flags: u32,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProceduralTexture {
    pub texture_type: ProceduralTextureType,
    pub scale: Vec4,
    pub black_point: f32,
    pub white_point: f32,
    pub lift: f32,
    pub gain: f32,
    pub octaves: u32,
    pub lacunarity: f32,
    pub amplitude_gain: f32,
    pub gamma: f32,
    pub low_frequency_scale: Vec4,
    pub high_frequency_scale: Vec4,
    pub low_frequency_translation: Vec4,
    pub high_frequency_translation: Vec4,
    pub invert: bool,
}

impl Default for ProceduralTexture {
    fn default() -> Self {
        Self {
            texture_type: ProceduralTextureType::None,
            scale: Vec4::ONE,
            black_point: 0.,
            white_point: 1.,
            lift: 0.,
            gain: 1.,
            octaves: 10,
            lacunarity: 2.,
            amplitude_gain: 0.75,
            gamma: 1.,
            low_frequency_scale: Vec4::ONE,
            high_frequency_scale: Vec4::ONE,
            low_frequency_translation: Vec4::ZERO,
            high_frequency_translation: Vec4::ZERO,
            invert: false,
        }
    }
}

impl ProceduralTexture {
    pub fn to_gpu(&self) -> GPUProceduralTexture {
        GPUProceduralTexture {
            texture_type: self.texture_type as u32,
            scale: self.scale,
            black_point: self.black_point,
            white_point: self.white_point,
            lift: self.lift,
            gain: self.gain,
            octaves: self.octaves.max(1),
            lacunarity: self.lacunarity,
            amplitude_gain: self.amplitude_gain,
            gamma: self.gamma,
            low_frequency_scale: self.low_frequency_scale,
            high_frequency_scale: self.high_frequency_scale,
            low_frequency_translation: self.low_frequency_translation,
            high_frequency_translation: self.high_frequency_translation,
            flags: self.invert as u32,
        }
    }
}
