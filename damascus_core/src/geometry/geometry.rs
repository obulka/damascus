use glam::Vec3;

use crate::materials::Material;

#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    //skew: Vec3,
}

#[derive(Default)]
pub struct Primitive {
    pub transform: Transform,
    pub material: Material,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
}

pub trait BasePrimitive {
    fn primitive(self) -> Primitive;
}

#[derive(Default)]
pub struct Sphere {
    pub radius: f32,
    primitive: Primitive,
}

impl BasePrimitive for Sphere {
    fn primitive(self) -> Primitive {
        self.primitive
    }
}
