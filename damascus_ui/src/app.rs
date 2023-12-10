use core::ops::RangeInclusive;
use std::{borrow::Cow, collections::HashMap};

use eframe::egui::{self, Checkbox, DragValue, Slider, TextStyle};
use egui_node_graph::*;
use exr;
use glam;

use damascus_core::{geometry, materials, scene};

use crate::viewport_3d::Viewport3d;

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct DamascusNodeData {
    template: DamascusNodeTemplate,
}

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusDataType {
    // Base types
    Bool,
    Integer,
    UnsignedInteger,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Image,

    // Composite types
    Camera,
    Primitive,
    Scene,
}

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Float and a ValueType of Vec2.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusValueType {
    // Base types
    Bool { value: bool },
    Integer { value: i32 },
    UnsignedInteger { value: u32 },
    Float { value: f32 },
    Vec2 { value: glam::Vec2 },
    Vec3 { value: glam::Vec3 },
    Vec4 { value: glam::Vec4 },
    Mat3 { value: glam::Mat3 },
    Mat4 { value: glam::Mat4 },
    Image { value: exr::prelude::AnyImage },

    // Composite types
    Camera { value: geometry::camera::Camera },
    Primitive { value: Vec<geometry::Primitive> },
    Scene { value: scene::Scene },
}

impl Default for DamascusValueType {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::Float { value: 0.0 }
    }
}

impl DamascusValueType {
    /// Tries to downcast this value type to an int
    pub fn try_to_int(self) -> anyhow::Result<i32> {
        if let DamascusValueType::Integer { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to int", self)
        }
    }

