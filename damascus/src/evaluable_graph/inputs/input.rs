// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crate::evaluable_graph::nodes::NodeId;

use super::{input_data::InputData, InputId};

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
}
