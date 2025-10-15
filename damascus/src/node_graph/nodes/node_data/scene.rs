// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene_graph::SceneGraph,
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
pub enum SceneInputData {
    #[default]
    Scene0,
    Scene1,
}

impl Enumerator for SceneInputData {}

impl NodeInputData for SceneInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Scene0 => InputData::SceneGraph(SceneGraph::default()),
            Self::Scene1 => InputData::SceneGraph(SceneGraph::default()),
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
pub enum SceneOutputData {
    #[default]
    SceneGraph,
}

impl Enumerator for SceneOutputData {}

impl NodeOutputData for SceneOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::SceneGraph => OutputData::SceneGraph,
        }
    }
}

pub struct SceneNode;

impl EvaluableNode for SceneNode {
    type Inputs = SceneInputData;
    type Outputs = SceneOutputData;
}
