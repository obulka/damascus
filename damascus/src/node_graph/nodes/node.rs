// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use super::{
    super::{
        inputs::{input::Input, InputId},
        outputs::{output::Output, OutputId},
        NodeGraph,
    },
    node_data::NodeData,
    NodeId,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub input_ids: Vec<InputId>,
    pub output_ids: Vec<OutputId>,
    pub data: NodeData,
}
