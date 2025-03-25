// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

const UNIFORM_BIND_GROUP: u32 = 0u;
const TEXTURE_BIND_GROUP: u32 = 1u;

#include Math
#include Random
#include Texture
#include CompositorRenderParameters
#include VertexShader


@group(TEXTURE_BIND_GROUP) @binding(0)
var _texture: texture_2d<f32>;


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var texture_dimensions: vec2u = textureDimensions(_texture);
    var current_pixel_indices: vec2f = uv_to_screen(
        in.uv_coordinate.xy,
        _render_state.resolution,
    ) * _render_state.zoom - _render_state.pan;

    var pixel_colour = vec4f(0.);

    if (
        current_pixel_indices.x >= 0.
        && current_pixel_indices.y >= 0.
        && current_pixel_indices.x < f32(texture_dimensions.x)
        && current_pixel_indices.y < f32(texture_dimensions.y)
    ) {
        pixel_colour = textureLoad(_texture, vec2u(current_pixel_indices), 0);
    }

    return grade_vec4(pixel_colour, _viewer_grade);
}
