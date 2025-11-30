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
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
    textures::evaluators::{
        TextureEvaluator, TextureEvaluatorId, TextureEvaluators, read::TextureReader,
    },
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

    fn evaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let texture_evaluator_id: TextureEvaluatorId =
            scene_graph.add_texture_evaluator(TextureEvaluators::TextureReader(
                TextureReader::default()
                    .filepath(
                        Self::Inputs::Filepath
                            .get_data(data_map)?
                            .try_to_filepath()?,
                    )
                    .finalize(),
            ));

        let scene_graph_id = SceneGraphId::TextureEvaluator(texture_evaluator_id);

        scene_graph[texture_evaluator_id].evaluate(device, queue, encoder);

        match output {
            Self::Outputs::Texture => Ok(InputData::SceneGraphId(scene_graph_id)),
        }
    }
}
