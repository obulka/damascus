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
    @location(0) texture_coordinate: vec4f,
    @builtin(position) frag_coordinate: vec4f, // pixel centers
}


@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4f {
    var pixel_colour = textureLoad(
        _texture,
        vec2u(in.texture_coordinate.xy),
        0,
    );

    return grade_vec4(pixel_colour, _viewer_grade);
}
