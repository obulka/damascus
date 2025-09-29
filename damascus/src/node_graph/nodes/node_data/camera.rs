// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    camera::Camera,
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeResult,
        outputs::output_data::{NodeOutputData, OutputData},
    },
    scene::Scene,
    Enumerator,
};

use super::NodeOperation;

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

impl Enumerator for CameraInputData {}

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
pub enum CameraOutputData {
    #[default]
    SceneGraph,
}

impl Enumerator for CameraOutputData {}

impl NodeOutputData for CameraOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::SceneGraph => OutputData::SceneGraph,
        }
    }
}

pub struct CameraNode;

impl NodeOperation for CameraNode {
    type Inputs = CameraInputData;
    type Outputs = CameraOutputData;

    fn evaluate(
        output: Self::Outputs,
        data_map: &mut HashMap<String, InputData>,
    ) -> NodeResult<InputData> {
        match output {
            Self::Outputs::SceneGraph => Ok(InputData::SceneGraph(
                Scene::default().render_camera(Camera::new(
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
            )),
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
