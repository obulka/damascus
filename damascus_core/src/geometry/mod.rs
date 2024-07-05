// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::{Mat3, Vec3};
use strum::{Display, EnumCount, EnumIter, EnumString};

pub mod camera;
mod primitive;

pub use primitive::{Primitive, Std430GPUPrimitive};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsStd430, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Transform {
    translation: Vec3,
    inverse_rotation: Mat3,
    uniform_scale: f32,
}

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
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
pub enum BlendType {
    #[default]
    Union,
    Subtraction,
    Intersection,
}

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Repetition {
    #[default]
    None,
    Finite,
    Infinite,
}
