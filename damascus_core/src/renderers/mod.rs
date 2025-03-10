// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;

pub mod compositor;
pub mod ray_marcher;

use super::DualDevice;

pub trait Renderer<G: Copy + Clone + AsStd430<Output = S>, S>: DualDevice<G, S> {}
