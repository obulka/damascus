// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::Vec3;

use super::{
    geometry::{
        camera::Camera,
        {Primitive, Std430GPUPrimitive},
    },
    lights::{Light, Std430GPULight},
    materials::{Material, Std430GPUMaterial},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUSceneParameters {
    num_primitives: u32,
    num_lights: u32,
    num_non_physical_lights: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
    pub atmosphere: Material,
}

impl Default for Scene {
    fn default() -> Self {
        let mut atmosphere = Material::default();
        atmosphere.diffuse_colour = Vec3::ZERO;
        atmosphere.refractive_index = 1.;
        Self {
            render_camera: Camera::default(),
            primitives: vec![],
            lights: vec![],
            atmosphere: atmosphere,
        }
    }
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 512;
    pub const MAX_LIGHTS: usize = 512;

    fn to_gpu(&self) -> GPUSceneParameters {
        GPUSceneParameters {
            num_primitives: Self::MAX_PRIMITIVES.min(self.primitives.len()) as u32,
            num_lights: Self::MAX_LIGHTS.min(self.lights.len() + self.num_emissive_primitives())
                as u32,
            num_non_physical_lights: Self::MAX_LIGHTS.min(self.lights.len()) as u32,
        }
    }

    pub fn create_gpu_primitives(&self) -> [Std430GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [Primitive::default().to_gpu().as_std430(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Self::MAX_PRIMITIVES) {
            let mut primitive = self.primitives[index].to_gpu();
            primitive.id = (index + 1) as u32;
            primitive_array[index] = primitive.as_std430();
        }
        primitive_array
    }

    pub fn create_gpu_lights(&self) -> [Std430GPULight; Self::MAX_LIGHTS] {
        let mut light_array = [Light::default().to_gpu().as_std430(); Self::MAX_LIGHTS];
        for index in 0..self.lights.len().min(Self::MAX_LIGHTS) {
            light_array[index] = self.lights[index].to_gpu().as_std430();
        }
        light_array
    }

    fn num_emissive_primitives(&self) -> usize {
        let mut count = 0;
        for primitive in self.primitives.iter() {
            if primitive.material.scaled_emissive_colour().length() > 0. {
                count += 1;
            }
        }
        count
    }

    pub fn emissive_primitive_indices(&self) -> [u32; Self::MAX_PRIMITIVES] {
        let mut emissive_indices = [0; Self::MAX_PRIMITIVES];
        let mut emissive_count = 0;
        for (index, primitive) in self.primitives.iter().enumerate() {
            if emissive_count >= Self::MAX_PRIMITIVES {
                break;
            }
            if primitive.material.scaled_emissive_colour().length() == 0. {
                continue;
            }
            emissive_indices[emissive_count] = index as u32;
            emissive_count += 1;
        }
        emissive_indices
    }

    pub fn scene_parameters(&self) -> Std430GPUSceneParameters {
        self.to_gpu().as_std430()
    }

    pub fn atmosphere(&self) -> Std430GPUMaterial {
        self.atmosphere.to_gpu().as_std430()
    }

    pub fn clear_primitives(&mut self) {
        self.primitives.clear();
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
}
