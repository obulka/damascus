// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#include TextureViewerConstants
#include Math
#include Random
#include Texture
#include TextureViewerRenderParameters


struct FragmentInput {
    // Pixel index to read from the texture
    @location(TEXTURE_COORDINATE_LOCATION) texture_coordinate: vec4f,
    // Pixel centers in pixel space ie. [0.5, 0.5], [0.5, 1.5], ...
    @builtin(position) frag_coordinate: vec4f,
}


@fragment
fn fs_main(in: FragmentInput) -> @location(PIXEL_COLOUR_LOCATION) vec4f {
    var pixel_colour = textureLoad(
        _texture,
        vec2u(in.texture_coordinate.xy),
        0,
    );

    return grade_vec4(pixel_colour, _viewer_grade);
}
