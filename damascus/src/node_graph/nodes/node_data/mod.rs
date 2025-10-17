// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, iter, str::FromStr};

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    Enumerator,
    node_graph::{
        NodeGraph,
        inputs::{
            InputId,
            input_data::{InputData, NodeInputData},
        },
        nodes::NodeId,
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene_graph::{SceneGraph, SceneGraphId},
};

use super::{NodeErrors, NodeResult};

mod axis;
mod camera;
mod grade;
mod light;
mod material;
mod primitive;
mod ray_marcher;
mod scene;
mod texture;

pub use axis::{AxisInputData, AxisNode, AxisOutputData};
pub use camera::{CameraInputData, CameraNode, CameraOutputData};
pub use grade::{GradeInputData, GradeNode, GradeOutputData};
pub use light::{LightInputData, LightNode, LightOutputData};
pub use material::{MaterialInputData, MaterialNode, MaterialOutputData};
pub use primitive::{PrimitiveInputData, PrimitiveNode, PrimitiveOutputData};
pub use ray_marcher::{RayMarcherInputData, RayMarcherNode, RayMarcherOutputData};
pub use scene::{SceneInputData, SceneNode, SceneOutputData};
pub use texture::{TextureInputData, TextureNode, TextureOutputData};

pub trait EvaluableNode {
    type Inputs: NodeInputData;
    type Outputs: NodeOutputData;

    fn add_to_node_graph(node_graph: &mut NodeGraph, node_id: NodeId) {
        Self::Inputs::add_to_node(node_graph, node_id);
        Self::Outputs::add_to_node(node_graph, node_id);
    }

    /// An iterable that visits all inputs which dynamically spawn other inputs.
    fn dynamic_inputs() -> impl Iterator<Item = Self::Inputs> {
        iter::empty()
    }

    fn dynamic_input_connected(node_graph: &mut NodeGraph, input_id: InputId) {
        for input in Self::dynamic_inputs() {
            let input_name: String = input.name();
            if let Some(input_number_as_str) = node_graph[input_id].name.strip_prefix(&input_name) {
                if input_number_as_str.is_empty() {
                    // The dynamic input by this name was connected
                    let node_id: NodeId = node_graph[input_id].node_id;
                    node_graph.insert_input(
                        node_id,
                        &format!("{input_name}1"),
                        input.default_data(),
                        node_graph.input_index(node_id, input_id) + 1,
                    );
                } else if let Ok(input_number) = input_number_as_str.parse::<usize>() {
                    // Assume the highest number was connected because disconnected
                    // inputs inbetween will be collapsed
                    let node_id: NodeId = node_graph[input_id].node_id;
                    let next_input_number: usize = input_number + 1;
                    node_graph.insert_input(
                        node_id,
                        &format!("{input_name}{next_input_number}"),
                        input.default_data(),
                        node_graph.input_index(node_id, input_id) + 1,
                    );
                }
            }
        }
    }

