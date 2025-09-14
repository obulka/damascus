// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use super::super::inputs::input_data::InputData;

use crate::Enumerator;

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumIter,
    EnumCount,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum OutputData {
    Mat4,
    #[default]
    RenderPass,
    Scene,
}

impl Enumerator for OutputData {}

impl OutputData {
    pub fn can_connect_to_input(&self, input: &InputData) -> bool {
        match input {
            InputData::Mat4(..) => *self == OutputData::Mat4,
            InputData::RenderPass(..) => *self == OutputData::RenderPass,
            InputData::Scene(..) => *self == OutputData::Scene,
            _ => false,
        }
    }
}
