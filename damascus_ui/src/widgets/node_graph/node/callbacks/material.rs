// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use egui_node_graph::NodeId;

use damascus_core::materials;

use super::{Graph, NodeCallbacks, NodeValueType};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct MaterialCallbacks;

impl NodeCallbacks for MaterialCallbacks {
    fn input_value_changed(&self, graph: &mut Graph, node_id: NodeId, input_name: &String) {
        if !input_name.ends_with("_texture") {
            println!("Material Callback for non texture {:?}", input_name);
            return;
        }

        println!("Material Callback for {:?}", input_name);
    }
}
