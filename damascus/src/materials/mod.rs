// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use slotmap::SlotMap;

mod material;

pub use material::{GPUMaterial, Material, Std430GPUMaterial};

slotmap::new_key_type! { pub struct MaterialId; }

pub type Materials = SlotMap<MaterialId, Material>;
