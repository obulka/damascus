// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


struct VertexOut {
    @location(0) uv_coordinate: vec4f,
    @builtin(position) frag_coordinate: vec4f,
}


var<private> v_positions: array<vec2f, 4> = array<vec2f, 4>(
    vec2f(1., 1.),
    vec2f(-1., 1.),
    vec2f(1., -1.),
    vec2f(-1., -1.),
);


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var out: VertexOut;
    out.frag_coordinate = vec4(v_positions[vertex_index], 0., 1.);
    out.uv_coordinate = vec4(v_positions[vertex_index], 0., 1.);

    return out;
}
