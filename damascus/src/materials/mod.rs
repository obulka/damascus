// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

mod material;
mod procedural_texture;

pub use material::{GPUMaterial, Material, Std430GPUMaterial};
pub use procedural_texture::{GPUProceduralTexture, ProceduralTexture, ProceduralTextureType};
