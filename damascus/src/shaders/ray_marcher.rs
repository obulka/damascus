// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use strum::{Display, EnumCount, EnumIter, EnumString};

use super::{PreprocessorDirectives, scene::ScenePreprocessorDirectives};

use crate::{Enumerator, render_passes::ray_marcher::RayMarcherRenderData, textures::AOVs};

pub const RAY_MARCHER_VERTEX_SHADER: &str =
    include_str!("./wgsl/pipelines/ray_marcher/vertex_shader.wgsl");
pub const RAY_MARCHER_FRAGMENT_SHADER: &str =
    include_str!("./wgsl/pipelines/ray_marcher/fragment_shader.wgsl");

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
pub enum RayMarcherPreprocessorDirectives {
    #[default]
    EnableAOVs,
    EnableLightSampling,
    SceneDirective(ScenePreprocessorDirectives),
}

impl Enumerator for RayMarcherPreprocessorDirectives {}

impl PreprocessorDirectives for RayMarcherPreprocessorDirectives {}

impl From<ScenePreprocessorDirectives> for RayMarcherPreprocessorDirectives {
    fn from(scene_directive: ScenePreprocessorDirectives) -> Self {
        Self::SceneDirective(scene_directive)
    }
}

impl RayMarcherPreprocessorDirectives {
    pub fn all_directives_for_ray_marcher() -> HashSet<Self> {
        HashSet::<RayMarcherPreprocessorDirectives>::from([
            Self::EnableAOVs,
            Self::EnableLightSampling,
        ])
    }

    pub fn directives_for_ray_marcher(ray_marcher: &RayMarcherRenderData) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if ray_marcher.output_aov > AOVs::Beauty {
            preprocessor_directives.insert(Self::EnableAOVs);
        }
        if ray_marcher.light_sampling {
            preprocessor_directives.insert(Self::EnableLightSampling);
        }

        preprocessor_directives
    }
}
