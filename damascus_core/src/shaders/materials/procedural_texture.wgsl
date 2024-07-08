// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


const NONE: u32 = 0u;
const GRADE: u32 = 1u;
const CHECKER_BOARD: u32 = 2u;
const FBM_NOISE: u32 = 3u;
const TURBULENCE_NOISE: u32 = 4u;

const INVERT: u32 = 1u;
const USE_TRAP_COLOUR: u32 = 2u;

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


#ifdef EnableNoise
const G4: f32 = 0.138196601;


var<private> PERM: array<u32, 512> = array<u32, 512>(
    151u, 160u, 137u, 91u, 90u, 15u, 131u, 13u, 201u, 95u, 96u, 53u, 194u, 233u,
    7u, 225u, 140u, 36u, 103u, 30u, 69u, 142u, 8u, 99u, 37u, 240u, 21u, 10u, 23u,
    190u, 6u, 148u, 247u, 120u, 234u, 75u, 0u, 26u, 197u, 62u, 94u, 252u, 219u,
    203u, 117u, 35u, 11u, 32u, 57u, 177u, 33u, 88u, 237u, 149u, 56u, 87u, 174u,
    20u, 125u, 136u, 171u, 168u, 68u, 175u, 74u, 165u, 71u, 134u, 139u, 48u,
    27u, 166u, 77u, 146u, 158u, 231u, 83u, 111u, 229u, 122u, 60u, 211u, 133u,
    230u, 220u, 105u, 92u, 41u, 55u, 46u, 245u, 40u, 244u, 102u, 143u, 54u, 65u,
    25u, 63u, 161u, 1u, 216u, 80u, 73u, 209u, 76u, 132u, 187u, 208u, 89u, 18u,
    169u, 200u, 196u, 135u, 130u, 116u, 188u, 159u, 86u, 164u, 100u, 109u, 198u,
    173u, 186u, 3u, 64u, 52u, 217u, 226u, 250u, 124u, 123u, 5u, 202u, 38u, 147u,
    118u, 126u, 255u, 82u, 85u, 212u, 207u, 206u, 59u, 227u, 47u, 16u, 58u, 17u,
    182u, 189u, 28u, 42u, 223u, 183u, 170u, 213u, 119u, 248u, 152u, 2u, 44u,
    154u, 163u, 70u, 221u, 153u, 101u, 155u, 167u, 43u, 172u, 9u, 129u, 22u,
    39u, 253u, 19u, 98u, 108u, 110u, 79u, 113u, 224u, 232u, 178u, 185u, 112u,
    104u, 218u, 246u, 97u, 228u, 251u, 34u, 242u, 193u, 238u, 210u, 144u, 12u,
    191u, 179u, 162u, 241u, 81u, 51u, 145u, 235u, 249u, 14u, 239u, 107u, 49u,
    192u, 214u, 31u, 181u, 199u, 106u, 157u, 184u, 84u, 204u, 176u, 115u, 121u,
    50u, 45u, 127u, 4u, 150u, 254u, 138u, 236u, 205u, 93u, 222u, 114u, 67u, 29u,
    24u, 72u, 243u, 141u, 128u, 195u, 78u, 66u, 215u, 61u, 156u, 180u,
    151u, 160u, 137u, 91u, 90u, 15u, 131u, 13u, 201u, 95u, 96u, 53u, 194u, 233u,
    7u, 225u, 140u, 36u, 103u, 30u, 69u, 142u, 8u, 99u, 37u, 240u, 21u, 10u, 23u,
    190u, 6u, 148u, 247u, 120u, 234u, 75u, 0u, 26u, 197u, 62u, 94u, 252u, 219u,
    203u, 117u, 35u, 11u, 32u, 57u, 177u, 33u, 88u, 237u, 149u, 56u, 87u, 174u,
    20u, 125u, 136u, 171u, 168u, 68u, 175u, 74u, 165u, 71u, 134u, 139u, 48u,
    27u, 166u, 77u, 146u, 158u, 231u, 83u, 111u, 229u, 122u, 60u, 211u, 133u,
    230u, 220u, 105u, 92u, 41u, 55u, 46u, 245u, 40u, 244u, 102u, 143u, 54u, 65u,
    25u, 63u, 161u, 1u, 216u, 80u, 73u, 209u, 76u, 132u, 187u, 208u, 89u, 18u,
    169u, 200u, 196u, 135u, 130u, 116u, 188u, 159u, 86u, 164u, 100u, 109u, 198u,
    173u, 186u, 3u, 64u, 52u, 217u, 226u, 250u, 124u, 123u, 5u, 202u, 38u, 147u,
    118u, 126u, 255u, 82u, 85u, 212u, 207u, 206u, 59u, 227u, 47u, 16u, 58u, 17u,
    182u, 189u, 28u, 42u, 223u, 183u, 170u, 213u, 119u, 248u, 152u, 2u, 44u,
    154u, 163u, 70u, 221u, 153u, 101u, 155u, 167u, 43u, 172u, 9u, 129u, 22u,
    39u, 253u, 19u, 98u, 108u, 110u, 79u, 113u, 224u, 232u, 178u, 185u, 112u,
    104u, 218u, 246u, 97u, 228u, 251u, 34u, 242u, 193u, 238u, 210u, 144u, 12u,
    191u, 179u, 162u, 241u, 81u, 51u, 145u, 235u, 249u, 14u, 239u, 107u, 49u,
    192u, 214u, 31u, 181u, 199u, 106u, 157u, 184u, 84u, 204u, 176u, 115u, 121u,
    50u, 45u, 127u, 4u, 150u, 254u, 138u, 236u, 205u, 93u, 222u, 114u, 67u, 29u,
    24u, 72u, 243u, 141u, 128u, 195u, 78u, 66u, 215u, 61u, 156u, 180u,
);


