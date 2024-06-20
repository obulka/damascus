// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use egui_node_graph::NodeId;

use damascus_core::materials::ProceduralTextureType;

use super::{super::NodeGraphResponse, Graph, NodeCallbacks, NodeValueType};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct ProceduralTextureCallbacks;

impl NodeCallbacks for ProceduralTextureCallbacks {
    fn input_value_changed(
        &self,
        graph: &mut Graph,
        node_id: NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        if input_name != "texture_type" {
            return Vec::new();
        }
        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_param.value() {
                        NodeValueType::ComboBox { ref value } => {
                            match value.as_enum::<ProceduralTextureType>() {
                                Ok(ProceduralTextureType::Grade) => {
                                    to_show.push("black_point");
                                    to_show.push("white_point");
                                    to_show.push("lift");
                                    to_show.push("gain");
                                    to_show.push("gamma");
                                    to_show.push("invert");

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
                                }
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
