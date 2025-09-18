// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{scene::Scene, Enumerator};

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
pub enum SceneInputData {
    #[default]
    Scene0,
    Scene1,
}

impl Enumerator for SceneInputData {}

impl NodeInputData for SceneInputData {
    fn default_data(&self) -> InputData {
        match self {
            Self::Scene0 => InputData::Scene(Scene::default()),
            Self::Scene1 => InputData::Scene(Scene::default()),
        }
    }
}
