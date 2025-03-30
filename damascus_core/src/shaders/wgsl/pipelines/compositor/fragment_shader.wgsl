// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include CompositorConstants
#include Math
#include Random
#include Texture
#include CompositorRenderParameters


struct FragmentInput {
    @location(0) uv_coordinate: vec4f,
    @builtin(position) frag_coordinate: vec4f, // pixel centers
}


@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var texture_dimensions = vec2f(textureDimensions(_texture));
    var current_pixel_indices: vec2f = uv_to_screen(
        in.uv_coordinate.xy,
        texture_dimensions,
    );// * _render_state.zoom - _render_state.pan;

    var pixel_colour = textureLoad(_texture, vec2u(current_pixel_indices), 0);

    // return grade_vec4(pixel_colour, _viewer_grade);
    return in.uv_coordinate;
}
