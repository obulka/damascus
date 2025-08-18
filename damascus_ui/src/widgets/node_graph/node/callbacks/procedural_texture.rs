// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use egui_node_graph::{Node, NodeId};

use damascus_core::materials::ProceduralTextureType;

use super::{
    super::{Graph, NodeData, NodeGraphResponse},
    NodeCallbacks, NodeGraph, NodeValueType, UIInput,
};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ProceduralTextureCallbacks;

impl ProceduralTextureCallbacks {
    fn use_trap_colour(graph: &Graph, node: &Node<NodeData>) -> bool {
        if let Ok(input_id) = node.get_input("use_trap_colour") {
            if let Some(input_param) = graph.inputs.get(input_id) {
                match input_param.value() {
                    NodeValueType::Bool { ref value } => return *value.value(),
                    _ => {}
                }
            }
        }
        false
    }
}

impl NodeCallbacks for ProceduralTextureCallbacks {
    fn input_value_changed(
        &self,
        node_graph: &mut NodeGraph,
        node_id: NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        if !["texture_type", "use_trap_colour"].contains(&input_name.as_str()) {
            return Vec::new();
        }
        let graph: &mut Graph = node_graph.graph_mut();

        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_name.as_str() {
                        "texture_type" => match input_param.value() {
                            NodeValueType::ComboBox { ref value } => {
                                match value.as_enum::<ProceduralTextureType>() {
                                    Ok(ProceduralTextureType::Grade) => {
                                        to_show.push("black_point");
                                        to_show.push("white_point");
                                        to_show.push("lift");
                                        to_show.push("gain");
                                        to_show.push("gamma");
                                        to_show.push("invert");
                                        to_show.push("use_trap_colour");
                                        if ProceduralTextureCallbacks::use_trap_colour(graph, node)
                                        {
                                            to_show.push("hue_rotation_angles");
                                        } else {
                                            to_hide.push("hue_rotation_angles");
                                        }

                                        to_hide.push("scale");
                                        to_hide.push("octaves");
                                        to_hide.push("lacunarity");
                                        to_hide.push("amplitude_gain");
                                        to_hide.push("low_frequency_scale");
                                        to_hide.push("high_frequency_scale");
                                        to_hide.push("low_frequency_translation");
                                        to_hide.push("high_frequency_translation");
                                    }
                                    Ok(ProceduralTextureType::Checkerboard) => {
                                        to_show.push("scale");
                                        to_show.push("black_point");
                                        to_show.push("white_point");
                                        to_show.push("lift");
                                        to_show.push("gain");
                                        to_show.push("gamma");
                                        to_show.push("invert");
                                        to_show.push("use_trap_colour");
                                        if ProceduralTextureCallbacks::use_trap_colour(graph, node)
                                        {
                                            to_show.push("hue_rotation_angles");
                                        } else {
                                            to_hide.push("hue_rotation_angles");
                                        }

                                        to_hide.push("octaves");
                                        to_hide.push("lacunarity");
                                        to_hide.push("amplitude_gain");
                                        to_hide.push("low_frequency_scale");
                                        to_hide.push("high_frequency_scale");
                                        to_hide.push("low_frequency_translation");
                                        to_hide.push("high_frequency_translation");
                                    }
                                    Ok(ProceduralTextureType::FBMNoise)
                                    | Ok(ProceduralTextureType::TurbulenceNoise) => {
                                        to_show.push("scale");
                                        to_show.push("black_point");
                                        to_show.push("white_point");
                                        to_show.push("lift");
                                        to_show.push("gain");
                                        to_show.push("octaves");
                                        to_show.push("lacunarity");
                                        to_show.push("amplitude_gain");
                                        to_show.push("gamma");
                                        to_show.push("low_frequency_scale");
                                        to_show.push("high_frequency_scale");
                                        to_show.push("low_frequency_translation");
                                        to_show.push("high_frequency_translation");
                                        to_show.push("invert");
                                        to_show.push("use_trap_colour");
                                        if ProceduralTextureCallbacks::use_trap_colour(graph, node)
                                        {
                                            to_show.push("hue_rotation_angles");
                                        } else {
                                            to_hide.push("hue_rotation_angles");
                                        }
                                    }
                                    _ => {
                                        to_hide.push("scale");
                                        to_hide.push("black_point");
                                        to_hide.push("white_point");
                                        to_hide.push("lift");
                                        to_hide.push("gain");
                                        to_hide.push("octaves");
                                        to_hide.push("lacunarity");
                                        to_hide.push("amplitude_gain");
                                        to_hide.push("gamma");
                                        to_hide.push("low_frequency_scale");
                                        to_hide.push("high_frequency_scale");
                                        to_hide.push("low_frequency_translation");
                                        to_hide.push("high_frequency_translation");
                                        to_hide.push("invert");
                                        to_show.push("use_trap_colour");
                                        if ProceduralTextureCallbacks::use_trap_colour(graph, node)
                                        {
                                            to_show.push("hue_rotation_angles");
                                        } else {
                                            to_hide.push("hue_rotation_angles");
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        "use_trap_colour" => match input_param.value() {
                            NodeValueType::Bool { ref value } => {
                                if *value.value() {
                                    to_show.push("hue_rotation_angles");
                                } else {
                                    to_hide.push("hue_rotation_angles");
                                }
                            }
                            _ => {}
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
