use glam::Vec3;

use crate::materials::{GPUMaterial, Material};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    //pub skew: [f32; 3],
}

#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    //pub skew: Vec3,
}

impl Transform {
    pub fn to_gpu(&self) -> GPUTransform {
        GPUTransform {
            position: self.position.to_array(),
            rotation: self.rotation.to_array(),
            scale: self.scale.to_array(),
            //skew: self.skew.to_array(),
        }
    }
}

pub enum Shapes {
    Sphere,
    Ellipsoid,
    CutSphere,
    HollowSphere,
    DeathStar,
    SolidAngle,
    RectangularPrism,
    RectangularPrismFrame,
    Rhombus,
    TriangularPrism,
    Cylinder,
    InfiniteCylinder,
    Plane,
    Capsule,
    Cone,
    InfiniteCone,
    CappedCone,
    RoundedCone,
    Torus,
    CappedTorus,
    Link,
    HexagonalPrism,
    Octahedron,
    Mandelbulb,
    Mandelbox,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUPrimitive {
    pub id: u32,
    pub shape: u32,
    pub transform: GPUTransform,
    pub material: GPUMaterial,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub custom_data: [f32; 4],
}

pub trait Primitive {
    fn id(self) -> u32;
    fn transform(self) -> Transform;
    fn material(self) -> Material;
    fn modifiers(self) -> u32;
    fn blend_strength(self) -> f32;
    fn num_children(self) -> u32;
    fn to_gpu(&self) -> GPUPrimitive;
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

    fn to_gpu(&self) -> GPUPrimitive {
        GPUPrimitive {
            id: self.id,
            shape: Shapes::Sphere as u32,
            transform: self.transform.to_gpu(),
            material: self.material.to_gpu(),
            modifiers: self.modifiers,
            blend_strength: self.blend_strength,
            num_children: self.num_children,
            custom_data: [self.radius, 0., 0., 0.],
        }
    }
}
