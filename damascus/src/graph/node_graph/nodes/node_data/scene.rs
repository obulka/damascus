// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::Mat4;
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{Root, RootId, SceneGraph, SceneGraphId, SceneGraphIdType},
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum SceneInputData {
    #[default]
    Scene,
    RenderCamera,
    Atmosphere,
    Axis,
}

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

#[derive(Copy, Default, EnumHashTraits!)]
pub enum SceneOutputData {
    #[default]
    RootId,
}

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

    fn output_data_is_compatible_with_input(
        output_data: &OutputData,
        input: &Self::Inputs,
    ) -> bool {
        match input {
            Self::Inputs::Scene => match *output_data {
                OutputData::SceneGraphId(location_type) => location_type.has_transform(),
                _ => false,
            },
            Self::Inputs::RenderCamera => {
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::Camera)
            }
            Self::Inputs::Atmosphere => {
                *output_data == OutputData::SceneGraphId(SceneGraphIdType::Material)
            }
            Self::Inputs::Axis => *output_data == OutputData::Mat4,
        }
    }

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let root_id: &RootId = input_data.as_root_id()?;

        scene_graph[*root_id].local_to_world =
            Self::Inputs::Axis.from_data_map(data_map)?.try_to_mat4()?;

        if let Ok(atmosphere_id) = Self::Inputs::Atmosphere
            .from_data_map(data_map)?
            .try_to_material_id()
        {
            scene_graph.set_atmosphere(*root_id, atmosphere_id);
        }

        if let Ok(render_camera_id) = Self::Inputs::RenderCamera
            .from_data_map(data_map)?
            .try_to_camera_id()
        {
            scene_graph.set_render_camera(*root_id, render_camera_id);
        }

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
        let root_id: RootId = match cached_input_data {
            Some(input_data) => input_data.try_to_root_id()?,
            None => scene_graph.add_root(Root::default()),
        };

        let scene_graph_id: SceneGraphId = root_id.into();
        let input_data_id = InputData::SceneGraphId(scene_graph_id);

        Self::add_dynamic_children_to_scene_graph(
            scene_graph,
            data_map,
            scene_graph_id,
            Self::Inputs::Scene,
        );

        Self::update_from_data_map(scene_graph, data_map, &input_data_id)?;

        match output {
            Self::Outputs::RootId => Ok(input_data_id),
        }
    }
}
