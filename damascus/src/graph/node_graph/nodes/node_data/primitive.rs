// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Vec3, Vec4};
use indoc::indoc;
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

    fn tooltip(&self) -> &str {
        match self {
            Self::Child => indoc! {
                "The children of this primitive.\n
                    These will be transformed using this primitive's
                    blend_type and transform.\n
                    If this primitive is a bounding volume, children
                    outside of its bounds will not be rendered, and
                    increased performance can be achieved when
                    rendering the children."
            },
            Self::Material => "The primitive's material.",
            Self::Shape => "The shape of the primitive.",
            Self::Radius => "The radius.",
            Self::Radii => "The radii of the ellipsoid.",
            Self::Height => "The height (y-axis).",
            Self::HollowRadius => "The radius of the sphere that is cut from the solid.",
            Self::HollowHeight => indoc! {
                "The height (y-axis) of the center of the sphere
                    that is cut from the solid, above solidRadius +
                    hollowRadius, the result will be a standard
                    sphere of radius solidRadius."
            },
            Self::SolidAngle => indoc! {
                "The angle between the edge of the solid angle and the
                    y-axis on [0-180] measured between the y-axis and wall
                    of the solid angle."
            },
            Self::Width => "The width (x-axis).",
            Self::Depth => "The depth (z-axis).",
            Self::Thickness => "The thickness of the walls.",
            Self::CornerRadius => {
                "The radius of the corners of the rhombus' xy-plane parallel face."
            }
            Self::Base => "The equilateral triangles edge length (xy-plane).",
            Self::Normal => "The normal direction of the plane.",
            Self::NegativeHeight => {
                "The distance along the negative y-axis before entering the dome."
            }
            Self::PositiveHeight => {
                "The distance along the positive y-axis before entering the dome."
            }
            Self::Angle => indoc! {
                "The angle between the tip and base of the cone [0-90]
                    measured between the y-axis and wall of the cone."
            },
            Self::LowerRadius => "The radius of the cone at y = -height/2.",
            Self::UpperRadius => "The radius of the cone at y = height/2.",
            Self::RingRadius => "The radius (xy-plane) of the ring of the torus.",
            Self::TubeRadius => "The radius of the tube of the torus.",
            Self::CapAngle => {
                "The angle (xy-plane, symmetric about y-axis) to cap at, in the range [0-180.]."
            }
            Self::RadialExtent => indoc! {
                "The maximum distance along the x, y, and z axes.
                    ie. The vertices are at +/-radial_extent on the x, y,
                    and z axes."
            },
            Self::Power => "One greater than the axes of symmetry in the xy-plane.",
            Self::Iterations => indoc! {
                "The number of iterations to compute, the higher this
                    is, the slower it will be to compute, but the more
                    detail the fractal will have."
            },
            Self::MaxSquareRadius => {
                "When the square radius has reached this length, stop iterating."
            }
            Self::Scale => {
                "The amount to scale the position between folds. Can be negative or positive."
            }
            Self::MinSquareRadius => "The minimum square radius to use when spherically folding.",
            Self::FoldingLimit => indoc! {
                "Clamp the position between +/- this value when
                    performing the box fold. Higher values will result
                    in a denser fractal.",
            },
            Self::EnableOrbitTrapColour => {
                "The orbital traps will affect the material of this primitive if enabled."
            }
            Self::EdgeRadius => "The thickness of the walls of the shape, if the shape is hollow.",
            Self::Repetition => indoc! {
                "Repeat objects in the scene with no extra memory
                    consumption. Note that if the repeated objects overlap
                    some strange things can occur."
            },
            Self::NegativeRepetitions => {
                "The number of repetitions along the negative x, y, and z axes."
            }
            Self::PositiveRepetitions => {
                "The number of repetitions along the positive x, y, and z axes."
            }
            Self::Spacing => "The spacing along each positive axis to repeat the objects.",
            Self::BoundingVolume => indoc! {
                "If enabled, this object will act as a bounding volume
                    for all its children. This means that until a ray hits
                    the bounding volume, none of the child object's signed
                    distance fields will be computed. This can vastly
                    improve performance, especially when many complex
                    objects are far from the camera. This option does
                    not always play well with lighting effects that depend
                    on the number of iterations in the computation such
                    as 'ambient occlusion' and 'softened shadows' due
                    to the variation near the surface of the bounding object."
            },
            Self::BlendType => indoc! {
                "The type of interaction this object will have with its children.\n
                    \tUnion: All objects will appear as normal.\n
                    \tSubtraction: This object will be subtracted from all of its\n
                    \t\tchildren, leaving holes.\n
                    \tIntersection: Only the region where this object and its\n
                    \t\tchildren overlap will remain.\n
                    \tSmooth Union: All children will smoothly blend together\n
                    \t\twith this object according to the 'blend strength'.\n
                    \tSmooth Subtraction:This object will be subtracted from all\n
                    \t\tof its children,  leaving holes that are smoothed\n
                    \t\taccording to the 'blend strength'.\n
                    \tSmooth Intersection: Only the region where this object\n
                    \t\tand its children overlap will remain, and the remaining\n
                    \t\tregions will be smoothed according to the 'blend\n
                    \t\tstrength'.",
            },
            Self::BlendStrength => "The amount to blend with this primitive's children.",
            Self::Mirror => "Mirror along the x, y, and z axes.",
            Self::Hollow => {
                "If enabled, the object will be hollow, with a thickness of 'wall thickness'."
            }
            Self::WallThickness => {
                "The thickness of the walls of the shape, if the primitive is hollow."
            }
            Self::Elongate => "Enable the elongation of the object.",
            Self::Elongation => "The elongation of the object along the respective axes.",
            Self::Axis => "The world matrix/axis of the primitive.",
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

    fn tooltip(&self) -> &str {
        match self {
            Self::Id => "A primitive geometry object.",
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

    fn update_from_data_map(
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

        Self::update_from_data_map(scene_graph, data_map, &input_data_id)?;

        match output {
            Self::Outputs::Id => Ok(input_data_id),
        }
    }
}