    /// Tries to downcast this value type to a uint
    pub fn try_to_uint(self) -> anyhow::Result<u32> {
        if let DamascusValueType::UnsignedInteger { value } = self {
            Ok(value)
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

    /// Tries to downcast this value type to a float
    pub fn try_to_float(self) -> anyhow::Result<f32> {
        if let DamascusValueType::Float { value } = self {
            Ok(value)
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
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Vec3", self)
        }
    }

    /// Tries to downcast this value type to a vector4
    pub fn try_to_vec4(self) -> anyhow::Result<glam::Vec4> {
        if let DamascusValueType::Vec4 { value } = self {
            Ok(value)
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
    pub fn try_to_image(self) -> anyhow::Result<exr::prelude::AnyImage> {
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
    pub fn try_to_primitive(self) -> anyhow::Result<Vec<geometry::Primitive>> {
        if let DamascusValueType::Primitive { value } = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to Primitive", self)
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

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum DamascusNodeTemplate {
    // Data creation
    Axis,
    Camera,
    Primitive,
    Scene,
    // Data processing
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamascusResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub struct DamascusGraphState {
    pub active_node: Option<NodeId>,
}

// =========== Then, you need to implement some traits ============

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<DamascusGraphState> for DamascusDataType {
    fn data_type_color(&self, _user_state: &mut DamascusGraphState) -> egui::Color32 {
        match self {
            // DamascusDataType::Bool => egui::Color32::from_rgb(255, 102, 0),
            // DamascusDataType::Integer => egui::Color32::from_rgb(255, 102, 0),
            // DamascusDataType::UnsignedInteger => egui::Color32::from_rgb(255, 102, 0),
            // DamascusDataType::Vec2 => egui::Color32::from_rgb(238, 207, 109),
            // DamascusDataType::Vec3 => egui::Color32::from_rgb(79, 0, 107),
            // DamascusDataType::Vec4 => egui::Color32::from_rgb(136, 55, 86),
            // DamascusDataType::Mat3 => egui::Color32::from_rgb(19, 216, 157),
            DamascusDataType::Mat4 => egui::Color32::from_rgb(18, 184, 196),
            DamascusDataType::Image => egui::Color32::from_rgb(243, 230, 255),
            DamascusDataType::Camera => egui::Color32::from_rgb(123, 10, 10),
            DamascusDataType::Primitive => egui::Color32::from_rgb(38, 109, 211),
            DamascusDataType::Scene => egui::Color32::from_rgb(153, 0, 77),
            _ => egui::Color32::WHITE,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusDataType::Bool => "boolean",
            DamascusDataType::Integer => "integer",
            DamascusDataType::UnsignedInteger => "unsigned integer",
            DamascusDataType::Float => "scalar float",
            DamascusDataType::Vec2 => "2d vector",
            DamascusDataType::Vec3 => "3d vector",
            DamascusDataType::Vec4 => "4d vector",
            DamascusDataType::Mat3 => "3x3 matrix",
            DamascusDataType::Mat4 => "4x4 matrix",
            DamascusDataType::Image => "image",
            DamascusDataType::Camera => "camera",
            DamascusDataType::Primitive => "primitive",
            DamascusDataType::Scene => "scene",
        })
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for DamascusNodeTemplate {
    type NodeData = DamascusNodeData;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;
    type UserState = DamascusGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusNodeTemplate::Axis => "axis",
            DamascusNodeTemplate::Camera => "camera",
            DamascusNodeTemplate::Primitive => "primitive",
            DamascusNodeTemplate::Scene => "scene",
        })
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        DamascusNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_bool = |graph: &mut DamascusGraph, name: &str, default: bool| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Bool,
                DamascusValueType::Bool { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_int = |graph: &mut DamascusGraph, name: &str, default: i32| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Integer,
                DamascusValueType::Integer { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_uint = |graph: &mut DamascusGraph, name: &str, default: u32| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::UnsignedInteger,
                DamascusValueType::UnsignedInteger { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_float = |graph: &mut DamascusGraph, name: &str, default: f32| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Float,
                DamascusValueType::Float { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector2 = |graph: &mut DamascusGraph, name: &str, default: glam::Vec2| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec2,
                DamascusValueType::Vec2 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector3 = |graph: &mut DamascusGraph, name: &str, default: glam::Vec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec3,
                DamascusValueType::Vec3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector4 = |graph: &mut DamascusGraph, name: &str, default: glam::Vec4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec4,
                DamascusValueType::Vec4 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_matrix3 = |graph: &mut DamascusGraph, name: &str, default: glam::Mat3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Mat3,
                DamascusValueType::Mat3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_matrix4 = |graph: &mut DamascusGraph, name: &str, default: glam::Mat4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Mat4,
                DamascusValueType::Mat4 { value: default },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_image =
            |graph: &mut DamascusGraph, name: &str, default: exr::prelude::AnyImage| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Image,
                    DamascusValueType::Image { value: default },
                    InputParamKind::ConnectionOnly,
                    true,
                );
            };
        let input_camera =
            |graph: &mut DamascusGraph, name: &str, default: geometry::camera::Camera| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Camera,
                    DamascusValueType::Camera { value: default },
                    InputParamKind::ConnectionOnly,
                    true,
                );
            };

        let input_primitive =
            |graph: &mut DamascusGraph, name: &str, default: Vec<geometry::Primitive>| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Primitive,
                    DamascusValueType::Primitive { value: default },
                    InputParamKind::ConnectionOnly,
                    true,
                );
            };

        let input_scene = |graph: &mut DamascusGraph, name: &str, default: scene::Scene| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Scene,
                DamascusValueType::Scene { value: default },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_matrix4 = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Mat4);
        };
        let output_image = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Image);
        };
        let output_camera = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Camera);
        };
        let output_primitive = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Primitive);
        };
        let output_scene = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Scene);
        };

        match self {
            DamascusNodeTemplate::Axis => {
                input_matrix4(graph, "axis", glam::Mat4::IDENTITY);
                input_vector3(graph, "translate", glam::Vec3::ZERO);
                input_vector3(graph, "rotate", glam::Vec3::ZERO);
                input_vector3(graph, "scale", glam::Vec3::ONE);
                output_matrix4(graph, "out");
            }
            DamascusNodeTemplate::Camera => {
                let default_camera = geometry::camera::Camera::default();
                input_float(graph, "focal_length", default_camera.focal_length);
                input_float(graph, "focal_distance", default_camera.focal_distance);
                input_float(graph, "f_stop", default_camera.f_stop);
                input_float(
                    graph,
                    "horizontal_aperture",
                    default_camera.horizontal_aperture,
                );
                input_float(graph, "near_plane", default_camera.near_plane);
                input_float(graph, "far_plane", default_camera.far_plane);
                input_matrix4(graph, "world_matrix", default_camera.world_matrix);
                input_bool(
                    graph,
                    "enable_depth_of_field",
                    default_camera.enable_depth_of_field,
                );
                output_camera(graph, "out");
            }
            DamascusNodeTemplate::Primitive => {
                let default_primitive = geometry::Primitive::default();
                input_primitive(graph, "siblings", vec![]);
                input_primitive(graph, "children", vec![]);
                input_uint(graph, "shape", default_primitive.shape as u32); // TODO make a dropdown for enums
                input_matrix4(graph, "transform", glam::Mat4::IDENTITY);
                // input_material(graph, "material", default_primitive.material); // TODO
                input_uint(graph, "modifiers", default_primitive.modifiers as u32); // TODO make this a series of bools
                input_float(graph, "blend_strength", default_primitive.blend_strength);
                input_vector4(graph, "dimensional_data", glam::Vec4::X); // TODO make this dynamic based on shape
                output_primitive(graph, "out");
            }
            DamascusNodeTemplate::Scene => {
                let default_scene = scene::Scene::default();
                input_camera(graph, "render_camera", default_scene.render_camera);
                input_primitive(graph, "primitives", default_scene.primitives);
                output_scene(graph, "out");
            }
        }
    }
}

