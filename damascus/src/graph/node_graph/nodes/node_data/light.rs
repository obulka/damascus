// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use indoc::indoc;
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    lights::{Light, LightId, LightType},
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum LightInputData {
    #[default]
    Child,
    LightType,
    Direction,
    Position,
    Iterations,
    Intensity,
    Falloff,
    Colour,
    ShadowHardness,
    SoftenShadows,
    Axis,
}

impl NodeInputData for LightInputData {
    fn default_data(&self) -> InputData {
        let default_light = Light::default();
        match self {
            Self::Child => InputData::SceneGraphId(SceneGraphId::None),
            Self::LightType => InputData::Enum(default_light.light_type.into()),
            Self::Direction => InputData::Vec3(Vec3::NEG_Y),
            Self::Position => InputData::Vec3(Vec3::Y),
            Self::Iterations => InputData::UInt(default_light.dimensional_data.x as u32),
            Self::Intensity => InputData::Float(default_light.intensity),
            Self::Falloff => InputData::UInt(default_light.falloff),
            Self::Colour => InputData::Vec3(default_light.colour),
            Self::ShadowHardness => InputData::Float(default_light.shadow_hardness),
            Self::SoftenShadows => InputData::Bool(default_light.soften_shadows),
            Self::Axis => InputData::Mat4(Mat4::IDENTITY),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::Child => indoc! {
                "Chain other non-physical lights in the scene, their axes will
                    be parented to this lights axis."
            },
            Self::LightType => indoc! {
                "The type of non-physical light to create.\n
                    \tPoint: A point light.\n
                    \tDirectional: A directional light.\n
                    \tAmbient: An ambient light (will be a uniform colour).\n
                    \tAmbient Occlusion: Ambient occlusion."
            },
            Self::Direction => "The direction vector of the light.",
            Self::Position => "The position of the point light.",
            Self::Iterations => "The number of iterations used to compute the occlusion.",
            Self::Intensity => "The time-averaged power on the surface of the light.",
            Self::Falloff => "The exponent of the falloff (point lights only).",
            Self::Colour => "The light colour.",
            Self::ShadowHardness => "The hardness of softened shadows.",
            Self::SoftenShadows => indoc! {
                "If enabled, the shadows will be softened (directional and
                    point lights only)."
            },
            Self::Axis => indoc! {
                "The world matrix to apply to the light (point and
                    directional only).\n
                    \tPoint: Will affect the position of the light.\n
                    \tDirectional: Will affect the direction vector of the light."
            },
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum LightOutputData {
    #[default]
    Id,
}

impl NodeOutputData for LightOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Id => OutputData::SceneGraphId(SceneGraphIdType::Light),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::Id => indoc! {
                "An non-physical light. For a physical light use an emissive
                    material on a primitive."
            },
        }
    }
}

pub struct LightNode;

impl EvaluableNode for LightNode {
    type Inputs = LightInputData;
    type Outputs = LightOutputData;

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
            Self::Inputs::Axis => *output_data == OutputData::Mat4,
            _ => false,
        }
    }

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let light_id: &LightId = input_data.as_light_id()?;

        let local_to_world: Mat4 = Self::Inputs::Axis.from_data_map(data_map)?.try_to_mat4()?;
        let light_type: LightType = Self::Inputs::LightType
            .from_data_map(data_map)?
            .try_to_enum()?;

        let dimensional_data: Vec3 = match light_type {
            LightType::Directional => (local_to_world
                * Vec4::from((
                    Self::Inputs::Direction
                        .from_data_map(data_map)?
                        .try_to_vec3()?,
                    1.,
                )))
            .xyz()
            .normalize(),
            LightType::Point => (local_to_world
                * Vec4::from((
                    Self::Inputs::Position
                        .from_data_map(data_map)?
                        .try_to_vec3()?,
                    1.,
                )))
            .xyz(),
            LightType::AmbientOcclusion => Vec3::new(
                Self::Inputs::Iterations
                    .from_data_map(data_map)?
                    .try_to_uint()? as f32,
                0.,
                0.,
            ),
            _ => Vec3::ZERO,
        };

        scene_graph[*light_id].light_type = light_type;
        scene_graph[*light_id].dimensional_data = dimensional_data;
        scene_graph[*light_id].intensity = Self::Inputs::Intensity
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*light_id].falloff = Self::Inputs::Falloff
            .from_data_map(data_map)?
            .try_to_uint()?;
        scene_graph[*light_id].colour = Self::Inputs::Colour
            .from_data_map(data_map)?
            .try_to_vec3()?;
        scene_graph[*light_id].shadow_hardness = Self::Inputs::ShadowHardness
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*light_id].soften_shadows = Self::Inputs::SoftenShadows
            .from_data_map(data_map)?
            .try_to_bool()?;

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
        let light_id: LightId = match cached_input_data {
            Some(input_data) => input_data.try_to_light_id()?,
            None => scene_graph.add_light(Light::default()),
        };

        let scene_graph_id: SceneGraphId = light_id.into();
        let input_data_id = InputData::SceneGraphId(scene_graph_id);

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
