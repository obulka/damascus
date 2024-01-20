use egui_node_graph::NodeId;

use super::{DamascusDataType, DamascusGraph, DamascusValueType, UIInput};

mod light;
mod primitive;
pub use light::LightCallbacks;
pub use primitive::PrimitiveCallbacks;

pub trait NodeCallbacks {
    fn show_input(&self, value_type: &mut DamascusValueType) {
        match value_type {
            DamascusValueType::Bool { ref mut value } => {
                value.ui_data_mut().show();
            }
            DamascusValueType::BVec3 { ref mut value } => {
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
            DamascusValueType::UVec3 { ref mut value } => {
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

    fn hide_input(&self, value_type: &mut DamascusValueType) {
        match value_type {
            DamascusValueType::Bool { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::BVec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::ComboBox { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Float { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Integer { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::UnsignedInteger { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::UVec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Mat3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Mat4 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Vec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            DamascusValueType::Vec4 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            _ => {}
        }
    }

    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String);
}
