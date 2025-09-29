// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    lights::{Light, Lights},
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeResult,
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene::Scene,
    Enumerator,
};

use super::NodeOperation;

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
    SceneGraph,
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
            Self::SceneGraph => InputData::SceneGraph(Scene::default()),
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
    SceneGraph,
}

impl Enumerator for LightOutputData {}

impl NodeOutputData for LightOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::SceneGraph => OutputData::SceneGraph,
        }
    }
}

pub struct LightNode;

impl NodeOperation for LightNode {
    type Inputs = LightInputData;
    type Outputs = LightOutputData;

    fn evaluate(
        output: Self::Outputs,
        data_map: &mut HashMap<String, InputData>,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::SceneGraph => {
                let mut scene: Scene = Self::Inputs::SceneGraph
                    .get_data(data_map)?
                    .try_to_scene()?;
                let local_to_world: Mat4 = Self::Inputs::Axis.get_data(data_map)?.try_to_mat4()?;
                let light_type: Lights =
                    Self::Inputs::LightType.get_data(data_map)?.try_to_enum()?;

                let dimensional_data: Vec3 = match light_type {
                    Lights::Directional => (local_to_world
                        * Vec4::from((
                            Self::Inputs::Direction.get_data(data_map)?.try_to_vec3()?,
                            1.,
                        )))
                    .xyz()
                    .normalize(),
                    Lights::Point => (local_to_world
                        * Vec4::from((
                            Self::Inputs::Position.get_data(data_map)?.try_to_vec3()?,
                            1.,
                        )))
                    .xyz(),
                    Lights::AmbientOcclusion => Vec3::new(
                        Self::Inputs::Iterations.get_data(data_map)?.try_to_uint()? as f32,
                        0.,
                        0.,
                    ),
                    _ => Vec3::ZERO,
                };

                scene.lights.push(Light {
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

                Ok(InputData::SceneGraph(scene))
            }
        }
    }
}
