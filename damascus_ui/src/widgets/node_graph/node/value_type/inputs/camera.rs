// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus::camera;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Camera {
    value: camera::Camera,
    ui_data: UIData,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            value: camera::Camera::default(),
            ui_data: UIData::default(),
        }
    }
}

impl UIInput<camera::Camera> for Camera {
    fn new(value: camera::Camera) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &camera::Camera {
        &self.value
    }

    fn deref(self) -> camera::Camera {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
