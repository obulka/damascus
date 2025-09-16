// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Mat4, Vec3};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::Enumerator;

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
            AxisInputData::Axis => InputData::Mat4(Mat4::IDENTITY),
            AxisInputData::Translate => InputData::Vec3(Vec3::ZERO),
            AxisInputData::Rotate => InputData::Vec3(Vec3::ZERO),
            AxisInputData::UniformScale => InputData::Float(1.),
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
