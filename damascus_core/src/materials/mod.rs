// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

mod material;
mod procedural_texture;

pub use material::{GPUMaterial, Material, Std430GPUMaterial};
pub use procedural_texture::{GPUProceduralTexture, ProceduralTexture, ProceduralTextureType};
