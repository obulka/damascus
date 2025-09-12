// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus::geometry::primitive::Primitive;

use super::{UIData, UIInput};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Primitives {
    value: Vec<Primitive>,
    ui_data: UIData,
}

impl UIInput<Vec<Primitive>> for Primitives {
    fn new(value: Vec<Primitive>) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &Vec<Primitive> {
        &self.value
    }

    fn deref(self) -> Vec<Primitive> {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
