// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use macro_rules_attribute::derive;

use crate::PreprocessorDirectivesTraits;

pub const TEXTURE_VIEWER_VERTEX_SHADER: &str =
    include_str!("../wgsl/pipelines/texture/view/vertex_shader.wgsl");
pub const TEXTURE_VIEWER_FRAGMENT_SHADER: &str =
    include_str!("../wgsl/pipelines/texture/view/fragment_shader.wgsl");

#[derive(Copy, Default, PreprocessorDirectivesTraits!)]
pub enum TextureViewerPreprocessorDirectives {
    #[default]
    None,
}
