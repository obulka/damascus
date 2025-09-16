// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{render_passes::RenderPasses, textures::Grade, Enumerator};

use super::{InputData, NodeInputData};

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum GradeInputData {
    #[default]
    Texture,
    BlackPoint,
    WhitePoint,
    Lift,
    Gain,
    Gamma,
    Invert,
}

impl Enumerator for GradeInputData {}

impl NodeInputData for GradeInputData {
    fn default_data(&self) -> InputData {
        let default_grade = Grade::default();
        match self {
            GradeInputData::Texture => InputData::RenderPass(RenderPasses::Black),
            GradeInputData::BlackPoint => InputData::Float(default_grade.black_point),
            GradeInputData::WhitePoint => InputData::Float(default_grade.white_point),
            GradeInputData::Lift => InputData::Float(default_grade.lift),
            GradeInputData::Gain => InputData::Float(default_grade.gain),
            GradeInputData::Gamma => InputData::Float(default_grade.gamma),
            GradeInputData::Invert => InputData::Bool(default_grade.invert),
        }
    }
}
