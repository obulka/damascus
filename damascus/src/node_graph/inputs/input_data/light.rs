// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{Mat4, Vec3};
use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{lights::Light, scene::Scene, Enumerator};

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
pub enum LightInputData {
    #[default]
    Scene,
    WorldMatrix,
    LightType,
    Direction,
    Position,
    Iterations,
    Intensity,
    Falloff,
    Colour,
    ShadowHardness,
    SoftenShadows,
}

impl Enumerator for LightInputData {}

impl NodeInputData for LightInputData {
    fn default_data(&self) -> InputData {
        let default_light = Light::default();
        match self {
            LightInputData::Scene => InputData::Scene(Scene::default()),
            LightInputData::WorldMatrix => InputData::Mat4(Mat4::IDENTITY),
            LightInputData::LightType => InputData::Enum(default_light.light_type.into()),
            LightInputData::Direction => InputData::Vec3(Vec3::NEG_Y),
            LightInputData::Position => InputData::Vec3(Vec3::Y),
            LightInputData::Iterations => InputData::UInt(default_light.dimensional_data.x as u32),
            LightInputData::Intensity => InputData::Float(default_light.intensity),
            LightInputData::Falloff => InputData::UInt(default_light.falloff),
            LightInputData::Colour => InputData::Vec3(default_light.colour),
            LightInputData::ShadowHardness => InputData::Float(default_light.shadow_hardness),
            LightInputData::SoftenShadows => InputData::Bool(default_light.soften_shadows),
        }
    }
}
