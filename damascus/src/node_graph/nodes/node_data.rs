// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{
        inputs::input_data::{
            AxisInputData, CameraInputData, GradeInputData, LightInputData, MaterialInputData,
            NodeInputData, PrimitiveInputData, RayMarcherInputData, SceneInputData,
            TextureInputData,
        },
        nodes::NodeId,
        outputs::output_data::OutputData,
        NodeGraph,
    },
    Enumerator,
};

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
    pub fn add_inputs_and_outputs_to_graph(&self, node_graph: &mut NodeGraph, node_id: NodeId) {
        match self {
            Self::Axis => {
                AxisInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Mat4);
            }
            Self::Camera => {
                CameraInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Scene);
            }
            Self::Grade => {
                GradeInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::RenderPass);
            }
            Self::Light => {
                LightInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Scene);
            }
            Self::Material => {
                MaterialInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Scene);
            }
            Self::Primitive => {
                PrimitiveInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Scene);
            }
            Self::RayMarcher => {
                RayMarcherInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::RenderPass);
            }
            Self::Scene => {
                SceneInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::Scene);
            }
            Self::Texture => {
                TextureInputData::add_to_node(node_graph, node_id);
                node_graph.add_output(node_id, OutputData::RenderPass);
            }
        }
    }
}
