use crevice::std140::AsStd140;
use glam::{Mat3, Vec3, Vec4};

use crate::materials::Material;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsStd140, serde::Serialize, serde::Deserialize)]
pub struct Transform {
    pub translation: Vec3,
    pub inverse_rotation: Mat3,
    pub uniform_scale: f32,
}

#[derive(Debug, Default, Copy, Clone, FromPrimitive, serde::Serialize, serde::Deserialize)]
pub enum Shapes {
    #[default]
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
#[derive(Debug, Default, Copy, Clone, AsStd140)]
pub struct GPUPrimitive {
    pub shape: u32,
    pub transform: Transform,
    pub material: Material,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub dimensional_data: Vec4,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
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
    pub fn to_gpu(&self) -> Std140GPUPrimitive {
        GPUPrimitive {
            shape: self.shape as u32,
            transform: self.transform,
            material: self.material,
            modifiers: self.modifiers,
            blend_strength: self.blend_strength,
            num_children: self.num_children,
            dimensional_data: self.dimensional_data,
        }
        .as_std140()
    }
}
