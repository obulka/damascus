// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


const NONE: u32 = 0u;
const WHITE: u32 = 1u;
const BLACK: u32 = 2u;
const CHECKERBOARD: u32 = 3u;
const CONSTANT: u32 = 4u;
const GRADE: u32 = 5u;
const NOISE: u32 = 6u;

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
    vec4f(0.299, 0.596, 0.211), 0.0,
    vec4f(0.587, -0.274, -0.523, 0.0),
    vec4f(0.114, -0.321, 0.311, 0.0),
    vec4f(0.0, 0.0, 0.0, 1.0),
);
const YIQA_TO_RGBA: mat4x4f = mat4x4f(
    vec4f(1., 1., 1., 0.0),
    vec4f(0.956, -0.272, -1.107, 0.0),
    vec4f(0.621, -0.647, 1.705, 0.0),
    vec4f(0.0, 0.0, 0.0, 1.0),
);


@group(STORAGE_BIND_GROUP) @binding(CHECKERBOARD_BINDING)
var<storage, read> _checkerboards: array<Checkerboard>;


@group(STORAGE_BIND_GROUP) @binding(NOISE_BINDING)
var<storage, read> _noises: array<Noise>;


@group(STORAGE_BIND_GROUP) @binding(GRADE_BINDING)
var<storage, read> _grades: array<Grade>;


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

fn procedurally_texture_f32(
    seed: vec4f,
    colour: f32,
    texture_type_and_index: vec2u,
) -> f32 {
    switch texture_type_and_index.x {
        case NONE, default {
            return colour;
        }
#ifdef EnableGrade
        case GRADE {
            return grade_f32(colour, _grades[texture_type_and_index.y]);
        }
#endif
#ifdef EnableCheckerboard
        case CHECKERBOARD {
            return colour * checkerboard(_checkerboards[texture_type_and_index.y].inverse_transform * seed);
        }
#endif
// Simply having this case slows things down, so allow it to be compiled out
#ifdef EnableNoise
        case NOISE {
            var noise: Noise = _noises[texture_type_and_index.y];
            return colour * octave_noise(noise.inverse_transform * seed, noise);
        }
#endif
    }
}


fn procedurally_texture_vec3f(
    seed: vec4f,
    colour: vec3f,
    texture_type_and_index: vec2u,
) -> vec3f {
    switch texture_type_and_index.x {
        case NONE, default {
            return colour;
        }
#ifdef EnableGrade
        case GRADE {
            return grade_vec3(colour, _grades[texture_type_and_index.y]);
        }
#endif
#ifdef EnableCheckerboard
        case CHECKERBOARD {
            return colour * vec3(
                checkerboard(_checkerboards[texture_type_and_index.y].inverse_transform * seed),
            );
        }
#endif
// Simply having this case slows things down, so allow it to be compiled out
#ifdef EnableNoise
        case NOISE {
            var noise: Noise = _noises[texture_type_and_index.y];
            return colour * vec3(octave_noise(noise.inverse_transform * seed, noise));
        }
#endif
    }
}


fn sample_equiangular(
    distance_since_last_bounce: f32,
    ray: ptr<function, Ray>,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) {
    // Get the material properties of the dielectric the ray is currently in
    var current_dielectric: Dielectric = peek_dielectric(nested_dielectrics);

    // If equiangular sampling is disabled or the dielectric does not scatter
    // light, compute the extinction and exit early
    if (
        _render_parameters.equiangular_samples == 0u
        || element_sum_vec3f(current_dielectric.scattering_colour) == 0.
    ) {
        (*ray).throughput *= exp(
            -current_dielectric.extinction_colour * distance_since_last_bounce,
        );
        return;
    }
}
