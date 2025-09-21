// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    geometry::primitives::{Primitive, Shapes},
    materials::Material,
    scene::Scene,
    Enumerator,
};

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

    fn compute_output(data_map: &mut HashMap<String, InputData>) -> InputResult<InputData> {
        let mut scene: Scene = Self::Siblings.get_data(data_map)?.try_to_scene()?;
        let mut descendants: Scene = Self::Children.get_data(data_map)?.try_to_scene()?;
        let material: Scene = Self::Material.get_data(data_map)?.try_to_scene()?;
        let shape: Shapes = Self::Shape.get_data(data_map)?.try_to_enum()?;

        let dimensional_data: Vec4 = match shape {
            Shapes::CappedCone | Shapes::RoundedCone => Vec4::new(
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::LowerRadius.get_data(data_map)?.try_to_float()?,
                Self::UpperRadius.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::CappedTorus => Vec4::new(
                Self::RingRadius.get_data(data_map)?.try_to_float()?,
                Self::TubeRadius.get_data(data_map)?.try_to_float()?,
                Self::CapAngle.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::Capsule => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::NegativeHeight.get_data(data_map)?.try_to_float()?,
                Self::PositiveHeight.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::Cone => Vec4::new(
                Self::Angle.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::CutSphere => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::Cylinder => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::DeathStar => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::HollowRadius.get_data(data_map)?.try_to_float()?,
                Self::HollowHeight.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::Ellipsoid => Vec4::from((Self::Radii.get_data(data_map)?.try_to_vec3()?, 0.)),
            Shapes::HexagonalPrism => Vec4::new(
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::Depth.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::HollowSphere => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::Thickness.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::InfiniteCone => {
                Vec4::new(Self::Angle.get_data(data_map)?.try_to_float()?, 0., 0., 0.)
            }
            Shapes::InfiniteCylinder => {
                Vec4::new(Self::Radius.get_data(data_map)?.try_to_float()?, 0., 0., 0.)
            }
            Shapes::Link => Vec4::new(
                Self::RingRadius.get_data(data_map)?.try_to_float()?,
                Self::TubeRadius.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::Mandelbox => Vec4::new(
                Self::Scale.get_data(data_map)?.try_to_float()?,
                Self::Iterations.get_data(data_map)?.try_to_uint()? as f32,
                Self::MinSquareRadius.get_data(data_map)?.try_to_float()?,
                Self::FoldingLimit.get_data(data_map)?.try_to_float()?,
            ),
            Shapes::Mandelbulb => Vec4::new(
                Self::Power.get_data(data_map)?.try_to_float()?,
                Self::Iterations.get_data(data_map)?.try_to_uint()? as f32,
                Self::MaxSquareRadius.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::Octahedron => Vec4::new(
                Self::RadialExtent.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
                0.,
            ),
            Shapes::Plane => Vec4::from((Self::Normal.get_data(data_map)?.try_to_vec3()?, 0.)),
            Shapes::RectangularPrism => Vec4::new(
                Self::Width.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::Depth.get_data(data_map)?.try_to_float()?,
                0.,
            ),
            Shapes::RectangularPrismFrame => Vec4::new(
                Self::Width.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::Depth.get_data(data_map)?.try_to_float()?,
                Self::Thickness.get_data(data_map)?.try_to_float()?,
            ),
            Shapes::Rhombus => Vec4::new(
                Self::Width.get_data(data_map)?.try_to_float()?,
                Self::Height.get_data(data_map)?.try_to_float()?,
                Self::Depth.get_data(data_map)?.try_to_float()?,
                Self::CornerRadius.get_data(data_map)?.try_to_float()?,
            ),
            Shapes::SolidAngle => Vec4::new(
                Self::Radius.get_data(data_map)?.try_to_float()?,
                Self::SolidAngle.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::Sphere => {
                Vec4::new(Self::Radius.get_data(data_map)?.try_to_float()?, 0., 0., 0.)
            }
            Shapes::Torus => Vec4::new(
                Self::RingRadius.get_data(data_map)?.try_to_float()?,
                Self::TubeRadius.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
            Shapes::TriangularPrism => Vec4::new(
                Self::Base.get_data(data_map)?.try_to_float()?,
                Self::Depth.get_data(data_map)?.try_to_float()?,
                0.,
                0.,
            ),
        };

        let local_to_world: Mat4 = Self::Axis.get_data(data_map)?.try_to_mat4()?;
        for descendant in descendants.primitives.iter_mut() {
            descendant.local_to_world = local_to_world * descendant.local_to_world;
        }

        scene.primitives.push(Primitive {
            shape: shape,
            local_to_world: local_to_world,
            material: Material::default(), // TODO assign materials by id
            hollow: Self::Hollow.get_data(data_map)?.try_to_bool()?,
            wall_thickness: Self::WallThickness.get_data(data_map)?.try_to_float()?,
            edge_radius: Self::EdgeRadius.get_data(data_map)?.try_to_float()?,
            mirror: Self::Mirror.get_data(data_map)?.try_to_bvec3()?,
            elongate: Self::Elongate.get_data(data_map)?.try_to_bool()?,
            elongation: Self::Elongation.get_data(data_map)?.try_to_vec3()?,
            repetition: Self::Repetition.get_data(data_map)?.try_to_enum()?,
            negative_repetitions: Self::NegativeRepetitions
                .get_data(data_map)?
                .try_to_uvec3()?,
            positive_repetitions: Self::PositiveRepetitions
                .get_data(data_map)?
                .try_to_uvec3()?,
            spacing: Self::Spacing.get_data(data_map)?.try_to_vec3()?,
            blend_type: Self::BlendType.get_data(data_map)?.try_to_enum()?,
            blend_strength: Self::BlendStrength.get_data(data_map)?.try_to_float()?,
            bounding_volume: Self::BoundingVolume.get_data(data_map)?.try_to_bool()?,
            num_descendants: descendants.primitives.len() as u32,
            dimensional_data: dimensional_data,
        });
        scene.merge(descendants);

        Ok(InputData::Scene(scene))
    }
}
