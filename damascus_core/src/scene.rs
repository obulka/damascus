use daggy::Dag;
use glam::Vec3;
use std::collections::HashMap;

use crate::geometry::{
    camera::{Camera, GPUCamera},
    Primitive,
};
use crate::materials::{GPUMaterial, Material};

#[derive(Debug, Default)]
pub struct Scene {
    // pub primitives: Dag<Box<dyn Primitive>, >,
    pub render_camera: Camera,
    pub materials: Vec<Material>,
}

impl Scene {
    pub const MAX_MATERIALS: usize = 512;

    pub fn create_gpu_materials(&self) -> [GPUMaterial; Self::MAX_MATERIALS] {
        let mut material_array = [GPUMaterial::default(); Self::MAX_MATERIALS];
        for (index, material) in self.materials.iter().enumerate() {
            material_array[index] = material.to_gpu_material();
        }
        material_array
    }
}
