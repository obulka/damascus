// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::{super::nodes::NodeId, output_data::OutputData, OutputId};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub id: OutputId,
    pub node_id: NodeId,
    pub data: OutputData,
}

impl Output {
    pub fn new(id: OutputId, node_id: NodeId, data: OutputData) -> Self {
        Self {
            id: id,
            node_id: node_id,
            data: data,
        }
    }
}
