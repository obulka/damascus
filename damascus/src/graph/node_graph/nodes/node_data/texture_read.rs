// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::SceneGraph,
    },
    textures::evaluators::{
        GPUTextureEvaluator, TextureEvaluator, read::TextureRead, view::TextureViewer,
    },
};

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
pub enum TextureReadInputData {
    #[default]
    Filepath,
}

impl Enumerator for TextureReadInputData {}

impl NodeInputData for TextureReadInputData {
    fn default_data(&self) -> InputData {
        let default_texture = TextureRead::default();
        match self {
            Self::Filepath => InputData::Filepath(default_texture.filepath),
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
pub enum TextureReadOutputData {
    #[default]
    Texture,
}

impl Enumerator for TextureReadOutputData {}

impl NodeOutputData for TextureReadOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Texture => OutputData::TextureEvaluator,
        }
    }
}

pub struct TextureReadNode;

impl EvaluableNode for TextureReadNode {
    type Inputs = TextureReadInputData;
    type Outputs = TextureReadOutputData;

    fn evaluate(
        _scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::Texture => Ok(InputData::TextureEvaluator(
                TextureEvaluator::TextureViewer {
                    texture_evaluator: TextureViewer::default()
                        .texture(TextureRead {
                            layers: 1,
                            filepath: Self::Inputs::Filepath
                                .get_data(data_map)?
                                .try_to_filepath()?,
                        })
                        .finalized(),
                },
            )),
        }
    }
}
