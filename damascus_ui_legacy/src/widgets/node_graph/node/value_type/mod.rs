// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;
use egui_node_graph::{NodeId, WidgetValueTrait};
use glam;

use damascus::{camera, geometry::primitives, lights, materials, render_passes, scene, Enumerator};

use super::{
    super::{NodeGraphResponse, NodeGraphState},
    NodeData,
};

mod inputs;
pub use inputs::{
    boolean::Bool, boolean_vec3::BVec3, camera::Camera, combo_box::ComboBox, filepath::Filepath,
    float::Float, integer::Integer, lights::Lights, mat3::Mat3, mat4::Mat4, material::Material,
    primitives::Primitives, procedural_texture::ProceduralTexture, render_passes::RenderPasses,
    scene::Scene, unsigned_integer::UnsignedInteger, unsigned_integer_vec2::UVec2,
    unsigned_integer_vec3::UVec3, vec2::Vec2, vec3::Vec3, vec4::Vec4, Collapsible, Colour,
    RangedInput, UIInput,
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
pub enum NodeValueType {
    Bool { value: Bool },
    BVec3 { value: BVec3 },
    ComboBox { value: ComboBox },
    Filepath { value: Filepath },
    Integer { value: Integer },
    UnsignedInteger { value: UnsignedInteger },
    UVec2 { value: UVec2 },
    UVec3 { value: UVec3 },
    Float { value: Float },
    Vec2 { value: Vec2 },
    Vec3 { value: Vec3 },
    Vec4 { value: Vec4 },
    Mat3 { value: Mat3 },
    Mat4 { value: Mat4 },
    Camera { value: Camera },
    Light { value: Lights },
    Material { value: Material },
    Primitive { value: Primitives },
    ProceduralTexture { value: ProceduralTexture },
    RenderPass { value: RenderPasses },
    Scene { value: Scene },
}

impl Default for NodeValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Bool {
            value: Bool::new(false),
        }
    }
}

impl NodeValueType {
    /// Tries to downcast this value type to a bool
    pub fn try_to_bool(self) -> anyhow::Result<bool> {
        if let NodeValueType::Bool { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to bool", self)
        }
    }

    /// Tries to downcast this value type to a BVec3
    pub fn try_to_bvec3(self) -> anyhow::Result<glam::BVec3> {
        if let NodeValueType::BVec3 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to BVec3", self)
        }
    }

    /// Tries to downcast this value type to an int
    pub fn try_to_int(self) -> anyhow::Result<i32> {
        if let NodeValueType::Integer { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to int", self)
        }
    }

    /// Tries to downcast this value type to a uint
    pub fn try_to_uint(self) -> anyhow::Result<u32> {
        if let NodeValueType::UnsignedInteger { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to uint", self)
        }
    }

    /// Tries to downcast this value type to a UVec3
    pub fn try_to_uvec2(self) -> anyhow::Result<glam::UVec2> {
        if let NodeValueType::UVec2 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to UVec2", self)
        }
    }

    /// Tries to downcast this value type to a UVec3
    pub fn try_to_uvec3(self) -> anyhow::Result<glam::UVec3> {
        if let NodeValueType::UVec3 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to UVec3", self)
        }
    }

    /// Tries to downcast this value type to an enum
    pub fn try_to_enum<E: Enumerator>(self) -> anyhow::Result<E> {
        if let NodeValueType::ComboBox { value } = self {
            Ok(value.deref().to_enumerator())
        } else {
            anyhow::bail!("Invalid cast from {:?} to combo_box", self)
        }
    }

    /// Tries to downcast this value type to a float
    pub fn try_to_filepath(self) -> anyhow::Result<String> {
        if let NodeValueType::Filepath { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Path", self)
        }
    }

    /// Tries to downcast this value type to a float
    pub fn try_to_float(self) -> anyhow::Result<f32> {
        if let NodeValueType::Float { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to float", self)
        }
    }

    /// Tries to downcast this value type to a vector2
    pub fn try_to_vec2(self) -> anyhow::Result<glam::Vec2> {
        if let NodeValueType::Vec2 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec2", self)
        }
    }

    /// Tries to downcast this value type to a vector3
    pub fn try_to_vec3(self) -> anyhow::Result<glam::Vec3> {
        if let NodeValueType::Vec3 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec3", self)
        }
    }

    /// Tries to downcast this value type to a vector4
    pub fn try_to_vec4(self) -> anyhow::Result<glam::Vec4> {
        if let NodeValueType::Vec4 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec4", self)
        }
    }

    /// Tries to downcast this value type to a mat3
    pub fn try_to_mat3(self) -> anyhow::Result<glam::Mat3> {
        if let NodeValueType::Mat3 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Mat3", self)
        }
    }

    /// Tries to downcast this value type to a mat4
    pub fn try_to_mat4(self) -> anyhow::Result<glam::Mat4> {
        if let NodeValueType::Mat4 { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Mat4", self)
        }
    }

    /// Tries to downcast this value type to a camera
    pub fn try_to_camera(self) -> anyhow::Result<camera::Camera> {
        if let NodeValueType::Camera { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Camera", self)
        }
    }

    /// Tries to downcast this value type to a primitive
    pub fn try_to_light(self) -> anyhow::Result<Vec<lights::Light>> {
        if let NodeValueType::Light { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Light", self)
        }
    }

    /// Tries to downcast this value type to a material
    pub fn try_to_material(self) -> anyhow::Result<materials::Material> {
        if let NodeValueType::Material { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Material", self)
        }
    }

    /// Tries to downcast this value type to a primitive
    pub fn try_to_primitive(self) -> anyhow::Result<Vec<primitives::Primitive>> {
        if let NodeValueType::Primitive { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Primitive", self)
        }
    }

    /// Tries to downcast this value type to a material
    pub fn try_to_procedural_texture(self) -> anyhow::Result<materials::ProceduralTexture> {
        if let NodeValueType::ProceduralTexture { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to ProceduralTexture", self)
        }
    }

    /// Tries to downcast this value type to a render_pass
    pub fn try_to_render_pass(self) -> anyhow::Result<Vec<render_passes::RenderPasses>> {
        if let NodeValueType::RenderPass { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to RayMarcher", self)
        }
    }

    /// Tries to downcast this value type to a scene
    pub fn try_to_scene(self) -> anyhow::Result<scene::Scene> {
        if let NodeValueType::Scene { value } = self {
            Ok(value.deref())
        } else {
            anyhow::bail!("Invalid cast from {:?} to Scene", self)
        }
    }
}

