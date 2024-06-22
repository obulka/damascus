// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct ViewportSettings {
    pub enable_dynamic_recompilation_for_materials: bool,
    pub enable_dynamic_recompilation_for_primitives: bool,
    pub enable_dynamic_recompilation_for_ray_marcher: bool,
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
            enable_dynamic_recompilation_for_materials: true,
            enable_dynamic_recompilation_for_primitives: false,
            enable_dynamic_recompilation_for_ray_marcher: true,
        }
    }
}

impl ViewportSettings {
    pub fn dynamic_recompilation_enabled(&self) -> bool {
        self.enable_dynamic_recompilation_for_primitives
            || self.enable_dynamic_recompilation_for_materials
            || self.enable_dynamic_recompilation_for_ray_marcher
    }
}
