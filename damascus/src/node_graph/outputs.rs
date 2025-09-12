// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

slotmap::new_key_type! { pub struct OutputId; }

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Output {
    pub id: OutputId,
    pub node: NodeId,
    pub data: OutputData,
}

pub type Outputs = SlotMap<OutputId, Output>;

#[derive(Clone, PartialEq, Debug, Eq, serde::Serialize, serde::Deserialize)]
pub enum OutputData {
    Mat4,
    Camera,
    Light,
    Material,
    Primitive,
    ProceduralTexture,
    RenderPass,
    Scene,
}
