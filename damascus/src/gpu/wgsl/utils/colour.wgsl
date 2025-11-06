// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

const RGB_TO_YIQ: mat3x3f = mat3x3f(
    vec3f(0.299, 0.596, 0.211),
    vec3f(0.587, -0.274, -0.523),
    vec3f(0.114, -0.321, 0.311),
);
const YIQ_TO_RGB: mat3x3f = mat3x3f(
    vec3f(1., 1., 1.),
    vec3f(0.956, -0.272, -1.107),
    vec3f(0.621, -0.647, 1.705),
);
const RGBA_TO_YIQA: mat4x4f = mat4x4f(
    vec4f(0.299, 0.596, 0.211, 0.),
    vec4f(0.587, -0.274, -0.523, 0.),
    vec4f(0.114, -0.321, 0.311, 0.),
    vec4f(0., 0., 0., 1.),
);
const YIQA_TO_RGBA: mat4x4f = mat4x4f(
    vec4f(1., 1., 1., 0.),
    vec4f(0.956, -0.272, -1.107, 0.),
    vec4f(0.621, -0.647, 1.705, 0.),
    vec4f(0., 0., 0., 1.),
);

fn transform_colour3f(
    transform: mat4x4f,
    colour: vec3f,
) -> vec3f {
    return abs(YIQ_TO_RGB * (transform * vec4f(RGB_TO_YIQ * colour, 1.0)).xyz);
}

fn transform_colour4f(
    transform: mat4x4f,
    colour: vec4f,
) -> vec4f {
    return abs(YIQA_TO_RGBA * (transform * (RGBA_TO_YIQA * colour)));
}
