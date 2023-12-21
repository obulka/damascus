use crevice::std140::AsStd140;
use glam::{Mat3, Mat4, Vec3, Vec4};

use crate::materials::Material;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsStd140, serde::Serialize, serde::Deserialize)]
pub struct Transform {
    translation: Vec3,
    inverse_rotation: Mat3,
    uniform_scale: f32,
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
    shape: u32,
    transform: Transform,
    material: Material,
    modifiers: u32,
    blend_strength: f32,
    num_children: u32,
    dimensional_data: Vec4,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Primitive {
    pub shape: Shapes,
    pub world_matrix: Mat4,
    pub material: Material,
    pub modifiers: u32,
    pub blend_strength: f32,
    pub num_children: u32,
    pub dimensional_data: Vec4,
}

impl Primitive {
    pub fn to_gpu(&self) -> Std140GPUPrimitive {
        let (scale, quaternion, translation) = self.world_matrix.to_scale_rotation_translation();
        GPUPrimitive {
            shape: self.shape as u32,
            transform: Transform {
                translation: translation,
                inverse_rotation: glam::Mat3::from_quat(quaternion).inverse(),
                uniform_scale: scale.x,
            },
            material: self.material,
            modifiers: self.modifiers,
            blend_strength: self.blend_strength,
            num_children: self.num_children,
            dimensional_data: self.dimensional_data,
        }
        .as_std140()
    }
}