pub struct AllDamascusNodeTemplates;
impl NodeTemplateIter for AllDamascusNodeTemplates {
    type Item = DamascusNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            DamascusNodeTemplate::Axis,
            DamascusNodeTemplate::Camera,
            DamascusNodeTemplate::Primitive,
            DamascusNodeTemplate::Scene,
        ]
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
                ui.add(Checkbox::new(value, ""));
            });
        };
        let create_int_ui =
            |ui: &mut egui::Ui, label: &str, value: &mut i32, range: RangeInclusive<i32>| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(Slider::new(value, range));
                });
            };
        let create_uint_ui =
            |ui: &mut egui::Ui, label: &str, value: &mut u32, range: RangeInclusive<u32>| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(Slider::new(value, range));
                });
            };
        let create_float_ui =
            |ui: &mut egui::Ui, label: &str, value: &mut f32, range: RangeInclusive<f32>| {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(Slider::new(value, range));
                });
            };
        let create_vec2_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Vec2| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(DragValue::new(&mut value.x));
                ui.add(DragValue::new(&mut value.y));
            });
        };
        let create_vec3_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Vec3| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(DragValue::new(&mut value.x));
                ui.add(DragValue::new(&mut value.y));
                ui.add(DragValue::new(&mut value.z));
            });
        };
        let create_vec4_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Vec4| {
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(DragValue::new(&mut value.x));
                ui.add(DragValue::new(&mut value.y));
                ui.add(DragValue::new(&mut value.z));
                ui.add(DragValue::new(&mut value.w));
            });
        };
        let create_mat3_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Mat3| {
            ui.vertical(|ui| {
                ui.label(label);
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.x_axis.x));
                    ui.add(DragValue::new(&mut value.x_axis.y));
                    ui.add(DragValue::new(&mut value.x_axis.z));
                });
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.y_axis.x));
                    ui.add(DragValue::new(&mut value.y_axis.y));
                    ui.add(DragValue::new(&mut value.y_axis.z));
                });
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.z_axis.x));
                    ui.add(DragValue::new(&mut value.z_axis.y));
                    ui.add(DragValue::new(&mut value.z_axis.z));
                });
            });
        };
        let create_mat4_ui = |ui: &mut egui::Ui, label: &str, value: &mut glam::Mat4| {
            ui.vertical(|ui| {
                ui.label(label);
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.x_axis.x));
                    ui.add(DragValue::new(&mut value.x_axis.y));
                    ui.add(DragValue::new(&mut value.x_axis.z));
                    ui.add(DragValue::new(&mut value.x_axis.w));
                });
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.y_axis.x));
                    ui.add(DragValue::new(&mut value.y_axis.y));
                    ui.add(DragValue::new(&mut value.y_axis.z));
                    ui.add(DragValue::new(&mut value.y_axis.w));
                });
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.z_axis.x));
                    ui.add(DragValue::new(&mut value.z_axis.y));
                    ui.add(DragValue::new(&mut value.z_axis.z));
                    ui.add(DragValue::new(&mut value.z_axis.w));
                });
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut value.w_axis.x));
                    ui.add(DragValue::new(&mut value.w_axis.y));
                    ui.add(DragValue::new(&mut value.w_axis.z));
                    ui.add(DragValue::new(&mut value.w_axis.w));
                });
            });
        };

        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            DamascusValueType::Bool { value } => {
                create_bool_ui(ui, param_name, value);
            }
            DamascusValueType::Integer { value } => {
                create_int_ui(ui, param_name, value, -50..=50);
            }
            DamascusValueType::UnsignedInteger { value } => {
                create_uint_ui(ui, param_name, value, 0..=100);
            }
            DamascusValueType::Float { value } => {
                create_float_ui(ui, param_name, value, 0.0..=100.0);
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

impl UserResponseTrait for DamascusResponse {}
impl NodeDataTrait for DamascusNodeData {
    type Response = DamascusResponse;
    type UserState = DamascusGraphState;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<DamascusNodeData, DamascusDataType, DamascusValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<DamascusResponse, DamascusNodeData>>
    where
        DamascusResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(DamascusResponse::SetActiveNode(node_id)));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(DamascusResponse::ClearActiveNode));
            }
        }

        responses
    }
}

