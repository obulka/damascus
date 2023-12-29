use eframe::egui;
use egui_node_graph::{NodeId, WidgetValueTrait};
use glam;
use ndarray;

use damascus_core::{geometry, lights, materials, renderers, scene};

use crate::panels::node_graph::{
    node_data::DamascusNodeData, node_graph_state::DamascusGraphState, response::DamascusResponse,
};

mod wrappers;
pub use wrappers::{ComboBox, Float, Integer, UnsignedInteger, Vec3, Vec4};

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
    Bool { value: bool },
    ComboBox { value: ComboBox },
    Integer { value: Integer },
    UnsignedInteger { value: UnsignedInteger },
    Float { value: Float },
    Vec2 { value: glam::Vec2 },
    Vec3 { value: Vec3 },
    Vec4 { value: Vec4 },
    Mat3 { value: glam::Mat3 },
    Mat4 { value: glam::Mat4 },
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
        Self::Bool { value: false }
    }
}

impl DamascusValueType {
    /// Tries to downcast this value type to an int
    pub fn try_to_int(self) -> anyhow::Result<i32> {
        if let DamascusValueType::Integer { value } = self {
            Ok(value.value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to int", self)
        }
    }

    /// Tries to downcast this value type to a uint
    pub fn try_to_uint(self) -> anyhow::Result<u32> {
        if let DamascusValueType::UnsignedInteger { value } = self {
            Ok(value.value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to uint", self)
        }
    }

    /// Tries to downcast this value type to a bool
    pub fn try_to_bool(self) -> anyhow::Result<bool> {
        if let DamascusValueType::Bool { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to bool", self)
        }
    }

    /// Tries to downcast this value type to a bool
    pub fn try_to_combo_box(self) -> anyhow::Result<ComboBox> {
        if let DamascusValueType::ComboBox { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to combo_box", self)
        }
    }

    /// Tries to downcast this value type to a float
    pub fn try_to_float(self) -> anyhow::Result<f32> {
        if let DamascusValueType::Float { value } = self {
            Ok(value.value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to float", self)
        }
    }

    /// Tries to downcast this value type to a vector2
    pub fn try_to_vec2(self) -> anyhow::Result<glam::Vec2> {
        if let DamascusValueType::Vec2 { value } = self {
            Ok(value)
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
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Mat3", self)
        }
    }

    /// Tries to downcast this value type to a mat4
    pub fn try_to_mat4(self) -> anyhow::Result<glam::Mat4> {
        if let DamascusValueType::Mat4 { value } = self {
            Ok(value)
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
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut DamascusGraphState,
        _node_data: &DamascusNodeData,
    ) -> Vec<DamascusResponse> {
        let create_bool_ui = |ui: &mut egui::Ui, label: &str, value: &mut bool| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::Checkbox::new(value, ""));
            });
        };
        let create_combo_box_ui = |ui: &mut egui::Ui, label: &str, value: &mut ComboBox| {
            ui.horizontal(|ui| {
                ui.label(label);
                egui::ComboBox::from_label("")
                    .selected_text(&value.selected)
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        for enum_option in value.options.iter() {
                            ui.selectable_value(
                                &mut value.selected,
                                enum_option.to_string(),
                                enum_option,
                            );
                        }
                    })
            });
        };
        let create_int_ui = |ui: &mut egui::Ui, label: &str, value: &mut Integer| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(
                    egui::Slider::new(&mut value.value, value.range.clone()).clamp_to_range(false),
                );
            });
        };
        let create_uint_ui = |ui: &mut egui::Ui, label: &str, value: &mut UnsignedInteger| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(
                    egui::Slider::new(&mut value.value, value.range.clone()).clamp_to_range(false),
                );
            });
        };
        let create_float_ui = |ui: &mut egui::Ui, label: &str, value: &mut Float| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(
                    egui::Slider::new(&mut value.value, value.range.clone()).clamp_to_range(false),
                );
            });
        };
        let create_vec2_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Vec2| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(&mut value.x));
                ui.add(egui::DragValue::new(&mut value.y));
            });
        };
        let create_vec3_ui = |ui: &mut egui::Ui, label: &str, value: &mut Vec3| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(&mut value.value[0]));
                ui.add(egui::DragValue::new(&mut value.value[1]));
                ui.add(egui::DragValue::new(&mut value.value[2]));
                if value.is_colour {
                    ui.color_edit_button_rgb(&mut value.value);
                }
            });
        };
        let create_vec4_ui = |ui: &mut egui::Ui, label: &str, value: &mut Vec4| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(egui::DragValue::new(&mut value.value[0]));
                ui.add(egui::DragValue::new(&mut value.value[1]));
                ui.add(egui::DragValue::new(&mut value.value[2]));
                ui.add(egui::DragValue::new(&mut value.value[3]));
                if value.is_colour {
                    ui.color_edit_button_rgba_unmultiplied(&mut value.value);
                }
            });
        };
        let create_mat3_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Mat3| {
            ui.vertical(|ui| {
                ui.label(label);
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut value.x_axis.x));
                    ui.add(egui::DragValue::new(&mut value.x_axis.y));
                    ui.add(egui::DragValue::new(&mut value.x_axis.z));
                });
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut value.y_axis.x));
                    ui.add(egui::DragValue::new(&mut value.y_axis.y));
                    ui.add(egui::DragValue::new(&mut value.y_axis.z));
                });
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut value.z_axis.x));
                    ui.add(egui::DragValue::new(&mut value.z_axis.y));
                    ui.add(egui::DragValue::new(&mut value.z_axis.z));
                });
            });
        };
        let create_mat4_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Mat4| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.vertical(|ui| {
                    ui.add(egui::DragValue::new(&mut value.x_axis.x));
                    ui.add(egui::DragValue::new(&mut value.x_axis.y));
                    ui.add(egui::DragValue::new(&mut value.x_axis.z));
                    ui.add(egui::DragValue::new(&mut value.x_axis.w));
                });
                ui.vertical(|ui| {
                    ui.add(egui::DragValue::new(&mut value.y_axis.x));
                    ui.add(egui::DragValue::new(&mut value.y_axis.y));
                    ui.add(egui::DragValue::new(&mut value.y_axis.z));
                    ui.add(egui::DragValue::new(&mut value.y_axis.w));
                });
                ui.vertical(|ui| {
                    ui.add(egui::DragValue::new(&mut value.z_axis.x));
                    ui.add(egui::DragValue::new(&mut value.z_axis.y));
                    ui.add(egui::DragValue::new(&mut value.z_axis.z));
                    ui.add(egui::DragValue::new(&mut value.z_axis.w));
                });
                ui.vertical(|ui| {
                    ui.add(egui::DragValue::new(&mut value.w_axis.x));
                    ui.add(egui::DragValue::new(&mut value.w_axis.y));
                    ui.add(egui::DragValue::new(&mut value.w_axis.z));
                    ui.add(egui::DragValue::new(&mut value.w_axis.w));
                });
            });
        };

        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            DamascusValueType::Bool { value } => {
                create_bool_ui(ui, param_name, value);
            }
            DamascusValueType::ComboBox { value } => {
                create_combo_box_ui(ui, param_name, value);
            }
            DamascusValueType::Integer { value } => {
                create_int_ui(ui, param_name, value);
            }
            DamascusValueType::UnsignedInteger { value } => {
                create_uint_ui(ui, param_name, value);
            }
            DamascusValueType::Float { value } => {
                create_float_ui(ui, param_name, value);
            }
            DamascusValueType::Vec2 { value } => {
                create_vec2_ui(ui, param_name, value);
            }
            DamascusValueType::Vec3 { value } => {
                create_vec3_ui(ui, param_name, value);
            }
            DamascusValueType::Vec4 { value } => {
                create_vec4_ui(ui, param_name, value);
            }
            DamascusValueType::Mat3 { value } => {
                create_mat3_ui(ui, param_name, value);
            }
            DamascusValueType::Mat4 { value } => {
                ui.horizontal(|ui| {
                    create_mat4_ui(ui, param_name, value);
                });
            }
            _ => {
                ui.label(param_name);
            }
        }

        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}
