use core::ops::RangeInclusive;
use std::borrow::Cow;

use egui_node_graph::{Graph, InputParamKind, NodeId, NodeTemplateIter, NodeTemplateTrait};
use indoc::indoc;

use damascus_core::{geometry, lights, materials, renderers, scene};

use crate::panels::node_graph::{
    data_type::DamascusDataType,
    node_data::DamascusNodeData,
    node_graph_state::DamascusGraphState,
    value_type::{
        Bool, ComboBox, DamascusValueType, Float, Integer, Mat3, Mat4, RangedInput, UIData,
        UIInput, UnsignedInteger, Vec2, Vec3, Vec4,
    },
    DamascusGraph,
};

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum DamascusNodeTemplate {
    Axis,
    Camera,
    Light,
    Material,
    Primitive,
    RayMarcher,
    Scene,
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
            DamascusNodeTemplate::Light => "light",
            DamascusNodeTemplate::Material => "material",
            DamascusNodeTemplate::Primitive => "primitive",
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
                        .with_ui_data(UIData::default().tooltip("The parent axis.")),
                );
                input_vector3(
                    graph,
                    "translate",
                    Vec3::new(
                        glam::Vec3::ZERO,
                        UIData::default().tooltip("The translation of this axis."),
                        false,
                    ),
                );
                input_vector3(
                    graph,
                    "rotate",
                    Vec3::new(
                        glam::Vec3::ZERO,
                        UIData::default().tooltip("The rotation of this axis."),
                        false,
                    ),
                );
                input_float(
                    graph,
                    "uniform_scale",
                    Float::with_range(
                        1.,
                        UIData::default().tooltip(indoc! {
                            "The uniform scale of this axis.\n
                            We use uniform scale because the signed distance 
                            fields cannot have their individual axes scaled."
                        }),
                        0.01..=10.0,
                    ),
                );
                output_matrix4(graph, "out");
            }
            DamascusNodeTemplate::Camera => {
                let default_camera = geometry::camera::Camera::default();
                input_float(
                    graph,
                    "focal_length",
                    Float::with_range(
                        default_camera.focal_length,
                        UIData::default().tooltip("The focal length of the camera."),
                        5.0..=100.,
                    ),
                );
                input_float(
                    graph,
                    "focal_distance",
                    Float::with_range(
                        default_camera.focal_distance,
                        UIData::default().tooltip("The focal distance of the camera."),
                        0.1..=10.,
                    ),
                );
                input_float(
                    graph,
                    "f_stop",
                    Float::with_range(
                        default_camera.f_stop,
                        UIData::default().tooltip("The f-stop of the camera."),
                        0.1..=30.,
                    ),
                );
                input_float(
                    graph,
                    "horizontal_aperture",
                    Float::with_range(
                        default_camera.horizontal_aperture,
                        UIData::default().tooltip("The horizontal aperture of the camera."),
                        0.1..=50.,
                    ),
                );
                input_float(
                    graph,
                    "near_plane",
                    Float::with_range(
                        default_camera.near_plane,
                        UIData::default().tooltip("The distance to the near plane of the camera."),
                        0.1..=10.,
                    ),
                );
                input_float(
                    graph,
                    "far_plane",
                    Float::with_range(
                        default_camera.far_plane,
                        UIData::default().tooltip("The distance to the far plane of the camera."),
                        11.0..=10000.,
                    ),
                );
                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(default_camera.world_matrix).with_ui_data(
                        UIData::default().tooltip("The world matrix/axis of the camera."),
                    ),
                );
                input_bool(
                    graph,
                    "enable_depth_of_field",
                    Bool::new(default_camera.enable_depth_of_field).with_ui_data(
                        UIData::default()
                            .tooltip("If enabled, this camera will render with depth of field."),
                    ),
                );
                output_camera(graph, "out");
            }
            DamascusNodeTemplate::Light => {
                let default_light = lights::Light::default();
                input_light(graph, "lights", vec![]);
                input_combo_box(
                    graph,
                    "light_type",
                    ComboBox::new::<lights::Lights>(default_light.light_type).with_ui_data(
                        UIData::default().tooltip(indoc! {
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
                    "dimensional_data",
                    Vec3::new(
                        default_light.dimensional_data,
                        UIData::default().tooltip(indoc! {
                            "The data needed by each individual light type.\n
                            \tPoint: The position.\n
                            \tDirectional: The direction vector.\n
                            \tAmbient: Nothing - control brightness via intensity.\n
                            \tAmbient Occlusion: Iterations is the x value.\n\n
                            TODO: make dynamic knobs"
                        }),
                        false,
                    ),
                );
                input_float(
                    graph,
                    "intensity",
                    Float::with_range(
                        default_light.intensity,
                        UIData::default().tooltip("The intensity of the light."),
                        0.0..=10.,
                    ),
                );
                input_uint(
                    graph,
                    "falloff",
                    UnsignedInteger::with_range(
                        default_light.falloff,
                        UIData::default()
                            .tooltip("The exponent of the falloff (point lights only)."),
                        0..=4,
                    ),
                );
                input_vector3(
                    graph,
                    "colour",
                    Vec3::new(
                        default_light.colour,
                        UIData::default().tooltip("The light colour."),
                        true,
                    ),
                );
                input_float(
                    graph,
                    "shadow_hardness",
                    Float::with_range(
                        default_light.shadow_hardness,
                        UIData::default().tooltip("The hardness of softened shadows."),
                        1.0..=100.,
                    ),
                );
                input_bool(
                    graph,
                    "soften_shadows",
                    Bool::new(default_light.soften_shadows).with_ui_data(
                        UIData::default().tooltip(indoc! {
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
                    Vec3::new(
                        default_material.diffuse_colour,
                        UIData::default().tooltip("The diffuse colour of the material."),
                        true,
                    ),
                );
                input_float(
                    graph,
                    "specular_probability",
                    Float::with_range(
                        default_material.specular_probability,
                        UIData::default().tooltip(indoc! {
                            "The probability that light will be specularly reflected
                            when it interacts with this material."
                        }),
                        0.0..=1.,
                    ),
                );
                input_float(
                    graph,
                    "specular_roughness",
                    Float::with_range(
                        default_material.specular_roughness,
                        UIData::default()
                            .tooltip("The roughness of the material when specularly reflected."),
                        0.0..=1.,
                    ),
                );
                input_vector3(
                    graph,
                    "specular_colour",
                    Vec3::new(
                        default_material.specular_colour,
                        UIData::default().tooltip("The specular colour of the material."),
                        true,
                    ),
                );
                input_float(
                    graph,
                    "transmissive_probability",
                    Float::with_range(
                        default_material.transmissive_probability,
                        UIData::default().tooltip(indoc! {
                            "The probability that light will be transmitted through
                            the material (before accounting for Fresnel) when it
                            interacts with this material."
                        }),
                        0.0..=1.,
                    ),
                );
                input_float(
                    graph,
                    "transmissive_roughness",
                    Float::with_range(
                        default_material.transmissive_roughness,
                        UIData::default()
                            .tooltip("The roughness when transmitted through the material."),
                        0.0..=1.,
                    ),
                );
                input_vector3(
                    graph,
                    "transmissive_colour",
                    Vec3::new(
                        default_material.transmissive_colour,
                        UIData::default().tooltip("The transmissive colour of the material."),
                        true,
                    ),
                );
                input_float(
                    graph,
                    "emissive_probability",
                    Float::with_range(
                        default_material.emissive_probability,
                        UIData::default().tooltip(
                            "The probability that light will be emitted from the material.",
                        ),
                        0.0..=1.,
                    ),
                );
                input_vector3(
                    graph,
                    "emissive_colour",
                    Vec3::new(
                        default_material.emissive_colour,
                        UIData::default().tooltip("The emissive colour of the material."),
                        true,
                    ),
                );
                input_float(
                    graph,
                    "refractive_index",
                    Float::with_range(
                        default_material.refractive_index,
                        UIData::default().tooltip("The index of refraction of the material."),
                        0.0..=1.,
                    ),
                );
                input_float(
                    graph,
                    "scattering_coefficient",
                    Float::with_range(
                        default_material.scattering_coefficient,
                        UIData::default().tooltip("The scattering coefficient of the material."),
                        0.0..=1.,
                    ),
                );
                input_vector3(
                    graph,
                    "scattering_colour",
                    Vec3::new(
                        default_material.scattering_colour,
                        UIData::default().tooltip("The scattering colour of the material."),
                        true,
                    ),
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
                    ComboBox::new::<geometry::Shapes>(default_primitive.shape)
                        .with_ui_data(UIData::default().tooltip("The shape of the primitive.")),
                );
                input_matrix4(
                    graph,
                    "world_matrix",
                    Mat4::new(glam::Mat4::IDENTITY).with_ui_data(
                        UIData::default().tooltip("The world matrix/axis of the primitive."),
                    ),
                );
                input_uint(
                    graph,
                    "modifiers",
                    UnsignedInteger::with_range(
                        default_primitive.modifiers as u32,
                        UIData::default(),
                        0..=100,
                    ),
                ); // TODO make this a series of bools
                input_float(
                    graph,
                    "blend_strength",
                    Float::with_range(
                        default_primitive.blend_strength,
                        UIData::default()
                            .tooltip("The amount to blend with this primitive's children."),
                        0.0..=1.,
                    ),
                );
                input_vector4(
                    graph,
                    "dimensional_data",
                    Vec4::new(
                        glam::Vec4::X,
                        UIData::default().tooltip(indoc! {
                            "The data required to dimension each object\n
                            TODO make the labels here dynamic."
                        }),
                        false,
                    ),
                ); // TODO make this dynamic based on shape
                output_primitive(graph, "out");
            }
            DamascusNodeTemplate::RayMarcher => {
                let default_ray_marcher = renderers::RayMarcher::default();
                input_scene(graph, "scene", default_ray_marcher.scene);
                input_uint(
                    graph,
                    "paths_per_pixel",
                    UnsignedInteger::with_range(
                        default_ray_marcher.paths_per_pixel,
                        UIData::default().tooltip("The number of paths to march for each pixel."),
                        1..=100,
                    ),
                );
                input_bool(
                    graph,
                    "roulette",
                    Bool::new(default_ray_marcher.roulette).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "Randomly terminate rays with a probability proportional
                            to the remaining strength, or throughput of a ray."
                        }),
                    ),
                );
                input_float(
                    graph,
                    "max_distance",
                    Float::with_range(
                        default_ray_marcher.max_distance,
                        UIData::default().tooltip(indoc! {
                            "Each ray, once spawned is only allowed to travel
                            this distance before it is culled."
                        }),
                        10.0..=10000.,
                    ),
                );
                input_uint(
                    graph,
                    "max_ray_steps",
                    UnsignedInteger::with_range(
                        default_ray_marcher.max_ray_steps,
                        UIData::default().tooltip(indoc! {
                            "Limits the number of times the rays can intersect
                            an object per subpixel."
                        }),
                        100..=100000,
                    ),
                );
                input_uint(
                    graph,
                    "max_bounces",
                    UnsignedInteger::with_range(
                        default_ray_marcher.max_bounces,
                        UIData::default().tooltip(indoc! {
                            "Limits the number of times the rays can intersect
                            an object per subpixel."
                        }),
                        0..=100,
                    ),
                );
                input_float(
                    graph,
                    "hit_tolerance",
                    Float::with_range(
                        default_ray_marcher.hit_tolerance,
                        UIData::default().tooltip(indoc! {
                            "The ray will be considered to have hit an object
                            when it is within this distance of its surface."
                        }),
                        0.00001..=0.1,
                    ),
                );
                input_float(
                    graph,
                    "shadow_bias",
                    Float::with_range(
                        default_ray_marcher.shadow_bias,
                        UIData::default().tooltip(indoc! {
                            "After intersecting an object the ray is offset from
                            the surface before continuing. Multiply that offset
                            distance by this factor."
                        }),
                        1.0..=5.0,
                    ),
                );
                input_float(
                    graph,
                    "max_brightness",
                    Float::with_range(
                        default_ray_marcher.max_brightness,
                        UIData::default().tooltip(indoc! {
                            "The maximum brightness of a pixel. This protects
                            against overflowing to infinity."
                        }),
                        1.0..=1000000.,
                    ),
                );
                input_vector3(
                    graph,
                    "seeds",
                    Vec3::new(
                        default_ray_marcher.seeds,
                        UIData::default().tooltip(indoc! {
                            "The seeds used to generate per-pixel, random seeds.
                            Be sure these are different on each render used for
                            adaptive sampling."
                        }),
                        false,
                    ),
                );
                input_bool(
                    graph,
                    "dynamic_level_of_detail",
                    Bool::new(default_ray_marcher.dynamic_level_of_detail).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "Increase the hit tolerance the farther the ray
                            travels without hitting a surface. This has performance
                            and antialiasing benefits."
                        }),
                    ),
                );
                input_uint(
                    graph,
                    "max_light_sampling_bounces",
                    UnsignedInteger::with_range(
                        default_ray_marcher.max_light_sampling_bounces,
                        UIData::default().tooltip(indoc! {
                            "The maximum number of bounces during light sampling.
                            Light sampling will be disabled if this is 0. Light
                            sampling means that each time a surface is hit, the
                            direct illumination from lights in the scene will be
                            computed, which helps to reduce noise very quickly.\nTODO"
                        }),
                        0..=50,
                    ),
                );
                input_bool(
                    graph,
                    "sample_hdri",
                    Bool::new(default_ray_marcher.sample_hdri).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "Include the HDRI in the list of lights that can be
                            sampled during light sampling.\nTODO"
                        }),
                    ),
                );
                input_bool(
                    graph,
                    "sample_all_lights",
                    Bool::new(default_ray_marcher.sample_all_lights).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "Sample every light in the scene during light sampling,
                            rather than just one random one. This will reduce noise
                            quickly but slow things down.\nTODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "light_sampling_bias",
                    Float::with_range(
                        default_ray_marcher.light_sampling_bias,
                        UIData::default().tooltip(indoc! {
                            "A fully biased (1) light sampling means that on each
                            light sample the ray will be initialised pointing
                            directly at the light. Reducing this bias means that
                            some rays will be pointed away from the light. This,
                            when combined with multiple 'max light sampling
                            bounces' allows the renderer to find difficult paths,
                            such as volumetric caustics.\nTODO"
                        }),
                        0.0..=1.,
                    ),
                );
                input_bool(
                    graph,
                    "secondary_sampling",
                    Bool::new(default_ray_marcher.secondary_sampling).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "Sample the artificial lights (those in the 'lights'
                            input) while casting shadow rays for light sampling.\nTODO"
                        }),
                    ),
                );
                input_float(
                    graph,
                    "hdri_offset_angle",
                    Float::with_range(
                        default_ray_marcher.hdri_offset_angle,
                        UIData::default().tooltip(
                            "Rotate the hdri image by this amount around the y-axis.\nTODO",
                        ),
                        0.0..=360.,
                    ),
                );
                input_combo_box(
                    graph,
                    "output_aov",
                    ComboBox::new::<renderers::AOVs>(default_ray_marcher.output_aov).with_ui_data(
                        UIData::default().tooltip(indoc! {
                            "The AOV type to output.\nThe stats AOV has the
                            average number of bounces in the red channel,
                            average number of steps in the green channel,
                            and the distance travelled in the blue channel.
                            Each is displayed as a fraction of the maximums."
                        }),
                    ),
                );
                input_bool(
                    graph,
                    "latlong",
                    Bool::new(default_ray_marcher.latlong).with_ui_data(
                        UIData::default()
                            .tooltip("Output a LatLong, 360 degree field of view image.\nTODO"),
                    ),
                );
                output_ray_marcher(graph, "out");
            }
            DamascusNodeTemplate::Scene => {
                let default_scene = scene::Scene::default();
                input_camera(graph, "render_camera", default_scene.render_camera);
                input_primitive(graph, "primitives", default_scene.primitives);
                input_light(graph, "lights", default_scene.lights);
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
            DamascusNodeTemplate::Light,
            DamascusNodeTemplate::Material,
            DamascusNodeTemplate::Primitive,
            DamascusNodeTemplate::RayMarcher,
            DamascusNodeTemplate::Scene,
        ]
    }
}
