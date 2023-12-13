use crevice::std140::AsStd140;

use crate::{
    geometry::{
        camera::Camera,
        {GPUPrimitive, Primitive, Std140GPUPrimitive},
    },
    lights::{GPULight, Light, Std140GPULight},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, AsStd140)]
pub struct SceneGlobals {
    num_primitives: u32,
    num_lights: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 512;
    pub const MAX_LIGHTS: usize = 512;

    pub fn create_gpu_primitives(&self) -> [Std140GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [GPUPrimitive::default().as_std140(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Scene::MAX_PRIMITIVES) {
            primitive_array[index] = self.primitives[index].to_gpu().as_std140();
        }
        primitive_array
    }

    pub fn create_gpu_lights(&self) -> [Std140GPULight; Self::MAX_LIGHTS] {
        let mut light_array = [GPULight::default().as_std140(); Self::MAX_LIGHTS];
        for index in 0..self.lights.len().min(Scene::MAX_LIGHTS) {
            light_array[index] = self.lights[index].to_gpu().as_std140();
        }
        light_array
    }

    pub fn create_scene_globals(&self) -> Std140SceneGlobals {
        return SceneGlobals {
            num_primitives: self.primitives.len() as u32,
            num_lights: self.lights.len() as u32,
        }.as_std140()
    }
}
