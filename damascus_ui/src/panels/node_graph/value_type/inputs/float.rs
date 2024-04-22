// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use core::ops::RangeInclusive;

use super::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Float {
    value: f32,
    ui_data: UIData,
    pub range: RangeInclusive<f32>,
}

impl Default for Float {
    fn default() -> Self {
        Self {
            value: 0.,
            ui_data: UIData::default(),
            range: 0.0..=1.,
        }
    }
}

impl UIInput<f32> for Float {
    fn new(value: f32) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &f32 {
        &self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl RangedInput<f32> for Float {
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    fn range_mut(&mut self) -> &mut RangeInclusive<f32> {
        &mut self.range
    }

    fn range(&self) -> &RangeInclusive<f32> {
        &self.range
    }
}
