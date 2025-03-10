// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::time::SystemTime;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2};

use super::Renderer;

use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUCompositorRenderState {
    resolution: Vec2,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CompositorRenderState {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub resolution: UVec2,
    pub paused: bool,
}

impl Default for CompositorRenderState {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            resolution: UVec2::ZERO,
            paused: true,
        }
    }
}

impl CompositorRenderState {}

impl DualDevice<GPUCompositorRenderState, Std430GPUCompositorRenderState>
    for CompositorRenderState
{
    fn to_gpu(&self) -> GPUCompositorRenderState {
        GPUCompositorRenderState {
            resolution: self.resolution.as_vec2(),
            flags: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUCompositor {
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Compositor {}

impl Default for Compositor {
    fn default() -> Self {
        Compositor {}
    }
}

impl Compositor {}

impl DualDevice<GPUCompositor, Std430GPUCompositor> for Compositor {
    fn to_gpu(&self) -> GPUCompositor {
        GPUCompositor { flags: 0 }
    }
}

impl Renderer<GPUCompositor, Std430GPUCompositor> for Compositor {}
