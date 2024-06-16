// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use damascus_core::geometry;

use super::{UIData, UIInput};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Primitives {
    value: Vec<geometry::Primitive>,
    ui_data: UIData,
}

impl UIInput<Vec<geometry::Primitive>> for Primitives {
    fn new(value: Vec<geometry::Primitive>) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &Vec<geometry::Primitive> {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}
