// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include RayMarcherConstants


struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
}


struct VertexOutput {
    @location(TEXTURE_UV_LOCATION) uv_coordinate: vec4f,
    @builtin(position) ndc_coordinate: vec4f, // <[-1, 1], [-1, 1], [0, 1]>
}


struct VertexData {
    uv_coordinate: vec2f,
}


@group(VERTEX_BIND_GROUP) @binding(VERTEX_DATA_BINDING)
var<storage, read> _vertex_data: array<VertexData>;


@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var uv_coordinate: vec2f = _vertex_data[vertex_input.vertex_index].uv_coordinate;

    var out: VertexOutput;
    out.ndc_coordinate = vec4(uv_coordinate, 0., 1.);
    out.uv_coordinate = vec4(uv_coordinate, 0., 1.);

    return out;
}
