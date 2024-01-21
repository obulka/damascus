use std::borrow::Cow;

use eframe::egui;
use egui_node_graph::DataTypeTrait;

use super::DamascusGraphState;

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Debug, Eq, serde::Serialize, serde::Deserialize)]
pub enum DamascusDataType {
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
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<DamascusGraphState> for DamascusDataType {
    fn data_type_color(&self, _user_state: &mut DamascusGraphState) -> egui::Color32 {
        match self {
            DamascusDataType::Mat4 => egui::Color32::from_rgb(18, 184, 196),
            DamascusDataType::Image => egui::Color32::from_rgb(243, 230, 255),
            DamascusDataType::Camera => egui::Color32::from_rgb(123, 10, 10),
            DamascusDataType::Light => egui::Color32::from_rgb(255, 204, 128),
            DamascusDataType::Material => egui::Color32::from_rgb(255, 102, 0),
            DamascusDataType::Primitive => egui::Color32::from_rgb(38, 109, 211),
            DamascusDataType::ProceduralTexture => egui::Color32::from_rgb(14, 73, 9),
            DamascusDataType::RayMarcher => egui::Color32::from_rgb(19, 216, 157),
            DamascusDataType::Scene => egui::Color32::from_rgb(153, 0, 77),
            _ => egui::Color32::WHITE,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusDataType::Bool => "boolean",
            DamascusDataType::BVec3 => "3d boolean vector",
            DamascusDataType::ComboBox => "combo box",
            DamascusDataType::Integer => "integer",
            DamascusDataType::UnsignedInteger => "unsigned integer",
            DamascusDataType::UVec3 => "3d unsigned integer vector",
            DamascusDataType::Float => "scalar float",
            DamascusDataType::Vec2 => "2d vector",
            DamascusDataType::Vec3 => "3d vector",
            DamascusDataType::Vec4 => "4d vector",
            DamascusDataType::Mat3 => "3x3 matrix",
            DamascusDataType::Mat4 => "4x4 matrix",
            DamascusDataType::Image => "image",
            DamascusDataType::Camera => "camera",
            DamascusDataType::Light => "light",
            DamascusDataType::Material => "material",
            DamascusDataType::Primitive => "primitive",
            DamascusDataType::ProceduralTexture => "procedural texture",
            DamascusDataType::RayMarcher => "ray marcher",
            DamascusDataType::Scene => "scene",
        })
    }
}
