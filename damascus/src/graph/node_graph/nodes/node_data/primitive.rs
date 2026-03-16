// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Vec3, Vec4};
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    geometry::primitives::{Primitive, PrimitiveId, Shapes},
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum PrimitiveInputData {
    #[default]
    Child,
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
    EnableOrbitTrapColour,
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

impl NodeInputData for PrimitiveInputData {
    fn default_data(&self) -> InputData {
        let default_primitive = Primitive::default();
        match self {
            Self::Child => InputData::SceneGraphId(SceneGraphId::None),
            Self::Material => InputData::SceneGraphId(SceneGraphId::None),
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
            Self::EnableOrbitTrapColour => {
                InputData::Bool(default_primitive.enable_orbit_trap_colour)
            }
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

#[derive(Copy, Default, EnumHashTraits!)]
pub enum PrimitiveOutputData {
    #[default]
    Id,
}

impl NodeOutputData for PrimitiveOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Id => OutputData::SceneGraphId(SceneGraphIdType::Primitive),
        }
    }
}

pub struct PrimitiveNode;

impl EvaluableNode for PrimitiveNode {
    type Inputs = PrimitiveInputData;
    type Outputs = PrimitiveOutputData;

    fn dynamic_inputs() -> impl Iterator<Item = Self::Inputs> {
        vec![Self::Inputs::Child].into_iter()
    }

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
        match input {
            Self::Inputs::Child => match *output_data {
                OutputData::SceneGraphId(location_type) => location_type.has_transform(),
                _ => false,
            },
            Self::Inputs::Material => {
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::Material)
            }
            Self::Inputs::Axis => *output_data == OutputData::Mat4,
            _ => false,
        }
    }

    fn update_data_model(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let primitive_id: &PrimitiveId = input_data.as_primitive_id()?;

        let shape: Shapes = Self::Inputs::Shape.from_data_map(data_map)?.try_to_enum()?;
        let dimensional_data: Vec4 = match shape {
            Shapes::CappedCone | Shapes::RoundedCone => Vec4::new(
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::LowerRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::UpperRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::CappedTorus => Vec4::new(
                Self::Inputs::RingRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::TubeRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::CapAngle
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::Capsule => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::NegativeHeight
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::PositiveHeight
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::Cone => Vec4::new(
                Self::Inputs::Angle
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::CutSphere => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::Cylinder => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::DeathStar => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::HollowRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::HollowHeight
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::Ellipsoid => Vec4::from((
                Self::Inputs::Radii.from_data_map(data_map)?.try_to_vec3()?,
                0.,
            )),
            Shapes::HexagonalPrism => Vec4::new(
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Depth
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::HollowSphere => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Thickness
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::InfiniteCone => Vec4::new(
                Self::Inputs::Angle
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
                0.,
            ),
            Shapes::InfiniteCylinder => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
                0.,
            ),
            Shapes::Link => Vec4::new(
                Self::Inputs::RingRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::TubeRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::Mandelbox => Vec4::new(
                Self::Inputs::Scale
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Iterations
                    .from_data_map(data_map)?
                    .try_to_uint()? as f32,
                Self::Inputs::MinSquareRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::FoldingLimit
                    .from_data_map(data_map)?
                    .try_to_float()?,
            ),
            Shapes::Mandelbulb => Vec4::new(
                Self::Inputs::Power
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Iterations
                    .from_data_map(data_map)?
                    .try_to_uint()? as f32,
                Self::Inputs::MaxSquareRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::Octahedron => Vec4::new(
                Self::Inputs::RadialExtent
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
                0.,
            ),
            Shapes::Plane => Vec4::from((
                Self::Inputs::Normal
                    .from_data_map(data_map)?
                    .try_to_vec3()?,
                0.,
            )),
            Shapes::RectangularPrism => Vec4::new(
                Self::Inputs::Width
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Depth
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
            ),
            Shapes::RectangularPrismFrame => Vec4::new(
                Self::Inputs::Width
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Depth
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Thickness
                    .from_data_map(data_map)?
                    .try_to_float()?,
            ),
            Shapes::Rhombus => Vec4::new(
                Self::Inputs::Width
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Height
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::Depth
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::CornerRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
            ),
            Shapes::SolidAngle => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::SolidAngle
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::Sphere => Vec4::new(
                Self::Inputs::Radius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
                0.,
            ),
            Shapes::Torus => Vec4::new(
                Self::Inputs::RingRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                Self::Inputs::TubeRadius
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
            Shapes::TriangularPrism => Vec4::new(
                Self::Inputs::Base.from_data_map(data_map)?.try_to_float()?,
                Self::Inputs::Depth
                    .from_data_map(data_map)?
                    .try_to_float()?,
                0.,
                0.,
            ),
        };

        scene_graph[*primitive_id].shape = shape;
        scene_graph[*primitive_id].local_to_world =
            Self::Inputs::Axis.from_data_map(data_map)?.try_to_mat4()?;
        scene_graph[*primitive_id].edge_radius = Self::Inputs::EdgeRadius
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*primitive_id].repetition = Self::Inputs::Repetition
            .from_data_map(data_map)?
            .try_to_enum()?;
        scene_graph[*primitive_id].negative_repetitions = Self::Inputs::NegativeRepetitions
            .from_data_map(data_map)?
            .try_to_uvec3()?;
        scene_graph[*primitive_id].positive_repetitions = Self::Inputs::PositiveRepetitions
            .from_data_map(data_map)?
            .try_to_uvec3()?;
        scene_graph[*primitive_id].spacing = Self::Inputs::Spacing
            .from_data_map(data_map)?
            .try_to_vec3()?;
        scene_graph[*primitive_id].blend_type = Self::Inputs::BlendType
            .from_data_map(data_map)?
            .try_to_enum()?;
        scene_graph[*primitive_id].blend_strength = Self::Inputs::BlendStrength
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*primitive_id].mirror = Self::Inputs::Mirror
            .from_data_map(data_map)?
            .try_to_bvec3()?;
        scene_graph[*primitive_id].hollow = Self::Inputs::Hollow
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*primitive_id].wall_thickness = Self::Inputs::WallThickness
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*primitive_id].elongate = Self::Inputs::Elongate
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*primitive_id].elongation = Self::Inputs::Elongation
            .from_data_map(data_map)?
            .try_to_vec3()?;
        scene_graph[*primitive_id].bounding_volume = Self::Inputs::BoundingVolume
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*primitive_id].enable_orbit_trap_colour = Self::Inputs::EnableOrbitTrapColour
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*primitive_id].dimensional_data = dimensional_data;

        Ok(())
    }

    fn evaluate(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        cached_input_data: Option<InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let primitive_id: PrimitiveId = match cached_input_data {
            Some(input_data) => input_data.try_to_primitive_id()?,
            None => scene_graph.add_primitive(Primitive::default()),
        };

        let scene_graph_id: SceneGraphId = primitive_id.into();
        let input_data_id = InputData::SceneGraphId(scene_graph_id);

        if let Ok(material_id) = Self::Inputs::Material
            .from_data_map(data_map)?
            .try_to_material_id()
        {
            scene_graph.set_material(primitive_id, material_id);
        }

        Self::add_dynamic_children_to_scene_graph(
            scene_graph,
            data_map,
            scene_graph_id,
            Self::Inputs::Child,
        );

        Self::update_data_model(scene_graph, data_map, &input_data_id)?;

        match output {
            Self::Outputs::Id => Ok(input_data_id),
        }
    }
}
