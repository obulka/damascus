// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    render_passes::{texture::view::TextureViewer, RenderPass, RenderPasses},
    textures::Texture,
    Enumerator,
};

use super::{InputData, InputResult, NodeInputData};

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

    fn compute_output(data_map: &mut HashMap<String, InputData>) -> InputResult<InputData> {
        Ok(InputData::RenderPass(RenderPasses::TextureViewer {
            render_pass: TextureViewer::default()
                .texture(Texture {
                    layers: 1,
                    filepath: Self::Filepath.get_data(data_map)?.try_to_filepath()?,
                })
                .finalized(),
        }))
    }
}
