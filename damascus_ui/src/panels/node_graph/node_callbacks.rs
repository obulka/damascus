use egui_node_graph::{InputId, NodeId};

use crate::panels::node_graph::{DamascusGraph, DamascusNodeTemplate, DamascusValueType, UIInput};

#[typetag::serde(tag = "type")]
pub trait NodeCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
struct LightCallbacks;

#[typetag::serde]
impl NodeCallbacks for LightCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        if input_name != "light_type" {
            return;
        }
        let mut dimensional_data_input_id: Option<InputId> = None;
        if let Some(node) = graph.nodes.get(node_id) {
            for (name, node_input_id) in &node.inputs {
                if *name == "dimensional_data" {
                    dimensional_data_input_id = Some(*node_input_id);
                    break;
                }
            }
        }
        if let Some(input_id) = dimensional_data_input_id {
            if let Some(input_param) = graph.inputs.get_mut(input_id) {
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
        }
    }
}

pub fn run_input_value_changed_callbacks(
    graph: &mut DamascusGraph,
    node_id: NodeId,
    input_name: &String,
) {
    let template: Option<DamascusNodeTemplate> = if let Some(node) = graph.nodes.get(node_id) {
        Some(node.user_data.template)
    } else {
        None
    };

    match template {
        Some(DamascusNodeTemplate::Light) => {
            LightCallbacks.input_value_changed(graph, node_id, input_name)
        }
        _ => {}
    }

    // }
    // if let Some(node_callbacks) = callbacks {
    //     if let Some(node) = self.state.graph.nodes.get_mut(node_id) {
    //         node_callbacks.input_value_changed(node, &input_name);
    //     }
    // }
}
