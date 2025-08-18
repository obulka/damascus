// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

struct Seed {
    state: u32,
    increment: u32,
}


/**
 * Update the underlying hash state on the seed and
 * return the next hash.
 *
 * Reference:
 *     - https://www.pcg-random.org
 *     - https://www.shadertoy.com/view/XlGcRh
 *
 * @arg seed: The random seed.
 *
 * @returns: A new uniformly distributed u32.
 */
fn next_hash(seed: ptr<function, Seed>) -> u32
{
    (*seed).state = (*seed).state * 747796405u + ((*seed).increment | 1u);

    var xor_shifted: u32 = (
        ((*seed).state >> (((*seed).state >> 28u) + 4u)
    ) ^ (*seed).state) * 277803737u;

    return (xor_shifted >> 22u) ^ xor_shifted;
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_f32(seed: ptr<function, Seed>) -> f32
{
    return f32(next_hash(seed)) / 4294967295.0; // 2^32 - 1
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec2f(seed: ptr<function, Seed>) -> vec2f {
    return vec2(random_f32(seed), random_f32(seed));
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec3f(seed: ptr<function, Seed>) -> vec3f {
    return vec3(random_f32(seed), random_f32(seed), random_f32(seed));
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
fn cosine_direction_in_hemisphere(seed: ptr<function, Seed>, axis: vec3f) -> vec3f {
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
fn uniform_point_in_unit_circle(seed: ptr<function, Seed>) -> vec2f {
    return vec2f(sqrt(random_f32(seed)), TWO_PI * random_f32(seed));
}


/**
 * Create a random point that lies within the unit circle.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random cartesian point, in the unit circle.
 */
fn uniform_point_in_circle(seed: ptr<function, Seed>, radius: f32) -> vec2f {
    var radius_angle: vec2f = uniform_point_in_unit_circle(seed);
    return radius_angle.x * radius * vec2(cos(radius_angle.y), sin(radius_angle.y));
}
