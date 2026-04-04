// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_node_graph::NodeId;

use super::{
    super::{Graph, NodeGraphResponse},
    NodeCallbacks, NodeGraph, NodeValueType, UIInput,
};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct RayMarcherCallbacks;

impl NodeCallbacks for RayMarcherCallbacks {
    fn input_value_changed(
        &self,
        node_graph: &mut NodeGraph,
        node_id: NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        if input_name != "light_sampling" {
            return Vec::new();
        }
        let graph: &mut Graph = node_graph.graph_mut();
        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_param.value() {
                        NodeValueType::Bool { ref value } => {
                            if *value.value() {
                                to_show.push("sample_atmosphere");
                                to_show.push("light_sampling_bias");
                                to_show.push("secondary_sampling");
                                to_show.push("max_light_sampling_bounces");
                                to_show.push("sample_atmosphere");
                            } else {
                                to_hide.push("sample_atmosphere");
                                to_hide.push("light_sampling_bias");
                                to_hide.push("secondary_sampling");
                                to_hide.push("max_light_sampling_bounces");
                                to_hide.push("sample_atmosphere");
                            }
                        }
                        _ => {}
                    }
                }
            }

            for input_name in to_hide.iter() {
                if let Ok(input_id) = node.get_input(input_name) {
                    if let Some(input_param) = graph.inputs.get_mut(input_id) {
                        self.hide_input(&mut input_param.value)
                    }
                }
            }
            for input_name in to_show.iter() {
                if let Ok(input_id) = node.get_input(input_name) {
                    if let Some(input_param) = graph.inputs.get_mut(input_id) {
                        self.show_input(&mut input_param.value)
                    }
                }
            }
        }
        Vec::new()
    }
}
