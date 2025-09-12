// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

slotmap::new_key_type! { pub struct NodeId; }

use super::{node::Node, NodeId};

pub mod node;
pub mod node_data;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Nodes {
    nodes: SlotMap<NodeId, Node>,
}
