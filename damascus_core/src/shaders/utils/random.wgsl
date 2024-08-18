// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_f32(seed: f32) -> f32 {
    return fract(
        sin(74.97544189533764 * seed + 95.32355126900637) * 47351.28749271963,
    );
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec2f(seed: vec2f) -> vec2f {
    return vec2(
        fract(
            sin(13.411893392589565 * seed.x + 52.204144534388156)
            * 29413.091862160447,
        ),
        fract(
            sin(92.277856455416342 * seed.y + 403.6482859667538)
            * 48117.120463081184,
        ),
    );
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec3f(seed: vec3f) -> vec3f {
    return vec3(
        fract(
            sin(74.449478093044163 * seed.x + 64.30389019737667)
            * 25014.958078049007,
        ),
        fract(
            sin(27.169012684474097 * seed.y + 97.24850458213037)
            * 68847.24754957597,
        ),
        fract(
            sin(10.889121360413057 * seed.z + 196.68616147949768)
            * 88220.55635443411,
        ),
    );
}


fn vec2f_to_random_f32(seed: vec2f) -> f32 {
    return fract(
        sin(dot(seed, vec2(18.424315764454885, 65.7214571881855)))
        * 23636.321902210882,
    );
}


fn vec3f_to_random_f32(seed: vec3f) -> f32 {
    return fract(
        sin(dot(seed, vec3(16.09228960839342, 78.80880762453727, 43.035762206218706)))
        * 36057.29583469442,
    );
}


/**
 * Create a random unit vector in the hemisphere aligned along the
 * given axis, with a distribution that is cosine weighted.
 *
 * @arg seed: The random seed.
 * @arg axis: The axis to align the hemisphere with.
 *
 * @returns: A random unit vector.
 */
fn cosine_direction_in_hemisphere(seed: vec2f, axis: vec3f) -> vec3f {
    var uniform_random_numbers: vec2f = random_vec2f(seed);
    var r: f32 = sqrt(uniform_random_numbers.x);
    var angle: f32 = TWO_PI * uniform_random_numbers.y;

    var secondary_axis: vec3f = select(
        vec3(1., 0., 0.),
        vec3(0., 1., 0.),
        abs(axis.x) > 1e-6,
    );
    var perpendicular_axis: vec3f = normalize(cross(secondary_axis, axis));
    var basis_axis: vec3f = cross(axis, perpendicular_axis);

    return normalize(
        perpendicular_axis * cos(angle) * r
        + basis_axis * sin(angle) * r
        + axis * sqrt(1. - uniform_random_numbers.x)
    );
}


/**
 * Create a random point that lies within the unit circle.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random point, (radius, angle) in the unit circle.
 */
fn uniform_point_in_unit_circle(seed: vec2f) -> vec2f {
    return vec2f(sqrt(random_f32(seed.x)), TWO_PI * random_f32(seed.y));
}
