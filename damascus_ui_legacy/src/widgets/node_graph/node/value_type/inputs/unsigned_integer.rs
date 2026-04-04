// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use core::ops::RangeInclusive;

use super::{RangedInput, UIData, UIInput};

#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UnsignedInteger {
    value: u32,
    ui_data: UIData,
    pub range: RangeInclusive<u32>,
}

impl Default for UnsignedInteger {
    fn default() -> Self {
        Self {
            value: 0,
            ui_data: UIData::default(),
            range: 0..=10,
        }
    }
}

impl UIInput<u32> for UnsignedInteger {
    fn new(value: u32) -> Self {
        Self {
            value: value,
            ..Default::default()
        }
    }

    fn value(&self) -> &u32 {
        &self.value
    }

    fn deref(self) -> u32 {
        self.value
    }

    fn ui_data(&self) -> &UIData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UIData {
        &mut self.ui_data
    }
}

impl RangedInput<u32, u32> for UnsignedInteger {
    fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }

    fn range_mut(&mut self) -> &mut RangeInclusive<u32> {
        &mut self.range
    }

    fn range(&self) -> &RangeInclusive<u32> {
        &self.range
    }
}
