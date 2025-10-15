// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, iter, ops::Range, str::FromStr};

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
        outputs::{
            OutputId,
            output_data::{NodeOutputData, OutputData},
        },
    },
    scene_graph::SceneGraph,
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

    /// An iterable that visits all inputs which dynamically spawn other inputs
    /// Returns the input and a range representing the minimum and maximum
    /// number of inputs which should spawn if the maximum is less than or
    /// equal to the minimum, then infinite inputs will be allowed to spawn
    fn dynamic_inputs() -> impl Iterator<Item = (Self::Inputs, Range<usize>)> {
        iter::empty()
    }

    fn add_to_graph(graph: &mut NodeGraph, node_id: NodeId) {
        Self::Inputs::add_to_node(graph, node_id);
        Self::Outputs::add_to_node(graph, node_id);
    }

    fn dynamic_input_connected(graph: &mut NodeGraph, input_id: InputId, _output_id: OutputId) {
        Self::dynamic_inputs().for_each(|(input, range)| {
            let input_name: String = input.name();
            if let Some(input_number_as_str) = graph[input_id].name.strip_prefix(&input_name) {
                let node_id: NodeId = graph[input_id].node_id;
                let next_input_index: usize = graph[node_id]
                    .input_ids
                    .iter()
                    .position(|&id| id == input_id)
                    .unwrap()
                    + 1;

                let allow_infinite_inputs: bool = range.end <= range.start;

                if input_number_as_str == "" {
                    // The first child was connected
                    graph.insert_input(
                        node_id,
                        &format!("{input_name}1"),
                        input.default_data(),
                        next_input_index,
                    );
                } else if let Ok(input_number) = input_number_as_str.parse::<usize>()
                    && (allow_infinite_inputs || input_number + 1 < range.end)
                {
                    // Assume the highest number was connected because disconnected
                    // inputs inbetween will be collapsed
                    let next_input_number: usize = input_number + 1;
                    graph.insert_input(
                        node_id,
                        &format!("{input_name}{next_input_number}"),
                        input.default_data(),
                        next_input_index,
                    );
                }
            }
        });
    }

    // fn input_value_changed(_graph: &mut NodeGraph, _node_id: NodeId, _input: &Self::Inputs) {}

    fn input_disconnected(_graph: &mut NodeGraph, _input_id: InputId, _output_id: OutputId) {}

    fn input_connected(graph: &mut NodeGraph, input_id: InputId, output_id: OutputId) {
        Self::dynamic_input_connected(graph, input_id, output_id);
    }

    fn output_compatible_with_named_input(output: &OutputData, input_name: &str) -> bool {
        match Self::Inputs::from_str(input_name) {
            Ok(input_variant) => Self::output_compatible_with_input(output, &input_variant),
            _ => false,
        }
    }

    fn output_compatible_with_input(output: &OutputData, input: &Self::Inputs) -> bool {
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

    // fn named_input_value_changed(graph: &mut NodeGraph, node_id: NodeId, input_name: &str) {
    //     if let Ok(input_variant) = Self::Inputs::from_str(input_name) {
    //         Self::input_value_changed(graph, node_id, &input_variant);
    //     }
    // }

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
    pub fn add_to_graph(&self, graph: &mut NodeGraph, node_id: NodeId) {
        match self {
            Self::Axis => {
                AxisNode::add_to_graph(graph, node_id);
            }
            Self::Camera => {
                CameraNode::add_to_graph(graph, node_id);
            }
            Self::Grade => {
                GradeNode::add_to_graph(graph, node_id);
            }
            Self::Light => {
                LightNode::add_to_graph(graph, node_id);
            }
            Self::Material => {
                MaterialNode::add_to_graph(graph, node_id);
            }
            Self::Primitive => {
                PrimitiveNode::add_to_graph(graph, node_id);
            }
            Self::RayMarcher => {
                RayMarcherNode::add_to_graph(graph, node_id);
            }
            Self::Scene => {
                SceneNode::add_to_graph(graph, node_id);
            }
            Self::Texture => {
                TextureNode::add_to_graph(graph, node_id);
            }
        }
    }

    pub fn output_compatible_with_input(&self, output: &OutputData, input_name: &str) -> bool {
        match self {
            Self::Axis => AxisNode::output_compatible_with_named_input(output, input_name),
            Self::Camera => CameraNode::output_compatible_with_named_input(output, input_name),
            Self::Grade => GradeNode::output_compatible_with_named_input(output, input_name),
            Self::Light => LightNode::output_compatible_with_named_input(output, input_name),
            Self::Material => MaterialNode::output_compatible_with_named_input(output, input_name),
            Self::Primitive => {
                PrimitiveNode::output_compatible_with_named_input(output, input_name)
            }
            Self::RayMarcher => {
                RayMarcherNode::output_compatible_with_named_input(output, input_name)
            }
            Self::Scene => SceneNode::output_compatible_with_named_input(output, input_name),
            Self::Texture => TextureNode::output_compatible_with_named_input(output, input_name),
        }
    }
}
