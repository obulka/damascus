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

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
    pub atmosphere: Material,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 512;
    pub const MAX_LIGHTS: usize = 512;

    fn to_gpu(&self) -> GPUSceneParameters {
        GPUSceneParameters {
            num_primitives: Self::MAX_PRIMITIVES.min(self.primitives.len()) as u32,
            num_lights: Self::MAX_LIGHTS.min(self.lights.len() + self.num_emissive_prims()) as u32,
            num_non_physical_lights: Self::MAX_LIGHTS.min(self.lights.len()) as u32,
        }
    }

    pub fn create_gpu_primitives(&self) -> [Std430GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [Primitive::default().to_gpu().as_std430(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Self::MAX_PRIMITIVES) {
            primitive_array[index] = self.primitives[index].to_gpu().as_std430();
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

    fn num_emissive_prims(&self) -> usize {
        let mut count = 0;
        for primitive in self.primitives.iter() {
            if primitive.material.emissive_probability > 0. {
                count += 1;
            }
        }
        count
    }

    pub fn scene_parameters(&self) -> Std430GPUSceneParameters {
        self.to_gpu().as_std430()
    }

    pub fn atmosphere(&self) -> Std430GPUMaterial {
        self.atmosphere.to_gpu().as_std430()
    }
}
