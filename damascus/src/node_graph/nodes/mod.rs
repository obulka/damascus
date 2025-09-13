// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use slotmap::SlotMap;

pub mod node;
pub mod node_data;

use node::Node;

slotmap::new_key_type! { pub struct NodeId; }

pub type Nodes = SlotMap<NodeId, Node>;
