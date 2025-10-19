// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::Vec2;

use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUVertex {
    uv_coordinate: Vec2,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Vertex {
    pub uv_coordinate: Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            uv_coordinate: Vec2::ZERO,
        }
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            uv_coordinate: Vec2::new(x, y),
        }
    }
}

impl DualDevice<GPUVertex, Std430GPUVertex> for Vertex {
    fn to_gpu(&self) -> GPUVertex {
        GPUVertex {
            uv_coordinate: self.uv_coordinate,
        }
    }
}

impl Vertex {
    pub fn quad_corner_vertices_2d() -> Vec<Std430GPUVertex> {
        vec![
            Self::new(1., 1.).as_std430(),
            Self::new(-1., 1.).as_std430(),
            Self::new(1., -1.).as_std430(),
            Self::new(-1., -1.).as_std430(),
        ]
    }

    pub fn quad_corner_indices_2d() -> Vec<u32> {
        vec![0, 1, 2, 3]
    }
}
