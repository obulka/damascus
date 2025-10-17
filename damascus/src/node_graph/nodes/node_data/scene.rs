// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::Mat4;
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeResult,
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene_graph::{Root, RootId, SceneGraph, SceneGraphId, SceneGraphIdType},
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
pub enum SceneInputData {
    #[default]
    Scene,
    RenderCamera,
    Atmosphere,
    Axis,
}

impl Enumerator for SceneInputData {}

impl NodeInputData for SceneInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Scene => InputData::SceneGraphId(SceneGraphId::None),
            Self::RenderCamera => InputData::SceneGraphId(SceneGraphId::None),
            Self::Atmosphere => InputData::SceneGraphId(SceneGraphId::None),
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
pub enum SceneOutputData {
    #[default]
    RootId,
}

impl Enumerator for SceneOutputData {}

impl NodeOutputData for SceneOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::RootId => OutputData::SceneGraphId(SceneGraphIdType::Root),
        }
    }
}

pub struct SceneNode;

impl EvaluableNode for SceneNode {
    type Inputs = SceneInputData;
    type Outputs = SceneOutputData;

    fn dynamic_inputs() -> impl Iterator<Item = Self::Inputs> {
        vec![Self::Inputs::Scene].into_iter()
    }

    fn output_is_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input {
            Self::Inputs::Scene => match *output {
                OutputData::SceneGraphId(location_type) => location_type.has_transform(),
                _ => false,
            },
            Self::Inputs::RenderCamera => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::Camera)
            }
            Self::Inputs::Atmosphere => {
                *output == OutputData::SceneGraphId(SceneGraphIdType::Material)
            }
            Self::Inputs::Axis => *output == OutputData::Mat4,
        }
    }

    fn evaluate(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let root_id: RootId = scene_graph.add_root(Root {
            atmosphere_id: Self::Inputs::Atmosphere
                .get_data(data_map)?
                .try_to_material_id()
                .ok(),
            render_camera_id: Self::Inputs::Atmosphere
                .get_data(data_map)?
                .try_to_camera_id()
                .ok(),
            local_to_world: Self::Inputs::Axis.get_data(data_map)?.try_to_mat4()?,
        });
        let scene_graph_id = SceneGraphId::Root(root_id);

        Self::add_dynamic_children_to_scene_graph(
            scene_graph,
            data_map,
            scene_graph_id,
            Self::Inputs::Scene,
        );

        match output {
            Self::Outputs::RootId => Ok(InputData::SceneGraphId(scene_graph_id)),
        }
    }
}
