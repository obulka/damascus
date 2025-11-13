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

fn evaluate_texture_f32(
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

fn evaluate_texture_vec3f(
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
