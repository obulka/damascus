// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use glam::{Mat4, Quat, Vec3};

use crate::node_graph::{
    inputs::{
        input_data::{
            AxisInputData, CameraInputData, GradeInputData, InputData, LightInputData,
            MaterialInputData, NodeInputData, PrimitiveInputData, RayMarcherInputData,
            SceneInputData, TextureInputData,
        },
        InputErrors, InputId, InputResult,
    },
    outputs::{output_data::OutputData, OutputId},
    NodeGraph,
};

use super::{node_data::NodeData, NodeId};

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
        _output_id: OutputId,
        mut data_map: HashMap<String, InputData>,
    ) -> InputResult<InputData> {
        match self.data {
            NodeData::Axis => AxisInputData::compute_output(&mut data_map),
            // NodeData::Camera => {}
            // NodeData::Grade => {}
            // NodeData::Light => {}
            // NodeData::Material => {}
            // NodeData::Primitive => {}
            // NodeData::RayMarcher => {}
            // NodeData::Scene => {}
            // NodeData::Texture => { }
            _ => Err(InputErrors::UnknownError),
        }
    }
}
