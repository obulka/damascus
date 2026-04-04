// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashMap;

use indoc::indoc;
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    camera::{Camera, CameraId},
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::{SceneGraph, SceneGraphIdType},
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
            Self::EnableDepthOfField => InputData::Bool(default_camera.enable_depth_of_field),
            Self::Latlong => InputData::Bool(default_camera.latlong),
            Self::Axis => InputData::Mat4(default_camera.camera_to_world),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::FocalLength => "The focal length of the camera.",
            Self::FocalDistance => "The focal distance of the camera.",
            Self::FStop => "The f-stop of the camera.",
            Self::HorizontalAperture => "The horizontal aperture of the camera.",
            Self::NearPlane => "The distance to the near plane of the camera.",
            Self::FarPlane => "The distance to the far plane of the camera.",
            Self::SensorResolution => indoc! {
                "The resolution of the camera sensor, used when generating a
                    texture by rendering through this camera.",
            },
            Self::EnableDepthOfField => "If enabled, this camera will render with depth of field.",
            Self::Latlong => "Output a LatLong, 360 degree field of view image.",
            Self::Axis => "The world matrix/axis of the camera.",
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

    fn tooltip(&self) -> &str {
        match self {
            Self::Id => "A camera.",
        }
    }
}

pub struct CameraNode;

impl EvaluableNode for CameraNode {
    type Inputs = CameraInputData;
    type Outputs = CameraOutputData;

    fn update_from_data_map(
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        input_data: &InputData,
    ) -> NodeResult<()> {
        let camera_id: &CameraId = input_data.as_camera_id()?;

        scene_graph[*camera_id].focal_length = Self::Inputs::FocalLength
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].focal_distance = Self::Inputs::FocalDistance
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].f_stop = Self::Inputs::FStop
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].horizontal_aperture = Self::Inputs::HorizontalAperture
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].near_plane = Self::Inputs::NearPlane
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].far_plane = Self::Inputs::FarPlane
            .from_data_map(data_map)?
            .try_to_float()?;
        scene_graph[*camera_id].sensor_resolution = Self::Inputs::SensorResolution
            .from_data_map(data_map)?
            .try_to_uvec2()?;
        scene_graph[*camera_id].enable_depth_of_field = Self::Inputs::EnableDepthOfField
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*camera_id].latlong = Self::Inputs::Latlong
            .from_data_map(data_map)?
            .try_to_bool()?;
        scene_graph[*camera_id].camera_to_world =
            Self::Inputs::Axis.from_data_map(data_map)?.try_to_mat4()?;

        Ok(())
    }

    fn evaluate(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        cached_input_data: Option<InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let camera_id: CameraId = match cached_input_data {
            Some(input_data) => input_data.try_to_camera_id()?,
            None => scene_graph.add_camera(Camera::default()),
        };

        let scene_graph_id = InputData::SceneGraphId(camera_id.into());

        Self::update_from_data_map(scene_graph, data_map, &scene_graph_id)?;

        match output {
            Self::Outputs::Id => Ok(scene_graph_id),
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
