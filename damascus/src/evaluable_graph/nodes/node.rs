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
    pub input_ids: Vec<InputId>,
    pub output_ids: Vec<OutputId>,
    pub data: NodeData,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            input_ids: vec![],
            output_ids: vec![],
            data: data,
        }
    }

    pub fn evaluate(
        &self,
        mut data_map: HashMap<String, InputData>,
        output_name: &str,
    ) -> NodeResult<InputData> {
        match self.data {
            NodeData::Axis => AxisNode::evaluate(
                &mut data_map,
                AxisOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Camera => CameraNode::evaluate(
                &mut data_map,
                CameraOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Grade => GradeNode::evaluate(
                &mut data_map,
                GradeOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Light => LightNode::evaluate(
                &mut data_map,
                LightOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Material => MaterialNode::evaluate(
                &mut data_map,
                MaterialOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Primitive => PrimitiveNode::evaluate(
                &mut data_map,
                PrimitiveOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::RayMarcher => RayMarcherNode::evaluate(
                &mut data_map,
                RayMarcherOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Scene => SceneNode::evaluate(
                &mut data_map,
                SceneOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Texture => TextureNode::evaluate(
                &mut data_map,
                TextureOutputData::from_str(output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
        }
    }
}
