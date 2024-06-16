// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use damascus_core::geometry::camera;

use super::{UIData, UIInput};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
