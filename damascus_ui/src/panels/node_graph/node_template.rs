use core::ops::RangeInclusive;
use std::borrow::Cow;

use egui_node_graph::{Graph, InputParamKind, NodeId, NodeTemplateIter, NodeTemplateTrait};

use damascus_core::{geometry, lights, materials, renderers, scene};

use crate::panels::node_graph::{
    data_type::DamascusDataType,
    node_data::DamascusNodeData,
    node_graph_state::DamascusGraphState,
    value_type::{ComboBox, DamascusValueType, Float, Integer, UnsignedInteger, Vec3, Vec4},
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
        let input_int =
            |graph: &mut DamascusGraph, name: &str, default: i32, range: RangeInclusive<i32>| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Integer,
                    DamascusValueType::Integer {
                        value: Integer::new(default, range),
                    },
                    InputParamKind::ConstantOnly,
                    true,
                );
            };
        let input_uint =
            |graph: &mut DamascusGraph, name: &str, default: u32, range: RangeInclusive<u32>| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::UnsignedInteger,
                    DamascusValueType::UnsignedInteger {
                        value: UnsignedInteger::new(default, range),
                    },
                    InputParamKind::ConstantOnly,
                    true,
                );
            };
        let input_float =
            |graph: &mut DamascusGraph, name: &str, default: f32, range: RangeInclusive<f32>| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DamascusDataType::Float,
                    DamascusValueType::Float {
                        value: Float::new(default, range),
                    },
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
                input_matrix4(graph, "axis", glam::Mat4::IDENTITY);
                input_vector3(graph, "translate", Vec3::new(glam::Vec3::ZERO, false));
                input_vector3(graph, "rotate", Vec3::new(glam::Vec3::ZERO, false));
                input_float(graph, "uniform_scale", 1., 0.01..=10.0);
                output_matrix4(graph, "out");
            }
            DamascusNodeTemplate::Camera => {
                let default_camera = geometry::camera::Camera::default();
                input_float(
                    graph,
                    "focal_length",
                    default_camera.focal_length,
                    5.0..=100.,
                );
                input_float(
                    graph,
                    "focal_distance",
                    default_camera.focal_distance,
                    0.1..=10.,
                );
                input_float(graph, "f_stop", default_camera.f_stop, 0.1..=30.);
                input_float(
                    graph,
                    "horizontal_aperture",
                    default_camera.horizontal_aperture,
                    0.1..=50.,
                );
                input_float(graph, "near_plane", default_camera.near_plane, 0.1..=10.);
                input_float(graph, "far_plane", default_camera.far_plane, 11.0..=10000.);
                input_matrix4(graph, "world_matrix", default_camera.world_matrix);
                input_bool(
                    graph,
                    "enable_depth_of_field",
                    default_camera.enable_depth_of_field,
                );
                output_camera(graph, "out");
            }
            DamascusNodeTemplate::Light => {
                let default_light = lights::Light::default();
                input_light(graph, "lights", vec![]);
                input_combo_box(
                    graph,
                    "light_type",
                    ComboBox::new::<lights::Lights>(default_light.light_type),
                );
                input_vector3(
                    graph,
                    "dimensional_data",
                    Vec3::new(default_light.dimensional_data, false),
                );
                input_float(graph, "intensity", default_light.intensity, 0.0..=10.);
                input_uint(graph, "falloff", default_light.falloff, 0..=4);
                input_vector3(graph, "colour", Vec3::new(default_light.colour, true));
                input_float(
                    graph,
                    "shadow_hardness",
                    default_light.shadow_hardness,
                    1.0..=100.,
                );
                input_bool(graph, "soften_shadows", default_light.soften_shadows);
                output_light(graph, "out");
            }
            DamascusNodeTemplate::Material => {
                let default_material = materials::Material::default();
                input_vector3(
                    graph,
                    "diffuse_colour",
                    Vec3::new(default_material.diffuse_colour, true),
                );
                input_float(
                    graph,
                    "specular_probability",
                    default_material.specular_probability,
                    0.0..=1.,
                );
                input_float(
                    graph,
                    "specular_roughness",
                    default_material.specular_roughness,
                    0.0..=1.,
                );
                input_vector3(
                    graph,
                    "specular_colour",
                    Vec3::new(default_material.specular_colour, true),
                );
                input_float(
                    graph,
                    "transmissive_probability",
                    default_material.transmissive_probability,
                    0.0..=1.,
                );
                input_float(
                    graph,
                    "transmissive_roughness",
                    default_material.transmissive_roughness,
                    0.0..=1.,
                );
                input_vector3(
                    graph,
                    "transmissive_colour",
                    Vec3::new(default_material.transmissive_colour, true),
                );
                input_float(
                    graph,
                    "emissive_probability",
                    default_material.emissive_probability,
                    0.0..=1.,
                );
                input_vector3(
                    graph,
                    "emissive_colour",
                    Vec3::new(default_material.emissive_colour, true),
                );
                input_float(
                    graph,
                    "refractive_index",
                    default_material.refractive_index,
                    0.0..=1.,
                );
                input_float(
                    graph,
                    "scattering_coefficient",
                    default_material.scattering_coefficient,
                    0.0..=1.,
                );
                input_vector3(
                    graph,
                    "scattering_colour",
                    Vec3::new(default_material.scattering_colour, true),
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
                    ComboBox::new::<geometry::Shapes>(default_primitive.shape),
                );
                input_matrix4(graph, "world_matrix", glam::Mat4::IDENTITY);
                input_uint(
                    graph,
                    "modifiers",
                    default_primitive.modifiers as u32,
                    0..=100,
                ); // TODO make this a series of bools
                input_float(
                    graph,
                    "blend_strength",
                    default_primitive.blend_strength,
                    0.0..=1.,
                );
                input_vector4(graph, "dimensional_data", Vec4::new(glam::Vec4::X, false)); // TODO make this dynamic based on shape
                output_primitive(graph, "out");
            }
            DamascusNodeTemplate::RayMarcher => {
                let default_ray_marcher = renderers::RayMarcher::default();
                input_scene(graph, "scene", default_ray_marcher.scene);
                input_uint(
                    graph,
                    "paths_per_pixel",
                    default_ray_marcher.paths_per_pixel,
                    1..=100,
                );
                input_bool(graph, "roulette", default_ray_marcher.roulette);
                input_float(
                    graph,
                    "max_distance",
                    default_ray_marcher.max_distance,
                    10.0..=10000.,
                );
                input_uint(
                    graph,
                    "max_ray_steps",
                    default_ray_marcher.max_ray_steps,
                    100..=100000,
                );
                input_uint(
                    graph,
                    "max_bounces",
                    default_ray_marcher.max_bounces,
                    0..=100,
                );
                input_float(
                    graph,
                    "hit_tolerance",
                    default_ray_marcher.hit_tolerance,
                    0.00001..=0.1,
                );
                input_float(
                    graph,
                    "shadow_bias",
                    default_ray_marcher.shadow_bias,
                    1.0..=5.0,
                );
                input_float(
                    graph,
                    "max_brightness",
                    default_ray_marcher.max_brightness,
                    1.0..=1000000.,
                );
                input_vector3(graph, "seeds", Vec3::new(default_ray_marcher.seeds, false));
                input_bool(
                    graph,
                    "enable_depth_of_field",
                    default_ray_marcher.enable_depth_of_field,
                );
                input_bool(
                    graph,
                    "dynamic_level_of_detail",
                    default_ray_marcher.dynamic_level_of_detail,
                );
                input_uint(
                    graph,
                    "max_light_sampling_bounces",
                    default_ray_marcher.max_light_sampling_bounces,
                    0..=50,
                );
                input_bool(graph, "sample_hdri", default_ray_marcher.sample_hdri);
                input_bool(
                    graph,
                    "sample_all_lights",
                    default_ray_marcher.sample_all_lights,
                );
                input_float(
                    graph,
                    "light_sampling_bias",
                    default_ray_marcher.light_sampling_bias,
                    0.0..=1.,
                );
                input_bool(
                    graph,
                    "secondary_sampling",
                    default_ray_marcher.secondary_sampling,
                );
                input_float(
                    graph,
                    "hdri_offset_angle",
                    default_ray_marcher.hdri_offset_angle,
                    0.0..=360.,
                );
                input_combo_box(
                    graph,
                    "output_aov",
                    ComboBox::new::<renderers::AOVs>(default_ray_marcher.output_aov),
                );
                input_bool(graph, "latlong", default_ray_marcher.latlong);
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
