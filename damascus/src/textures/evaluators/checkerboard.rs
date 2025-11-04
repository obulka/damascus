// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use glam::Mat4;

use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUCheckerboard {
    flags: u32,
    inverse_transform: Mat4,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Checkerboard {
    pub transform: Mat4,
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self {
            transform: Mat4::IDENTITY,
        }
    }
}

impl DualDevice<GPUCheckerboard, Std430GPUCheckerboard> for Checkerboard {
    fn to_gpu(&self) -> GPUCheckerboard {
        GPUCheckerboard {
            flags: 0,
            inverse_transform: self.transform.inverse(),
        }
    }
}
