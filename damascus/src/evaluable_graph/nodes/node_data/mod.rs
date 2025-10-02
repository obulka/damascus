// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    evaluable_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeId,
        outputs::output_data::NodeOutputData,
        EvaluableGraph,
    },
    Enumerator,
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

    fn add_to_graph(graph: &mut EvaluableGraph, node_id: NodeId) {
        Self::Inputs::add_to_node(graph, node_id);
        Self::Outputs::add_to_node(graph, node_id);
    }

    fn evaluate(
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
    pub fn add_to_graph(&self, graph: &mut EvaluableGraph, node_id: NodeId) {
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
}
