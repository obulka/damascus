// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumIter, EnumString};

use crate::Enumerator;

#[derive(
    Debug,
    Display,
    Default,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum OutputData {
    Mat4,
    Camera,
    Light,
    Material,
    Primitive,
    ProceduralTexture,
    #[default]
    RenderPass,
    Scene,
}

impl Enumerator for OutputData {}
