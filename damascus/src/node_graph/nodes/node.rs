// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, str::FromStr};

use crate::node_graph::{
    inputs::{input_data::InputData, InputId},
    outputs::OutputId,
};

use super::{
    node_data::{
        AxisNode, AxisOutputData, CameraNode, CameraOutputData, GradeNode, GradeOutputData,
        LightNode, LightOutputData, MaterialNode, MaterialOutputData, NodeData, NodeOperation,
        PrimitiveNode, PrimitiveOutputData, RayMarcherNode, RayMarcherOutputData, SceneNode,
        SceneOutputData, TextureNode, TextureOutputData,
    },
    NodeErrors, NodeId, NodeResult,
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub input_ids: Vec<InputId>,
    pub output_ids: Vec<OutputId>,
    pub data: NodeData,
}

impl Node {
    pub fn new(id: NodeId, data: NodeData) -> Self {
        Self {
            id: id,
            input_ids: vec![],
            output_ids: vec![],
            data: data,
        }
    }

    pub fn evaluate(
        &self,
        output_name: &str,
        mut data_map: HashMap<String, InputData>,
    ) -> NodeResult<InputData> {
        match self.data {
            NodeData::Axis => AxisNode::evaluate(
                AxisOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Camera => CameraNode::evaluate(
                CameraOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Grade => GradeNode::evaluate(
                GradeOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Light => LightNode::evaluate(
                LightOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Material => MaterialNode::evaluate(
                MaterialOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Primitive => PrimitiveNode::evaluate(
                PrimitiveOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::RayMarcher => RayMarcherNode::evaluate(
                RayMarcherOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Scene => SceneNode::evaluate(
                SceneOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
            NodeData::Texture => TextureNode::evaluate(
                TextureOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
                &mut data_map,
            ),
        }
    }
}
