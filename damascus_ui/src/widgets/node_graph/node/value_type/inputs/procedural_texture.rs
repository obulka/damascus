// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use damascus_core::materials;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
