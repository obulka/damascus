// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use egui_node_graph::{NodeId, UserResponseTrait};

use super::node::NodeTemplate;

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeGraphResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    InputValueChanged(NodeId, NodeTemplate, String),
}

impl UserResponseTrait for NodeGraphResponse {}
