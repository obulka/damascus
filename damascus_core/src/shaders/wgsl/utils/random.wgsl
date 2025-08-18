// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

// https://www.pcg-random.org
fn update_seed(seed: ptr<function, u32>) -> u32
{
    *seed = *seed * 747796405u + 2891336453u;
    var word: u32 = ((*seed >> ((*seed >> 28u) + 4u)) ^ *seed) * 277803737u;
    word = (word >> 22u) ^ word;
    return word;
}


fn random_f32(seed: ptr<function, u32>) -> f32
{
    return f32(update_seed(seed)) / 4294967295.0; // 2^32 - 1
}


// convert 2D seed to 1D
fn vec2u_seed_to_u32(seed: vec2u) -> u32
{
    return 19u * seed.x + 47u * seed.y + 101u;
}


// convert 2D seed to 1D
fn vec4u_seed_to_u32(seed: vec4u) -> u32
{
    return 19u * seed.x + 47u * seed.y + 101u * seed.z + 131u * seed.w + 173u;
}


// Random value in normal distribution (with mean=0 and sd=1)
fn normal_distribution(seed: ptr<function, u32>) -> f32
{
    // Thanks to https://stackoverflow.com/a/6178290
    var theta: f32 = TWO_PI * random_f32(seed);
    var rho: f32 = sqrt(-2.0 * log(random_f32(seed)));
    return rho * cos(theta);
}


// Calculate a random direction
fn random_direction(seed: ptr<function, u32>) -> vec3f
{
    // Thanks to https://math.stackexchange.com/a/1585996
    var x: f32 = normal_distribution(seed);
    var y: f32 = normal_distribution(seed);
    var z: f32 = normal_distribution(seed);
    return normalize(vec3f(x, y, z));
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec2f(seed: ptr<function, u32>) -> vec2f {
    return vec2(random_f32(seed), random_f32(seed));
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec3f(seed: ptr<function, u32>) -> vec3f {
    return vec3(random_f32(seed), random_f32(seed), random_f32(seed));
}


// fn vec2f_to_random_f32(seed: vec2f) -> f32 {
//     return fract(
//         sin(dot(seed, vec2(18.424315764454885, 65.7214571881855)))
//         * 23636.321902210882,
//     );
// }


// fn vec3f_to_random_f32(seed: vec3f) -> f32 {
//     return fract(
//         sin(dot(seed, vec3(16.09228960839342, 78.80880762453727, 43.035762206218706)))
//         * 36057.29583469442,
//     );
// }


/**
 * Create a random unit vector in the hemisphere aligned along the
 * given axis, with a distribution that is cosine weighted.
 *
 * @arg seed: The random seed.
 * @arg axis: The axis to align the hemisphere with.
 *
 * @returns: A random unit vector.
 */
fn cosine_direction_in_hemisphere(seed: ptr<function, u32>, axis: vec3f) -> vec3f {
    return normalize(axis + random_direction(seed));
}


/**
 * Create a random point that lies within the unit circle.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random point, (radius, angle) in the unit circle.
 */
fn uniform_point_in_unit_circle(seed: ptr<function, u32>) -> vec2f {
    return vec2f(sqrt(random_f32(seed)), TWO_PI * random_f32(seed));
}

/**
 * Create a random point that lies within the unit circle.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random cartesian point, in the unit circle.
 */
fn uniform_point_in_circle(seed: ptr<function, u32>, radius: f32) -> vec2f {
    var radius_angle: vec2f = uniform_point_in_unit_circle(seed);
    return radius_angle.x * radius * vec2(cos(radius_angle.y), sin(radius_angle.y));
}
