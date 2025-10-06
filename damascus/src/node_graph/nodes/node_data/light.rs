// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    lights::{Light, LightId, LightType},
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeResult,
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    Enumerator,
};

use super::EvaluableNode;

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
pub enum LightInputData {
    #[default]
    Siblings,
    Children,
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

impl Enumerator for LightInputData {}

impl NodeInputData for LightInputData {
    fn default_data(&self) -> InputData {
        let default_light = Light::default();
        match self {
            Self::Siblings | Self::Children => InputData::SceneGraphId(SceneGraphId::None),
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
pub enum LightOutputData {
    #[default]
    Id,
}

impl Enumerator for LightOutputData {}

impl NodeOutputData for LightOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Id => OutputData::SceneGraphId(SceneGraphIdType::Light),
        }
    }
}

pub struct LightNode;

impl EvaluableNode for LightNode {
    type Inputs = LightInputData;
    type Outputs = LightOutputData;

    fn output_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input {
            Self::Inputs::Siblings | Self::Inputs::Children => match *output {
                OutputData::SceneGraphId(location_type) => location_type.has_transform(),
                _ => false,
            },
            Self::Inputs::Axis => *output == OutputData::Mat4,
            _ => false,
        }
    }

    fn evaluate(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let siblings_id: SceneGraphId = Self::Inputs::Siblings
            .get_data(data_map)?
            .try_to_scene_graph_id()?;
        let children_id: SceneGraphId = Self::Inputs::Children
            .get_data(data_map)?
            .try_to_scene_graph_id()?;

        let local_to_world: Mat4 = Self::Inputs::Axis.get_data(data_map)?.try_to_mat4()?;
        let light_type: LightType = Self::Inputs::LightType.get_data(data_map)?.try_to_enum()?;

        let dimensional_data: Vec3 = match light_type {
            LightType::Directional => (local_to_world
                * Vec4::from((
                    Self::Inputs::Direction.get_data(data_map)?.try_to_vec3()?,
                    1.,
                )))
            .xyz()
            .normalize(),
            LightType::Point => (local_to_world
                * Vec4::from((
                    Self::Inputs::Position.get_data(data_map)?.try_to_vec3()?,
                    1.,
                )))
            .xyz(),
            LightType::AmbientOcclusion => Vec3::new(
                Self::Inputs::Iterations.get_data(data_map)?.try_to_uint()? as f32,
                0.,
                0.,
            ),
            _ => Vec3::ZERO,
        };

        let light_id: LightId = scene_graph.add_light(Light {
            light_type: light_type,
            dimensional_data: dimensional_data,
            intensity: Self::Inputs::Intensity.get_data(data_map)?.try_to_float()?,
            falloff: Self::Inputs::Falloff.get_data(data_map)?.try_to_uint()?,
            colour: Self::Inputs::Colour.get_data(data_map)?.try_to_vec3()?,
            shadow_hardness: Self::Inputs::ShadowHardness
                .get_data(data_map)?
                .try_to_float()?,
            soften_shadows: Self::Inputs::SoftenShadows
                .get_data(data_map)?
                .try_to_bool()?,
        });

        match output {
            Self::Outputs::SceneGraph => Ok(InputData::SceneGraph(scene_graph)),
        }
    }
}
