// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;

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
#[serde(default)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
    pub atmosphere: Material,
}

impl Default for Scene {
    fn default() -> Self {
        let mut atmosphere = Material::default();
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
    pub fn max_primitives(max_buffer_size: usize) -> usize {
        max_buffer_size / size_of::<Std430GPUPrimitive>()
    }

    pub fn max_lights(max_buffer_size: usize) -> usize {
        max_buffer_size / size_of::<Std430GPULight>()
    }

    fn num_primitives(&self, max_primitives: usize) -> u32 {
        max_primitives.min(self.primitives.len()) as u32
    }

    fn num_lights(&self, max_lights: usize) -> u32 {
        max_lights.min(self.lights.len() + self.num_emissive_primitives()) as u32
    }

    fn num_non_physical_lights(&self, max_lights: usize) -> u32 {
        max_lights.min(self.lights.len()) as u32
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

    fn to_gpu(&self, max_primitives: usize, max_lights: usize) -> GPUSceneParameters {
        GPUSceneParameters {
            num_primitives: self.num_primitives(max_primitives),
            num_lights: self.num_lights(max_lights),
            num_non_physical_lights: self.num_non_physical_lights(max_lights),
        }
    }

    pub fn create_gpu_primitives(&self, max_primitives: usize) -> Vec<Std430GPUPrimitive> {
        let mut primitives = vec![Primitive::default().to_gpu().as_std430(); max_primitives];
        for index in 0..self.primitives.len().min(max_primitives) {
            let mut primitive = self.primitives[index].to_gpu();
            primitive.id = (index + 1) as u32;
            primitives[index] = primitive.as_std430();
        }
        primitives
    }

    pub fn create_gpu_lights(&self, max_lights: usize) -> Vec<Std430GPULight> {
        let mut lights = vec![Light::default().to_gpu().as_std430(); max_lights];
        for index in 0..self.lights.len().min(max_lights) {
            lights[index] = self.lights[index].to_gpu().as_std430();
        }
        lights
    }

    pub fn emissive_primitive_indices(&self, max_primitives: usize) -> Vec<u32> {
        let mut emissive_indices = vec![0; max_primitives];
        let mut emissive_count = 0;
        for (index, primitive) in self.primitives.iter().enumerate() {
            if emissive_count >= max_primitives {
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

    pub fn scene_parameters(
        &self,
        max_primitives: usize,
        max_lights: usize,
    ) -> Std430GPUSceneParameters {
        self.to_gpu(max_primitives, max_lights).as_std430()
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
