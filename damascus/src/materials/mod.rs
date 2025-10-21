// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{UVec2, Vec3};
use slotmap::SlotMap;

use crate::{DualDevice, textures::Texture};

slotmap::new_key_type! { pub struct MaterialId; }

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUMaterial {
    diffuse_colour: Vec3,
    specular_probability: f32,
    specular_colour: Vec3,
    specular_roughness: f32,
    extinction_colour: Vec3,
    transmissive_probability: f32,
    emissive_colour: Vec3,
    transmissive_roughness: f32,
    scattering_colour: Vec3,
    refractive_index: f32,
    pub diffuse_colour_texture: UVec2,
    pub specular_probability_texture: UVec2,
    pub specular_roughness_texture: UVec2,
    pub specular_colour_texture: UVec2,
    pub transmissive_probability_texture: UVec2,
    pub transmissive_roughness_texture: UVec2,
    pub extinction_colour_texture: UVec2,
    pub emissive_colour_texture: UVec2,
    pub refractive_index_texture: UVec2,
    pub scattering_colour_texture: UVec2,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Material {
    pub diffuse_colour: Vec3,
    pub diffuse_colour_texture: Texture,
    pub specular_probability: f32,
    pub specular_probability_texture: Texture,
    pub specular_roughness: f32,
    pub specular_roughness_texture: Texture,
    pub specular_colour: Vec3,
    pub specular_colour_texture: Texture,
    pub transmissive_probability: f32,
    pub transmissive_probability_texture: Texture,
    pub transmissive_roughness: f32,
    pub transmissive_roughness_texture: Texture,
    pub extinction_coefficient: f32,
    pub transmissive_colour: Vec3,
    pub transmissive_colour_texture: Texture,
    pub emissive_intensity: f32,
    pub emissive_colour: Vec3,
    pub emissive_colour_texture: Texture,
    pub refractive_index: f32,
    pub refractive_index_texture: Texture,
    pub scattering_coefficient: f32,
    pub scattering_colour: Vec3,
    pub scattering_colour_texture: Texture,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: Vec3::ONE,
            diffuse_colour_texture: Texture::default(),
            specular_probability: 0.,
            specular_probability_texture: Texture::default(),
            specular_roughness: 0.,
            specular_roughness_texture: Texture::default(),
            specular_colour: Vec3::ONE,
            specular_colour_texture: Texture::default(),
            transmissive_probability: 0.,
            transmissive_probability_texture: Texture::default(),
            transmissive_roughness: 0.,
            transmissive_roughness_texture: Texture::default(),
            extinction_coefficient: 0.,
            transmissive_colour: Vec3::ONE,
            transmissive_colour_texture: Texture::default(),
            emissive_intensity: 0.,
            emissive_colour: Vec3::ONE,
            emissive_colour_texture: Texture::default(),
            refractive_index: 1.3,
            refractive_index_texture: Texture::default(),
            scattering_coefficient: 0.,
            scattering_colour: Vec3::ONE,
            scattering_colour_texture: Texture::default(),
        }
    }
}

impl Material {
    pub fn scaled_emissive_colour(&self) -> Vec3 {
        self.emissive_colour * self.emissive_intensity
    }

    pub fn is_emissive(&self) -> bool {
        self.scaled_emissive_colour().length_squared() > 0.
    }
}

impl DualDevice<GPUMaterial, Std430GPUMaterial> for Material {
    fn to_gpu(&self) -> GPUMaterial {
        GPUMaterial {
            diffuse_colour: self.diffuse_colour,
            specular_probability: self.specular_probability,
            specular_colour: self.specular_colour,
            specular_roughness: self.specular_roughness,
            transmissive_probability: self
                .transmissive_probability
                .min(1. - self.specular_probability),
            extinction_colour: (1. - self.transmissive_colour.clamp(Vec3::ZERO, Vec3::ONE))
                * self.extinction_coefficient,
            transmissive_roughness: self.transmissive_roughness,
            emissive_colour: self.scaled_emissive_colour(),
            scattering_colour: self.scattering_colour * self.scattering_coefficient,
            refractive_index: self.refractive_index,
            diffuse_colour_texture: self.diffuse_colour_texture.to_gpu(),
            specular_probability_texture: self.specular_probability_texture.to_gpu(),
            specular_roughness_texture: self.specular_roughness_texture.to_gpu(),
            specular_colour_texture: self.specular_colour_texture.to_gpu(),
            transmissive_probability_texture: self.transmissive_probability_texture.to_gpu(),
            transmissive_roughness_texture: self.transmissive_roughness_texture.to_gpu(),
            extinction_colour_texture: self.transmissive_colour_texture.to_gpu(),
            emissive_colour_texture: self.emissive_colour_texture.to_gpu(),
            refractive_index_texture: self.refractive_index_texture.to_gpu(),
            scattering_colour_texture: self.scattering_colour_texture.to_gpu(),
        }
    }
}

pub type Materials = SlotMap<MaterialId, Material>;
