// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, f32::consts::PI};

use glam::{EulerRot, Mat4, Quat, Vec3};
use indoc::indoc;
use macro_rules_attribute::derive;

use crate::{
    EnumHashTraits,
    graph::{
        node_graph::{
            inputs::input_data::{InputData, NodeInputData},
            nodes::{NodeResult, node_data::EvaluableNode},
            outputs::output_data::{NodeOutputData, OutputData},
        },
        scene_graph::SceneGraph,
    },
};

#[derive(Copy, Default, EnumHashTraits!)]
pub enum AxisInputData {
    #[default]
    Axis,
    Translate,
    Rotate,
    UniformScale,
}

impl NodeInputData for AxisInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Axis => InputData::Mat4(Mat4::IDENTITY),
            Self::Translate => InputData::Vec3(Vec3::ZERO),
            Self::Rotate => InputData::Vec3(Vec3::ZERO),
            Self::UniformScale => InputData::Float(1.),
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::Axis => "The parent axis.",
            Self::Translate => "The translation of this axis.",
            Self::Rotate => "The rotation of this axis.",
            Self::UniformScale => indoc! {
                "The uniform scale of this axis.\n
                    We use uniform scale because the signed distance
                    fields cannot have their individual axes scaled."
            },
        }
    }
}

#[derive(Copy, Default, EnumHashTraits!)]
pub enum AxisOutputData {
    #[default]
    Axis,
}

impl NodeOutputData for AxisOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Axis => OutputData::Mat4,
        }
    }

    fn tooltip(&self) -> &str {
        match self {
            Self::Axis => "A transformation matrix.",
        }
    }
}

pub struct AxisNode;

impl EvaluableNode for AxisNode {
    type Inputs = AxisInputData;
    type Outputs = AxisOutputData;

    fn evaluate(
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        _scene_graph: &mut SceneGraph,
        data_map: &mut HashMap<String, InputData>,
        _cached_input_data: Option<InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let rotate: Vec3 = Self::Inputs::Rotate
            .from_data_map(data_map)?
            .try_to_vec3()?
            * PI
            / 180.;
        let quaternion = Quat::from_euler(EulerRot::XYZ, rotate.x, rotate.y, rotate.z);

        let axis: Mat4 = Self::Inputs::Axis.from_data_map(data_map)?.try_to_mat4()?
            * Mat4::from_scale_rotation_translation(
                Vec3::splat(
                    Self::Inputs::UniformScale
                        .from_data_map(data_map)?
                        .try_to_float()?,
                ),
                quaternion,
                Self::Inputs::Translate
                    .from_data_map(data_map)?
                    .try_to_vec3()?,
            );

        match output {
            Self::Outputs::Axis => Ok(InputData::Mat4(axis)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_names() {
        assert_eq!(AxisInputData::Axis.name(), "Axis");
        assert_eq!(AxisInputData::Translate.name(), "Translate");
        assert_eq!(AxisInputData::Rotate.name(), "Rotate");
        assert_eq!(AxisInputData::UniformScale.name(), "UniformScale");
    }

    #[test]
    fn test_labels() {
        assert_eq!(AxisInputData::Axis.label(), "axis");
        assert_eq!(AxisInputData::Translate.label(), "translate");
        assert_eq!(AxisInputData::Rotate.label(), "rotate");
        assert_eq!(AxisInputData::UniformScale.label(), "uniform scale");
    }

    #[test]
    fn test_defaults() {
        assert_eq!(
            AxisInputData::Axis
                .default_data()
                .try_to_mat4()
                .expect("Data should be a mat4"),
            Mat4::IDENTITY
        );
        assert_eq!(
            AxisInputData::Translate
                .default_data()
                .try_to_vec3()
                .expect("Data should be a vec3"),
            Vec3::ZERO
        );
        assert_eq!(
            AxisInputData::Rotate
                .default_data()
                .try_to_vec3()
                .expect("Data should be a vec3"),
            Vec3::ZERO
        );
        assert_eq!(
            AxisInputData::UniformScale
                .default_data()
                .try_to_float()
                .expect("Data should be a float"),
            1.
        );
    }
}
