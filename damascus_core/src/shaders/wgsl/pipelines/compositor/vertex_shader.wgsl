// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include CompositorConstants
#include Math
#include Texture
#include CompositorRenderParameters


struct VertexOutput {
    @location(0) uv_coordinate: vec4f,
    @builtin(position) ndc_coordinate: vec4f, // <[-1, 1], [-1, 1], [0, 1]>
}


var<private> v_positions: array<vec2f, 4> = array<vec2f, 4>(
    vec2f(1., 1.),
    vec2f(-1., 1.),
    vec2f(1., -1.),
    vec2f(-1., -1.),
);


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var texture_dimensions = vec2f(textureDimensions(_texture));
    var out: VertexOutput;
    out.uv_coordinate = vec4(v_positions[vertex_index], 0., 1.);
    out.ndc_coordinate = vec4(
        screen_to_uv(
            uv_to_screen(
                v_positions[vertex_index],
                _render_state.resolution,
            ) + _render_state.pan,
            _render_state.resolution,
        ) / _render_state.zoom,
        0.,
        1.,
    );

    return out;
}
