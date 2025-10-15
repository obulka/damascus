// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::{Enumerator, shaders::PreprocessorDirectives};

pub const TEXTURE_VIEWER_VERTEX_SHADER: &str =
    include_str!("../wgsl/pipelines/texture/view/vertex_shader.wgsl");
pub const TEXTURE_VIEWER_FRAGMENT_SHADER: &str =
    include_str!("../wgsl/pipelines/texture/view/fragment_shader.wgsl");

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum TextureViewerPreprocessorDirectives {
    #[default]
    None,
}

impl Enumerator for TextureViewerPreprocessorDirectives {}

impl PreprocessorDirectives for TextureViewerPreprocessorDirectives {}
