// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use egui_node_graph::NodeId;

use super::{Graph, NodeDataType, NodeGraphState, NodeValueType, UIInput};

mod light;
mod primitive;
mod procedural_texture;
pub use light::LightCallbacks;
pub use primitive::PrimitiveCallbacks;
pub use procedural_texture::ProceduralTextureCallbacks;

pub trait NodeCallbacks {
    fn show_input(&self, value_type: &mut NodeValueType) {
        match value_type {
            NodeValueType::Bool { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::BVec3 { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::ComboBox { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Float { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Integer { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::UnsignedInteger { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::UVec3 { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Mat3 { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Mat4 { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Vec3 { ref mut value } => {
                value.ui_data_mut().show();
            }
            NodeValueType::Vec4 { ref mut value } => {
                value.ui_data_mut().show();
            }
            _ => {}
        }
    }

    fn hide_input(&self, value_type: &mut NodeValueType) {
        match value_type {
            NodeValueType::Bool { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::BVec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::ComboBox { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Float { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Integer { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::UnsignedInteger { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::UVec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Mat3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Mat4 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Vec3 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            NodeValueType::Vec4 { ref mut value } => {
                value.ui_data_mut().hide();
            }
            _ => {}
        }
    }

    fn input_value_changed(&self, graph: &mut Graph, node_id: NodeId, input_name: &String);
}
