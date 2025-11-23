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
    textures::evaluators::{
        GPUTextureEvaluator, TextureEvaluators, read::TextureRead, view::TextureViewer,
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum TextureReadInputData {
    #[default]
    Filepath,
}

impl NodeInputData for TextureReadInputData {
    fn default_data(&self) -> InputData {
        let default_texture = TextureRead::default();
        match self {
            Self::Filepath => InputData::Filepath(default_texture.filepath),
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
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        // match output {
        //     Self::Outputs::Texture => Ok(InputData::SceneGraphId(
        //         scene_graph
        //             .add_texture_evaluator(TextureEvaluators::TextureViewer(
        //                 TextureViewer::default()
        //                     .texture(TextureRead {
        //                         layers: 1,
        //                         filepath: Self::Inputs::Filepath
        //                             .get_data(data_map)?
        //                             .try_to_filepath()?,
        //                     })
        //                     .finalized(),
        //             ))
        //             .into(),
        //     )),
        // }
        Err(NodeErrors::NotImplementedError)
    }
}
