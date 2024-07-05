// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use egui_node_graph::{GraphEditorState, NodeId, UserStateTrait};

use super::node::{value_type::NodeValueType, NodeData, NodeDataType, NodeTemplate};

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NodeGraphState {
    pub active_node: Option<NodeId>,
}

impl UserStateTrait for NodeGraphState {}

pub type NodeGraphEditorState =
    GraphEditorState<NodeData, NodeDataType, NodeValueType, NodeTemplate, NodeGraphState>;