type DamascusGraph = Graph<DamascusNodeData, DamascusDataType, DamascusValueType>;
type DamascusEditorState = GraphEditorState<
    DamascusNodeData,
    DamascusDataType,
    DamascusValueType,
    DamascusNodeTemplate,
    DamascusGraphState,
>;

pub struct Damascus {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: DamascusEditorState,
    user_state: DamascusGraphState,
    viewport_3d: Option<Viewport3d>,
}

#[cfg(feature = "persistence")]
const PERSISTENCE_KEY: &str = "damascus";

#[cfg(feature = "persistence")]
impl Damascus {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();
        Self {
            state,
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(cc),
        }
    }
}

#[cfg(not(feature = "persistence"))]
impl Damascus {
    /// If the persistence feature is enabled, Called once before the first frame.
    /// Load previous app state (if any).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: DamascusEditorState::default(),
            user_state: DamascusGraphState::default(),
            viewport_3d: Viewport3d::new(cc),
        }
    }
}

impl eframe::App for Damascus {
    #[cfg(feature = "persistence")]
    /// If the persistence function is enabled,
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.state);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                    ui.menu_button("SubMenu", |ui| {
                        ui.menu_button("SubMenu", |ui| {
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                            let _ = ui.button("Item");
                        });
                        ui.menu_button("SubMenu", |ui| {
                            if ui.button("Open...").clicked() {
                                ui.close_menu();
                            }
                            let _ = ui.button("Item");
                        });
                        let _ = ui.button("Item");
                        if ui.button("Open...").clicked() {
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("SubMenu", |ui| {
                        let _ = ui.button("Item1");
                        let _ = ui.button("Item2");
                        let _ = ui.button("Item3");
                        let _ = ui.button("Item4");
                        if ui.button("Open...").clicked() {
                            ui.close_menu();
                        }
                    });
                    let _ = ui.button("Very long text for this item");
                });
            });
        });
        egui::SidePanel::right("properties")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("test");
                });
            });

        let graph_response = egui::TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllDamascusNodeTemplates, &mut self.user_state)
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    DamascusResponse::SetActiveNode(node) => {
                        self.user_state.active_node = Some(node)
                    }
                    DamascusResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let value_type = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => value,
                    Err(error) => {
                        ctx.debug_painter().text(
                            egui::pos2(10.0, 35.0),
                            egui::Align2::LEFT_TOP,
                            format!("Error: {}", error),
                            TextStyle::Button.resolve(&ctx.style()),
                            egui::Color32::RED,
                        );

                        DamascusValueType::Float { value: 0.0 }
                    }
                };
                if let Some(ref mut viewport_3d) = &mut self.viewport_3d {
                    match value_type {
                        DamascusValueType::Mat4 { value } => {}
                        DamascusValueType::Image { value } => {}
                        DamascusValueType::Camera { value } => {
                            viewport_3d.scene.render_camera = value;
                        }
                        DamascusValueType::Primitive { value } => {
                            viewport_3d.scene.primitives = value;
                        }
                        DamascusValueType::Scene { value } => {
                            viewport_3d.scene = value;
                        }
                        _ => {}
                    }
                }
            } else {
                self.user_state.active_node = None;
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default())
            .show(ctx, |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        viewport_3d.custom_painting(ui);
                    }
                });
            });
    }
}

