// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::ops::{Range, RangeInclusive};

use crate::graph::node_graph::{NodeGraph, inputs::input_data::InputData, nodes::NodeId};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub node_id: NodeId,
    pub name: String,
    pub data: InputData,
}

impl Input {
    pub fn new(node_id: NodeId, name: String, data: InputData) -> Self {
        Self {
            node_id: node_id,
            name: name,
            data: data,
        }
    }

    pub fn input_help(&self, node_graph: &NodeGraph) -> String {
        node_graph[self.node_id].data.input_help(&self.name)
    }

    pub fn input_suggested_range(&self, node_graph: &NodeGraph) -> Option<RangeInclusive<f64>> {
        node_graph[self.node_id]
            .data
            .input_suggested_range(&self.name)
    }

    pub fn input_range(&self, node_graph: &NodeGraph) -> Option<Range<f64>> {
        node_graph[self.node_id].data.input_range(&self.name)
    }
}
