// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use damascus_core::shaders::ray_marcher::RayMarcherCompilerSettings;

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewSettings {
    pub zoom: f32,
}

impl Default for TextureViewSettings {
    fn default() -> Self {
        Self { zoom: 1. }
    }
}

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

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ViewportSettings {
    pub compiler_settings: RayMarcherCompilerSettings,
    pub texture_view: TextureViewSettings,
    pub ray_marcher_view: RayMarcherViewSettings,
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
            compiler_settings: RayMarcherCompilerSettings::default(),
            texture_view: TextureViewSettings::default(),
            ray_marcher_view: RayMarcherViewSettings::default(),
        }
    }
}
