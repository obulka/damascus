// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::borrow::Cow;

use egui_node_graph;
use indoc::indoc;
use strum::{EnumIter, IntoEnumIterator};

use damascus::{camera, geometry, lights, materials, render_passes::ray_marcher, scene, textures};

use super::{Graph, NodeGraph, NodeGraphResponse, NodeGraphState, NodeOutputCache};

pub mod callbacks;
mod data_type;
mod node_data;
pub mod value_type;

use callbacks::{
    LightCallbacks, NodeCallbacks, PrimitiveCallbacks, ProceduralTextureCallbacks,
    RayMarcherCallbacks,
};
pub use data_type::NodeDataType;
pub use node_data::NodeData;
use value_type::{
    BVec3, Bool, Camera, Collapsible, Colour, ComboBox, Filepath, Float, Lights, Mat4, Material,
    NodeValueType, Primitives, ProceduralTexture, RangedInput, RenderPasses, Scene, UIData,
    UIInput, UVec2, UVec3, UnsignedInteger, Vec3, Vec4,
};

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(
    Clone, Copy, EnumIter, Eq, Debug, Hash, PartialEq, serde::Serialize, serde::Deserialize,
)]
pub enum NodeTemplate {
    Axis,
    Camera,
    Light,
    Grade,
    Material,
    Primitive,
    ProceduralTexture,
    RayMarcher,
    Scene,
    Texture,
}

impl NodeCallbacks for NodeTemplate {
    fn input_value_changed(
        &self,
        node_graph: &mut NodeGraph,
        node_id: egui_node_graph::NodeId,
        input_name: &String,
    ) -> Vec<NodeGraphResponse> {
        match self {
            NodeTemplate::Light => {
                LightCallbacks.input_value_changed(node_graph, node_id, input_name)
            }
            NodeTemplate::Primitive => {
                PrimitiveCallbacks.input_value_changed(node_graph, node_id, input_name)
            }
            NodeTemplate::ProceduralTexture => {
                ProceduralTextureCallbacks.input_value_changed(node_graph, node_id, input_name)
            }
            NodeTemplate::RayMarcher => {
                RayMarcherCallbacks.input_value_changed(node_graph, node_id, input_name)
            }
            _ => Vec::new(),
        }
    }

    fn input_disconnected(
        &self,
        _graph: &mut NodeGraph,
        _input_id: egui_node_graph::InputId,
        _output_id: egui_node_graph::OutputId,
    ) -> Vec<NodeGraphResponse> {
        match self {
            _ => Vec::new(),
        }
    }

