// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::{super::nodes::NodeId, input_data::InputData, InputId};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub id: InputId,
    pub node_id: NodeId,
    pub data: InputData,
}

impl Input {
    pub fn new(id: InputId, node_id: NodeId, data: InputData) -> Self {
        Self {
            id: id,
            node_id: node_id,
            data: data,
        }
    }
}
