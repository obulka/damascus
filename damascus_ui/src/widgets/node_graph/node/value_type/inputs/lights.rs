// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use damascus_core::lights;

use super::{UIData, UIInput};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Lights {
    value: Vec<lights::Light>,
    ui_data: UIData,
}

impl UIInput<Vec<lights::Light>> for Lights {
    fn new(value: Vec<lights::Light>) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &Vec<lights::Light> {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
