// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashMap, f32::consts::PI};

use glam::{EulerRot, Mat4, Quat, Vec3};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{
    node_graph::{
        inputs::input_data::{InputData, NodeInputData},
        nodes::NodeResult,
        outputs::output_data::{NodeOutputData, OutputData},
    },
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
pub enum AxisInputData {
    #[default]
    Axis,
    Translate,
    Rotate,
    UniformScale,
}

impl Enumerator for AxisInputData {}

impl NodeInputData for AxisInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Axis => InputData::Mat4(Mat4::IDENTITY),
            Self::Translate => InputData::Vec3(Vec3::ZERO),
            Self::Rotate => InputData::Vec3(Vec3::ZERO),
            Self::UniformScale => InputData::Float(1.),
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
pub enum AxisOutputData {
    #[default]
    Axis,
}

impl Enumerator for AxisOutputData {}

impl NodeOutputData for AxisOutputData {
    fn default_data(&self) -> OutputData {
        match self {
            Self::Axis => OutputData::Mat4,
        }
    }
}

pub struct AxisNode;

impl NodeOperation for AxisNode {
    type Inputs = AxisInputData;
    type Outputs = AxisOutputData;

    fn evaluate(
        data_map: &mut HashMap<String, InputData>,
        output: Self::Outputs,
    ) -> NodeResult<InputData> {
        let rotate: Vec3 = Self::Inputs::Rotate.get_data(data_map)?.try_to_vec3()? * PI / 180.;
        let quaternion = Quat::from_euler(EulerRot::XYZ, rotate.x, rotate.y, rotate.z);

        let axis: Mat4 = Self::Inputs::Axis.get_data(data_map)?.try_to_mat4()?
            * Mat4::from_scale_rotation_translation(
                Vec3::splat(
                    Self::Inputs::UniformScale
                        .get_data(data_map)?
                        .try_to_float()?,
                ),
                quaternion,
                Self::Inputs::Translate.get_data(data_map)?.try_to_vec3()?,
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
