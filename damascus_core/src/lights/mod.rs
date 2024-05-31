// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::Vec3;
use strum::{Display, EnumIter, EnumString};

#[derive(
    Debug, Default, Display, Copy, Clone, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
)]
pub enum Lights {
    #[default]
    Directional,
    Point,
    Ambient,
    AmbientOcclusion,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPULight {
    light_type: u32,
    dimensional_data: Vec3,
    intensity: f32,
    falloff: u32,
    colour: Vec3,
    shadow_hardness: f32,
    soften_shadows: u32,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub light_type: Lights,
    pub dimensional_data: Vec3,
    pub intensity: f32,
    pub falloff: u32,
    pub colour: Vec3,
    pub shadow_hardness: f32,
    pub soften_shadows: bool,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            light_type: Lights::Directional,
            dimensional_data: Vec3::new(0., -1., 0.),
            intensity: 1.,
            falloff: 2,
            colour: Vec3::ONE,
            shadow_hardness: 1.,
            soften_shadows: false,
        }
    }
}

impl Light {
    pub fn to_gpu(&self) -> GPULight {
        GPULight {
            light_type: self.light_type as u32,
            dimensional_data: self.dimensional_data,
            intensity: self.intensity,
            falloff: self.falloff,
            colour: self.colour,
            shadow_hardness: self.shadow_hardness,
            soften_shadows: self.soften_shadows as u32,
        }
    }
}
