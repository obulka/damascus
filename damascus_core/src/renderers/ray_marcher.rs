// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::time::SystemTime;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2, Vec3};
use strum::{Display, EnumIter, EnumString};

use super::Renderer;

use crate::{scene::Scene, DualDevice};

#[derive(
    Debug,
    Default,
    Display,
    Copy,
    Clone,
    EnumIter,
    EnumString,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum AOVs {
    #[default]
    Beauty,
    WorldPosition,
    LocalPosition,
    Normals,
    Depth,
    Cryptomatte,
    Stats,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcherRenderState {
    paths_rendered_per_pixel: f32,
    resolution: Vec2,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherRenderState {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub paths_rendered_per_pixel: u32,
    pub resolution: UVec2,
    pub paused: bool,
}

impl Default for RayMarcherRenderState {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            paths_rendered_per_pixel: 0,
            resolution: UVec2::ZERO,
            paused: true,
        }
    }
}

impl RayMarcherRenderState {}

impl DualDevice<GPURayMarcherRenderState, Std430GPURayMarcherRenderState>
    for RayMarcherRenderState
{
    fn to_gpu(&self) -> GPURayMarcherRenderState {
        GPURayMarcherRenderState {
            paths_rendered_per_pixel: self.paths_rendered_per_pixel as f32,
            resolution: self.resolution.as_vec2(),
            flags: self.paused as u32,
        }
    }
}

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
    equiangular_samples: u32,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    output_aov: u32,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub equiangular_samples: u32,
    pub max_light_sampling_bounces: u32,
    pub sample_atmosphere: bool,
    pub light_sampling_bias: f32,
    pub secondary_sampling: bool,
    pub output_aov: AOVs,
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
            seeds: Vec3::new(1111., 2222., 3333.),
            dynamic_level_of_detail: true,
            equiangular_samples: 0,
            max_light_sampling_bounces: 1,
            sample_atmosphere: false,
            light_sampling_bias: 0.,
            secondary_sampling: false,
            output_aov: AOVs::default(),
        }
    }
}

impl RayMarcher {
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
        self.equiangular_samples = default_ray_marcher.equiangular_samples;
        self.max_light_sampling_bounces = default_ray_marcher.max_light_sampling_bounces;
        self.sample_atmosphere = default_ray_marcher.sample_atmosphere;
        self.light_sampling_bias = default_ray_marcher.light_sampling_bias;
        self.secondary_sampling = default_ray_marcher.secondary_sampling;
    }
}

impl DualDevice<GPURayMarcher, Std430GPURayMarcher> for RayMarcher {
    fn to_gpu(&self) -> GPURayMarcher {
        GPURayMarcher {
            max_distance: self.max_distance.max(1e-8),
            max_ray_steps: self.max_ray_steps.max(1),
            max_bounces: self.max_bounces.max(1),
            hit_tolerance: self.hit_tolerance.max(0.),
            shadow_bias: self.shadow_bias,
            max_brightness: self.max_brightness,
            seeds: self.seeds,
            equiangular_samples: self.equiangular_samples,
            max_light_sampling_bounces: self.max_light_sampling_bounces,
            light_sampling_bias: self.light_sampling_bias * self.light_sampling_bias,
            output_aov: self.output_aov as u32,
            flags: self.dynamic_level_of_detail as u32
                | (self.sample_atmosphere as u32) << 1
                | (self.secondary_sampling as u32) << 2,
        }
    }
}

impl Renderer<GPURayMarcher, Std430GPURayMarcher> for RayMarcher {}
