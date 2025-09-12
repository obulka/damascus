// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::{
    super::{input::InputId, output::OutputId},
    node_data::NodeData,
    NodeId,
};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
    pub data: NodeData,
}
