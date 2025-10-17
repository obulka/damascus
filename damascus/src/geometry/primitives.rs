// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::{BVec3, Mat4, UVec3, Vec3, Vec4};
use slotmap::SlotMap;
use strum::{Display, EnumCount, EnumIter, EnumString};

use super::{BlendType, Repetition, Transform};
use crate::{DualDevice, Enumerator, Transformable};

slotmap::new_key_type! { pub struct PrimitiveId; }

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Shapes {
    #[default]
    CappedCone,
    CappedTorus,
    Capsule,
    Cone,
    CutSphere,
    Cylinder,
    DeathStar,
    Ellipsoid,
    HexagonalPrism,
    HollowSphere,
    InfiniteCone,
    InfiniteCylinder,
    Link,
    Mandelbox,
    Mandelbulb,
    Octahedron,
    Plane,
    RectangularPrism,
    RectangularPrismFrame,
    Rhombus,
    RoundedCone,
    SolidAngle,
    Sphere,
    Torus,
    TriangularPrism,
}

impl Enumerator for Shapes {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUPrimitive {
    pub id: u32,
    pub material_id: u32,
    pub num_descendants: u32,
    shape: u32,
    modifiers: u32,
    negative_repetitions: Vec3,
    blend_strength: f32,
    positive_repetitions: Vec3,
    wall_thickness: f32,
    spacing: Vec3,
    edge_radius: f32,
    elongation: Vec3,
    dimensional_data: Vec4,
    transform: Transform,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Primitive {
    pub shape: Shapes,
    pub local_to_world: Mat4,
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
    pub dimensional_data: Vec4,
}

impl Default for Primitive {
    fn default() -> Self {
        Self {
            shape: Shapes::Sphere,
            local_to_world: Mat4::IDENTITY,
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
            dimensional_data: 0.5 * Vec4::X,
        }
    }
}

impl Transformable for Primitive {
    fn transform(&mut self, local_to_world: &Mat4) {
        self.local_to_world *= local_to_world;
    }
}

impl Primitive {}

impl DualDevice<GPUPrimitive, Std430GPUPrimitive> for Primitive {
    fn to_gpu(&self) -> GPUPrimitive {
        let (scale, quaternion, translation) = self.local_to_world.to_scale_rotation_translation();
        GPUPrimitive {
            id: 0,
            material_id: 0, // 0 will be a default material, modify this before sending to the gpu
            num_descendants: 0,
            shape: self.shape as u32,
            modifiers: self.repetition as u32
                | (self.elongate as u32) << 2
                | (self.mirror.x as u32) << 3
                | (self.mirror.y as u32) << 4
                | (self.mirror.z as u32) << 5
                | (self.hollow as u32) << 6
                | if self.blend_type > BlendType::Union && !self.bounding_volume {
                    1 << self.blend_type as u32 + 6
                } else {
                    0
                }
                | (self.bounding_volume as u32) << 9,
            negative_repetitions: self.negative_repetitions.as_vec3(),
            blend_strength: self.blend_strength,
            positive_repetitions: self.positive_repetitions.as_vec3(),
            wall_thickness: self.wall_thickness,
            spacing: self.spacing,
            edge_radius: self.edge_radius,
            elongation: self.elongation,
            dimensional_data: self.dimensional_data,
            transform: Transform {
                translation: translation,
                inverse_rotation: glam::Mat3::from_quat(quaternion).inverse(),
                uniform_scale: scale.x,
            },
        }
    }
}

pub type Primitives = SlotMap<PrimitiveId, Primitive>;
