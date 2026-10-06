// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use slotmap::SparseSecondaryMap;

use damascus::graph::node_graph::nodes::NodeId;

pub mod node_data;

pub use node_data::NodeData;

pub type NodeGraphUIState = SparseSecondaryMap<NodeId, NodeData>;
