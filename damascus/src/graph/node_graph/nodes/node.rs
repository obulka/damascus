// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, str::FromStr};

use crate::graph::{
    node_graph::{
        inputs::{InputId, input_data::InputData},
        nodes::{
            NodeErrors, NodeResult,
            node_data::{
                AxisNode, AxisOutputData, CameraNode, CameraOutputData, EvaluableNode, GradeNode,
                GradeOutputData, LightNode, LightOutputData, MaterialNode, MaterialOutputData,
                NodeData, PrimitiveNode, PrimitiveOutputData, RayMarcherNode, RayMarcherOutputData,
                SceneNode, SceneOutputData, TextureReadNode, TextureReadOutputData,
            },
        },
        outputs::OutputId,
    },
    scene_graph::SceneGraph,
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

    pub fn reevaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        node_data: NodeData,
        mut data_map: HashMap<String, InputData>,
        input_data: InputData,
        output_name: String,
    ) -> NodeResult<InputData> {
        match node_data {
            NodeData::Axis => AxisNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                AxisOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Camera => CameraNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                CameraOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Grade => GradeNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                GradeOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Light => LightNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                LightOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Material => MaterialNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                MaterialOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Primitive => PrimitiveNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                PrimitiveOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::RayMarcher => RayMarcherNode::reevaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                input_data,
                RayMarcherOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Scene => SceneNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                SceneOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::TextureRead => TextureReadNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                TextureReadOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
        }
    }

    pub fn evaluate(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        node_data: NodeData,
        mut data_map: HashMap<String, InputData>,
        output_name: String,
    ) -> NodeResult<InputData> {
        match node_data {
            NodeData::Axis => AxisNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                AxisOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Camera => CameraNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                CameraOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Grade => GradeNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                GradeOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Light => LightNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                LightOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Material => MaterialNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                MaterialOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Primitive => PrimitiveNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                PrimitiveOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::RayMarcher => RayMarcherNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                RayMarcherOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::Scene => SceneNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                SceneOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
            NodeData::TextureRead => TextureReadNode::evaluate(
                device,
                queue,
                encoder,
                scene_graph,
                &mut data_map,
                TextureReadOutputData::from_str(&output_name)
                    .map_err(|error| NodeErrors::ParseOutputError(error.to_string()))?,
            ),
        }
    }
}
