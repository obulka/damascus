// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    evaluable_graph::{
        inputs::input_data::{InputData, NodeInputData},
        outputs::output_data::{NodeOutputData, OutputData},
    },
    render_passes::RenderPasses,
    textures::Grade,
    Enumerator,
};

use super::EvaluableNode;

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
            Self::Texture => InputData::RenderPass(RenderPasses::Black),
            Self::BlackPoint => InputData::Float(default_grade.black_point),
            Self::WhitePoint => InputData::Float(default_grade.white_point),
            Self::Lift => InputData::Float(default_grade.lift),
            Self::Gain => InputData::Float(default_grade.gain),
            Self::Gamma => InputData::Float(default_grade.gamma),
            Self::Invert => InputData::Bool(default_grade.invert),
        }
    }
}

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
pub enum GradeOutputData {
    #[default]
    GradedImage,
}

impl Enumerator for GradeOutputData {}

impl NodeOutputData for GradeOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::GradedImage => OutputData::RenderPass,
        }
    }
}

pub struct GradeNode;

impl EvaluableNode for GradeNode {
    type Inputs = GradeInputData;
    type Outputs = GradeOutputData;
}
