// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::{BVec3, Mat3, Mat4, UVec3, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use super::materials::{GPUMaterial, Material};

pub mod camera;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsStd430, serde::Serialize, serde::Deserialize)]
pub struct Transform {
    translation: Vec3,
    inverse_rotation: Mat3,
    uniform_scale: f32,
}

#[derive(
    Debug, Display, Default, Copy, Clone, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
)]
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

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BlendType {
    #[default]
    Union,
    Subtraction,
    Intersection,
    SmoothUnion,
    SmoothSubtraction,
    SmoothIntersection,
}

#[derive(
    Debug, Display, Default, Copy, Clone, EnumIter, EnumString, serde::Serialize, serde::Deserialize,
)]
pub enum Repetition {
    #[default]
    None,
    Finite,
    Infinite,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUPrimitive {
    pub id: u32,
    shape: u32,
    transform: Transform,
    material: GPUMaterial,
    modifiers: u32,
    negative_repetitions: Vec3,
    positive_repetitions: Vec3,
    spacing: Vec3,
    blend_strength: f32,
    wall_thickness: f32,
    edge_radius: f32,
    elongation: Vec3,
    num_descendants: u32,
    dimensional_data: Vec4,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Primitive {
    pub shape: Shapes,
    pub world_matrix: Mat4,
    pub material: Material,
    pub edge_radius: f32,
    pub repetition: Repetition,
    pub negative_repetitions: UVec3,
    pub positive_repetitions: UVec3,
    pub spacing: Vec3,
    pub blend_type: BlendType,
    pub blend_strength: f32,
    pub mirror: BVec3,
    pub hollow: bool,
    pub wall_thickness: f32,
    pub elongate: bool,
    pub elongation: Vec3,
    pub bounding_volume: bool,
    pub num_descendants: u32,
    pub dimensional_data: Vec4,
}

impl Default for Primitive {
    fn default() -> Self {
        Self {
            shape: Shapes::Sphere,
            world_matrix: Mat4::IDENTITY,
            material: Material::default(),
            edge_radius: 0.,
            repetition: Repetition::None,
            negative_repetitions: UVec3::ZERO,
            positive_repetitions: UVec3::ONE,
            blend_type: BlendType::Union,
            blend_strength: 0.,
            spacing: Vec3::ONE,
            mirror: BVec3::FALSE,
            hollow: false,
            wall_thickness: 0.01,
            elongate: false,
            elongation: Vec3::ZERO,
            bounding_volume: false,
            num_descendants: 0,
            dimensional_data: Vec4::ONE,
        }
    }
}

impl Primitive {
    pub fn to_gpu(&self) -> GPUPrimitive {
        let (scale, quaternion, translation) = self.world_matrix.to_scale_rotation_translation();
        GPUPrimitive {
            id: 0,
            shape: self.shape as u32,
            transform: Transform {
                translation: translation,
                inverse_rotation: glam::Mat3::from_quat(quaternion).inverse(),
                uniform_scale: scale.x,
            },
            material: self.material.to_gpu(),
            modifiers: self.repetition as u32
                | if self.elongate { 4 } else { 0 }
                | if self.mirror.x { 8 } else { 0 }
                | if self.mirror.y { 16 } else { 0 }
                | if self.mirror.z { 32 } else { 0 }
                | if self.hollow { 64 } else { 0 }
                | if self.blend_type > BlendType::Union && !self.bounding_volume {
                    1 << self.blend_type as u32 + BlendType::COUNT as u32
                } else {
                    0
                }
                | if self.bounding_volume { 4096 } else { 0 },
            negative_repetitions: self.negative_repetitions.as_vec3(),
            positive_repetitions: self.positive_repetitions.as_vec3(),
            spacing: self.spacing,
            blend_strength: self.blend_strength,
            wall_thickness: self.wall_thickness,
            edge_radius: self.edge_radius,
            elongation: self.elongation,
            num_descendants: self.num_descendants,
            dimensional_data: self.dimensional_data,
        }
    }
}
