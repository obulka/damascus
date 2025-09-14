// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::{HashMap, HashSet};

use glam::{Mat4, Vec3};
use quick_cache::{
    unsync::{Cache, DefaultLifecycle},
    DefaultHashBuilder, OptionsBuilder, UnitWeighter,
};

use crate::{
    camera::Camera,
    geometry::primitives::Primitive,
    lights::Light,
    materials::Material,
    render_passes::{ray_marcher::RayMarcherRenderData, RenderPasses},
    scene::Scene,
    textures::{Grade, Texture},
};

pub mod edges;
pub mod inputs;
pub mod nodes;
pub mod outputs;

use edges::Edges;
use inputs::{input::Input, input_data::InputData, InputId, Inputs};
use nodes::{node::Node, node_data::NodeData, NodeId, Nodes};
use outputs::{output::Output, output_data::OutputData, OutputId, Outputs};

pub type OutputCache = Cache<OutputId, InputData>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct NodeGraph {
    pub nodes: Nodes,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub edges: Edges,
    #[serde(skip)]
    pub cache: OutputCache,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::default(),
            inputs: Inputs::default(),
            outputs: Outputs::default(),
            edges: Edges::default(),
            cache: OutputCache::with_options(
                OptionsBuilder::new()
                    .estimated_items_capacity(10000)
                    .weight_capacity(10000)
                    .build()
                    .unwrap(),
                UnitWeighter,
                DefaultHashBuilder::default(),
                DefaultLifecycle::default(),
            ),
        }
    }

    pub fn from_nodes(&self, node_ids: &HashSet<NodeId>) -> Self {
        let mut new_graph: Self = self.clone();

        for node_id in self.iter_nodes() {
            if node_ids.contains(&node_id) {
                continue;
            }
            let (_node, disconnected_edges) = new_graph.remove_node(node_id);
            for (input_id, _output_id) in disconnected_edges.iter() {
                new_graph.edges.remove_input(*input_id);
            }
        }

        new_graph
    }

    pub fn valid_edge(&self, input_id: InputId, output_id: OutputId) -> bool {
        let input_node_id: NodeId = self[input_id].node_id;
        let output_node_id: NodeId = self[output_id].node_id;

        input_node_id != output_node_id
            && self
                .ancestor_node_ids(output_node_id)
                .get(&input_node_id)
                .is_none()
    }

    pub fn add_node(&mut self, data: NodeData) -> NodeId {
        let node_id: NodeId = self.nodes.insert_with_key(|node_id| Node {
            id: node_id,
            input_ids: vec![],
            output_ids: vec![],
            data: data,
        });
        match data {
            NodeData::Axis => {
                self.add_input(node_id, "axis", InputData::Mat4(Mat4::IDENTITY));
                self.add_input(node_id, "translate", InputData::Vec3(Vec3::ZERO));
                self.add_input(node_id, "rotate", InputData::Vec3(Vec3::ZERO));
                self.add_input(node_id, "uniform_scale", InputData::Float(1.));
                self.add_output(node_id, OutputData::Mat4);
            }
            NodeData::Camera => {
                let default_camera = Camera::default();
                self.add_input(
                    node_id,
                    "focal_length",
                    InputData::Float(default_camera.focal_length),
                );
                self.add_input(
                    node_id,
                    "focal_distance",
                    InputData::Float(default_camera.focal_distance),
                );
                self.add_input(node_id, "f_stop", InputData::Float(default_camera.f_stop));
                self.add_input(
                    node_id,
                    "horizontal_aperture",
                    InputData::Float(default_camera.horizontal_aperture),
                );
                self.add_input(
                    node_id,
                    "near_plane",
                    InputData::Float(default_camera.near_plane),
                );
                self.add_input(
                    node_id,
                    "far_plane",
                    InputData::Float(default_camera.far_plane),
                );
                self.add_input(
                    node_id,
                    "sensor_resolution",
                    InputData::UVec2(default_camera.sensor_resolution),
                );
                self.add_input(
                    node_id,
                    "world_matrix",
                    InputData::Mat4(default_camera.camera_to_world),
                );
                self.add_input(
                    node_id,
                    "enable_depth_of_field",
                    InputData::Bool(default_camera.enable_depth_of_field),
                );
                self.add_input(node_id, "latlong", InputData::Bool(default_camera.latlong));
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Light => {
                let default_light = Light::default();
                self.add_input(node_id, "scene", InputData::Scene(Scene::default()));
                self.add_input(node_id, "world_matrix", InputData::Mat4(Mat4::IDENTITY));
                self.add_input(
                    node_id,
                    "light_type",
                    InputData::Enum(default_light.light_type.into()),
                );
                self.add_input(node_id, "direction", InputData::Vec3(Vec3::NEG_Y));
                self.add_input(node_id, "position", InputData::Vec3(Vec3::Y));
                self.add_input(
                    node_id,
                    "iterations",
                    InputData::UInt(default_light.dimensional_data.x as u32),
                );
                self.add_input(
                    node_id,
                    "intensity",
                    InputData::Float(default_light.intensity),
                );
                self.add_input(node_id, "falloff", InputData::UInt(default_light.falloff));
                self.add_input(node_id, "colour", InputData::Vec3(default_light.colour));
                self.add_input(
                    node_id,
                    "shadow_hardness",
                    InputData::Float(default_light.shadow_hardness),
                );
                self.add_input(
                    node_id,
                    "soften_shadows",
                    InputData::Bool(default_light.soften_shadows),
                );
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Grade => {
                let default_grade = Grade::default();
                self.add_input(
                    node_id,
                    "texture",
                    InputData::RenderPass(RenderPasses::Black),
                );
                self.add_input(
                    node_id,
                    "black_point",
                    InputData::Float(default_grade.black_point),
                );
                self.add_input(
                    node_id,
                    "white_point",
                    InputData::Float(default_grade.white_point),
                );
                self.add_input(node_id, "lift", InputData::Float(default_grade.lift));
                self.add_input(node_id, "gain", InputData::Float(default_grade.gain));
                self.add_input(node_id, "gamma", InputData::Float(default_grade.gamma));
                self.add_input(node_id, "invert", InputData::Bool(default_grade.invert));
                self.add_output(node_id, OutputData::RenderPass);
            }
            NodeData::Material => {
                let default_material = Material::default();
                self.add_input(
                    node_id,
                    "diffuse_colour",
                    InputData::Vec3(default_material.diffuse_colour),
                );
                self.add_input(
                    node_id,
                    "diffuse_colour_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "specular_probability",
                    InputData::Float(default_material.specular_probability),
                );
                self.add_input(
                    node_id,
                    "specular_probability_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "specular_roughness",
                    InputData::Float(default_material.specular_roughness),
                );
                self.add_input(
                    node_id,
                    "specular_roughness_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "specular_colour",
                    InputData::Vec3(default_material.specular_colour),
                );
                self.add_input(
                    node_id,
                    "specular_colour_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "transmissive_probability",
                    InputData::Float(default_material.transmissive_probability),
                );
                self.add_input(
                    node_id,
                    "transmissive_probability_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "transmissive_roughness",
                    InputData::Float(default_material.transmissive_roughness),
                );
                self.add_input(
                    node_id,
                    "transmissive_roughness_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "extinction_coefficient",
                    InputData::Float(default_material.extinction_coefficient),
                );
                self.add_input(
                    node_id,
                    "transmissive_colour",
                    InputData::Vec3(default_material.transmissive_colour),
                );
                self.add_input(
                    node_id,
                    "transmissive_colour_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "emissive_intensity",
                    InputData::Float(default_material.emissive_intensity),
                );
                self.add_input(
                    node_id,
                    "emissive_colour",
                    InputData::Vec3(default_material.emissive_colour),
                );
                self.add_input(
                    node_id,
                    "emissive_colour_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "refractive_index",
                    InputData::Float(default_material.refractive_index),
                );
                self.add_input(
                    node_id,
                    "refractive_index_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_input(
                    node_id,
                    "scattering_coefficient",
                    InputData::Float(default_material.scattering_coefficient),
                );
                self.add_input(
                    node_id,
                    "scattering_colour",
                    InputData::Vec3(default_material.scattering_colour),
                );
                self.add_input(
                    node_id,
                    "scattering_colour_texture",
                    InputData::RenderPass(RenderPasses::White),
                );
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Primitive => {
                let default_primitive = Primitive::default();
                self.add_input(node_id, "siblings", InputData::Scene(Scene::default()));
                self.add_input(node_id, "children", InputData::Scene(Scene::default()));
                self.add_input(node_id, "material", InputData::Scene(Scene::default()));
                self.add_input(
                    node_id,
                    "shape",
                    InputData::Enum(default_primitive.shape.into()),
                );

                // Sphere dimensions
                self.add_input(node_id, "radius", InputData::Float(0.5));

                // Ellipsoid dimensions
                self.add_input(node_id, "radii", InputData::Vec3(Vec3::splat(0.5)));

                // Cut Sphere dimensions
                self.add_input(node_id, "height", InputData::Float(0.25));

                // Death Star dimensions
                self.add_input(node_id, "hollow_radius", InputData::Float(0.5));
                self.add_input(node_id, "hollow_height", InputData::Float(0.75));

                // Solid Angle Dimensions
                self.add_input(node_id, "solid_angle", InputData::Float(30.));

                // Rectangular Prism Dimensions
                self.add_input(node_id, "width", InputData::Float(0.5));
                self.add_input(node_id, "depth", InputData::Float(0.75));

                // Hollow Sphere dimensions
                self.add_input(node_id, "thickness", InputData::Float(0.05));

                // Rhombus Dimensions
                self.add_input(node_id, "corner_radius", InputData::Float(0.05));

                // Triangular Prism Dimensions
                self.add_input(node_id, "base", InputData::Float(0.5));

                // Plane Dimensions
                self.add_input(node_id, "normal", InputData::Vec3(Vec3::Z));

                // Capsule Dimensions
                self.add_input(node_id, "negative_height", InputData::Float(0.25));
                self.add_input(node_id, "positive_height", InputData::Float(0.25));

                // Cone Dimensions
                self.add_input(node_id, "angle", InputData::Float(30.));

                // Capped Cone Dimensions
                self.add_input(node_id, "lower_radius", InputData::Float(0.25));
                self.add_input(node_id, "upper_radius", InputData::Float(0.125));

                // Torus Dimensions
                self.add_input(node_id, "ring_radius", InputData::Float(0.3));
                self.add_input(node_id, "tube_radius", InputData::Float(0.2));

                // Capped Torus Dimensions
                self.add_input(node_id, "cap_angle", InputData::Float(30.));

                // Octahedron Dimensions
                self.add_input(node_id, "radial_extent", InputData::Float(0.5));

                // Mandelbulb Dimensions
                self.add_input(node_id, "power", InputData::Float(8.));
                self.add_input(node_id, "iterations", InputData::UInt(10));
                self.add_input(node_id, "max_square_radius", InputData::Float(4.));

                // Mandelbox Dimensions
                self.add_input(node_id, "scale", InputData::Float(-1.75));
                self.add_input(node_id, "min_square_radius", InputData::Float(0.001));
                self.add_input(node_id, "folding_limit", InputData::Float(0.8));

                // Remaining inputs
                self.add_input(
                    node_id,
                    "world_matrix",
                    InputData::Mat4(default_primitive.local_to_world),
                );
                self.add_input(
                    node_id,
                    "edge_radius",
                    InputData::Float(default_primitive.edge_radius),
                );
                self.add_input(
                    node_id,
                    "repetition",
                    InputData::Enum(default_primitive.repetition.into()),
                );
                self.add_input(
                    node_id,
                    "negative_repetitions",
                    InputData::UVec3(default_primitive.negative_repetitions),
                );
                self.add_input(
                    node_id,
                    "positive_repetitions",
                    InputData::UVec3(default_primitive.positive_repetitions),
                );
                self.add_input(
                    node_id,
                    "spacing",
                    InputData::Vec3(default_primitive.spacing),
                );
                self.add_input(
                    node_id,
                    "bounding_volume",
                    InputData::Bool(default_primitive.bounding_volume),
                );
                self.add_input(
                    node_id,
                    "blend_type",
                    InputData::Enum(default_primitive.blend_type.into()),
                );
                self.add_input(
                    node_id,
                    "blend_strength",
                    InputData::Float(default_primitive.blend_strength),
                );
                self.add_input(
                    node_id,
                    "mirror",
                    InputData::BVec3(default_primitive.mirror),
                );
                self.add_input(node_id, "hollow", InputData::Bool(default_primitive.hollow));
                self.add_input(
                    node_id,
                    "wall_thickness",
                    InputData::Float(default_primitive.wall_thickness),
                );
                self.add_input(
                    node_id,
                    "elongate",
                    InputData::Bool(default_primitive.elongate),
                );
                self.add_input(
                    node_id,
                    "elongation",
                    InputData::Vec3(default_primitive.elongation),
                );
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::RayMarcher => {
                let default_ray_marcher = RayMarcherRenderData::default();
                self.add_input(
                    node_id,
                    "scene",
                    InputData::Scene(default_ray_marcher.scene),
                );
                self.add_input(
                    node_id,
                    "max_ray_steps",
                    InputData::UInt(default_ray_marcher.max_ray_steps),
                );
                self.add_input(
                    node_id,
                    "max_bounces",
                    InputData::UInt(default_ray_marcher.max_bounces),
                );
                self.add_input(
                    node_id,
                    "hit_tolerance",
                    InputData::Float(default_ray_marcher.hit_tolerance),
                );
                self.add_input(
                    node_id,
                    "shadow_bias",
                    InputData::Float(default_ray_marcher.shadow_bias),
                );
                self.add_input(
                    node_id,
                    "max_brightness",
                    InputData::Float(default_ray_marcher.max_brightness),
                );
                self.add_input(node_id, "seed", InputData::UInt(default_ray_marcher.seed));
                self.add_input(
                    node_id,
                    "dynamic_level_of_detail",
                    InputData::Bool(default_ray_marcher.dynamic_level_of_detail),
                );
                self.add_input(
                    node_id,
                    "equiangular_samples",
                    InputData::UInt(default_ray_marcher.equiangular_samples),
                );
                self.add_input(
                    node_id,
                    "light_sampling",
                    InputData::Bool(default_ray_marcher.light_sampling),
                );
                self.add_input(
                    node_id,
                    "max_light_sampling_bounces",
                    InputData::UInt(default_ray_marcher.max_light_sampling_bounces),
                );
                self.add_input(
                    node_id,
                    "sample_atmosphere",
                    InputData::Bool(default_ray_marcher.sample_atmosphere),
                );
                self.add_input(
                    node_id,
                    "light_sampling_bias",
                    InputData::Float(default_ray_marcher.light_sampling_bias),
                );
                self.add_input(
                    node_id,
                    "secondary_sampling",
                    InputData::Bool(default_ray_marcher.secondary_sampling),
                );
                self.add_input(
                    node_id,
                    "output_aov",
                    InputData::Enum(default_ray_marcher.output_aov.into()),
                );
                self.add_output(node_id, OutputData::RenderPass);
            }
            NodeData::Scene => {
                self.add_input(node_id, "scene1", InputData::Scene(Scene::default()));
                self.add_input(node_id, "scene2", InputData::Scene(Scene::default()));
                self.add_output(node_id, OutputData::Scene);
            }
            NodeData::Texture => {
                let default_texture = Texture::default();
                self.add_input(
                    node_id,
                    "filepath",
                    InputData::Filepath(default_texture.filepath),
                );
                self.add_output(node_id, OutputData::RenderPass);
            }
        }

        node_id
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> (Node, Vec<(InputId, OutputId)>) {
        let mut disconnected_edges = vec![];

        disconnected_edges.extend(
            self.edges
                .disconnect_inputs(self[node_id].input_ids.clone()),
        );
        disconnected_edges.extend(
            self.edges
                .disconnect_outputs(self[node_id].output_ids.clone()),
        );

        for input in self[node_id].input_ids.clone().iter() {
            self.inputs.remove(*input);
        }
        for output in self[node_id].output_ids.clone().iter() {
            self.outputs.remove(*output);
        }
        let removed_node = self.nodes.remove(node_id).expect("Node must exist.");

        (removed_node, disconnected_edges)
    }

    pub fn add_input(&mut self, node_id: NodeId, name: &str, data: InputData) -> InputId {
        let input_id = self
            .inputs
            .insert_with_key(|input_id| Input::new(input_id, node_id, name.to_string(), data));
        self[node_id].input_ids.push(input_id);
        input_id
    }

    pub fn remove_input(&mut self, input_id: InputId) {
        let node_id = self[input_id].node_id;
        self[node_id].input_ids.retain(|id| *id != input_id);
        self.inputs.remove(input_id);
        self.edges.remove_input(input_id);
    }

    pub fn add_output(&mut self, node_id: NodeId, data: OutputData) -> OutputId {
        let output_id = self
            .outputs
            .insert_with_key(|output_id| Output::new(output_id, node_id, data));
        self[node_id].output_ids.push(output_id);
        output_id
    }

    pub fn remove_output(&mut self, output_id: OutputId) {
        let node_id = self[output_id].node_id;
        self[node_id].output_ids.retain(|id| *id != output_id);
        self.outputs.remove(output_id);
        self.edges.remove_output(output_id);
    }

    pub fn try_get_parent(&self, input_id: InputId) -> Option<&OutputId> {
        self.edges.parent(input_id)
    }

    pub fn try_get_children(&self, output_id: OutputId) -> Option<&HashSet<InputId>> {
        self.edges.children(output_id)
    }

    pub fn try_get_input(&self, input_id: InputId) -> Option<&Input> {
        self.inputs.get(input_id)
    }

    pub fn get_input(&self, input_id: InputId) -> &Input {
        &self[input_id]
    }

    pub fn try_get_output(&self, output_id: OutputId) -> Option<&Output> {
        self.outputs.get(output_id)
    }

    pub fn get_output(&self, output_id: OutputId) -> &Output {
        &self[output_id]
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys()
    }

    pub fn descendant_output_ids(&self, node_id: NodeId) -> HashSet<OutputId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut output_ids = HashSet::<OutputId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            output_ids.extend(self[search_node_id].output_ids.iter().map(|output_id| {
                if let Some(input_ids) = self.edges.children(*output_id) {
                    nodes_to_search
                        .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                }
                output_id
            }));
        }
        output_ids
    }

    pub fn descendant_node_ids(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut descendant_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self[search_node_id]
                .output_ids
                .iter()
                .for_each(|output_id| {
                    if let Some(input_ids) = self.edges.children(*output_id) {
                        nodes_to_search
                            .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                        descendant_ids
                            .extend(input_ids.iter().map(|input_id| self[*input_id].node_id));
                    }
                });
        }
        descendant_ids
    }

    pub fn ancestor_node_ids(&self, node_id: NodeId) -> HashSet<NodeId> {
        let mut nodes_to_search: Vec<NodeId> = vec![node_id];
        let mut ancestor_ids = HashSet::<NodeId>::new();
        while let Some(search_node_id) = nodes_to_search.pop() {
            self[search_node_id].input_ids.iter().for_each(|input_id| {
                if let Some(parent_output_id) = self.edges.parent(*input_id) {
                    nodes_to_search.push(self[*parent_output_id].node_id);
                    ancestor_ids.insert(self[*parent_output_id].node_id);
                }
            });
        }
        ancestor_ids
    }

    pub fn merge(&mut self, other: &mut Self) -> HashMap<NodeId, NodeId> {
        let mut other_to_new_node_ids = HashMap::<NodeId, NodeId>::new();
        let mut edges_to_recreate = HashMap::<OutputId, HashSet<InputId>>::new();
        let mut other_to_new_outputs = HashMap::<OutputId, OutputId>::new();
        for node_id in self.iter_nodes().collect::<HashSet<_>>().into_iter() {
            if let Some(mut other_node) = other.nodes.remove(node_id) {
                // Move the node to this node graph and update its id
                let new_node_id: NodeId = self.nodes.insert_with_key(|new_node_id| {
                    other_node.id = new_node_id;
                    other_node
                });

                // Update the nodes inputs with new ids, and the new node's id
                let mut new_inputs: Vec<InputId> = self[new_node_id].input_ids.clone();
                for input_id in new_inputs.iter_mut() {
                    if let Some(mut input) = other.inputs.remove(*input_id) {
                        input.node_id = new_node_id;
                        let new_id = self.inputs.insert_with_key(|new_id| {
                            input.id = new_id;
                            input
                        });
                        if let Some(output_id) = other.edges.parent(*input_id) {
                            // Maintain a list of edges to duplicate
                            if let Some(inputs) = edges_to_recreate.get_mut(output_id) {
                                inputs.insert(new_id);
                            } else {
                                let mut inputs = HashSet::<InputId>::new();
                                inputs.insert(new_id);
                                edges_to_recreate.insert(*output_id, inputs);
                            }
                        }
                        *input_id = new_id;
                    }
                }

                // Update the outputs with new ids, and the new node's id
                let mut new_outputs: Vec<OutputId> = self[new_node_id].output_ids.clone();
                for output_id in new_outputs.iter_mut() {
                    if let Some(mut output) = other.outputs.remove(*output_id) {
                        output.node_id = new_node_id;
                        let new_id = self.outputs.insert_with_key(|new_id| {
                            output.id = new_id;
                            output
                        });
                        // Maintain a LUT of the original to new ids
                        other_to_new_outputs.insert(*output_id, new_id);
                        *output_id = new_id;
                    }
                }

                self[new_node_id].input_ids = new_inputs;
                self[new_node_id].output_ids = new_outputs;
                other_to_new_node_ids.insert(node_id, new_node_id);
            }
        }

        // Form equivalent edges
        for (other_output_id, new_input_ids) in edges_to_recreate.iter() {
            if let Some(new_output_id) = other_to_new_outputs.get(other_output_id) {
                for new_input_id in new_input_ids.iter() {
                    self.edges.insert(*new_input_id, *new_output_id);
                }
            }
        }

        other_to_new_node_ids
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_index_traits {
    ($id_type:ty, $output_type:ty, $arena:ident) => {
        impl std::ops::Index<$id_type> for NodeGraph {
            type Output = $output_type;

            fn index(&self, index: $id_type) -> &Self::Output {
                self.$arena.get(index).unwrap_or_else(|| {
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
                })
            }
        }

        impl std::ops::IndexMut<$id_type> for NodeGraph {
            fn index_mut(&mut self, index: $id_type) -> &mut Self::Output {
                self.$arena.get_mut(index).unwrap_or_else(|| {
                    panic!(
                        "{} index error for {}[{:?}]",
                        stringify!($id_type),
                        stringify!($arena),
                        index
                    )
                })
            }
        }
    };
}

impl_index_traits!(NodeId, Node, nodes);
impl_index_traits!(InputId, Input, inputs);
impl_index_traits!(OutputId, Output, outputs);

#[cfg(test)]
mod tests {
    use strum::{EnumCount, IntoEnumIterator};

    use super::*;

    #[test]
    fn test_node_creation() {
        let mut node_graph = NodeGraph::new();

        for node_data in NodeData::iter() {
            node_graph.add_node(node_data);
        }

        assert_eq!(node_graph.nodes.len(), NodeData::COUNT);
        assert_eq!(node_graph.edges.len(), 0);
    }
}
