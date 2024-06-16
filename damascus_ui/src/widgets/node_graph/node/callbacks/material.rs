// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use egui_node_graph::{InputId, NodeId, OutputId};

use super::{super::NodeGraphResponse, Graph, NodeCallbacks};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct MaterialCallbacks;

impl NodeCallbacks for MaterialCallbacks {
    fn input_connected(
        &self,
        graph: &mut Graph,
        input_id: InputId,
        _output_id: OutputId,
    ) -> Vec<NodeGraphResponse> {
        let input = graph.get_input(input_id);
        let node_id: NodeId = input.node;

        let mut input_name = String::new();
        for (param_name, param_id) in &graph[node_id].inputs {
            if *param_id == input_id {
                input_name = param_name.to_string();
                break;
            }
        }

        if !input_name.ends_with("_texture") {
            return Vec::new();
        }
        vec![NodeGraphResponse::CheckPreprocessorDirectives]
    }

    fn input_disconnected(
        &self,
        graph: &mut Graph,
        input_id: InputId,
        _output_id: OutputId,
    ) -> Vec<NodeGraphResponse> {
        let input = graph.get_input(input_id);
        let node_id: NodeId = input.node;

        let mut input_name = String::new();
        for (param_name, param_id) in &graph[node_id].inputs {
            if *param_id == input_id {
                input_name = param_name.to_string();
                break;
            }
        }

        if !input_name.ends_with("_texture") {
            return Vec::new();
        }
        vec![NodeGraphResponse::CheckPreprocessorDirectives]
    }
}
