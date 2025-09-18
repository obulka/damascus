// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{textures::Texture, Enumerator};

use super::{InputData, NodeInputData};

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
