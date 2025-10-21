// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


// ------- Flag bit masks --------


struct RenderParameters {
    resolution: vec2f, // Resolution of the entire view area, not the image or its display size
    frame: u32,
    flags: u32,
}


struct RenderState {
    pan: vec2f,
    zoom: f32,
    flags: u32,
}


// Global render settings
@group(UNIFORM_BIND_GROUP) @binding(RENDER_PARAMETERS_BINDING)
var<uniform> _render_parameters: RenderParameters;

@group(UNIFORM_BIND_GROUP) @binding(RENDER_STATE_BINDING)
var<uniform> _render_state: RenderState;

@group(UNIFORM_BIND_GROUP) @binding(VIEWER_GRADE_BINDING)
var<uniform> _viewer_grade: Grade;

@group(TEXTURE_BIND_GROUP) @binding(TEXTURE_BINDING)
var _texture: texture_2d<f32>;