type OutputsCache = HashMap<OutputId, DamascusValueType>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &DamascusGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<DamascusValueType> {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    struct Evaluator<'a> {
        graph: &'a DamascusGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(
            graph: &'a DamascusGraph,
            outputs_cache: &'a mut OutputsCache,
            node_id: NodeId,
        ) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }

        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<DamascusValueType> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }

        fn populate_output(
            &mut self,
            name: &str,
            value: DamascusValueType,
        ) -> anyhow::Result<DamascusValueType> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }

        fn input_bool(&mut self, name: &str) -> anyhow::Result<bool> {
            self.evaluate_input(name)?.try_to_bool()
        }

        fn output_bool(&mut self, name: &str, value: bool) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Bool { value })
        }

        fn input_int(&mut self, name: &str) -> anyhow::Result<i32> {
            self.evaluate_input(name)?.try_to_int()
        }

        fn output_int(&mut self, name: &str, value: i32) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Integer { value })
        }

        fn output_uint(&mut self, name: &str, value: u32) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::UnsignedInteger { value })
        }

        fn input_uint(&mut self, name: &str) -> anyhow::Result<u32> {
            self.evaluate_input(name)?.try_to_uint()
        }

        fn input_float(&mut self, name: &str) -> anyhow::Result<f32> {
            self.evaluate_input(name)?.try_to_float()
        }

        fn output_float(&mut self, name: &str, value: f32) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Float { value })
        }

        fn input_vector2(&mut self, name: &str) -> anyhow::Result<glam::Vec2> {
            self.evaluate_input(name)?.try_to_vec2()
        }

        fn output_vector2(
            &mut self,
            name: &str,
            value: glam::Vec2,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Vec2 { value })
        }

        fn input_vector3(&mut self, name: &str) -> anyhow::Result<glam::Vec3> {
            self.evaluate_input(name)?.try_to_vec3()
        }

        fn output_vector3(
            &mut self,
            name: &str,
            value: glam::Vec3,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Vec3 { value })
        }

        fn input_vector4(&mut self, name: &str) -> anyhow::Result<glam::Vec4> {
            self.evaluate_input(name)?.try_to_vec4()
        }

        fn output_vector4(
            &mut self,
            name: &str,
            value: glam::Vec4,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Vec4 { value })
        }

        fn input_matrix3(&mut self, name: &str) -> anyhow::Result<glam::Mat3> {
            self.evaluate_input(name)?.try_to_mat3()
        }

        fn output_matrix3(
            &mut self,
            name: &str,
            value: glam::Mat3,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Mat3 { value })
        }

        fn input_matrix4(&mut self, name: &str) -> anyhow::Result<glam::Mat4> {
            self.evaluate_input(name)?.try_to_mat4()
        }

        fn output_matrix4(
            &mut self,
            name: &str,
            value: glam::Mat4,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Mat4 { value })
        }

        fn input_image(&mut self, name: &str) -> anyhow::Result<exr::prelude::AnyImage> {
            self.evaluate_input(name)?.try_to_image()
        }

        fn output_image(
            &mut self,
            name: &str,
            value: exr::prelude::AnyImage,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Image { value })
        }

        fn input_camera(&mut self, name: &str) -> anyhow::Result<geometry::camera::Camera> {
            self.evaluate_input(name)?.try_to_camera()
        }

        fn output_camera(
            &mut self,
            name: &str,
            value: geometry::camera::Camera,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Camera { value })
        }

        fn input_primitive(&mut self, name: &str) -> anyhow::Result<Vec<geometry::Primitive>> {
            self.evaluate_input(name)?.try_to_primitive()
        }

        fn output_primitive(
            &mut self,
            name: &str,
            value: Vec<geometry::Primitive>,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Primitive { value })
        }

        fn input_scene(&mut self, name: &str) -> anyhow::Result<scene::Scene> {
            self.evaluate_input(name)?.try_to_scene()
        }

        fn output_scene(
            &mut self,
            name: &str,
            value: scene::Scene,
        ) -> anyhow::Result<DamascusValueType> {
            self.populate_output(name, DamascusValueType::Scene { value })
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        DamascusNodeTemplate::Axis => {
            let input_axis = evaluator.input_matrix4("axis")?;
            let translate = evaluator.input_vector3("translate")?;
            let rotate = evaluator.input_vector3("rotate")? * std::f32::consts::PI / 180.0;
            let scale = evaluator.input_vector3("scale")?;

            let quaternion =
                glam::Quat::from_euler(glam::EulerRot::ZXY, rotate.x, rotate.y, rotate.z);

            evaluator.output_matrix4(
                "out",
                input_axis
                    * glam::Mat4::from_scale_rotation_translation(scale, quaternion, translate),
            )
        }
        DamascusNodeTemplate::Camera => {
            let focal_length = evaluator.input_float("focal_length")?;
            let horizontal_aperture = evaluator.input_float("horizontal_aperture")?;
            let near_plane = evaluator.input_float("near_plane")?;
            let far_plane = evaluator.input_float("far_plane")?;
            let focal_distance = evaluator.input_float("focal_distance")?;
            let f_stop = evaluator.input_float("f_stop")?;
            let world_matrix = evaluator.input_matrix4("world_matrix")?;
            let enable_depth_of_field = evaluator.input_bool("enable_depth_of_field")?;
            evaluator.output_camera(
                "out",
                geometry::camera::Camera::new(
                    1., // TODO use the root resolution or add a resolution knob
                    focal_length,
                    horizontal_aperture,
                    near_plane,
                    far_plane,
                    focal_distance,
                    f_stop,
                    world_matrix,
                    enable_depth_of_field,
                ),
            )
        }
        DamascusNodeTemplate::Primitive => {
            let mut scene_primitives = evaluator.input_primitive("siblings")?;
            let mut children = evaluator.input_primitive("children")?;
            let shape_number = evaluator.input_uint("shape")?;
            let modifiers = evaluator.input_uint("modifiers")?;
            let blend_strength = evaluator.input_float("blend_strength")?;
            let dimensional_data = evaluator.input_vector4("dimensional_data")?;
            if let Some(shape) = num::FromPrimitive::from_u32(shape_number) {
                let primitive = geometry::Primitive {
                    shape: shape,                              // TODO
                    transform: geometry::Transform::default(), // TODO
                    material: materials::Material::default(),  // TODO
                    modifiers: modifiers,
                    blend_strength: blend_strength,
                    num_children: children.len() as u32,
                    dimensional_data: dimensional_data,
                };
                scene_primitives.push(primitive);
            }
            scene_primitives.append(&mut children);
            evaluator.output_primitive("out", scene_primitives)
        }
        DamascusNodeTemplate::Scene => {
            let render_camera = evaluator.input_camera("render_camera")?;
            let primitives = evaluator.input_primitive("primitives")?;
            evaluator.output_scene(
                "out",
                scene::Scene {
                    render_camera: render_camera,
                    primitives: primitives,
                },
            )
        }
    }
}

fn populate_output(
    graph: &DamascusGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: DamascusValueType,
) -> anyhow::Result<DamascusValueType> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &DamascusGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<DamascusValueType> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok((*other_value).clone())
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok((*outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated"))
            .clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
