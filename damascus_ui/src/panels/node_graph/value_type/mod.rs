use std::fmt::Display;
use std::str::FromStr;

use eframe::egui;
use egui_node_graph::{NodeId, WidgetValueTrait};
use glam;
use ndarray;
use strum::IntoEnumIterator;

use damascus_core::{geometry, lights, materials, renderers, scene};

use super::{
    node::DamascusNodeData, node_graph_state::DamascusGraphState, response::DamascusResponse,
};

mod inputs;
pub use inputs::{
    boolean::Bool, combo_box::ComboBox, create_drag_value_ui, float::Float, integer::Integer,
    mat3::Mat3, mat4::Mat4, unsigned_integer::UnsignedInteger, vec2::Vec2, vec3::Vec3, vec4::Vec4,
    Colour, RangedInput, UIInput,
};
mod ui_data;
pub use ui_data::UIData;

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Float and a ValueType of Vec2.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DamascusValueType {
    // Base types
    Bool { value: Bool },
    ComboBox { value: ComboBox },
    Integer { value: Integer },
    UnsignedInteger { value: UnsignedInteger },
    Float { value: Float },
    Vec2 { value: Vec2 },
    Vec3 { value: Vec3 },
    Vec4 { value: Vec4 },
    Mat3 { value: Mat3 },
    Mat4 { value: Mat4 },
    Image { value: ndarray::Array4<f32> },

    // Composite types
    Camera { value: geometry::camera::Camera },
    Light { value: Vec<lights::Light> },
    Material { value: materials::Material },
    Primitive { value: Vec<geometry::Primitive> },
    RayMarcher { value: renderers::RayMarcher },
    Scene { value: scene::Scene },
}

impl Default for DamascusValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Bool {
            value: Bool::new(false),
        }
    }
}

impl DamascusValueType {
    /// Tries to downcast this value type to an int
    pub fn try_to_int(self) -> anyhow::Result<i32> {
        if let DamascusValueType::Integer { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to int", self)
        }
    }

    /// Tries to downcast this value type to a uint
    pub fn try_to_uint(self) -> anyhow::Result<u32> {
        if let DamascusValueType::UnsignedInteger { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to uint", self)
        }
    }

    /// Tries to downcast this value type to a bool
    pub fn try_to_bool(self) -> anyhow::Result<bool> {
        if let DamascusValueType::Bool { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to bool", self)
        }
    }

    /// Tries to downcast this value type to an enum
    pub fn try_to_enum<E: IntoEnumIterator + Display + FromStr>(self) -> anyhow::Result<E> {
        if let DamascusValueType::ComboBox { value } = self {
            value.as_enum::<E>()
        } else {
            anyhow::bail!("Invalid cast from {:?} to combo_box", self)
        }
    }

    /// Tries to downcast this value type to a float
    pub fn try_to_float(self) -> anyhow::Result<f32> {
        if let DamascusValueType::Float { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to float", self)
        }
    }

    /// Tries to downcast this value type to a vector2
    pub fn try_to_vec2(self) -> anyhow::Result<glam::Vec2> {
        if let DamascusValueType::Vec2 { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec2", self)
        }
    }

    /// Tries to downcast this value type to a vector3
    pub fn try_to_vec3(self) -> anyhow::Result<glam::Vec3> {
        if let DamascusValueType::Vec3 { value } = self {
            Ok(value.as_vec3())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec3", self)
        }
    }

    /// Tries to downcast this value type to a vector4
    pub fn try_to_vec4(self) -> anyhow::Result<glam::Vec4> {
        if let DamascusValueType::Vec4 { value } = self {
            Ok(value.as_vec4())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec4", self)
        }
    }

    /// Tries to downcast this value type to a mat3
    pub fn try_to_mat3(self) -> anyhow::Result<glam::Mat3> {
        if let DamascusValueType::Mat3 { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Mat3", self)
        }
    }

    /// Tries to downcast this value type to a mat4
    pub fn try_to_mat4(self) -> anyhow::Result<glam::Mat4> {
        if let DamascusValueType::Mat4 { value } = self {
            Ok(*value.value())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Mat4", self)
        }
    }

    /// Tries to downcast this value type to an image
    pub fn try_to_image(self) -> anyhow::Result<ndarray::Array4<f32>> {
        if let DamascusValueType::Image { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Image", self)
        }
    }

    /// Tries to downcast this value type to a camera
    pub fn try_to_camera(self) -> anyhow::Result<geometry::camera::Camera> {
        if let DamascusValueType::Camera { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Camera", self)
        }
    }

    /// Tries to downcast this value type to a primitive
    pub fn try_to_light(self) -> anyhow::Result<Vec<lights::Light>> {
        if let DamascusValueType::Light { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Light", self)
        }
    }

    /// Tries to downcast this value type to a material
    pub fn try_to_material(self) -> anyhow::Result<materials::Material> {
        if let DamascusValueType::Material { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Material", self)
        }
    }

    /// Tries to downcast this value type to a primitive
    pub fn try_to_primitive(self) -> anyhow::Result<Vec<geometry::Primitive>> {
        if let DamascusValueType::Primitive { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Primitive", self)
        }
    }

    /// Tries to downcast this value type to a ray_marcher
    pub fn try_to_ray_marcher(self) -> anyhow::Result<renderers::RayMarcher> {
        if let DamascusValueType::RayMarcher { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to RayMarcher", self)
        }
    }

    /// Tries to downcast this value type to a scene
    pub fn try_to_scene(self) -> anyhow::Result<scene::Scene> {
        if let DamascusValueType::Scene { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Scene", self)
        }
    }
}

impl WidgetValueTrait for DamascusValueType {
    type Response = DamascusResponse;
    type UserState = DamascusGraphState;
    type NodeData = DamascusNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut DamascusGraphState,
        node_data: &DamascusNodeData,
    ) -> Vec<DamascusResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        let value_changed = match self {
            DamascusValueType::Bool { value } => value.create_ui(ui, param_name),
            DamascusValueType::ComboBox { value } => value.create_ui(ui, param_name),
            DamascusValueType::Integer { value } => RangedInput::create_ui(value, ui, param_name),
            DamascusValueType::UnsignedInteger { value } => {
                RangedInput::create_ui(value, ui, param_name)
            }
            DamascusValueType::Float { value } => RangedInput::create_ui(value, ui, param_name),
            DamascusValueType::Vec2 { value } => value.create_ui(ui, param_name),
            DamascusValueType::Vec3 { value } => value.create_ui(ui, param_name),
            DamascusValueType::Vec4 { value } => value.create_ui(ui, param_name),
            DamascusValueType::Mat3 { value } => value.create_ui(ui, param_name),
            DamascusValueType::Mat4 { value } => value.create_ui(ui, param_name),
            _ => {
                ui.label(param_name);
                false
            }
        };

        if value_changed {
            return vec![DamascusResponse::InputValueChanged(
                node_id,
                node_data.template,
                param_name.to_string(),
            )];
        }
        Vec::new()
    }
}
