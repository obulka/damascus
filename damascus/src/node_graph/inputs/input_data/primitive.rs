// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::Vec3;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{geometry::primitives::Primitive, scene::Scene, Enumerator};

use super::{InputData, InputResult, NodeInputData};

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
pub enum PrimitiveInputData {
    #[default]
    Siblings,
    Children,
    Material,
    Shape,
    Radius,
    Radii,
    Height,
    HollowRadius,
    HollowHeight,
    SolidAngle,
    Width,
    Depth,
    Thickness,
    CornerRadius,
    Base,
    Normal,
    NegativeHeight,
    PositiveHeight,
    Angle,
    LowerRadius,
    UpperRadius,
    RingRadius,
    TubeRadius,
    CapAngle,
    RadialExtent,
    Power,
    Iterations,
    MaxSquareRadius,
    Scale,
    MinSquareRadius,
    FoldingLimit,
    EdgeRadius,
    Repetition,
    NegativeRepetitions,
    PositiveRepetitions,
    Spacing,
    BoundingVolume,
    BlendType,
    BlendStrength,
    Mirror,
    Hollow,
    WallThickness,
    Elongate,
    Elongation,
    Axis,
}

impl Enumerator for PrimitiveInputData {}

impl NodeInputData for PrimitiveInputData {
    fn default_data(&self) -> InputData {
        let default_primitive = Primitive::default();
        match self {
            Self::Siblings => InputData::Scene(Scene::default()),
            Self::Children => InputData::Scene(Scene::default()),
            Self::Material => InputData::Scene(Scene::default()),
            Self::Shape => InputData::Enum(default_primitive.shape.into()),
            Self::Radius => InputData::Float(0.5),
            Self::Radii => InputData::Vec3(Vec3::splat(0.5)),
            Self::Height => InputData::Float(0.25),
            Self::HollowRadius => InputData::Float(0.5),
            Self::HollowHeight => InputData::Float(0.75),
            Self::SolidAngle => InputData::Float(30.),
            Self::Width => InputData::Float(0.5),
            Self::Depth => InputData::Float(0.75),
            Self::Thickness => InputData::Float(0.05),
            Self::CornerRadius => InputData::Float(0.05),
            Self::Base => InputData::Float(0.5),
            Self::Normal => InputData::Vec3(Vec3::Z),
            Self::NegativeHeight => InputData::Float(0.25),
            Self::PositiveHeight => InputData::Float(0.25),
            Self::Angle => InputData::Float(30.),
            Self::LowerRadius => InputData::Float(0.25),
            Self::UpperRadius => InputData::Float(0.125),
            Self::RingRadius => InputData::Float(0.3),
            Self::TubeRadius => InputData::Float(0.2),
            Self::CapAngle => InputData::Float(30.),
            Self::RadialExtent => InputData::Float(0.5),
            Self::Power => InputData::Float(8.),
            Self::Iterations => InputData::UInt(10),
            Self::MaxSquareRadius => InputData::Float(4.),
            Self::Scale => InputData::Float(-1.75),
            Self::MinSquareRadius => InputData::Float(0.001),
            Self::FoldingLimit => InputData::Float(0.8),
            Self::EdgeRadius => InputData::Float(default_primitive.edge_radius),
            Self::Repetition => InputData::Enum(default_primitive.repetition.into()),
            Self::NegativeRepetitions => InputData::UVec3(default_primitive.negative_repetitions),
            Self::PositiveRepetitions => InputData::UVec3(default_primitive.positive_repetitions),
            Self::Spacing => InputData::Vec3(default_primitive.spacing),
            Self::BoundingVolume => InputData::Bool(default_primitive.bounding_volume),
            Self::BlendType => InputData::Enum(default_primitive.blend_type.into()),
            Self::BlendStrength => InputData::Float(default_primitive.blend_strength),
            Self::Mirror => InputData::BVec3(default_primitive.mirror),
            Self::Hollow => InputData::Bool(default_primitive.hollow),
            Self::WallThickness => InputData::Float(default_primitive.wall_thickness),
            Self::Elongate => InputData::Bool(default_primitive.elongate),
            Self::Elongation => InputData::Vec3(default_primitive.elongation),
            Self::Axis => InputData::Mat4(default_primitive.local_to_world),
        }
    }
}
