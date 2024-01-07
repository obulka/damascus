use egui_node_graph::{InputId, NodeId};

use crate::panels::node_graph::{DamascusGraph, DamascusNodeTemplate, DamascusValueType, UIInput};

pub trait NodeCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}
