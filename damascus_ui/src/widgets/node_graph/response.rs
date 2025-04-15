// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_node_graph::{NodeId, UserResponseTrait};

use super::node::NodeTemplate;

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NodeGraphResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    InputValueChanged(NodeId, NodeTemplate, String),
    CheckPreprocessorDirectives,
    ReconstructRenderResources,
}

impl UserResponseTrait for NodeGraphResponse {}
