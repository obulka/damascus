// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::textures;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Texture {
    value: textures::Texture,
    ui_data: UIData,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            value: textures::Texture::default(),
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<textures::Texture> for Texture {
    fn new(value: textures::Texture) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &textures::Texture {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
