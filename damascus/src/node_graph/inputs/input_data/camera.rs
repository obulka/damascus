// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{camera::Camera, Enumerator};

use super::{InputData, NodeInputData};

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
    WorldMatrix,
    EnableDepthOfField,
    Latlong,
}

impl Enumerator for CameraInputData {}

impl NodeInputData for CameraInputData {
    fn default_data(&self) -> InputData {
        let default_camera = Camera::default();
        match self {
            CameraInputData::FocalLength => InputData::Float(default_camera.focal_length),
            CameraInputData::FocalDistance => InputData::Float(default_camera.focal_distance),
            CameraInputData::FStop => InputData::Float(default_camera.f_stop),
            CameraInputData::HorizontalAperture => {
                InputData::Float(default_camera.horizontal_aperture)
            }
            CameraInputData::NearPlane => InputData::Float(default_camera.near_plane),
            CameraInputData::FarPlane => InputData::Float(default_camera.far_plane),
            CameraInputData::SensorResolution => InputData::UVec2(default_camera.sensor_resolution),
            CameraInputData::WorldMatrix => InputData::Mat4(default_camera.camera_to_world),
            CameraInputData::EnableDepthOfField => {
                InputData::Bool(default_camera.enable_depth_of_field)
            }
            CameraInputData::Latlong => InputData::Bool(default_camera.latlong),
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
