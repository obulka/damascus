use daggy::Dag;
use glam::Vec3;
use std::collections::HashMap;

use crate::geometry::{camera::Camera, Primitive};
use crate::materials::Material;

#[derive(Default)]
pub struct Scene {
    // pub materials: HashMap<u32, Material>,
    // pub cameras: HashMap<u32, Camera>,
    // pub primitives: Dag<Box<dyn Primitive>, >,
    pub render_camera: u32,
}
