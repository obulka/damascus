// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use indoc::indoc;
use macro_rules_attribute::derive;

use crate::{
    EnumTraits, Enumerator,
    graph::{
        node_graph::{NodeGraph, nodes::NodeId},
        scene_graph::SceneGraphIdType,
    },
};

#[derive(Copy, Default, EnumTraits!)]
pub enum OutputData {
    #[default]
    Mat4,
    SceneGraphId(SceneGraphIdType),
}

pub trait NodeOutputData: Enumerator + Eq {
    fn default_data(&self) -> OutputData;

    fn name(&self) -> String {
        self.to_string()
    }

    fn label(&self) -> String {
        self.variant_label()
    }

    fn tooltip(&self) -> &str {
        indoc! {
            "The one who built this node was selfish and lazy, and did
                not bother to write a tooltip."
        }
    }

    fn add_to_node(node_graph: &mut NodeGraph, node_id: NodeId) {
        Self::iter().for_each(|output| {
            node_graph.add_output(node_id, &output.name(), output.default_data());
        });
    }
}
