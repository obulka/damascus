// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{UVec2, Vec3};
use slotmap::SlotMap;

use crate::{
    DualDevice,
    textures::evaluators::{TextureEvaluator, TextureEvaluatorId},
};

slotmap::new_key_type! { pub struct MaterialId; }

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUMaterial {
    pub diffuse_colour: Vec3,
    pub specular_probability: f32,
    pub specular_colour: Vec3,
    pub specular_roughness: f32,
    pub extinction_colour: Vec3,
    pub transmissive_probability: f32,
    pub emissive_colour: Vec3,
    pub transmissive_roughness: f32,
    pub scattering_colour: Vec3,
    pub refractive_index: f32,
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

impl GPUMaterial {
    pub fn is_emissive(&self) -> bool {
        self.emissive_colour.length_squared() > 0.
    }
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Material {
    pub diffuse_colour: Vec3,
    pub diffuse_colour_texture_id: Option<TextureEvaluatorId>,
    pub specular_probability: f32,
    pub specular_probability_texture_id: Option<TextureEvaluatorId>,
    pub specular_roughness: f32,
    pub specular_roughness_texture_id: Option<TextureEvaluatorId>,
    pub specular_colour: Vec3,
    pub specular_colour_texture_id: Option<TextureEvaluatorId>,
    pub transmissive_probability: f32,
    pub transmissive_probability_texture_id: Option<TextureEvaluatorId>,
    pub transmissive_roughness: f32,
    pub transmissive_roughness_texture_id: Option<TextureEvaluatorId>,
    pub extinction_coefficient: f32,
    pub transmissive_colour: Vec3,
    pub transmissive_colour_texture_id: Option<TextureEvaluatorId>,
    pub emissive_intensity: f32,
    pub emissive_colour: Vec3,
    pub emissive_colour_texture_id: Option<TextureEvaluatorId>,
    pub refractive_index: f32,
    pub refractive_index_texture_id: Option<TextureEvaluatorId>,
    pub scattering_coefficient: f32,
    pub scattering_colour: Vec3,
    pub scattering_colour_texture_id: Option<TextureEvaluatorId>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse_colour: Vec3::ONE,
            diffuse_colour_texture_id: None,
            specular_probability: 0.,
            specular_probability_texture_id: None,
            specular_roughness: 0.,
            specular_roughness_texture_id: None,
            specular_colour: Vec3::ONE,
            specular_colour_texture_id: None,
            transmissive_probability: 0.,
            transmissive_probability_texture_id: None,
            transmissive_roughness: 0.,
            transmissive_roughness_texture_id: None,
            extinction_coefficient: 0.,
            transmissive_colour: Vec3::ONE,
            transmissive_colour_texture_id: None,
            emissive_intensity: 0.,
            emissive_colour: Vec3::ONE,
            emissive_colour_texture_id: None,
            refractive_index: 1.3,
            refractive_index_texture_id: None,
            scattering_coefficient: 0.,
            scattering_colour: Vec3::ONE,
            scattering_colour_texture_id: None,
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

    pub fn diffuse_colour(mut self, diffuse_colour: Vec3) -> Self {
        self.diffuse_colour = diffuse_colour;
        self
    }

    pub fn specular_probability(mut self, specular_probability: f32) -> Self {
        self.specular_probability = specular_probability;
        self
    }

    pub fn specular_roughness(mut self, specular_roughness: f32) -> Self {
        self.specular_roughness = specular_roughness;
        self
    }

    pub fn specular_colour(mut self, specular_colour: Vec3) -> Self {
        self.specular_colour = specular_colour;
        self
    }

    pub fn transmissive_probability(mut self, transmissive_probability: f32) -> Self {
        self.transmissive_probability = transmissive_probability;
        self
    }

    pub fn transmissive_roughness(mut self, transmissive_roughness: f32) -> Self {
        self.transmissive_roughness = transmissive_roughness;
        self
    }

    pub fn extinction_coefficient(mut self, extinction_coefficient: f32) -> Self {
        self.extinction_coefficient = extinction_coefficient;
        self
    }

    pub fn emissive_intensity(mut self, emissive_intensity: f32) -> Self {
        self.emissive_intensity = emissive_intensity;
        self
    }

    pub fn emissive_colour(mut self, emissive_colour: Vec3) -> Self {
        self.emissive_colour = emissive_colour;
        self
    }

    pub fn refractive_index(mut self, refractive_index: f32) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn scattering_coefficient(mut self, scattering_coefficient: f32) -> Self {
        self.scattering_coefficient = scattering_coefficient;
        self
    }

    pub fn scattering_colour(mut self, scattering_colour: Vec3) -> Self {
        self.scattering_colour = scattering_colour;
        self
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
            diffuse_colour_texture: TextureEvaluator::White.to_gpu(),
            specular_probability_texture: TextureEvaluator::White.to_gpu(),
            specular_roughness_texture: TextureEvaluator::White.to_gpu(),
            specular_colour_texture: TextureEvaluator::White.to_gpu(),
            transmissive_probability_texture: TextureEvaluator::White.to_gpu(),
            transmissive_roughness_texture: TextureEvaluator::White.to_gpu(),
            extinction_colour_texture: TextureEvaluator::White.to_gpu(),
            emissive_colour_texture: TextureEvaluator::White.to_gpu(),
            refractive_index_texture: TextureEvaluator::White.to_gpu(),
            scattering_colour_texture: TextureEvaluator::White.to_gpu(),
        }
    }
}

pub type Materials = SlotMap<MaterialId, Material>;
