// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::collections::HashSet;
use std::time::SystemTime;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2};
use strum::{Display, EnumIter, EnumString};

use super::shaders::{self, PreprocessorDirectives};

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

mod ray_marcher;
pub use ray_marcher::{RayMarcher, Std430GPURayMarcher};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURenderState {
    paths_rendered_per_pixel: f32,
    resolution: Vec2,
    flags: u32,
}

pub struct RenderState {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub paths_rendered_per_pixel: u32,
    pub resolution: UVec2,
    pub paused: bool,
    pub preprocessor_directives: HashSet<PreprocessorDirectives>,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            paths_rendered_per_pixel: 0,
            resolution: UVec2::ZERO,
            paused: true,
            preprocessor_directives: shaders::all_directives_for_primitive(),
        }
    }
}

impl RenderState {
    fn to_gpu(&self) -> GPURenderState {
        GPURenderState {
            paths_rendered_per_pixel: self.paths_rendered_per_pixel as f32,
            resolution: self.resolution.as_vec2(),
            flags: self.paused as u32,
        }
    }

    pub fn as_std_430(&self) -> Std430GPURenderState {
        self.to_gpu().as_std430()
    }
}
