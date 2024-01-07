use egui_node_graph::NodeId;

use super::{DamascusGraph, DamascusValueType, UIInput};

mod light;
pub use light::LightCallbacks;

pub trait NodeCallbacks {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}
