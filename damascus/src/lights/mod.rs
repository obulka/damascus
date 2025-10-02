// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::Vec3;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{DualDevice, Enumerator};

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
    serde::Serialize,
    serde::Deserialize,
)]
pub enum LightType {
    Directional,
    Point,
    Ambient,
    #[default]
    AmbientOcclusion,
}

impl Enumerator for LightType {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPULight {
    light_type: u32,
    falloff: u32,
    soften_shadows: u32,
    dimensional_data: Vec3,
    intensity: f32,
    colour: Vec3,
    shadow_hardness: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Light {
    pub light_type: LightType,
    pub dimensional_data: Vec3,
    pub intensity: f32,
    pub falloff: u32,
    pub colour: Vec3,
    pub shadow_hardness: f32,
    pub soften_shadows: bool,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightType::default(),
            dimensional_data: Vec3::X,
            intensity: 1.,
            falloff: 2,
            colour: Vec3::ONE,
            shadow_hardness: 1.,
            soften_shadows: false,
        }
    }
}

impl Light {}

impl DualDevice<GPULight, Std430GPULight> for Light {
    fn to_gpu(&self) -> GPULight {
        GPULight {
            light_type: self.light_type as u32,
            falloff: self.falloff,
            soften_shadows: self.soften_shadows as u32,
            dimensional_data: self.dimensional_data,
            intensity: self.intensity,
            colour: self.colour,
            shadow_hardness: self.shadow_hardness,
        }
    }
}
