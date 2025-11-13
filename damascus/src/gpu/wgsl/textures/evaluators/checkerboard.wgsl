// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

struct Checkerboard {
    flags: u32,
    inverse_transform: mat4x4f,
}

#ifdef EnableCheckerboard

fn checkerboard(seed: vec4f) -> f32 {
    var normalized_seed: vec3f = normalize(seed.xyz);
    var spherical_seed = vec2(
        atan2(normalized_seed.x, normalized_seed.z),
        acos(normalized_seed.y),
    ) * seed.w;
    var square_signal: vec2f = sign(fract(spherical_seed * 0.5) - 0.5);
    return 0.5 - 0.25 * square_signal.x * square_signal.y;
}

#endif
