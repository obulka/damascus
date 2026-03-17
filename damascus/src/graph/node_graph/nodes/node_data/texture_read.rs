// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeErrors, NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphIdType},
    },
    textures::evaluators::{TextureEvaluatorId, TextureEvaluators, read::TextureReader},
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum TextureReadInputData {
    #[default]
    Filepath,
}

impl NodeInputData for TextureReadInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Filepath => InputData::Filepath(String::new()),
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum TextureReadOutputData {
    #[default]
    Texture,
}

impl NodeOutputData for TextureReadOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Texture => OutputData::SceneGraphId(SceneGraphIdType::TextureEvaluator),
        }
    }
}

pub struct TextureReadNode;

impl EvaluableNode for TextureReadNode {
    type Inputs = TextureReadInputData;
    type Outputs = TextureReadOutputData;

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let texture_evaluator_id: &TextureEvaluatorId = input_data.as_texture_evaluator_id()?;

        match &mut scene_graph[*texture_evaluator_id] {
            TextureEvaluators::TextureReader(texture_reader) => {
                texture_reader.set_filepath(
                    Self::Inputs::Filepath
                        .from_data_map(data_map)?
                        .try_to_filepath()?,
                );

                Ok(())
            }
            _ => Err(NodeErrors::InvalidCachedData),
        }
    }

    fn evaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        cached_input_data: Option<InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let texture_evaluator_id: TextureEvaluatorId = match cached_input_data {
            Some(input_data) => input_data.try_to_texture_evaluator_id()?,
            None => scene_graph
                .add_texture_evaluator(TextureEvaluators::TextureReader(TextureReader::default())),
        };

        let scene_graph_id = InputData::SceneGraphId(texture_evaluator_id.into());

        Self::update_from_data_map(scene_graph, data_map, &scene_graph_id)?;

        scene_graph[texture_evaluator_id].evaluate(device, queue, encoder);

        match output {
            Self::Outputs::Texture => Ok(InputData::SceneGraphId(texture_evaluator_id.into())),
        }
    }
}