var<private> GRAD4: array<vec4f, 32> = array<vec4f, 32>(
    vec4(0., 1., 1., 1.), vec4(0., 1., 1., -1.),
    vec4(0., 1., -1., 1.), vec4(0., 1., -1., -1.),
    vec4(0., -1., 1., 1.), vec4(0., -1., 1., -1.),
    vec4(0., -1., -1., 1.), vec4(0., -1., -1., -1.),
    vec4(1., 0., 1., 1.), vec4(1., 0., 1., -1.),
    vec4(1., 0., -1., 1.), vec4(1., 0., -1., -1.),
    vec4(-1., 0., 1., 1.), vec4(-1., 0., 1., -1.),
    vec4(-1., 0., -1., 1.), vec4(-1., 0., -1., -1.),
    vec4(1., 1., 0., 1.), vec4(1., 1., 0., -1.),
    vec4(1., -1., 0., 1.), vec4(1., -1., 0., -1.),
    vec4(-1., 1., 0., 1.), vec4(-1., 1., 0., -1.),
    vec4(-1., -1., 0., 1.), vec4(-1., -1., 0., -1.),
    vec4(1., 1., 1., 0.), vec4(1., 1., -1., 0.),
    vec4(1., -1., 1., 0.), vec4(1., -1., -1., 0.),
    vec4(-1., 1., 1., 0.), vec4(-1., 1., -1., 0.),
    vec4(-1., -1., 1., 0.), vec4(-1., -1., -1., 0.),
);


