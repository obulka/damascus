// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use strum::{EnumCount, EnumIter, EnumString};

use super::{process_shader_source, Compiler, PreprocessorDirectives};

use crate::renderers::compositor::{Compositor, GPUCompositor, Std430GPUCompositor};

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

pub fn compositor_vertex_shader(
    preprocessor_directives: &HashSet<CompositorPreprocessorDirectives>,
) -> String {
    process_shader_source(
        include_str!("./wgsl/pipelines/compositor/vertex_shader.wgsl"),
        preprocessor_directives,
    )
}

pub fn compositor_fragment_shader(
    preprocessor_directives: &HashSet<CompositorPreprocessorDirectives>,
) -> String {
    process_shader_source(
        include_str!("./wgsl/pipelines/compositor/fragment_shader.wgsl"),
        preprocessor_directives,
    )
}

pub fn all_directives_for_compositor() -> HashSet<CompositorPreprocessorDirectives> {
    HashSet::<CompositorPreprocessorDirectives>::from([])
}

pub fn directives_for_compositor(
    _compositor: &Compositor,
) -> HashSet<CompositorPreprocessorDirectives> {
    let preprocessor_directives = HashSet::<CompositorPreprocessorDirectives>::new();
    //TODO
    preprocessor_directives
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompositorCompiler {}

impl Default for CompositorCompiler {
    fn default() -> Self {
        Self {}
    }
}

impl CompositorCompiler {}

impl Compiler<Compositor, CompositorPreprocessorDirectives> for CompositorCompiler {
    fn dynamic_directives(
        &self,
        _options: &Compositor,
    ) -> HashSet<CompositorPreprocessorDirectives> {
        let preprocessor_directives = HashSet::<CompositorPreprocessorDirectives>::new();
        //TODO
        preprocessor_directives
    }
}
