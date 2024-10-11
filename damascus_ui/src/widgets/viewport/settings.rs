// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PipelineSettings2D {
    pub max_primitives: usize, // TODO Remove these
    pub max_lights: usize,
}

impl Default for PipelineSettings2D {
    fn default() -> Self {
        Self {
            max_primitives: 1024,
            max_lights: 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PipelineSettings3D {
    pub max_primitives: usize,
    pub max_lights: usize,
}

impl Default for PipelineSettings3D {
    fn default() -> Self {
        Self {
            max_primitives: 1024,
            max_lights: 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompilerSettings {
    pub enable_dynamic_recompilation_for_materials: bool,
    pub enable_dynamic_recompilation_for_primitives: bool,
    pub enable_dynamic_recompilation_for_ray_marcher: bool,
    pub enable_dynamic_recompilation_for_lights: bool,
}

impl Default for CompilerSettings {
    fn default() -> Self {
        Self {
            enable_dynamic_recompilation_for_materials: true,
            enable_dynamic_recompilation_for_primitives: true,
            enable_dynamic_recompilation_for_ray_marcher: true,
            enable_dynamic_recompilation_for_lights: true,
        }
    }
}

impl CompilerSettings {
    pub fn dynamic_recompilation_enabled(&self) -> bool {
        self.enable_dynamic_recompilation_for_primitives
            || self.enable_dynamic_recompilation_for_materials
            || self.enable_dynamic_recompilation_for_ray_marcher
            || self.enable_dynamic_recompilation_for_lights
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum ViewportActiveState {
    Viewport2D,
    #[default]
    Viewport3D,
    SeparateWindows,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ViewportSettings {
    pub compiler_settings: CompilerSettings,
    pub pipeline_settings_2d: PipelineSettings2D,
    pub pipeline_settings_3d: PipelineSettings3D,
    pub active_state: ViewportActiveState,
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
            compiler_settings: CompilerSettings::default(),
            pipeline_settings_2d: PipelineSettings2D::default(),
            pipeline_settings_3d: PipelineSettings3D::default(),
            active_state: ViewportActiveState::default(),
        }
    }
}
