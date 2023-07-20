use glam::{Vec3, Vec4};

use crate::materials::{GPUMaterial, Material};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUTransform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    //pub skew: [f32; 3],
}

#[derive(Debug, Default)]
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
    pub shape: u32,
    pub transform: GPUTransform,
    pub material: GPUMaterial,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub dimensional_data: [f32; 4],
}

#[derive(Debug, Default)]
pub struct Primitive {
    pub shape: Shapes,
    pub transform: Transform,
    pub material: Material,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub dimensional_data: Vec4,
}

impl Primitive {
    fn to_gpu(&self) -> GPUPrimitive {
        GPUPrimitive {
            shape: self.shape as u32,
            transform: self.transform.to_gpu(),
            material: self.material.to_gpu(),
            modifiers: self.modifiers,
            blend_strength: self.blend_strength,
            num_children: self.num_children,
            dimensional_data: self.dimensional_data.to_array(),
        }
    }
}
