// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::materials;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ProceduralTexture {
    value: materials::ProceduralTexture,
    ui_data: UIData,
}

impl Default for ProceduralTexture {
    fn default() -> Self {
        Self {
            value: materials::ProceduralTexture::default(),
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<materials::ProceduralTexture> for ProceduralTexture {
    fn new(value: materials::ProceduralTexture) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &materials::ProceduralTexture {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
