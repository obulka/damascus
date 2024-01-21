use crevice::std430::AsStd430;

use super::{
    geometry::{
        camera::Camera,
        {Primitive, Std430GPUPrimitive},
    },
    lights::{Light, Std430GPULight},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, AsStd430)]
pub struct SceneParameters {
    num_primitives: u32,
    num_lights: u32,
    num_non_physical_lights: u32,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 512;
    pub const MAX_LIGHTS: usize = 512;

    pub fn create_gpu_primitives(&self) -> [Std430GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [Primitive::default().to_gpu(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Scene::MAX_PRIMITIVES) {
            primitive_array[index] = self.primitives[index].to_gpu();
        }
        primitive_array
    }

    pub fn create_gpu_lights(&self) -> [Std430GPULight; Self::MAX_LIGHTS] {
        let mut light_array = [Light::default().to_gpu(); Self::MAX_LIGHTS];
        for index in 0..self.lights.len().min(Scene::MAX_LIGHTS) {
            light_array[index] = self.lights[index].to_gpu();
        }
        light_array
    }

    fn num_emissive_prims(&self) -> u32 {
        let mut count = 0;
        for primitive in self.primitives.iter() {
            if primitive.material.emissive_probability > 0. {
                count += 1;
            }
        }
        count
    }

    pub fn create_scene_parameters(&self) -> Std430SceneParameters {
        return SceneParameters {
            num_primitives: self.primitives.len() as u32,
            num_lights: self.lights.len() as u32 + self.num_emissive_prims(),
            num_non_physical_lights: self.lights.len() as u32,
        }
        .as_std430();
    }
}
