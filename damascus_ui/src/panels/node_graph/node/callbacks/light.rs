use egui_node_graph::NodeId;

use damascus_core::lights;

use super::{DamascusGraph, DamascusValueType, NodeCallbacks};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LightCallbacks;

impl NodeCallbacks for LightCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        if input_name != "light_type" {
            return;
        }
        if let Some(node) = graph.nodes.get(node_id) {
            let mut to_hide = vec![];
            let mut to_show = vec![];
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get(input_id) {
                    match input_param.value() {
                        DamascusValueType::ComboBox { ref value } => {
                            match value.as_enum::<lights::Lights>() {
                                Ok(lights::Lights::Directional) => {
                                    to_show.push("direction");
                                    to_hide.push("iterations");
                                    to_hide.push("position")
                                }
                                Ok(lights::Lights::Point) => {
                                    to_hide.push("direction");
                                    to_hide.push("iterations");
                                    to_show.push("position");
                                }
                                Ok(lights::Lights::AmbientOcclusion) => {
                                    to_hide.push("direction");
                                    to_show.push("iterations");
                                    to_hide.push("position")
                                }
                                _ => {
                                    to_hide.push("direction");
                                    to_hide.push("iterations");
                                    to_hide.push("position");
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
    }
}
