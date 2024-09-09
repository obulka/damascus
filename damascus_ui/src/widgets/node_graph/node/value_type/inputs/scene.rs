// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::scene;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Scene {
    value: scene::Scene,
    ui_data: UIData,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            value: scene::Scene::default(),
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<scene::Scene> for Scene {
    fn new(value: scene::Scene) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &scene::Scene {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
