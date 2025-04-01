// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::{
    shaders::{compositor::CompositorCompilerSettings, ray_marcher::RayMarcherCompilerSettings},
    Settings,
};

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ViewportCompilerSettings {
    pub ray_marcher: RayMarcherCompilerSettings,
    pub compositor: CompositorCompilerSettings,
}

impl Settings for ViewportCompilerSettings {}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompositorViewSettings {
    pub zoom: f32,
    pub viewer_gain: f32,
    pub viewer_gamma: f32,
}

impl Default for CompositorViewSettings {
    fn default() -> Self {
        Self {
            zoom: 1.,
            viewer_gain: 1.,
            viewer_gamma: 1.,
        }
    }
}

impl Settings for CompositorViewSettings {}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherViewSettings {
    pub max_primitives: usize,
    pub max_lights: usize,
}

impl Default for RayMarcherViewSettings {
    fn default() -> Self {
        Self {
            max_primitives: 1024,
            max_lights: 1024,
        }
    }
}

impl Settings for RayMarcherViewSettings {}

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ViewportSettings {
    pub compiler_settings: ViewportCompilerSettings,
    pub compositor_view: CompositorViewSettings,
    pub ray_marcher_view: RayMarcherViewSettings,
}

impl Settings for ViewportSettings {}
