// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::render_passes;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPasses {
    value: Vec<render_passes::RenderPasses>,
    ui_data: UIData,
}

impl Default for RenderPasses {
    fn default() -> Self {
        Self {
            value: vec![],
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<Vec<render_passes::RenderPasses>> for RenderPasses {
    fn new(value: Vec<render_passes::RenderPasses>) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &Vec<render_passes::RenderPasses> {
        &self.value
    }

    fn deref(self) -> Vec<render_passes::RenderPasses> {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
