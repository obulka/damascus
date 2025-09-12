// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_node_graph::NodeId;

use damascus::lights;

use super::{
    super::{Graph, NodeGraphResponse},
    NodeCallbacks, NodeGraph, NodeValueType, UIInput,
};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LightCallbacks;

impl NodeCallbacks for LightCallbacks {
    fn input_value_changed(
        &self,
        node_graph: &mut NodeGraph,
        node_id: NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        if input_name != "light_type" {
            return Vec::new();
        }
        let graph: &mut Graph = node_graph.graph_mut();
        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_param.value() {
                        NodeValueType::ComboBox { value } => match value.value().as_enumerator() {
                            lights::Lights::Directional => {
                                to_show.push("direction");
                                to_hide.push("iterations");
                                to_hide.push("position")
                            }
                            lights::Lights::Point => {
                                to_hide.push("direction");
                                to_hide.push("iterations");
                                to_show.push("position");
                            }
                            lights::Lights::AmbientOcclusion => {
                                to_hide.push("direction");
                                to_show.push("iterations");
                                to_hide.push("position")
                            }
                            _ => {
                                to_hide.push("direction");
                                to_hide.push("iterations");
                                to_hide.push("position");
                            }
                        },
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
