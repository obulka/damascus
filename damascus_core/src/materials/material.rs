// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::Vec3;

use super::{GPUProceduralTexture, ProceduralTexture};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUMaterial {
    diffuse_colour: Vec3,
    diffuse_texture: GPUProceduralTexture,
    specular_probability: f32,
    specular_roughness: f32,
    specular_colour: Vec3,
    transmissive_probability: f32,
    transmissive_roughness: f32,
    extinction_colour: Vec3,
    emissive_probability: f32,
    emissive_colour: Vec3,
    refractive_index: f32,
    scattering_colour: Vec3,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub diffuse_colour: Vec3,
    pub diffuse_texture: ProceduralTexture,
    pub specular_probability: f32,
    pub specular_roughness: f32,
    pub specular_colour: Vec3,
    pub transmissive_probability: f32,
    pub transmissive_roughness: f32,
    pub extinction_coefficient: f32,
    pub transmissive_colour: Vec3,
    pub emissive_probability: f32,
    pub emissive_colour: Vec3,
    pub refractive_index: f32,
    pub scattering_coefficient: f32,
    pub scattering_colour: Vec3,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: Vec3::ONE,
            diffuse_texture: ProceduralTexture::default(),
            specular_probability: 0.,
            specular_roughness: 0.,
            specular_colour: Vec3::ONE,
            transmissive_probability: 0.,
            transmissive_roughness: 0.,
            transmissive_colour: Vec3::ONE,
            extinction_coefficient: 0.,
            emissive_probability: 0.,
            emissive_colour: Vec3::new(1., 0.8, 0.5),
            refractive_index: 1.3,
            scattering_coefficient: 0.,
            scattering_colour: Vec3::ONE,
        }
    }
}

impl Material {
    pub fn to_gpu(&self) -> GPUMaterial {
        GPUMaterial {
            diffuse_colour: self.diffuse_colour,
            diffuse_texture: self.diffuse_texture.to_gpu(),
            specular_probability: self.specular_probability,
            specular_roughness: self.specular_roughness,
            specular_colour: self.specular_colour,
            transmissive_probability: self
                .transmissive_probability
                .min(1. - self.specular_probability),
            transmissive_roughness: self.transmissive_roughness,
            extinction_colour: (1. - self.transmissive_colour.clamp(Vec3::ZERO, Vec3::ONE))
                * self.extinction_coefficient,
            emissive_probability: self.emissive_probability,
            emissive_colour: self.emissive_colour,
            refractive_index: self.refractive_index,
            scattering_colour: self.scattering_colour * self.scattering_coefficient,
        }
    }
}
