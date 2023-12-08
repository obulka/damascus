use crate::geometry::{
    camera::Camera,
    {GPUPrimitive, Primitive},
};

#[derive(Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub primitives: Vec<Primitive>,
}

impl Scene {
    pub const MAX_PRIMITIVES: usize = 1024;

    pub fn create_gpu_primitives(&self) -> [GPUPrimitive; Self::MAX_PRIMITIVES] {
        let mut primitive_array = [GPUPrimitive::default(); Self::MAX_PRIMITIVES];
        for index in 0..self.primitives.len().min(Scene::MAX_PRIMITIVES) {
            primitive_array[index] = self.primitives[index].to_gpu();
        }
        primitive_array
    }
}