    fn input_connected(
        &self,
        _graph: &mut NodeGraph,
        _input_id: egui_node_graph::InputId,
        _output_id: egui_node_graph::OutputId,
    ) -> Vec<NodeGraphResponse> {
        match self {
            _ => Vec::new(),
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl egui_node_graph::NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = NodeDataType;
    type ValueType = NodeValueType;
    type UserState = NodeGraphState;
    type CategoryType = (); // TODO think about adding categories

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            NodeTemplate::Axis => "axis",
            NodeTemplate::Camera => "camera",
            NodeTemplate::Light => "light",
            NodeTemplate::Grade => "grade",
            NodeTemplate::Material => "material",
            NodeTemplate::Primitive => "primitive",
            NodeTemplate::ProceduralTexture => "procedural texture",
            NodeTemplate::RayMarcher => "ray marcher",
            NodeTemplate::Scene => "scene",
            NodeTemplate::Texture => "texture",
        })
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        NodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph,
        _user_state: &mut Self::UserState,
        node_id: egui_node_graph::NodeId,
    ) {
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template

        // We define some closures here to avoid boilerplate. Note that this is
        // entirely optional.
        let input_bool = |graph: &mut Graph, name: &str, default: Bool| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Bool,
                NodeValueType::Bool { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_bool_vector3 = |graph: &mut Graph, name: &str, default: BVec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::BVec3,
                NodeValueType::BVec3 { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_combo_box = |graph: &mut Graph, name: &str, default: ComboBox| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::ComboBox,
                NodeValueType::ComboBox { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        // let input_int = |graph: &mut Graph, name: &str, default: Integer| {
        //     graph.add_input_param(
        //         node_id,
        //         name.to_string(),
        //         NodeDataType::Integer,
        //         NodeValueType::Integer { value: default },
        //         egui_node_graph::InputParamKind::ConstantOnly,
        //         true,
        //     );
        // };
        let input_uint = |graph: &mut Graph, name: &str, default: UnsignedInteger| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::UnsignedInteger,
                NodeValueType::UnsignedInteger { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_uint_vector2 = |graph: &mut Graph, name: &str, default: UVec2| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::UVec2,
                NodeValueType::UVec2 { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_uint_vector3 = |graph: &mut Graph, name: &str, default: UVec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::UVec3,
                NodeValueType::UVec3 { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_filepath = |graph: &mut Graph, name: &str, default: Filepath| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Filepath,
                NodeValueType::Filepath { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_float = |graph: &mut Graph, name: &str, default: Float| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Float,
                NodeValueType::Float { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        // let input_vector2 = |graph: &mut Graph, name: &str, default: Vec2| {
        //     graph.add_input_param(
        //         node_id,
        //         name.to_string(),
        //         NodeDataType::Vec2,
        //         NodeValueType::Vec2 { value: default },
        //         egui_node_graph::InputParamKind::ConstantOnly,
        //         true,
        //     );
        // };
        let input_vector3 = |graph: &mut Graph, name: &str, default: Vec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Vec3,
                NodeValueType::Vec3 { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector4 = |graph: &mut Graph, name: &str, default: Vec4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Vec4,
                NodeValueType::Vec4 { value: default },
                egui_node_graph::InputParamKind::ConstantOnly,
                true,
            );
        };
        // let input_matrix3 = |graph: &mut Graph, name: &str, default: Mat3| {
        //     graph.add_input_param(
        //         node_id,
        //         name.to_string(),
        //         NodeDataType::Mat3,
        //         NodeValueType::Mat3 { value: default },
        //         egui_node_graph::InputParamKind::ConstantOnly,
        //         true,
        //     );
        // };
        let input_matrix4 = |graph: &mut Graph, name: &str, default: Mat4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Mat4,
                NodeValueType::Mat4 { value: default },
                egui_node_graph::InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_camera = |graph: &mut Graph, name: &str, default: Camera| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Camera,
                NodeValueType::Camera { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_light = |graph: &mut Graph, name: &str, default: Lights| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Light,
                NodeValueType::Light { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_material = |graph: &mut Graph, name: &str, default: Material| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Material,
                NodeValueType::Material { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };

        let input_primitive = |graph: &mut Graph, name: &str, default: Primitives| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Primitive,
                NodeValueType::Primitive { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };

        let input_procedural_texture =
            |graph: &mut Graph, name: &str, default: ProceduralTexture| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    NodeDataType::ProceduralTexture,
                    NodeValueType::ProceduralTexture { value: default },
                    egui_node_graph::InputParamKind::ConnectionOnly,
                    true,
                );
            };
        let input_scene = |graph: &mut Graph, name: &str, default: Scene| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::Scene,
                NodeValueType::Scene { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };
        let input_render_pass = |graph: &mut Graph, name: &str, default: RenderPasses| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                NodeDataType::RenderPass,
                NodeValueType::RenderPass { value: default },
                egui_node_graph::InputParamKind::ConnectionOnly,
                true,
            );
        };

        let output_matrix4 = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Mat4);
        };
        let output_camera = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Camera);
        };
        let output_light = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Light);
        };
        let output_material = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Material);
        };
        let output_primitive = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Primitive);
        };
        let output_procedural_texture = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::ProceduralTexture);
        };
        let output_scene = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::Scene);
        };
        let output_render_pass = |graph: &mut Graph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), NodeDataType::RenderPass);
        };

        match self {
            NodeTemplate::Axis => {
                input_matrix4(
                    graph,
                    "axis",
                    Mat4::new(glam::Mat4::IDENTITY)
                        .with_ui_data(UIData::default().with_tooltip("The parent axis.")),
                );
                input_vector3(
                    graph,
                    "translate",
                    Vec3::new(glam::Vec3::ZERO).with_ui_data(
                        UIData::default().with_tooltip("The translation of this axis."),
                    ),
                );
                input_vector3(
                    graph,
                    "rotate",
                    Vec3::new(glam::Vec3::ZERO)
                        .with_ui_data(UIData::default().with_tooltip("The rotation of this axis.")),
                );
                input_float(
                    graph,
                    "uniform_scale",
                    Float::new(1.)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The uniform scale of this axis.\n
                            We use uniform scale because the signed distance
                            fields cannot have their individual axes scaled."
                        }))
                        .with_range(0.01..=10.0),
                );
                output_matrix4(graph, "out");
            }
            NodeTemplate::Camera => {
                let default_camera = camera::Camera::default();
                input_float(
                    graph,
                    "focal_length",
                    Float::new(default_camera.focal_length)
                        .with_ui_data(
                            UIData::default().with_tooltip("The focal length of the camera."),
                        )
                        .with_range(5.0..=100.),
                );
                input_float(
                    graph,
                    "focal_distance",
                    Float::new(default_camera.focal_distance)
                        .with_ui_data(
                            UIData::default().with_tooltip("The focal distance of the camera."),
                        )
                        .with_range(0.1..=10.),
                );
                input_float(
                    graph,
                    "f_stop",
                    Float::new(default_camera.f_stop)
                        .with_ui_data(UIData::default().with_tooltip("The f-stop of the camera."))
                        .with_range(0.1..=30.),
                );
                input_float(
                    graph,
                    "horizontal_aperture",
                    Float::new(default_camera.horizontal_aperture)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The horizontal aperture of the camera."),
                        )
                        .with_range(0.1..=50.),
                );
                input_float(
                    graph,
                    "near_plane",
                    Float::new(default_camera.near_plane)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The distance to the near plane of the camera."),
                        )
                        .with_range(0.1..=10.),
                );
                input_float(
                    graph,
                    "far_plane",
                    Float::new(default_camera.far_plane)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The distance to the far plane of the camera."),
                        )
                        .with_range(11.0..=10000.),
                );
                input_uint_vector2(
                    graph,
                    "sensor_resolution",
                    UVec2::new(default_camera.sensor_resolution)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The resolution of the camera sensor, used
                                when generating a texture by rendering
                                through this camera.",
                        }))
                        .with_range(1..=4096),
                );
                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(default_camera.camera_to_world).with_ui_data(
                        UIData::default().with_tooltip("The world matrix/axis of the camera."),
                    ),
                );
                input_bool(
                    graph,
                    "enable_depth_of_field",
                    Bool::new(default_camera.enable_depth_of_field).with_ui_data(
                        UIData::default().with_tooltip(
                            "If enabled, this camera will render with depth of field.",
                        ),
                    ),
                );
                input_bool(
                    graph,
                    "latlong",
                    Bool::new(default_camera.latlong).with_ui_data(
                        UIData::default()
                            .with_tooltip("Output a LatLong, 360 degree field of view image."),
                    ),
                );
                output_camera(graph, "out");
            }
            NodeTemplate::Light => {
                let default_light = lights::Light::default();
                input_light(
                    graph,
                    "lights",
                    Lights::new(vec![]).with_ui_data(
                        UIData::default()
                            .with_tooltip("Chain other non-physical lights in the scene."),
                    ),
                );
                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(glam::Mat4::IDENTITY).with_ui_data(UIData::default().with_tooltip(
                        indoc! {
                            "The world matrix to apply to the light (point and
                            directional only).\n
                            \tPoint: Will affect the position of the light.\n
                            \tDirectional: Will affect the direction vector of the light."
                        },
                    )),
                );
                input_combo_box(
                    graph,
                    "light_type",
                    ComboBox::from(default_light.light_type).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The type of non-physical light to create.\n
                            \tPoint: A point light.\n
                            \tDirectional: A directional light.\n
                            \tAmbient: An ambient light (will be a uniform colour).\n
                            \tAmbient Occlusion: Ambient occlusion."
                        }),
                    ),
                );
                input_vector3(
                    graph,
                    "direction",
                    Vec3::new(glam::Vec3::NEG_Y).with_ui_data(
                        UIData::default()
                            .with_tooltip("The direction vector of the light.")
                            .with_hidden(),
                    ),
                );
                input_vector3(
                    graph,
                    "position",
                    Vec3::new(glam::Vec3::Y).with_ui_data(
                        UIData::default()
                            .with_tooltip("The position of the point light.")
                            .with_hidden(),
                    ),
                );
                input_uint(
                    graph,
                    "iterations",
                    UnsignedInteger::new(default_light.dimensional_data.x as u32)
                        .with_ui_data(UIData::default().with_tooltip(
                            "The number of iterations used to compute the occlusion.",
                        ))
                        .with_range(1..=10),
                );
                input_float(
                    graph,
                    "intensity",
                    Float::new(default_light.intensity)
                        .with_ui_data(UIData::default().with_tooltip("The intensity of the light."))
                        .with_range(0.0..=10.),
                );
                input_uint(
                    graph,
                    "falloff",
                    UnsignedInteger::new(default_light.falloff)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The exponent of the falloff (point lights only)."),
                        )
                        .with_range(0..=4),
                );
                input_vector3(
                    graph,
                    "colour",
                    Vec3::new(default_light.colour)
                        .with_ui_data(UIData::default().with_tooltip("The light colour."))
                        .as_colour(),
                );
                input_float(
                    graph,
                    "shadow_hardness",
                    Float::new(default_light.shadow_hardness)
                        .with_ui_data(
                            UIData::default().with_tooltip("The hardness of softened shadows."),
                        )
                        .with_range(1.0..=100.),
                );
                input_bool(
                    graph,
                    "soften_shadows",
                    Bool::new(default_light.soften_shadows).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "If enabled, the shadows will be softened (directional
                            and point lights only)."
                        }),
                    ),
                );
                output_light(graph, "out");
            }
            NodeTemplate::Grade => {
                let default_grade = textures::Grade::default();
                input_render_pass(
                    graph,
                    "texture",
                    RenderPasses::new(vec![]).with_ui_data(UIData::default().with_tooltip(
                        indoc! {
                            "A render pass which results in the production of the
                            texture to grade."
                        },
                    )),
                );
                input_float(
                    graph,
                    "black_point",
                    Float::new(default_grade.black_point).with_ui_data(
                        UIData::default().with_tooltip("The black point of the texture."),
                    ),
                );
                input_float(
                    graph,
                    "white_point",
                    Float::new(default_grade.white_point).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The white point of the texture."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "lift",
                    Float::new(default_grade.lift).with_ui_data(UIData::default().with_tooltip(
                        indoc! {
                            "The lift to apply to the texture."
                        },
                    )),
                );
                input_float(
                    graph,
                    "gain",
                    Float::new(default_grade.gain)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The gain to apply to the texture colour."),
                        )
                        .with_range(0.0001..=10.),
                );
                input_float(
                    graph,
                    "gamma",
                    Float::new(default_grade.gamma)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The gamma to apply to the texture. This is computed
                            by raising the colour to the power of 1/gamma."
                        }))
                        .with_range(0.01..=2.),
                );
                input_bool(
                    graph,
                    "invert",
                    Bool::new(default_grade.invert)
                        .with_ui_data(UIData::default().with_tooltip("Invert the colour.")),
                );
                output_render_pass(graph, "out");
            }
            NodeTemplate::Material => {
                let default_material = materials::Material::default();
                input_vector3(
                    graph,
                    "diffuse_colour",
                    Vec3::new(default_material.diffuse_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The diffuse colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "diffuse_colour_texture",
                    ProceduralTexture::new(default_material.diffuse_colour_texture).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Texture that affects the diffuse colour of this material."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "specular_probability",
                    Float::new(default_material.specular_probability).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The probability that light will be specularly reflected
                            when it interacts with this material."
                        }),
                    ),
                );
                input_procedural_texture(
                    graph,
                    "specular_probability_texture",
                    ProceduralTexture::new(default_material.specular_probability_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the specular probability of this material."
                        })),
                );
                input_float(
                    graph,
                    "specular_roughness",
                    Float::new(default_material.specular_roughness).with_ui_data(
                        UIData::default().with_tooltip(
                            "The roughness of the material when specularly reflected.",
                        ),
                    ),
                );
                input_procedural_texture(
                    graph,
                    "specular_roughness_texture",
                    ProceduralTexture::new(default_material.specular_roughness_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the specular roughness of this material."
                        })),
                );
                input_vector3(
                    graph,
                    "specular_colour",
                    Vec3::new(default_material.specular_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The specular colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "specular_colour_texture",
                    ProceduralTexture::new(default_material.specular_colour_texture).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Texture that affects the specular colour of this material."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "transmissive_probability",
                    Float::new(default_material.transmissive_probability).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The probability that light will be transmitted through
                            the material (before accounting for Fresnel) when it
                            interacts with this material."
                        }),
                    ),
                );
                input_procedural_texture(
                    graph,
                    "transmissive_probability_texture",
                    ProceduralTexture::new(default_material.transmissive_probability_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the transmissive probability of this material."
                        })),
                );
                input_float(
                    graph,
                    "transmissive_roughness",
                    Float::new(default_material.transmissive_roughness).with_ui_data(
                        UIData::default()
                            .with_tooltip("The roughness when transmitted through the material."),
                    ),
                );
                input_procedural_texture(
                    graph,
                    "transmissive_roughness_texture",
                    ProceduralTexture::new(default_material.transmissive_roughness_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the transmissive roughness of this material."
                        })),
                );
                input_float(
                    graph,
                    "extinction_coefficient",
                    Float::new(default_material.extinction_coefficient).with_ui_data(
                        UIData::default()
                            .with_tooltip("The extinction coefficient of the material."),
                    ),
                );
                input_vector3(
                    graph,
                    "transmissive_colour",
                    Vec3::new(default_material.transmissive_colour)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The transmitted colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "transmissive_colour_texture",
                    ProceduralTexture::new(default_material.transmissive_colour_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the transmissive colour of this material."
                        })),
                );
                input_float(
                    graph,
                    "emissive_intensity",
                    Float::new(default_material.emissive_intensity)
                        .with_ui_data(UIData::default().with_tooltip(
                            "The intensity of light that will be emitted from the material.",
                        ))
                        .with_range(0.0..=100.0),
                );
                input_vector3(
                    graph,
                    "emissive_colour",
                    Vec3::new(default_material.emissive_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The emissive colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "emissive_colour_texture",
                    ProceduralTexture::new(default_material.emissive_colour_texture).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Texture that affects the emissive colour of this material."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "refractive_index",
                    Float::new(default_material.refractive_index)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The index of refraction of the material."),
                        )
                        .with_range(0.1..=5.),
                );
                input_procedural_texture(
                    graph,
                    "refractive_index_texture",
                    ProceduralTexture::new(default_material.refractive_index_texture).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Texture that affects the refractive index of this material."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "scattering_coefficient",
                    Float::new(default_material.scattering_coefficient).with_ui_data(
                        UIData::default()
                            .with_tooltip("The scattering coefficient of the material."),
                    ),
                );
                input_vector3(
                    graph,
                    "scattering_colour",
                    Vec3::new(default_material.scattering_colour)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The scattering colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "scattering_colour_texture",
                    ProceduralTexture::new(default_material.scattering_colour_texture)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Texture that affects the scattering colour of this material."
                        })),
                );
                output_material(graph, "out");
            }
            NodeTemplate::Primitive => {
                let default_primitive = geometry::primitive::Primitive::default();
                input_primitive(
                    graph,
                    "siblings",
                    Primitives::new(vec![]).with_ui_data(UIData::default().with_tooltip(indoc! {
                        "The siblings of this primitive.\n
                        These will be unaffected by this primitive's
                        blend_type and transform."
                    })),
                );
                input_primitive(
                    graph,
                    "children",
                    Primitives::new(vec![]).with_ui_data(UIData::default().with_tooltip(indoc! {
                        "The children of this primitive.\n
                        These will be transformed using this primitive's
                        blend_type and transform.\n
                        If this primitive is a bounding volume, children
                        outside of its bounds will not be rendered, and
                        increased performance can be achieved when
                        rendering the children."
                    })),
                );
                input_material(
                    graph,
                    "material",
                    Material::new(default_primitive.material)
                        .with_ui_data(UIData::default().with_tooltip("The primitive's material.")),
                );
                input_combo_box(
                    graph,
                    "shape",
                    ComboBox::from(default_primitive.shape).with_ui_data(
                        UIData::default().with_tooltip("The shape of the primitive."),
                    ),
                );
                // Sphere dimensions
                input_float(
                    graph,
                    "radius",
                    Float::new(0.5)
                        .with_ui_data(UIData::default().with_tooltip("The radius."))
                        .with_range(0.0..=10.),
                );

                // Ellipsoid dimensions
                input_vector3(
                    graph,
                    "radii",
                    Vec3::new(glam::Vec3::splat(0.5)).with_ui_data(
                        UIData::default()
                            .with_tooltip("The radii of the ellipsoid.")
                            .with_hidden(),
                    ),
                );

                // Cut Sphere dimensions
                input_float(
                    graph,
                    "height",
                    Float::new(0.25)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The height (y-axis).")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Death Star dimensions
                input_float(
                    graph,
                    "hollow_radius",
                    Float::new(0.5)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(
                                    "The radius of the sphere that is cut from the solid.",
                                )
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );
                input_float(
                    graph,
                    "hollow_height",
                    Float::new(0.75)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The height (y-axis) of the center of the sphere
                                    that is cut from the solid, above solidRadius +
                                    hollowRadius, the result will be a standard
                                    sphere of radius solidRadius."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Solid Angle Dimensions
                input_float(
                    graph,
                    "solid_angle",
                    Float::new(30.)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The angle between the edge of the solid angle and the
                                    y-axis on [0-180] measured between the y-axis and wall
                                    of the solid angle."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=180.),
                );

                // Rectangular Prism Dimensions
                input_float(
                    graph,
                    "width",
                    Float::new(0.5)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The width (x-axis).")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );
                input_float(
                    graph,
                    "depth",
                    Float::new(0.75)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The depth (z-axis).")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Hollow Sphere dimensions
                input_float(
                    graph,
                    "thickness",
                    Float::new(0.05).with_ui_data(
                        UIData::default()
                            .with_tooltip("The thickness of the walls.")
                            .with_hidden(),
                    ),
                );

                // Rhombus Dimensions
                input_float(
                    graph,
                    "corner_radius",
                    Float::new(0.05).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The radius of the corners of the rhombus' xy-plane parallel face.",
                            )
                            .with_hidden(),
                    ),
                );

                // Triangular Prism Dimensions
                input_float(
                    graph,
                    "base",
                    Float::new(0.5)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The equilateral triangles edge length (xy-plane).")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Plane Dimensions
                input_vector3(
                    graph,
                    "normal",
                    Vec3::new(glam::Vec3::Z).with_ui_data(
                        UIData::default()
                            .with_tooltip("The normal direction of the plane.")
                            .with_hidden(),
                    ),
                );

                // Capsule Dimensions
                input_float(
                    graph,
                    "negative_height",
                    Float::new(0.25)
                        .with_ui_data(UIData::default().with_tooltip(
                            "The distance along the negative y-axis before entering the dome.",
                        ).with_hidden())
                        .with_range(0.0..=10.),
                );
                input_float(
                    graph,
                    "positive_height",
                    Float::new(0.25)
                        .with_ui_data(UIData::default().with_tooltip(
                            "The distance along the positive y-axis before entering the dome.",
                        ).with_hidden())
                        .with_range(0.0..=10.),
                );

                // Cone Dimensions
                input_float(
                    graph,
                    "angle",
                    Float::new(30.)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The angle between the tip and base of the cone [0-90]
                                    measured between the y-axis and wall of the cone."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=90.),
                );

                // Capped Cone Dimensions
                input_float(
                    graph,
                    "lower_radius",
                    Float::new(0.25)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The radius of the cone at y = -height/2.")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );
                input_float(
                    graph,
                    "upper_radius",
                    Float::new(0.125)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The radius of the cone at y = height/2.")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Torus Dimensions
                input_float(
                    graph,
                    "ring_radius",
                    Float::new(0.3)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The radius (xy-plane) of the ring of the torus.")
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );
                input_float(
                    graph,
                    "tube_radius",
                    Float::new(0.2)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The radius of the tube of the torus.")
                                .with_hidden(),
                        )
                        .with_range(0.0..=5.),
                );

                // Capped Torus Dimensions
                input_float(
                    graph,
                    "cap_angle",
                    Float::new(30.)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The angle (xy-plane, symmetric about y-axis) to
                                    cap at, in the range [0-180.]."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=180.),
                );

                // Octahedron Dimensions
                input_float(
                    graph,
                    "radial_extent",
                    Float::new(0.5)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The maximum distance along the x, y, and z axes.
                                    ie. The vertices are at +/-radial_extent on the x, y,
                                    and z axes."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=10.),
                );

                // Mandelbulb Dimensions
                input_float(
                    graph,
                    "power",
                    Float::new(8.)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(
                                    "One greater than the axes of symmetry in the xy-plane.",
                                )
                                .with_hidden(),
                        )
                        .with_range(2.0..=30.),
                );
                input_uint(
                    graph,
                    "iterations",
                    UnsignedInteger::new(10)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The number of iterations to compute, the higher this
                                    is, the slower it will be to compute, but the more
                                    detail the fractal will have."
                                })
                                .with_hidden(),
                        )
                        .with_range(1..=30),
                );
                input_float(
                    graph,
                    "max_square_radius",
                    Float::new(4.)
                        .with_ui_data(UIData::default().with_tooltip(
                            "When the square radius has reached this length, stop iterating.",
                        ).with_hidden())
                        .with_range(1.0..=9.),
                );

                // Mandelbox Dimensions
                input_float(
                    graph,
                    "scale",
                    Float::new(-1.75)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The amount to scale the position between folds.
                                    Can be negative or positive.",
                                })
                                .with_hidden(),
                        )
                        .with_range(-5.0..=5.),
                );
                input_float(
                    graph,
                    "min_square_radius",
                    Float::new(0.001)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(
                                    "The minimum square radius to use when spherically folding.",
                                )
                                .with_hidden(),
                        )
                        .with_range(0.001..=1.),
                );
                input_float(
                    graph,
                    "folding_limit",
                    Float::new(0.8)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "Clamp the position between +/- this value when
                                    performing the box fold. Higher values will result
                                    in a denser fractal.",
                                })
                                .with_hidden(),
                        )
                        .with_range(0.01..=2.),
                );

                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(default_primitive.local_to_world).with_ui_data(
                        UIData::default().with_tooltip("The world matrix/axis of the primitive."),
                    ),
                );
                input_float(
                    graph,
                    "edge_radius",
                    Float::new(default_primitive.edge_radius).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The thickness of the walls of the shape, if
                            the shape is hollow.",
                        }),
                    ),
                );
                input_combo_box(
                    graph,
                    "repetition",
                    ComboBox::from(default_primitive.repetition).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Repeat objects in the scene with no extra memory
                            consumption. Note that if the repeated objects overlap
                            some strange things can occur."
                        }),
                    ),
                );
                input_uint_vector3(
                    graph,
                    "negative_repetitions",
                    UVec3::new(default_primitive.negative_repetitions).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The number of repetitions along the negative x, y, and z axes.",
                            )
                            .with_hidden(),
                    ),
                );
                input_uint_vector3(
                    graph,
                    "positive_repetitions",
                    UVec3::new(default_primitive.positive_repetitions).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The number of repetitions along the positive x, y, and z axes.",
                            )
                            .with_hidden(),
                    ),
                );
                input_vector3(
                    graph,
                    "spacing",
                    Vec3::new(default_primitive.spacing).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The spacing along each positive axis to repeat the objects.",
                            )
                            .with_hidden(),
                    ),
                );
                input_bool(
                    graph,
                    "bounding_volume",
                    Bool::new(default_primitive.bounding_volume).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "If enabled, this object will act as a bounding volume
                            for all its children. This means that until a ray hits
                            the bounding volume, none of the child object's signed
                            distance fields will be computed. This can vastly
                            improve performance, especially when many complex
                            objects are far from the camera. This option does
                            not always play well with lighting effects that depend
                            on the number of iterations in the computation such
                            as 'ambient occlusion' and 'softened shadows' due
                            to the variation near the surface of the bounding object."
                        }),
                    ),
                );
                input_combo_box(
                    graph,
                    "blend_type",
                    ComboBox::from(default_primitive.blend_type).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The type of interaction this object will have with its children.\n
                            \tUnion: All objects will appear as normal.\n
                            \tSubtraction: This object will be subtracted from all of its\n
                            \t\tchildren, leaving holes.\n
                            \tIntersection: Only the region where this object and its\n
                            \t\tchildren overlap will remain.\n
                            \tSmooth Union: All children will smoothly blend together\n
                            \t\twith this object according to the 'blend strength'.\n
                            \tSmooth Subtraction:This object will be subtracted from all\n
                            \t\tof its children,  leaving holes that are smoothed\n
                            \t\taccording to the 'blend strength'.\n
                            \tSmooth Intersection: Only the region where this object\n
                            \t\tand its children overlap will remain, and the remaining\n
                            \t\tregions will be smoothed according to the 'blend\n
                            \t\tstrength'.",
                        }),
                    ),
                );
                input_float(
                    graph,
                    "blend_strength",
                    Float::new(default_primitive.blend_strength).with_ui_data(
                        UIData::default()
                            .with_tooltip("The amount to blend with this primitive's children."),
                    ),
                );
                input_bool_vector3(
                    graph,
                    "mirror",
                    BVec3::new(default_primitive.mirror).with_ui_data(
                        UIData::default().with_tooltip("Mirror along the x, y, and z axes."),
                    ),
                );
                input_bool(
                    graph,
                    "hollow",
                    Bool::new(default_primitive.hollow).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "If enabled, the object will be hollow, with a
                            thickness of 'wall thickness'."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "wall_thickness",
                    Float::new(default_primitive.wall_thickness)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The thickness of the walls of the shape, if
                                    the shape is hollow.",
                                })
                                .with_hidden(),
                        )
                        .with_range(0.001..=1.),
                );
                input_bool(
                    graph,
                    "elongate",
                    Bool::new(default_primitive.elongate).with_ui_data(
                        UIData::default().with_tooltip("Enable the elongation of the object."),
                    ),
                );
                input_vector3(
                    graph,
                    "elongation",
                    Vec3::new(default_primitive.elongation).with_ui_data(
                        UIData::default()
                            .with_tooltip("The elongation of the object along the respective axes.")
                            .with_hidden(),
                    ),
                );
                output_primitive(graph, "out");
            }
            NodeTemplate::ProceduralTexture => {
                let default_procedural_texture = materials::ProceduralTexture::default();
                input_combo_box(
                    graph,
                    "texture_type",
                    ComboBox::from(default_procedural_texture.texture_type).with_ui_data(
                        UIData::default().with_tooltip("The type of texture to use."),
                    ),
                );
                input_vector4(
                    graph,
                    "scale",
                    Vec4::new(default_procedural_texture.scale)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The scale factor of the texture.")
                                .with_hidden(),
                        )
                        .with_collapsed(),
                );
                input_float(
                    graph,
                    "black_point",
                    Float::new(default_procedural_texture.grade.black_point).with_ui_data(
                        UIData::default()
                            .with_tooltip("The black point of the texture.")
                            .with_hidden(),
                    ),
                );
                input_float(
                    graph,
                    "white_point",
                    Float::new(default_procedural_texture.grade.white_point).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "The white point of the texture."
                            })
                            .with_hidden(),
                    ),
                );
                input_float(
                    graph,
                    "lift",
                    Float::new(default_procedural_texture.grade.lift).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "The lift to apply to the texture."
                            })
                            .with_hidden(),
                    ),
                );
                input_float(
                    graph,
                    "gain",
                    Float::new(default_procedural_texture.grade.gain)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The gain to apply to the texture colour.")
                                .with_hidden(),
                        )
                        .with_range(0.0001..=10.),
                );
                input_uint(
                    graph,
                    "octaves",
                    UnsignedInteger::new(default_procedural_texture.octaves)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The number of different frequencies to superimpose.")
                                .with_hidden(),
                        )
                        .with_range(1..=12),
                );
                input_float(
                    graph,
                    "lacunarity",
                    Float::new(default_procedural_texture.lacunarity)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The lacunarity is the initial frequency of the noise,
                                    and the amount to scale the frequency for each octave."
                                })
                                .with_hidden(),
                        )
                        .with_range(1.0..=10.),
                );
                input_float(
                    graph,
                    "amplitude_gain",
                    Float::new(default_procedural_texture.amplitude_gain)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The gain to apply to the texture. This scales the
                                    noise amplitude between octaves."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.0..=2.),
                );
                input_float(
                    graph,
                    "gamma",
                    Float::new(default_procedural_texture.grade.gamma)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The gamma to apply to the texture. This is computed
                                    by raising the colour to the power of 1/gamma."
                                })
                                .with_hidden(),
                        )
                        .with_range(0.01..=2.),
                );
                input_vector4(
                    graph,
                    "low_frequency_scale",
                    Vec4::new(default_procedural_texture.low_frequency_scale).with_ui_data(
                        UIData::default()
                            .with_tooltip("The amount to scale lower frequencies between octaves.")
                            .with_hidden(),
                    ),
                );
                input_vector4(
                    graph,
                    "high_frequency_scale",
                    Vec4::new(default_procedural_texture.high_frequency_scale).with_ui_data(
                        UIData::default()
                            .with_tooltip("The amount to scale higher frequencies between octaves.")
                            .with_hidden(),
                    ),
                );
                input_vector4(
                    graph,
                    "low_frequency_translation",
                    Vec4::new(default_procedural_texture.low_frequency_translation).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The amount to translate lower frequencies between octaves.",
                            )
                            .with_hidden(),
                    ),
                );
                input_vector4(
                    graph,
                    "high_frequency_translation",
                    Vec4::new(default_procedural_texture.high_frequency_translation).with_ui_data(
                        UIData::default()
                            .with_tooltip(
                                "The amount to translate higher frequencies between octaves.",
                            )
                            .with_hidden(),
                    ),
                );
                input_bool(
                    graph,
                    "invert",
                    Bool::new(default_procedural_texture.grade.invert).with_ui_data(
                        UIData::default()
                            .with_tooltip("Invert the colour.")
                            .with_hidden(),
                    ),
                );
                input_bool(
                    graph,
                    "use_trap_colour",
                    Bool::new(default_procedural_texture.use_trap_colour).with_ui_data(
                        UIData::default()
                            .with_tooltip("Multiply by the trap colour when rendering fractals.")
                            .with_hidden(),
                    ),
                );
                input_vector3(
                    graph,
                    "hue_rotation_angles",
                    Vec3::new(default_procedural_texture.hue_rotation_angles).with_ui_data(
                        UIData::default()
                            .with_tooltip("Rotation of the colour.")
                            .with_hidden(),
                    ),
                );

                output_procedural_texture(graph, "out");
            }
            NodeTemplate::RayMarcher => {
                let default_ray_marcher = ray_marcher::RayMarcherRenderData::default();
                input_scene(
                    graph,
                    "scene",
                    Scene::new(default_ray_marcher.scene)
                        .with_ui_data(UIData::default().with_tooltip("The scene to render.")),
                );
                input_uint(
                    graph,
                    "max_ray_steps",
                    UnsignedInteger::new(default_ray_marcher.max_ray_steps)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Limits the number of times the rays can intersect
                            an object per subpixel."
                        }))
                        .with_range(100..=100000),
                );
                input_uint(
                    graph,
                    "max_bounces",
                    UnsignedInteger::new(default_ray_marcher.max_bounces)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Limits the number of times the rays can intersect
                            an object per subpixel."
                        }))
                        .with_range(1..=100),
                );
                input_float(
                    graph,
                    "hit_tolerance",
                    Float::new(default_ray_marcher.hit_tolerance)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The ray will be considered to have hit an object
                            when it is within this distance of its surface."
                        }))
                        .with_range(0.00001..=0.1),
                );
                input_float(
                    graph,
                    "shadow_bias",
                    Float::new(default_ray_marcher.shadow_bias)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "After intersecting an object the ray is offset from
                            the surface before continuing. Multiply that offset
                            distance by this factor."
                        }))
                        .with_range(1.0..=5.0),
                );
                input_float(
                    graph,
                    "max_brightness",
                    Float::new(default_ray_marcher.max_brightness)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The maximum brightness of a pixel. This protects
                            against overflowing to infinity."
                        }))
                        .with_range(1.0..=1000000.),
                );
                input_uint(
                    graph,
                    "seed",
                    UnsignedInteger::new(default_ray_marcher.seed)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The seed used to generate per-pixel, random seeds.
                            Be sure this is different for each parallel render."
                        }))
                        .with_range(0..=100),
                );
                input_bool(
                    graph,
                    "dynamic_level_of_detail",
                    Bool::new(default_ray_marcher.dynamic_level_of_detail).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Increase the hit tolerance the farther the ray
                            travels without hitting a surface. This has performance
                            and antialiasing benefits."
                        }),
                    ),
                );
                input_uint(
                    graph,
                    "equiangular_samples",
                    UnsignedInteger::new(default_ray_marcher.equiangular_samples)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The number of equi-angular samples to perform if
                            the extinction/scattering coefficients are greater
                            than 0. This enables participating media such as
                            fog/smoke/clouds to be traced."
                        }))
                        .with_range(0..=10),
                );
                input_bool(
                    graph,
                    "light_sampling",
                    Bool::new(default_ray_marcher.light_sampling).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Send a ray towards light sources when hitting a diffuse surface."
                        }),
                    ),
                );
                input_uint(
                    graph,
                    "max_light_sampling_bounces",
                    UnsignedInteger::new(default_ray_marcher.max_light_sampling_bounces)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(indoc! {
                                    "The maximum number of bounces during light sampling.
                            Light sampling will be disabled if this is 0. Light
                            sampling means that each time a surface is hit, the
                            direct illumination from lights in the scene will be
                            computed, which helps to reduce noise very quickly.\nTODO"
                                })
                                .with_hidden(),
                        )
                        .with_range(0..=50),
                );
                input_bool(
                    graph,
                    "sample_atmosphere",
                    Bool::new(default_ray_marcher.sample_atmosphere).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "Include the skybox in the list of lights that can be
                                sampled during light sampling."
                            })
                            .with_hidden(),
                    ),
                );
                input_float(
                    graph,
                    "light_sampling_bias",
                    Float::new(default_ray_marcher.light_sampling_bias).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "A fully biased (1) light sampling means that on each
                            light sample the ray will be initialised pointing
                            directly at the light. Reducing this bias means that
                            some rays will be pointed away from the light. This,
                            when combined with multiple 'max light sampling
                            bounces' allows the renderer to find difficult paths,
                            such as volumetric caustics.\nTODO"
                            })
                            .with_hidden(),
                    ),
                );
                input_bool(
                    graph,
                    "secondary_sampling",
                    Bool::new(default_ray_marcher.secondary_sampling).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "Sample the artificial lights (those in the 'lights'
                                input) while casting shadow rays for light sampling.\nTODO"
                            })
                            .with_hidden(),
                    ),
                );
                input_combo_box(
                    graph,
                    "output_aov",
                    ComboBox::from(default_ray_marcher.output_aov).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The AOV type to output.\nThe stats AOV has the
                            average number of bounces in the red channel,
                            average number of steps in the green channel,
                            and the distance travelled in the blue channel.
                            Each is displayed as a fraction of the maximums."
                        }),
                    ),
                );
                output_render_pass(graph, "out");
            }
            NodeTemplate::Scene => {
                let default_scene = scene::Scene::default();
                input_camera(
                    graph,
                    "render_camera",
                    Camera::new(default_scene.render_camera).with_ui_data(
                        UIData::default().with_tooltip("The camera to render the scene through."),
                    ),
                );
                input_primitive(
                    graph,
                    "primitives",
                    Primitives::new(default_scene.primitives).with_ui_data(
                        UIData::default().with_tooltip("The primitives in the scene."),
                    ),
                );
                input_light(
                    graph,
                    "lights",
                    Lights::new(default_scene.lights)
                        .with_ui_data(UIData::default().with_tooltip("The lights in the scene.")),
                );
                input_material(
                    graph,
                    "atmosphere",
                    Material::new(default_scene.atmosphere)
                        .with_ui_data(UIData::default().with_tooltip(
                        "The material to apply to the atmosphere (smoke, extinction, hdri, etc.).",
                    )),
                );
                output_scene(graph, "out");
            }
            NodeTemplate::Texture => {
                let default_texture = textures::Texture::default();
                input_filepath(
                    graph,
                    "filepath",
                    Filepath::new(default_texture.filepath)
                        .with_ui_data(UIData::default().with_tooltip("The path to the texture.")),
                );
                output_render_pass(graph, "out");
            }
        }
    }
}

pub struct AllNodeTemplates;
impl egui_node_graph::NodeTemplateIter for AllNodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user.
        Self::Item::iter().collect()
    }
}
