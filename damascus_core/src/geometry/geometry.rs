use glam::Vec3;

use crate::materials::Material;

#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    //skew: Vec3,
}

pub trait Primitive {
    fn id(self) -> u32;
    fn transform(self) -> Transform;
    fn material(self) -> Material;
    fn modifiers(self) -> u32;
    fn blend_strength(self) -> f32;
    fn num_children(self) -> u32;
}

#[derive(Default)]
pub struct Sphere {
    pub id: u32,
    pub transform: Transform,
    pub material: Material,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub radius: f32,
}

impl Primitive for Sphere {
    fn id(self) -> u32 {
        self.id
    }

    fn transform(self) -> Transform {
        self.transform
    }

    fn material(self) -> Material {
        self.material
    }

    fn modifiers(self) -> u32 {
        self.modifiers
    }

    fn blend_strength(self) -> f32 {
        self.blend_strength
    }

    fn num_children(self) -> u32 {
        self.num_children
    }
}
