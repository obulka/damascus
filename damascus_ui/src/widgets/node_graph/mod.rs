// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus::graph::node_graph::{self, nodes::NodeId};

use super::Widget;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NodeGraphMessage {
    SetActiveNode(NodeId),
    ClearActiveNode,
    InputValueChanged(NodeId, String),
    CheckPreprocessorDirectives,
    ReconstructRenderResources,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    pub active_node: Option<NodeId>,
    pub node_graph: node_graph::NodeGraph,
}

impl Widget<NodeGraphMessage> for NodeGraph {}
