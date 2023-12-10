use crevice::std140::AsStd140;

use crate::geometry::{
    camera::Camera,
    {GPUPrimitive, Primitive, Std140GPUPrimitive},
};

#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 512;

    pub fn create_gpu_primitives(&self) -> [Std140GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [GPUPrimitive::default().as_std140(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Scene::MAX_PRIMITIVES) {
            primitive_array[index] = self.primitives[index].to_gpu().as_std140();
        }
        primitive_array
    }
}
