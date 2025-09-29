// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{inputs::input_data::InputData, nodes::NodeId, NodeGraph},
    Enumerator,
};

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
    SceneGraph,
}

impl Enumerator for OutputData {}

impl OutputData {
    pub fn can_connect_to_input(&self, input: &InputData) -> bool {
        match input {
            InputData::Mat4(..) => *self == OutputData::Mat4,
            InputData::RenderPass(..) => *self == OutputData::RenderPass,
            InputData::SceneGraph(..) => *self == OutputData::SceneGraph,
            _ => false,
        }
    }
}

pub trait NodeOutputData: Enumerator + Eq {
    fn default_data(&self) -> OutputData;

    fn name(&self) -> String {
        self.to_string()
    }

    fn label(&self) -> String {
        self.variant_snake_case()
    }

    fn add_to_node(node_graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|output| {
            node_graph.add_output(node_id, &output.name(), output.default_data());
        });
    }
}
