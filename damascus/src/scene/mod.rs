// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;

use super::{
    camera::Camera,
    geometry::primitives::{Primitive, Std430GPUPrimitive},
    lights::{Light, Std430GPULight},
    materials::Material,
};
use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUSceneParameters {
    num_primitives: u32,
    num_lights: u32,
    num_non_physical_lights: u32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
    pub fn render_camera(mut self, render_camera: Camera) -> Self {
        self.render_camera = render_camera;
        self
    }

    pub fn atmosphere(mut self, atmosphere: Material) -> Self {
        self.atmosphere = atmosphere;
        self
    }

    pub fn lights(mut self, lights: Vec<Light>) -> Self {
        self.lights = lights;
        self
    }

    pub fn primitives(mut self, primitives: Vec<Primitive>) -> Self {
        self.primitives = primitives;
        self
    }

    pub fn max_primitives_in_buffer(max_buffer_size: usize) -> usize {
        max_buffer_size / size_of::<Std430GPUPrimitive>()
    }

    pub fn max_lights_in_buffer(max_buffer_size: usize) -> usize {
        max_buffer_size / size_of::<Std430GPULight>()
    }

    pub fn num_emissive_primitives(&self) -> usize {
        let mut count = 0;
        for primitive in self.primitives.iter() {
            if primitive.material.scaled_emissive_colour().length() > 0. {
                count += 1;
            }
        }
        count
    }

    pub fn create_gpu_primitives(&self) -> Vec<Std430GPUPrimitive> {
        let mut gpu_primitives: Vec<Std430GPUPrimitive> = self
            .primitives
            .iter()
            .enumerate()
            .map(|(index, primitive)| {
                let mut gpu_primitive = primitive.to_gpu();
                gpu_primitive.id = (index + 1) as u32;
                gpu_primitive.as_std430()
            })
            .collect();
        if gpu_primitives.is_empty() {
            gpu_primitives.push(Primitive::default().as_std430());
        }
        gpu_primitives
    }

    pub fn create_gpu_lights(&self) -> Vec<Std430GPULight> {
        let mut gpu_lights: Vec<Std430GPULight> =
            self.lights.iter().map(|light| light.as_std430()).collect();
        if gpu_lights.is_empty() {
            gpu_lights.push(Light::default().as_std430());
        }
        gpu_lights
    }

    pub fn emissive_primitive_indices(&self) -> Vec<u32> {
        let mut emissive_indices = vec![];
        for (index, primitive) in self.primitives.iter().enumerate() {
            if primitive.material.scaled_emissive_colour().length() == 0. {
                continue;
            }
            emissive_indices.push(index as u32);
        }
        if emissive_indices.is_empty() {
            emissive_indices.push(0);
        }
        emissive_indices
    }

    pub fn clear_primitives(&mut self) {
        self.primitives.clear();
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
}

impl DualDevice<GPUSceneParameters, Std430GPUSceneParameters> for Scene {
    fn to_gpu(&self) -> GPUSceneParameters {
        GPUSceneParameters {
            num_primitives: self.primitives.len() as u32,
            num_lights: (self.lights.len() + self.num_emissive_primitives()) as u32,
            num_non_physical_lights: self.lights.len() as u32,
        }
    }
}
