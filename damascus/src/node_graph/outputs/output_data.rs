// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    node_graph::{NodeGraph, nodes::NodeId},
    scene_graph::SceneGraphIdType,
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
    #[default]
    Mat4,
    RenderPass,
    SceneGraphId(SceneGraphIdType),
}

impl Enumerator for OutputData {}

pub trait NodeOutputData: Enumerator + Eq {
    fn default_data(&self) -> OutputData;

    fn name(&self) -> String {
        self.to_string()
    }

    fn label(&self) -> String {
        self.variant_label()
    }

    fn add_to_node(graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|output| {
            graph.add_output(node_id, &output.name(), output.default_data());
        });
    }
}
