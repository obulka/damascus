// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::borrow::Cow;

use eframe::egui;
use egui_node_graph::DataTypeTrait;

use super::NodeGraphState;

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(Clone, PartialEq, Debug, Eq, serde::Serialize, serde::Deserialize)]
pub enum NodeDataType {
    // Base types
    Bool,
    BVec3,
    ComboBox,
    Integer,
    UnsignedInteger,
    UVec3,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Image,

    // Composite types
    Camera,
    Light,
    Material,
    Primitive,
    ProceduralTexture,
    RayMarcher,
    Scene,
    Texture,
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<NodeGraphState> for NodeDataType {
    fn data_type_color(&self, _user_state: &mut NodeGraphState) -> egui::Color32 {
        match self {
            NodeDataType::Mat4 => egui::Color32::from_rgb(18, 184, 196),
            NodeDataType::Image => egui::Color32::from_rgb(243, 230, 255),
            NodeDataType::Camera => egui::Color32::from_rgb(123, 10, 10),
            NodeDataType::Light => egui::Color32::from_rgb(255, 204, 128),
            NodeDataType::Material => egui::Color32::from_rgb(255, 102, 0),
            NodeDataType::Primitive => egui::Color32::from_rgb(38, 109, 211),
            NodeDataType::ProceduralTexture => egui::Color32::from_rgb(14, 73, 9),
            NodeDataType::RayMarcher => egui::Color32::from_rgb(19, 216, 157),
            NodeDataType::Scene => egui::Color32::from_rgb(153, 0, 77),
            NodeDataType::Texture => egui::Color32::from_rgb(70, 0, 128),
            _ => egui::Color32::WHITE,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            NodeDataType::Bool => "boolean",
            NodeDataType::BVec3 => "3d boolean vector",
            NodeDataType::ComboBox => "combo box",
            NodeDataType::Integer => "integer",
            NodeDataType::UnsignedInteger => "unsigned integer",
            NodeDataType::UVec3 => "3d unsigned integer vector",
            NodeDataType::Float => "scalar float",
            NodeDataType::Vec2 => "2d vector",
            NodeDataType::Vec3 => "3d vector",
            NodeDataType::Vec4 => "4d vector",
            NodeDataType::Mat3 => "3x3 matrix",
            NodeDataType::Mat4 => "4x4 matrix",
            NodeDataType::Image => "image",
            NodeDataType::Camera => "camera",
            NodeDataType::Light => "light",
            NodeDataType::Material => "material",
            NodeDataType::Primitive => "primitive",
            NodeDataType::ProceduralTexture => "procedural texture",
            NodeDataType::RayMarcher => "ray marcher",
            NodeDataType::Scene => "scene",
            NodeDataType::Texture => "texture",
        })
    }
}
