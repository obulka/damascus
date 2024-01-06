use egui_node_graph::NodeId;

use crate::panels::node_graph::{DamascusNodeTemplate, DamascusGraph, node_data::DamascusNodeData};

#[typetag::serde(tag = "type")]
pub trait NodeCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
struct LightCallbacks;

#[typetag::serde]
impl NodeCallbacks for LightCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        println!("{:?} changed!!", input_name);
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

    let callbacks: Option<Box<dyn NodeCallbacks>> = match template {
        Some(DamascusNodeTemplate::Light) => Some(Box::new(LightCallbacks)),
        _ => None,
    };
    if let Some(node_callbacks) = callbacks {
        node_callbacks.input_value_changed(graph, node_id, input_name);
    }
    //     for (name, node_input_id) in &node.inputs {
    //         if *name == input_name {
    //             input_id = Some(*node_input_id);
    //             break;
    //         }
    //     }
    // }
    // if let Some(node_callbacks) = callbacks {
    //     if let Some(node) = self.state.graph.nodes.get_mut(node_id) {
    //         node_callbacks.input_value_changed(node, &input_name);
    //     }
    // }
}
