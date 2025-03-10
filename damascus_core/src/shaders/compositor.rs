// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use strum::{EnumCount, EnumIter, EnumString};

use super::{process_shader_source, CompilerSettings, PreprocessorDirectives};

use crate::{
    renderers::compositor::{Compositor, GPUCompositor, Std430GPUCompositor},
    Settings,
};

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
pub enum CompositorPreprocessorDirectives {}

impl PreprocessorDirectives for CompositorPreprocessorDirectives {}

pub fn compositing_shader(
    preprocessor_directives: &HashSet<CompositorPreprocessorDirectives>,
) -> String {
    process_shader_source(
        include_str!("./wgsl/renderers/compositor/compositor.wgsl"),
        preprocessor_directives,
    )
}

pub fn all_directives_for_compositor() -> HashSet<CompositorPreprocessorDirectives> {
    HashSet::<CompositorPreprocessorDirectives>::from([])
}

pub fn directives_for_compositor(
    compositor: &Compositor,
) -> HashSet<CompositorPreprocessorDirectives> {
    let mut preprocessor_directives = HashSet::<CompositorPreprocessorDirectives>::new();

    preprocessor_directives
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompositorCompilerSettings {}

impl Default for CompositorCompilerSettings {
    fn default() -> Self {
        Self {}
    }
}

impl CompositorCompilerSettings {}

impl Settings for CompositorCompilerSettings {}

impl
    CompilerSettings<
        CompositorPreprocessorDirectives,
        Compositor,
        GPUCompositor,
        Std430GPUCompositor,
    > for CompositorCompilerSettings
{
    fn directives(&self, renderer: &Compositor) -> HashSet<CompositorPreprocessorDirectives> {
        let mut preprocessor_directives = HashSet::<CompositorPreprocessorDirectives>::new();

        preprocessor_directives
    }

    fn dynamic_recompilation_enabled(&self) -> bool {
        true
    }
}
