// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use crevice::std430::AsStd430;
use glam::Vec3;

use crate::{renderers::AOVs, scene::Scene};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcher {
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: Vec3,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    // TODO variance & adaptive sampling
    output_aov: u32,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RayMarcher {
    pub scene: Scene,
    pub max_distance: f32,
    pub max_ray_steps: u32,
    pub max_bounces: u32,
    pub hit_tolerance: f32,
    pub shadow_bias: f32,
    pub max_brightness: f32,
    pub seeds: Vec3,
    pub dynamic_level_of_detail: bool,
    pub max_light_sampling_bounces: u32,
    pub sample_atmosphere: bool,
    pub light_sampling_bias: f32,
    pub secondary_sampling: bool,
    // TODO variance & adaptive sampling
    pub output_aov: AOVs,
    // TODO resolution
    pub latlong: bool,
}

impl Default for RayMarcher {
    fn default() -> Self {
        RayMarcher {
            scene: Scene::default(),
            max_distance: 100.,
            max_ray_steps: 1000,
            max_bounces: 1,
            hit_tolerance: 0.0001,
            shadow_bias: 1.,
            max_brightness: 999999999.9,
            seeds: Vec3::new(1., 2., 3.),
            dynamic_level_of_detail: true,
            max_light_sampling_bounces: 1,
            sample_atmosphere: false,
            light_sampling_bias: 1.,
            secondary_sampling: false,
            output_aov: AOVs::default(),
            latlong: false,
        }
    }
}

impl RayMarcher {
    fn to_gpu(&self) -> GPURayMarcher {
        GPURayMarcher {
            max_distance: self.max_distance.max(1e-8),
            max_ray_steps: self.max_ray_steps.max(1),
            max_bounces: self.max_bounces.max(1),
            hit_tolerance: self.hit_tolerance.max(0.),
            shadow_bias: self.shadow_bias,
            max_brightness: self.max_brightness,
            seeds: self.seeds,
            max_light_sampling_bounces: self.max_light_sampling_bounces,
            light_sampling_bias: self.light_sampling_bias,
            output_aov: self.output_aov as u32,
            flags: self.dynamic_level_of_detail as u32
                | (self.sample_atmosphere as u32) << 1
                | (self.secondary_sampling as u32) << 2
                | (self.latlong as u32) << 3,
        }
    }

    pub fn render_parameters(&self) -> Std430GPURayMarcher {
        self.to_gpu().as_std430()
    }

    pub fn reset_render_parameters(&mut self) {
        let default_ray_marcher = Self::default();

        self.max_distance = default_ray_marcher.max_distance;
        self.max_ray_steps = default_ray_marcher.max_ray_steps;
        self.max_bounces = default_ray_marcher.max_bounces;
        self.hit_tolerance = default_ray_marcher.hit_tolerance;
        self.shadow_bias = default_ray_marcher.shadow_bias;
        self.max_brightness = default_ray_marcher.max_brightness;
        self.seeds = default_ray_marcher.seeds;
        self.dynamic_level_of_detail = default_ray_marcher.dynamic_level_of_detail;
        self.max_light_sampling_bounces = default_ray_marcher.max_light_sampling_bounces;
        self.sample_atmosphere = default_ray_marcher.sample_atmosphere;
        self.light_sampling_bias = default_ray_marcher.light_sampling_bias;
        self.secondary_sampling = default_ray_marcher.secondary_sampling;
    }
}
