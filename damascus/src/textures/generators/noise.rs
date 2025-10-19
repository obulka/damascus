// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;

use crate::DualDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUGrade {
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    gamma: f32,
    flags: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Grade {
    pub black_point: f32,
    pub white_point: f32,
    pub lift: f32,
    pub gain: f32,
    pub gamma: f32,
    pub invert: bool,
}

impl Default for Grade {
    fn default() -> Self {
        Self {
            black_point: 0.,
            white_point: 1.,
            lift: 0.,
            gain: 1.,
            gamma: 1.,
            invert: false,
        }
    }
}

impl Grade {
    pub fn black_point(mut self, black_point: f32) -> Self {
        self.black_point = black_point;
        self
    }

    pub fn white_point(mut self, white_point: f32) -> Self {
        self.white_point = white_point;
        self
    }

    pub fn lift(mut self, lift: f32) -> Self {
        self.lift = lift;
        self
    }

    pub fn gain(mut self, gain: f32) -> Self {
        self.gain = gain;
        self
    }

    pub fn gamma(mut self, gamma: f32) -> Self {
        self.gamma = gamma;
        self
    }
}

impl DualDevice<GPUGrade, Std430GPUGrade> for Grade {
    fn to_gpu(&self) -> GPUGrade {
        GPUGrade {
            black_point: self.black_point,
            white_point: self.white_point,
            lift: self.lift,
            gain: self.gain,
            gamma: 1. / self.gamma,
            flags: self.invert as u32,
        }
    }
}
