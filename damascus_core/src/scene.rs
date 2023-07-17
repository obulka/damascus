use glam::Vec3;
use std::collections::HashMap;

use crate::geometry::{
    camera::{Camera, GPUCamera},
    {GPUPrimitive, Primitive},
};

#[derive(Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 1024;

    pub fn create_gpu_primitives(&self) -> [GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [GPUPrimitive::default(); Self::MAX_PRIMITIVES];
        for (index, primitive) in self.primitives.iter().enumerate() {
            primitive_array[index] = primitive.to_gpu();
        }
        primitive_array
    }
}
