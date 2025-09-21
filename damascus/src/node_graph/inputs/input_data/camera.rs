// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use glam::{Mat4, UVec2};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{camera::Camera, scene::Scene, Enumerator};

use super::{InputData, InputResult, NodeInputData};

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

    fn compute_output(data_map: &mut HashMap<String, InputData>) -> InputResult<InputData> {
        Ok(InputData::Scene(
            Scene::default().render_camera(Camera::new(
                Self::FocalLength.get_data(data_map)?.try_to_float()?,
                Self::HorizontalAperture
                    .get_data(data_map)?
                    .try_to_float()?,
                Self::NearPlane.get_data(data_map)?.try_to_float()?,
                Self::FarPlane.get_data(data_map)?.try_to_float()?,
                Self::FocalDistance.get_data(data_map)?.try_to_float()?,
                Self::FStop.get_data(data_map)?.try_to_float()?,
                Self::SensorResolution.get_data(data_map)?.try_to_uvec2()?,
                Self::EnableDepthOfField.get_data(data_map)?.try_to_bool()?,
                Self::Latlong.get_data(data_map)?.try_to_bool()?,
                Self::Axis.get_data(data_map)?.try_to_mat4()?,
            )),
        ))
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
