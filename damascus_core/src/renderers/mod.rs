// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;

pub mod ray_marcher;

pub trait Renderer<G: Copy + Clone + AsStd430<Output = S>, S>:
    Default + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn to_gpu(&self) -> G;

    fn as_std_430(&self) -> S {
        self.to_gpu().as_std430()
    }
}
