// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::{super::nodes::NodeId, output_data::OutputData, OutputId};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub node_id: NodeId,
    pub name: String,
    pub data: OutputData,
}

impl Output {
    pub fn new(node_id: NodeId, name: String, data: OutputData) -> Self {
        Self {
            node_id: node_id,
            name: name,
            data: data,
        }
    }
}
