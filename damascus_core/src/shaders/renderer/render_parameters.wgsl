// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


// Flag bit masks
const DYNAMIC_LEVEL_OF_DETAIL: u32 = 1u;
const SAMPLE_ATMOSPHERE: u32 = 2u;
const SECONDARY_SAMPLING: u32 = 4u;
const LATLONG: u32 = 8u;


struct RenderParameters {
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: vec3f,
    equiangular_samples: u32,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    output_aov: u32,
    flags: u32,
}


struct SceneParameters {
    num_primitives: u32,
    num_lights: u32,
    num_non_physical_lights: u32,
}


struct RenderStats {
    paths_rendered_per_pixel: f32,
}


// Global render settings
@group(0) @binding(0)
var<uniform> _render_parameters: RenderParameters;


@group(0) @binding(1)
var<uniform> _scene_parameters: SceneParameters;


@group(0) @binding(2)
var<uniform> _render_stats: RenderStats;
