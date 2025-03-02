// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![allow(long_running_const_eval)]

use crevice::std430::AsStd430;

pub mod geometry;
pub mod lights;
pub mod materials;
pub mod renderers;
pub mod scene;
pub mod shaders;
pub mod textures;

pub trait DualDevice<G: Copy + Clone + AsStd430<Output = S>, S>:
    Default + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn to_gpu(&self) -> G;

    fn as_std430(&self) -> S {
        self.to_gpu().as_std430()
    }
}

pub trait Settings: Copy + Clone + serde::Serialize + for<'a> serde::Deserialize<'a> {}