impl WidgetValueTrait for NodeValueType {
    type Response = NodeGraphResponse;
    type UserState = NodeGraphState;
    type NodeData = NodeData;

    /// This method will be called for each input parameter with a widget with an disconnected
    /// input only. To display UI for connected inputs use [`WidgetValueTrait::value_widget_connected`].
    /// The return value is a vector of custom response objects which can be used
    /// to implement handling of side effects. If unsure, the response Vec can
    /// be empty.
    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        let value_changed = match self {
            NodeValueType::Bool { value } => value.create_ui(ui, param_name),
            NodeValueType::BVec3 { value } => value.create_ui(ui, param_name),
            NodeValueType::ComboBox { value } => value.create_ui(ui, param_name),
            NodeValueType::Integer { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::UnsignedInteger { value } => {
                RangedInput::create_ui(value, ui, param_name)
            }
            NodeValueType::UVec2 { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::UVec3 { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::Filepath { value } => value.create_ui(ui, param_name),
            NodeValueType::Float { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::Vec2 { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::Vec3 { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::Vec4 { value } => RangedInput::create_ui(value, ui, param_name),
            NodeValueType::Mat3 { value } => value.create_ui(ui, param_name),
            NodeValueType::Mat4 { value } => value.create_ui(ui, param_name),
            NodeValueType::Material { value } => value.create_ui(ui, param_name),
            NodeValueType::Camera { value } => value.create_ui(ui, param_name),
            NodeValueType::Light { value } => value.create_ui(ui, param_name),
            NodeValueType::Primitive { value } => value.create_ui(ui, param_name),
            NodeValueType::ProceduralTexture { value } => value.create_ui(ui, param_name),
            NodeValueType::Scene { value } => value.create_ui(ui, param_name),
            NodeValueType::RenderPass { value } => value.create_ui(ui, param_name),
        };

        if value_changed {
            return vec![NodeGraphResponse::InputValueChanged(
                node_id,
                node_data.template,
                param_name.to_string(),
            )];
        }
        Vec::new()
    }

    /// This method will be called for each input parameter with a widget with a connected
    /// input only. To display UI for diconnected inputs use [`WidgetValueTrait::value_widget`].
    /// The return value is a vector of custom response objects which can be used
    /// to implement handling of side effects. If unsure, the response Vec can
    /// be empty.
    ///
    /// Shows the input name label by default.
    fn value_widget_connected(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        let value_changed = match self {
            NodeValueType::Mat4 { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::Material { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::Camera { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::Light { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::Primitive { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::ProceduralTexture { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::RenderPass { value } => value.create_ui_connected(ui, param_name),
            NodeValueType::Scene { value } => value.create_ui_connected(ui, param_name),
            _ => {
                ui.add(egui::Button::new(param_name).stroke(egui::Stroke::NONE));
                false
            }
        };

        if value_changed {
            return vec![NodeGraphResponse::InputValueChanged(
                node_id,
                node_data.template,
                param_name.to_string(),
            )];
        }
        Vec::new()
    }
}
