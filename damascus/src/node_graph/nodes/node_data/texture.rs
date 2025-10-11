// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::{node_data::EvaluableNode, NodeResult},
        outputs::output_data::{NodeOutputData, OutputData},
    },
    render_passes::{texture::view::TextureViewer, RenderPass, RenderPasses},
    scene_graph::SceneGraph,
    textures::Texture,
    Enumerator,
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
pub enum TextureInputData {
    #[default]
    Filepath,
}

impl Enumerator for TextureInputData {}

impl NodeInputData for TextureInputData {
    fn default_data(&self) -> InputData {
        let default_texture = Texture::default();
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
pub enum TextureOutputData {
    #[default]
    Texture,
}

impl Enumerator for TextureOutputData {}

impl NodeOutputData for TextureOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Texture => OutputData::RenderPass,
        }
    }
}

pub struct TextureNode;

impl EvaluableNode for TextureNode {
    type Inputs = TextureInputData;
    type Outputs = TextureOutputData;

    fn evaluate(
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::Texture => Ok(InputData::RenderPass(RenderPasses::TextureViewer {
                render_pass: TextureViewer::default()
                    .texture(Texture {
                        layers: 1,
                        filepath: Self::Inputs::Filepath
                            .get_data(data_map)?
                            .try_to_filepath()?,
                    })
                    .finalized(),
            })),
        }
    }
}
