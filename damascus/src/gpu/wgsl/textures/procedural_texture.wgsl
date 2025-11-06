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


@group(STORAGE_BIND_GROUP) @binding(CHECKERBOARD_BINDING)
var<storage, read> _checkerboards: array<Checkerboard>;


@group(STORAGE_BIND_GROUP) @binding(NOISE_BINDING)
var<storage, read> _noises: array<Noise>;


@group(STORAGE_BIND_GROUP) @binding(GRADE_BINDING)
var<storage, read> _grades: array<Grade>;

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
