// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use strum::{EnumCount, EnumIter, EnumString};

use super::PreprocessorDirectives;

pub const TEXTURE_VIEWER_VERTEX_SHADER: String =
    include_str!("./wgsl/pipelines/texture_viewer/vertex_shader.wgsl");
pub const TEXTURE_VIEWER_FRAGMENT_SHADER: String =
    include_str!("./wgsl/pipelines/texture_viewer/fragment_shader.wgsl");

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Hash,
    EnumString,
    EnumCount,
    EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum TextureViewerPreprocessorDirectives {}

impl PreprocessorDirectives for TextureViewerPreprocessorDirectives {}
