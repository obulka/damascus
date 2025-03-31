// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include CompositorConstants
#include Math
#include Texture
#include CompositorRenderParameters


struct VertexInput {
    @location(0) uv_coordinate: vec2f,
}


struct VertexOutput {
    @location(0) texture_coordinate: vec4f,
    @builtin(position) ndc_coordinate: vec4f, // <[-1, 1], [-1, 1], [0, 1]>
}


@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var texture_uv: vec2f = vertex_input.uv_coordinate;

    var texture_dimensions = vec2f(textureDimensions(_texture));

    var out: VertexOutput;
    out.texture_coordinate = vec4(
        uv_to_screen(vec2f(texture_uv.x, -texture_uv.y), texture_dimensions),
        0.,
        1.,
    );

    texture_uv.y *=
        _render_state.resolution.x * texture_dimensions.y
        / (_render_state.resolution.y * texture_dimensions.x);

    out.ndc_coordinate = vec4(
        screen_to_uv(
            uv_to_screen(
                texture_uv,
                _render_state.resolution,
            ) + _render_state.pan,
            _render_state.resolution,
        ) / _render_state.zoom,
        0.,
        1.,
    );

    return out;
}
