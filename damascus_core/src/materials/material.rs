use crevice::std140::AsStd140;
use glam::Vec3;

use super::{GPUProceduralTexture, ProceduralTexture};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd140, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub diffuse_colour: Vec3,
    pub diffuse_texture: GPUProceduralTexture,
    pub specular_probability: f32,
    pub specular_roughness: f32,
    pub specular_colour: Vec3,
    pub transmissive_probability: f32,
    pub transmissive_roughness: f32,
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
            diffuse_texture: ProceduralTexture::default().to_gpu(),
            specular_probability: 0.,
            specular_roughness: 0.,
            specular_colour: Vec3::ONE,
            transmissive_probability: 0.,
            transmissive_roughness: 0.,
            transmissive_colour: Vec3::ONE,
            emissive_probability: 0.,
            emissive_colour: Vec3::new(1., 0.8, 0.5),
            refractive_index: 1.3,
            scattering_coefficient: 0.,
            scattering_colour: Vec3::ONE,
        }
    }
}
