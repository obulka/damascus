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
        mut input_data: HashMap<String, InputData>,
    ) -> InputResult<InputData> {
        match self.data {
            NodeData::Axis => {
                let input_axis: Mat4 = AxisInputData::Axis
                    .get_data(&mut input_data)?
                    .try_to_mat4()?;

                let translate: Vec3 = AxisInputData::Translate
                    .get_data(&mut input_data)?
                    .try_to_vec3()?;

                let rotate: Vec3 = AxisInputData::Rotate
                    .get_data(&mut input_data)?
                    .try_to_vec3()?
                    * std::f32::consts::PI
                    / 180.;

                let uniform_scale: f32 = AxisInputData::UniformScale
                    .get_data(&mut input_data)?
                    .try_to_float()?;

                let quaternion =
                    Quat::from_euler(glam::EulerRot::XYZ, rotate.x, rotate.y, rotate.z);

                let input_data = InputData::Mat4(
                    input_axis
                        * glam::Mat4::from_scale_rotation_translation(
                            glam::Vec3::splat(uniform_scale),
                            quaternion,
                            translate,
                        ),
                );
                Ok(input_data)
            }
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