    fn dynamic_input_disconnected(node_graph: &mut NodeGraph, input_id: InputId) {
        for input in Self::dynamic_inputs() {
            let input_name: String = input.name();
            if let Some(input_number_as_str) = node_graph[input_id].name.strip_prefix(&input_name)
                && (input_number_as_str.is_empty() || input_number_as_str.parse::<usize>().is_ok())
            {
                node_graph.remove_input(input_id);

                let node_id: NodeId = node_graph[input_id].node_id;
                for next_input_index in
                    node_graph.input_index(node_id, input_id)..node_graph[node_id].input_ids.len()
                {
                    let next_input_id: InputId = node_graph[node_id].input_ids[next_input_index];
                    if let Some(next_input_number_as_str) =
                        node_graph[next_input_id].name.strip_prefix(&input_name)
                        && let Ok(next_input_number) = next_input_number_as_str.parse::<usize>()
                    {
                        if next_input_number > 1 {
                            let new_input_number: usize = next_input_number - 1;
                            node_graph[next_input_id].name =
                                format!("{input_name}{new_input_number}");
                        } else {
                            node_graph[next_input_id].name = input.name();
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn add_dynamic_children_to_scene_graph(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        parent_id: SceneGraphId,
        child_input: Self::Inputs,
    ) {
        let child_input_base_name: String = child_input.name();
        let mut child_input_name: String = child_input_base_name.clone();
        let mut input_number: usize = 0;
        while let Some(child_input_data) = data_map.remove(&child_input_name) {
            if let Ok(child_id) = child_input_data.try_to_scene_graph_id() {
                if child_id == SceneGraphId::None {
                    return;
                }
                scene_graph.add_child(parent_id, child_id);
            } else {
                return;
            }

            input_number += 1;
            child_input_name = format!("{}{}", child_input_base_name, input_number);
        }
    }

    fn output_is_compatible_with_named_input(output: &OutputData, input_name: &str) -> bool {
        for input in Self::dynamic_inputs() {
            if let Some(input_number_as_str) = input_name.strip_prefix(&input.name())
                && (input_number_as_str.is_empty() || input_number_as_str.parse::<usize>().is_ok())
            {
                return Self::output_is_compatible_with_input(output, &input);
            }
        }

        match Self::Inputs::from_str(input_name) {
            Ok(input_variant) => Self::output_is_compatible_with_input(output, &input_variant),
            _ => false,
        }
    }

    fn output_is_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
        match input.default_data() {
            InputData::Mat4(..) => *output == OutputData::Mat4,
            InputData::RenderPass(..) => *output == OutputData::RenderPass,
            InputData::SceneGraphId(..) => match *output {
                OutputData::SceneGraphId(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn evaluate(
        _scene_graph: &mut SceneGraph,
        _data_map: &mut HashMap<String, InputData>,
        _output: Self::Outputs,
    ) -> NodeResult<InputData> {
        Err(NodeErrors::NotImplementedError)
    }
}

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum NodeData {
    Axis,
    Camera,
    Light,
    Grade,
    Material,
    Primitive,
    RayMarcher,
    Scene,
    #[default]
    Texture,
}

impl Enumerator for NodeData {}

impl NodeData {
    pub fn dynamic_input_connected(&self, node_graph: &mut NodeGraph, input_id: InputId) {
        match self {
            Self::Primitive => {
                PrimitiveNode::dynamic_input_connected(node_graph, input_id);
            }
            Self::RayMarcher => {
                RayMarcherNode::dynamic_input_connected(node_graph, input_id);
            }
            _ => {}
        }
    }

    pub fn dynamic_input_disconnected(&self, node_graph: &mut NodeGraph, input_id: InputId) {
        match self {
            Self::Primitive => {
                PrimitiveNode::dynamic_input_disconnected(node_graph, input_id);
            }
            Self::RayMarcher => {
                RayMarcherNode::dynamic_input_disconnected(node_graph, input_id);
            }
            _ => {}
        }
    }

    pub fn add_to_node_graph(&self, node_graph: &mut NodeGraph, node_id: NodeId) {
        match self {
            Self::Axis => {
                AxisNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Camera => {
                CameraNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Grade => {
                GradeNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Light => {
                LightNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Material => {
                MaterialNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Primitive => {
                PrimitiveNode::add_to_node_graph(node_graph, node_id);
            }
            Self::RayMarcher => {
                RayMarcherNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Scene => {
                SceneNode::add_to_node_graph(node_graph, node_id);
            }
            Self::Texture => {
                TextureNode::add_to_node_graph(node_graph, node_id);
            }
        }
    }

    pub fn output_compatible_with_input(&self, output: &OutputData, input_name: &str) -> bool {
        match self {
            Self::Axis => AxisNode::output_is_compatible_with_named_input(output, input_name),
            Self::Camera => CameraNode::output_is_compatible_with_named_input(output, input_name),
            Self::Grade => GradeNode::output_is_compatible_with_named_input(output, input_name),
            Self::Light => LightNode::output_is_compatible_with_named_input(output, input_name),
            Self::Material => {
                MaterialNode::output_is_compatible_with_named_input(output, input_name)
            }
            Self::Primitive => {
                PrimitiveNode::output_is_compatible_with_named_input(output, input_name)
            }
            Self::RayMarcher => {
                RayMarcherNode::output_is_compatible_with_named_input(output, input_name)
            }
            Self::Scene => SceneNode::output_is_compatible_with_named_input(output, input_name),
            Self::Texture => TextureNode::output_is_compatible_with_named_input(output, input_name),
        }
    }
}
