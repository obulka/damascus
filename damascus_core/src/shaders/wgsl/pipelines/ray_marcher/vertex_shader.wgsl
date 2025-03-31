// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


struct VertexInput {
    @location(0) vertex_coordinate: vec2f,
}


struct VertexOutput {
    @location(0) uv_coordinate: vec4f,
    @builtin(position) ndc_coordinate: vec4f, // <[-1, 1], [-1, 1], [0, 1]>
}


@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.ndc_coordinate = vec4(vertex_input.vertex_coordinate, 0., 1.);
    out.uv_coordinate = vec4(vertex_input.vertex_coordinate, 0., 1.);

    return out;
}
