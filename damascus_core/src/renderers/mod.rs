// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

use std::time::SystemTime;

use crevice::std430::AsStd430;
use strum::{Display, EnumIter, EnumString};

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
pub struct GPURenderStats {
    paths_rendered_per_pixel: f32,
}

pub struct RenderStats {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub paths_rendered_per_pixel: u32,
}

impl Default for RenderStats {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 60.,
            paths_rendered_per_pixel: 0,
        }
    }
}

impl RenderStats {
    fn to_gpu(&self) -> GPURenderStats {
        GPURenderStats {
            paths_rendered_per_pixel: self.paths_rendered_per_pixel as f32,
        }
    }

    pub fn as_std_430(&self) -> Std430GPURenderStats {
        self.to_gpu().as_std430()
    }
}
