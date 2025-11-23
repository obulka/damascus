// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    camera::Camera,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphId, SceneGraphIdType},
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum CameraInputData {
    #[default]
    FocalLength,
    FocalDistance,
    FStop,
    HorizontalAperture,
    NearPlane,
    FarPlane,
    SensorResolution,
    EnableDepthOfField,
    Latlong,
    Axis,
}

impl NodeInputData for CameraInputData {
    fn default_data(&self) -> InputData {
        let default_camera = Camera::default();
        match self {
            Self::FocalLength => InputData::Float(default_camera.focal_length),
            Self::FocalDistance => InputData::Float(default_camera.focal_distance),
            Self::FStop => InputData::Float(default_camera.f_stop),
            Self::HorizontalAperture => InputData::Float(default_camera.horizontal_aperture),
            Self::NearPlane => InputData::Float(default_camera.near_plane),
            Self::FarPlane => InputData::Float(default_camera.far_plane),
            Self::SensorResolution => InputData::UVec2(default_camera.sensor_resolution),
            Self::Axis => InputData::Mat4(default_camera.camera_to_world),
            Self::EnableDepthOfField => InputData::Bool(default_camera.enable_depth_of_field),
            Self::Latlong => InputData::Bool(default_camera.latlong),
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum CameraOutputData {
    #[default]
    Id,
}

impl NodeOutputData for CameraOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Id => OutputData::SceneGraphId(SceneGraphIdType::Camera),
        }
    }
}

pub struct CameraNode;

impl EvaluableNode for CameraNode {
    type Inputs = CameraInputData;
    type Outputs = CameraOutputData;

    fn evaluate(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::Id => Ok(InputData::SceneGraphId(SceneGraphId::Camera(
                scene_graph.add_camera(Camera::new(
                    Self::Inputs::FocalLength
                        .get_data(data_map)?
                        .try_to_float()?,
                    Self::Inputs::HorizontalAperture
                        .get_data(data_map)?
                        .try_to_float()?,
                    Self::Inputs::NearPlane.get_data(data_map)?.try_to_float()?,
                    Self::Inputs::FarPlane.get_data(data_map)?.try_to_float()?,
                    Self::Inputs::FocalDistance
                        .get_data(data_map)?
                        .try_to_float()?,
                    Self::Inputs::FStop.get_data(data_map)?.try_to_float()?,
                    Self::Inputs::SensorResolution
                        .get_data(data_map)?
                        .try_to_uvec2()?,
                    Self::Inputs::EnableDepthOfField
                        .get_data(data_map)?
                        .try_to_bool()?,
                    Self::Inputs::Latlong.get_data(data_map)?.try_to_bool()?,
                    Self::Inputs::Axis.get_data(data_map)?.try_to_mat4()?,
                )),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labels() {
        assert_eq!(
            CameraInputData::EnableDepthOfField.label(),
            "enable depth of field"
        );
        assert_eq!(
            CameraInputData::SensorResolution.label(),
            "sensor resolution"
        );
    }
}
