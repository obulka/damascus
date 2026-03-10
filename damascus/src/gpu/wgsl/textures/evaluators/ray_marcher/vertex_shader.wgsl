// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include RayMarcherConstants
#include Texture
#include RayMarcherBindings

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
}

struct VertexOutput {
    // Pass the pixel indices to the fragment shader
    @location(TEXTURE_COORDINATE_LOCATION) texture_coordinate: vec4f,
    // NDC location of the vertex <[-1, 1], [-1, 1], [0, 1], (0, inf)>
    @builtin(position) ndc_coordinate: vec4f,
}

struct VertexData {
    uv_coordinate: vec2f,
}

@group(VERTEX_BIND_GROUP) @binding(VERTEX_DATA_BINDING)
var<storage, read> _vertex_data: array<VertexData>;

@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var uv_coordinate: vec2f = _vertex_data[vertex_input.vertex_index].uv_coordinate;

    var texture_dimensions = vec2f(textureDimensions(_progressive_rendering_texture));

    var out: VertexOutput;
    out.texture_coordinate = vec4(
        uv_to_pixels(vec2f(uv_coordinate.x, -uv_coordinate.y), texture_dimensions),
        0.,
        1.,
    );

    out.ndc_coordinate = vec4(uv_coordinate, 0., 1.);

    return out;
}
