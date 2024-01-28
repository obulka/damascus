use std::borrow::Cow;

use egui_node_graph::{Graph, InputParamKind, NodeId, NodeTemplateIter, NodeTemplateTrait};
use indoc::indoc;
use strum::{EnumIter, IntoEnumIterator};

use damascus_core::{geometry, lights, materials, renderers, scene};

use super::{
    value_type::{
        BVec3, Bool, Colour, ComboBox, DamascusValueType, Float, Integer, Mat3, Mat4, RangedInput,
        UIData, UIInput, UVec3, UnsignedInteger, Vec2, Vec3, Vec4,
    },
    DamascusDataType, DamascusGraph, DamascusGraphState, DamascusResponse,
};

mod callbacks;
mod node_data;
pub use callbacks::NodeCallbacks;
use callbacks::{LightCallbacks, PrimitiveCallbacks};
pub use node_data::DamascusNodeData;

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy, EnumIter, Eq, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DamascusNodeTemplate {
    Axis,
    Camera,
    Light,
    Material,
    Primitive,
    ProceduralTexture,
    RayMarcher,
    Scene,
}

impl NodeCallbacks for DamascusNodeTemplate {
    fn input_value_changed(&self, graph: &mut DamascusGraph, node_id: NodeId, input_name: &String) {
        match self {
            DamascusNodeTemplate::Light => {
                LightCallbacks.input_value_changed(graph, node_id, input_name)
            }
            DamascusNodeTemplate::Primitive => {
                PrimitiveCallbacks.input_value_changed(graph, node_id, input_name)
            }
            _ => {}
        }
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for DamascusNodeTemplate {
    type NodeData = DamascusNodeData;
    type DataType = DamascusDataType;
    type ValueType = DamascusValueType;
    type UserState = DamascusGraphState;
    type CategoryType = (); // TODO think about adding categories

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            DamascusNodeTemplate::Axis => "axis",
            DamascusNodeTemplate::Camera => "camera",
            DamascusNodeTemplate::Light => "light",
            DamascusNodeTemplate::Material => "material",
            DamascusNodeTemplate::Primitive => "primitive",
            DamascusNodeTemplate::ProceduralTexture => "procedural texture",
            DamascusNodeTemplate::RayMarcher => "ray marcher",
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
        let input_bool = |graph: &mut DamascusGraph, name: &str, default: Bool| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Bool,
                DamascusValueType::Bool { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_bool_vector3 = |graph: &mut DamascusGraph, name: &str, default: BVec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::BVec3,
                DamascusValueType::BVec3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_combo_box = |graph: &mut DamascusGraph, name: &str, default: ComboBox| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::ComboBox,
                DamascusValueType::ComboBox { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_int = |graph: &mut DamascusGraph, name: &str, default: Integer| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Integer,
                DamascusValueType::Integer { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_uint = |graph: &mut DamascusGraph, name: &str, default: UnsignedInteger| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::UnsignedInteger,
                DamascusValueType::UnsignedInteger { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_uint_vector3 = |graph: &mut DamascusGraph, name: &str, default: UVec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::UVec3,
                DamascusValueType::UVec3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_float = |graph: &mut DamascusGraph, name: &str, default: Float| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Float,
                DamascusValueType::Float { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector2 = |graph: &mut DamascusGraph, name: &str, default: Vec2| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec2,
                DamascusValueType::Vec2 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector3 = |graph: &mut DamascusGraph, name: &str, default: Vec3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec3,
                DamascusValueType::Vec3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_vector4 = |graph: &mut DamascusGraph, name: &str, default: Vec4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Vec4,
                DamascusValueType::Vec4 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_matrix3 = |graph: &mut DamascusGraph, name: &str, default: Mat3| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Mat3,
                DamascusValueType::Mat3 { value: default },
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_matrix4 = |graph: &mut DamascusGraph, name: &str, default: Mat4| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Mat4,
                DamascusValueType::Mat4 { value: default },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_image = |graph: &mut DamascusGraph, name: &str, default: ndarray::Array4<f32>| {
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

        let input_light = |graph: &mut DamascusGraph, name: &str, default: Vec<lights::Light>| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DamascusDataType::Light,
                DamascusValueType::Light { value: default },
                InputParamKind::ConnectionOnly,
                true,
            );
        };

        let input_material =
            |graph: &mut DamascusGraph, name: &str, default: materials::Material| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Material,
                    DamascusValueType::Material { value: default },
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

        let input_procedural_texture =
            |graph: &mut DamascusGraph, name: &str, default: materials::ProceduralTexture| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::ProceduralTexture,
                    DamascusValueType::ProceduralTexture { value: default },
                    InputParamKind::ConnectionOnly,
                    true,
                );
            };

        let input_ray_marcher =
            |graph: &mut DamascusGraph, name: &str, default: renderers::RayMarcher| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::RayMarcher,
                    DamascusValueType::RayMarcher { value: default },
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
        let output_light = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Light);
        };
        let output_material = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Material);
        };
        let output_primitive = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Primitive);
        };
        let output_procedural_texture = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                DamascusDataType::ProceduralTexture,
            );
        };
        let output_ray_marcher = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::RayMarcher);
        };
        let output_scene = |graph: &mut DamascusGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DamascusDataType::Scene);
        };

        match self {
            DamascusNodeTemplate::Axis => {
                input_matrix4(
                    graph,
                    "axis",
                    Mat4::new(glam::Mat4::IDENTITY)
                        .with_ui_data(UIData::default().with_tooltip("The parent axis.")),
                );
                input_vector3(
                    graph,
                    "translate",
                    Vec3::from_vec3(glam::Vec3::ZERO).with_ui_data(
                        UIData::default().with_tooltip("The translation of this axis."),
                    ),
                );
                input_vector3(
                    graph,
                    "rotate",
                    Vec3::from_vec3(glam::Vec3::ZERO)
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
            DamascusNodeTemplate::Camera => {
                let default_camera = geometry::camera::Camera::default();
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
                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(default_camera.world_matrix).with_ui_data(
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
                output_camera(graph, "out");
            }
            DamascusNodeTemplate::Light => {
                let default_light = lights::Light::default();
                input_light(graph, "lights", vec![]);
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
                    ComboBox::from_enum::<lights::Lights>(default_light.light_type).with_ui_data(
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
                    Vec3::from_vec3(default_light.dimensional_data).with_ui_data(
                        UIData::default().with_tooltip("The direction vector of the light."),
                    ),
                );
                input_vector3(
                    graph,
                    "position",
                    Vec3::from_vec3(glam::Vec3::ZERO).with_ui_data(
                        UIData::default()
                            .with_tooltip("The position of the point light.")
                            .with_hidden(),
                    ),
                );
                input_uint(
                    graph,
                    "iterations",
                    UnsignedInteger::new(1)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip(
                                    "The number of iterations used to compute the occlusion.",
                                )
                                .with_hidden(),
                        )
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
                    Vec3::from_vec3(default_light.colour)
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
            DamascusNodeTemplate::Material => {
                let default_material = materials::Material::default();
                input_vector3(
                    graph,
                    "diffuse_colour",
                    Vec3::from_vec3(default_material.diffuse_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The diffuse colour of the material."),
                        )
                        .as_colour(),
                );
                input_procedural_texture(
                    graph,
                    "diffuse_texture",
                    materials::ProceduralTexture::default(),
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
                input_float(
                    graph,
                    "specular_roughness",
                    Float::new(default_material.specular_roughness).with_ui_data(
                        UIData::default().with_tooltip(
                            "The roughness of the material when specularly reflected.",
                        ),
                    ),
                );
                input_vector3(
                    graph,
                    "specular_colour",
                    Vec3::from_vec3(default_material.specular_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The specular colour of the material."),
                        )
                        .as_colour(),
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
                input_float(
                    graph,
                    "transmissive_roughness",
                    Float::new(default_material.transmissive_roughness).with_ui_data(
                        UIData::default()
                            .with_tooltip("The roughness when transmitted through the material."),
                    ),
                );
                input_vector3(
                    graph,
                    "transmissive_colour",
                    Vec3::from_vec3(default_material.transmissive_colour)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The transmissive colour of the material."),
                        )
                        .as_colour(),
                );
                input_float(
                    graph,
                    "emissive_probability",
                    Float::new(default_material.emissive_probability).with_ui_data(
                        UIData::default().with_tooltip(
                            "The probability that light will be emitted from the material.",
                        ),
                    ),
                );
                input_vector3(
                    graph,
                    "emissive_colour",
                    Vec3::from_vec3(default_material.emissive_colour)
                        .with_ui_data(
                            UIData::default().with_tooltip("The emissive colour of the material."),
                        )
                        .as_colour(),
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
                    Vec3::from_vec3(default_material.scattering_colour)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The scattering colour of the material."),
                        )
                        .as_colour(),
                );
                output_material(graph, "out");
            }
            DamascusNodeTemplate::Primitive => {
                let default_primitive = geometry::Primitive::default();
                input_primitive(graph, "siblings", vec![]);
                input_primitive(graph, "children", vec![]);
                input_material(graph, "material", default_primitive.material);
                input_combo_box(
                    graph,
                    "shape",
                    ComboBox::from_enum::<geometry::Shapes>(default_primitive.shape).with_ui_data(
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
                    Vec3::from_vec3(glam::Vec3::splat(0.5)).with_ui_data(
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
                    Vec3::from_vec3(glam::Vec3::Z).with_ui_data(
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
                    Mat4::new(default_primitive.world_matrix).with_ui_data(
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
                    ComboBox::from_enum::<geometry::Repetition>(default_primitive.repetition)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Repeat objects in the scene with no extra memory
                            consumption. Note that if the repeated objects overlap
                            some strange things can occur."
                        })),
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
                    Vec3::from_vec3(default_primitive.spacing).with_ui_data(
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
                    ComboBox::from_enum::<geometry::BlendType>(default_primitive.blend_type)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
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
                        })),
                );
                input_float(
                    graph,
                    "blend_strength",
                    Float::new(default_primitive.blend_strength).with_ui_data(
                        UIData::default()
                            .with_tooltip("The amount to blend with this primitive's children.")
                            .with_hidden(),
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
                    Float::new(default_primitive.wall_thickness).with_ui_data(
                        UIData::default()
                            .with_tooltip(indoc! {
                                "The thickness of the walls of the shape, if
                                the shape is hollow.",
                            })
                            .with_hidden(),
                    ),
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
                    Vec3::from_vec3(default_primitive.elongation).with_ui_data(
                        UIData::default()
                            .with_tooltip("The elongation of the object along the respective axes.")
                            .with_hidden(),
                    ),
                );
                output_primitive(graph, "out");
            }
            DamascusNodeTemplate::ProceduralTexture => {
                let default_procedural_texture = materials::ProceduralTexture::default();
                input_combo_box(
                    graph,
                    "texture_type",
                    ComboBox::from_enum::<materials::ProceduralTextureType>(
                        default_procedural_texture.texture_type,
                    )
                    .with_ui_data(UIData::default().with_tooltip(indoc! {
                        "TODO",
                    })),
                );
                input_float(
                    graph,
                    "black_point",
                    Float::new(default_procedural_texture.black_point).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "TODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "white_point",
                    Float::new(default_procedural_texture.white_point).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "TODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "lift",
                    Float::new(default_procedural_texture.lift).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "TODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "gamma",
                    Float::new(default_procedural_texture.gamma).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "TODO"
                        }),
                    ),
                );
                output_procedural_texture(graph, "out");
            }
            DamascusNodeTemplate::RayMarcher => {
                let default_ray_marcher = renderers::RayMarcher::default();
                input_scene(graph, "scene", default_ray_marcher.scene);
                input_uint(
                    graph,
                    "paths_per_pixel",
                    UnsignedInteger::new(default_ray_marcher.paths_per_pixel)
                        .with_ui_data(
                            UIData::default()
                                .with_tooltip("The number of paths to march for each pixel."),
                        )
                        .with_range(1..=100),
                );
                input_bool(
                    graph,
                    "roulette",
                    Bool::new(default_ray_marcher.roulette).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Randomly terminate rays with a probability proportional
                            to the remaining strength, or throughput of a ray."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "max_distance",
                    Float::new(default_ray_marcher.max_distance)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "Each ray, once spawned is only allowed to travel
                            this distance before it is culled."
                        }))
                        .with_range(10.0..=10000.),
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
                        .with_range(0..=100),
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
                input_vector3(
                    graph,
                    "seeds",
                    Vec3::from_vec3(default_ray_marcher.seeds).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "The seeds used to generate per-pixel, random seeds.
                            Be sure these are different on each render used for
                            adaptive sampling."
                        }),
                    ),
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
                    "max_light_sampling_bounces",
                    UnsignedInteger::new(default_ray_marcher.max_light_sampling_bounces)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The maximum number of bounces during light sampling.
                            Light sampling will be disabled if this is 0. Light
                            sampling means that each time a surface is hit, the
                            direct illumination from lights in the scene will be
                            computed, which helps to reduce noise very quickly.\nTODO"
                        }))
                        .with_range(0..=50),
                );
                input_bool(
                    graph,
                    "sample_hdri",
                    Bool::new(default_ray_marcher.sample_hdri).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Include the HDRI in the list of lights that can be
                            sampled during light sampling.\nTODO"
                        }),
                    ),
                );
                input_bool(
                    graph,
                    "sample_all_lights",
                    Bool::new(default_ray_marcher.sample_all_lights).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Sample every light in the scene during light sampling,
                            rather than just one random one. This will reduce noise
                            quickly but slow things down.\nTODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "light_sampling_bias",
                    Float::new(default_ray_marcher.light_sampling_bias).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "A fully biased (1) light sampling means that on each
                            light sample the ray will be initialised pointing
                            directly at the light. Reducing this bias means that
                            some rays will be pointed away from the light. This,
                            when combined with multiple 'max light sampling
                            bounces' allows the renderer to find difficult paths,
                            such as volumetric caustics.\nTODO"
                        }),
                    ),
                );
                input_bool(
                    graph,
                    "secondary_sampling",
                    Bool::new(default_ray_marcher.secondary_sampling).with_ui_data(
                        UIData::default().with_tooltip(indoc! {
                            "Sample the artificial lights (those in the 'lights'
                            input) while casting shadow rays for light sampling.\nTODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "hdri_offset_angle",
                    Float::new(default_ray_marcher.hdri_offset_angle)
                        .with_ui_data(UIData::default().with_tooltip(
                            "Rotate the hdri image by this amount around the y-axis.\nTODO",
                        ))
                        .with_range(0.0..=360.),
                );
                input_combo_box(
                    graph,
                    "output_aov",
                    ComboBox::from_enum::<renderers::AOVs>(default_ray_marcher.output_aov)
                        .with_ui_data(UIData::default().with_tooltip(indoc! {
                            "The AOV type to output.\nThe stats AOV has the
                            average number of bounces in the red channel,
                            average number of steps in the green channel,
                            and the distance travelled in the blue channel.
                            Each is displayed as a fraction of the maximums."
                        })),
                );
                input_bool(
                    graph,
                    "latlong",
                    Bool::new(default_ray_marcher.latlong).with_ui_data(
                        UIData::default().with_tooltip(
                            "Output a LatLong, 360 degree field of view image.\nTODO",
                        ),
                    ),
                );
                output_ray_marcher(graph, "out");
            }
            DamascusNodeTemplate::Scene => {
                let default_scene = scene::Scene::default();
                input_camera(graph, "render_camera", default_scene.render_camera);
                input_primitive(graph, "primitives", default_scene.primitives);
                input_light(graph, "lights", default_scene.lights);
                input_material(graph, "atmosphere", default_scene.atmosphere);
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
        // will use to display it to the user.
        Self::Item::iter().collect()
    }
}
