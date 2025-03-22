// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


// ------- Flag bit masks --------


struct RenderParameters {
    flags: u32,
}


struct RenderState {
    resolution: vec2f,
    pan: vec2f,
    zoom: f32,
    flags: u32,
}


// Global render settings
@group(UNIFORM_BIND_GROUP) @binding(0)
var<uniform> _render_parameters: RenderParameters;


@group(UNIFORM_BIND_GROUP) @binding(1)
var<uniform> _render_state: RenderState;
