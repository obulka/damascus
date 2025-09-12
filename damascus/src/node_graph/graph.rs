// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use slotmap::SlotMap;

use super::{edges::Edges, inputs::Inputs, nodes::Nodes, outputs::Outputs};

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Graph {
    pub nodes: Nodes,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub edges: Edges,
}