var<private> SIMPLEX: array<vec4u, 64> = array<vec4u, 64>(
    vec4(0u, 1u, 2u, 3u), vec4(0u, 1u, 3u, 2u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 2u, 3u, 1u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(1u, 2u, 3u, 0u),
    vec4(0u, 2u, 1u, 3u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 3u, 1u, 2u), vec4(0u, 3u, 2u, 1u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(1u, 3u, 2u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(1u, 2u, 0u, 3u), vec4(0u, 0u, 0u, 0u),
    vec4(1u, 3u, 0u, 2u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(2u, 3u, 0u, 1u), vec4(2u, 3u, 1u, 0u),
    vec4(1u, 0u, 2u, 3u), vec4(1u, 0u, 3u, 2u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(2u, 0u, 3u, 1u),
    vec4(0u, 0u, 0u, 0u), vec4(2u, 1u, 3u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(2u, 0u, 1u, 3u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(3u, 0u, 1u, 2u), vec4(3u, 0u, 2u, 1u),
    vec4(0u, 0u, 0u, 0u), vec4(3u, 1u, 2u, 0u),
    vec4(2u, 1u, 0u, 3u), vec4(0u, 0u, 0u, 0u),
    vec4(0u, 0u, 0u, 0u), vec4(0u, 0u, 0u, 0u),
    vec4(3u, 1u, 0u, 2u), vec4(0u, 0u, 0u, 0u),
    vec4(3u, 2u, 0u, 1u), vec4(3u, 2u, 1u, 0u),
);


/**
 * 4D Perlin simplex noise
 *
 * Adapted from https://github.com/BogdanDenis/OpenGL/blob/master/simplexnoise.cpp
 *
 * Copyright (c) 2007-2012 Eliot Eshelman
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as publ  by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * @arg seed: The seed for the noise.
 *
 * @returns: Noise value in the range [-1, 1], value of 0 on all integer
 *     coordinates.
 */
fn perlin_simplex_noise(seed: vec4f) -> f32 {
    var i: vec4f = floor(seed + element_sum_vec4f(seed) * 0.309016994);
    var x0: vec4f = seed - i + element_sum_vec4f(i) * G4;

    var c: u32 = (
        (u32(x0.x > x0.y) << 5u)
        | (u32(x0.x > x0.z) << 4u)
        | (u32(x0.y > x0.z) << 3u)
        | (u32(x0.x > x0.w) << 2u)
        | (u32(x0.y > x0.w) << 1u)
        | u32(x0.z > x0.w)
    );
    var i1 = vec4u(SIMPLEX[c] >= vec4(3u));
    var i2 = vec4u(SIMPLEX[c] >= vec4(2u));
    var i3 = vec4u(SIMPLEX[c] >= vec4(1u));

    var x1: vec4f = x0 - vec4f(i1) + G4;
    var x2: vec4f = x0 - vec4f(i2) + 2. * G4;
    var x3: vec4f = x0 - vec4f(i3) + 3. * G4;
    var x4: vec4f = x0 - 1. + 4. * G4;

    var ii = vec4u(vec4i(i) & vec4i(255));

    var gi0: u32 = PERM[ii.x + PERM[ii.y + PERM[ii.z + PERM[ii.w]]]] % 32u;
    var gi1: u32 = PERM[
        ii.x + i1.x + PERM[ii.y + i1.y + PERM[ii.z + i1.z + PERM[ii.w + i1.w]]]
    ] % 32u;
    var gi2: u32 = PERM[
        ii.x + i2.x + PERM[ii.y + i2.y + PERM[ii.z + i2.z + PERM[ii.w + i2.w]]]
    ] % 32u;
    var gi3: u32 = PERM[
        ii.x + i3.x + PERM[ii.y + i3.y + PERM[ii.z + i3.z + PERM[ii.w + i3.w]]]
    ] % 32u;
    var gi4: u32 = PERM[ii.x + 1u + PERM[ii.y + 1u + PERM[ii.z + 1u + PERM[ii.w + 1u]]]] % 32u;

    var n: f32 = 0.;

    var t0: f32 = positive_part_f32(0.6 - dot(x0, x0));
    t0 *= t0;
    n += t0 * t0 * dot(GRAD4[gi0], x0);

    var t1: f32 = positive_part_f32(0.6 - dot(x1, x1));
    t1 *= t1;
    n += t1 * t1 * dot(GRAD4[gi1], x1);

    var t2: f32 = positive_part_f32(0.6 - dot(x2, x2));
    t2 *= t2;
    n += t2 * t2 * dot(GRAD4[gi2], x2);

    var t3: f32 = positive_part_f32(0.6 - dot(x3, x3));
    t3 *= t3;
    n += t3 * t3 * dot(GRAD4[gi3], x3);

    var t4: f32 = positive_part_f32(0.6 - dot(x4, x4));
    t4 *= t4;
    n += t4 * t4 * dot(GRAD4[gi4], x4);

    return 27. * n;
}


/**
 * Octave noise.
 *
 * @arg seed: The noise seed.
 * @arg texture: The texture properties.
 * @arg turbulence: Use the absolute value of the simplex noise if true.
 *
 * @returns: The noise value in the range [-1, 1].
 */
fn octave_noise(
    seed: vec4f,
    texture: ProceduralTexture,
    turbulence: bool,
) -> f32 {
    var output: f32 = 0.;
    var frequency: f32 = texture.lacunarity;
    var amplitude: f32 = 1.;
    var max_amplitude: f32 = 0.;
    var translation: vec4f;
    var scale: vec4f;

    for (var octave=0u; octave < texture.octaves; octave++) {
        var octave_fraction = f32(octave) / f32(texture.octaves);
        scale = (
            (texture.high_frequency_scale * octave_fraction)
            + (texture.low_frequency_scale * (1. - octave_fraction))
        );
        translation = (
            (texture.high_frequency_translation * octave_fraction)
            + (texture.low_frequency_translation * (1. - octave_fraction))
        );

        var simplex_noise: f32 = amplitude * perlin_simplex_noise(
            (seed / scale - translation) * frequency / texture.scale,
        );
        output += select(simplex_noise, abs(simplex_noise), turbulence);

        frequency *= texture.lacunarity;
        max_amplitude += amplitude;
        amplitude *= texture.amplitude_gain;
    }

    return abs(output / max_amplitude);
}
#endif


struct ProceduralTexture {
    texture_type: u32,
    scale: vec4f,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    octaves: u32,
    lacunarity: f32,
    amplitude_gain: f32,
    gamma: f32,
    low_frequency_scale: vec4f,
    high_frequency_scale: vec4f,
    low_frequency_translation: vec4f,
    high_frequency_translation: vec4f,
    hue_rotation: mat3x3f,
    flags: u32,
}



fn trap_texture(
    trap_colour: vec3f,
    current_colour: vec3f,
    texture: ProceduralTexture,
) -> vec3f {
    if !bool(texture.flags & USE_TRAP_COLOUR) {
        return current_colour;
    }
    return abs(YIQ_TO_RGB * (
        texture.hue_rotation * (
            RGB_TO_YIQ * (trap_colour * current_colour)
        )
    ));
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


fn grade_f32(colour: f32, texture: ProceduralTexture) -> f32 {
    return select(
        pow(
            texture.gain * (
                (1. - texture.lift)
                * saturate_f32(
                    select(
                        colour,
                        1. - colour,
                        bool(texture.flags & INVERT),
                    ) - texture.black_point,
                ) / (texture.white_point - texture.black_point)
                + texture.lift
            ),
            1. / texture.gamma,
        ),
        0.,
        texture.white_point == texture.black_point,
    );
}


fn grade_vec3(colour: vec3f, texture: ProceduralTexture) -> vec3f {
    return select(
        pow(
            texture.gain * (
                (1. - texture.lift)
                * saturate_vec3f(
                    select(
                        colour,
                        1. - colour,
                        bool(texture.flags & INVERT),
                    ) - texture.black_point,
                ) / (texture.white_point - texture.black_point)
                + texture.lift
            ),
            vec3(1. / texture.gamma),
        ),
        vec3f(),
        texture.white_point == texture.black_point,
    );
}


fn procedurally_texture_f32(
    seed: vec4f,
    colour: f32,
    texture: ProceduralTexture,
) -> f32 {
    switch texture.texture_type {
        case NONE, default {
            return colour;
        }
#ifdef EnableGrade
        case GRADE {
            return grade_f32(colour, texture);
        }
#endif
#ifdef EnableCheckerboard
        case CHECKER_BOARD {
            return colour * grade_f32(checkerboard(seed / texture.scale), texture);
        }
#endif
// Simply having this case slows things down, so allow it to be compiled out
#ifdef EnableNoise
        case FBM_NOISE, TURBULENCE_NOISE {
            // FBM Noise
            return colour * grade_f32(
                octave_noise(seed, texture, texture.texture_type == TURBULENCE_NOISE),
                texture,
            );
        }
#endif
    }
}


fn procedurally_texture_vec3f(
    seed: vec4f,
    colour: vec3f,
    texture: ProceduralTexture,
) -> vec3f {
    switch texture.texture_type {
        case NONE, default {
            return colour;
        }
#ifdef EnableGrade
        case GRADE {
            return grade_vec3(colour, texture);
        }
#endif
#ifdef EnableCheckerboard
        case CHECKER_BOARD {
            return colour * vec3(grade_f32(checkerboard(seed / texture.scale), texture));
        }
#endif
// Simply having this case slows things down, so allow it to be compiled out
#ifdef EnableNoise
        case FBM_NOISE, TURBULENCE_NOISE {
            return colour * vec3(grade_f32(
                octave_noise(seed, texture, texture.texture_type == TURBULENCE_NOISE),
                texture,
            ));
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
