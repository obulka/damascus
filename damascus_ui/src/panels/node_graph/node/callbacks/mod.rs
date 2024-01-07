use egui_node_graph::{InputId, Node, NodeId};

use super::{DamascusGraph, DamascusNodeData, DamascusValueType, UIInput};

mod light;
pub use light::LightCallbacks;

pub trait NodeCallbacks {
    fn show_input(
        &self,
        graph: &mut DamascusGraph,
        node: &Node<DamascusNodeData>,
        input_name: &str,
    ) {
        if let Ok(input_id) = node.get_input(input_name) {
            if let Some(input_param) = graph.inputs.get_mut(input_id) {
                match &mut input_param.value {
                    DamascusValueType::Bool { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::ComboBox { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Float { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Integer { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::UnsignedInteger { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Mat3 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Mat4 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Vec3 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Vec4 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    _ => {}
                }
            }
        }
    }

    fn hide_input(
        &self,
        graph: &mut DamascusGraph,
        node: &Node<DamascusNodeData>,
        input_name: &str,
    ) {
        if let Ok(input_id) = node.get_input(input_name) {
            if let Some(input_param) = graph.inputs.get_mut(input_id) {
                match &mut input_param.value {
                    DamascusValueType::Bool { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::ComboBox { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Float { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Integer { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::UnsignedInteger { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Mat3 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Mat4 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Vec3 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    DamascusValueType::Vec4 { ref mut value } => {
                        value.ui_data_mut().show();
                    }
                    _ => {}
                }
            }
        }
    }

    fn hide_inputs(
        &self,
        graph: &mut DamascusGraph,
        node: &Node<DamascusNodeData>,
        input_names: &Vec<&str>,
    ) {
        for input_name in input_names.iter() {
            if let Ok(input_id) = node.get_input(input_name) {
                if let Some(input_param) = graph.inputs.get_mut(input_id) {
                    match &mut input_param.value {
                        DamascusValueType::Vec3 { ref mut value } => {
                            value.ui_data_mut().show();
                        }
                        DamascusValueType::Float { ref mut value } => {
                            value.ui_data_mut().show();
                        }
                        _ => {}
                    }
                }
            }
        }
        // for input in input_names.iter() {
        //     self.hide_input(graph, node, input);
        // }
    }

    fn show_inputs(
        &self,
        graph: &mut DamascusGraph,
        node: &Node<DamascusNodeData>,
        input_names: Vec<&str>,
    ) {
        for input in input_names.iter() {
            self.show_input(graph, node, input);
        }
    }

    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}
