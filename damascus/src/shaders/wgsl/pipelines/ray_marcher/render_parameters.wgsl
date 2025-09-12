// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


// ------- Flag bit masks --------
// RenderParameters
const DYNAMIC_LEVEL_OF_DETAIL: u32 = 1u;
const SAMPLE_ATMOSPHERE: u32 = 2u;
const SECONDARY_SAMPLING: u32 = 4u;

// RenderState
const PAUSED: u32 = 1u;


struct RenderParameters {
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seed: u32,
    equiangular_samples: u32,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    output_aov: u32,
    flags: u32,
}


struct SceneParameters {
    num_primitives: u32,
    // Number of emissive prims + num_non_physical_lights
    num_lights: u32,
    num_non_physical_lights: u32,
}


struct RenderState {
    paths_rendered_per_pixel: u32,
    flags: u32,
}


// Global render settings
@group(UNIFORM_BIND_GROUP) @binding(RENDER_PARAMETERS_BINDING)
var<uniform> _render_parameters: RenderParameters;


@group(UNIFORM_BIND_GROUP) @binding(SCENE_PARAMETERS_BINDING)
var<uniform> _scene_parameters: SceneParameters;


@group(UNIFORM_BIND_GROUP) @binding(RENDER_STATE_BINDING)
var<uniform> _render_state: RenderState;
