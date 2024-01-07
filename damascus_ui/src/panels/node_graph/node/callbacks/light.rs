use egui_node_graph::NodeId;

use super::{DamascusGraph, DamascusValueType, NodeCallbacks, UIInput};

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct LightCallbacks;

impl NodeCallbacks for LightCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        if input_name != "light_type" {
            return;
        }
        if let Some(node) = graph.nodes.get(node_id) {
            for (name, input_id) in &node.inputs {
                if *name == "dimensional_data" {
                    if let Some(input_param) = graph.inputs.get_mut(*input_id) {
                        match &mut input_param.value {
                            DamascusValueType::Vec3 { ref mut value } => {
                                if *value.ui_data().hidden() {
                                    value.ui_data_mut().show();
                                } else {
                                    value.ui_data_mut().hide();
                                }
                            }
                            _ => {}
                        }
                    }
                    break;
                }
            }
        }
    }
}
