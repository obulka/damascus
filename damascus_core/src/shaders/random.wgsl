// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_f32(seed: f32) -> f32 {
    return fract(sin(seed * 91.3458) * 47453.5453123);
}

/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec2f(seed: vec2<f32>) -> vec2<f32> {
    return vec2(
        random_f32(seed.x),
        random_f32(seed.y),
    );
}


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random_vec3f(seed: vec3<f32>) -> vec3<f32> {
    return vec3(
        random_f32(seed.x),
        random_f32(seed.y),
        random_f32(seed.z),
    );
}


fn vec2f_to_random_f32(seed: vec2<f32>) -> f32 {
    return fract(sin(dot(seed, vec2(12.9898, 78.233))) * 43758.5453123);
}


fn vec3f_to_random_f32(seed: vec3<f32>) -> f32 {
    return fract(sin(dot(seed, vec3(12.9898, 78.233, 34.532))) * 43758.5453123);
}


/**
 * Create a random unit vector in the hemisphere aligned along the
 * z-axis, with a distribution that is cosine weighted.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random unit vector.
 */
fn cosine_direction_in_z_hemisphere(seed: vec2<f32>) -> vec3<f32>
{
    var uniform_random_numbers: vec2<f32> = random_vec2f(seed);
    var r: f32 = sqrt(uniform_random_numbers.x);
    var angle: f32 = TWO_PI * uniform_random_numbers.y;

    var x: f32 = r * cos(angle);
    var y: f32 = r * sin(angle);

    return vec3(x, y, sqrt(positive_part_f32(1. - uniform_random_numbers.x)));
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
fn cosine_direction_in_hemisphere(seed: vec2<f32>, axis: vec3<f32>) -> vec3<f32> {
    return normalize(align_with_direction(
        vec3(0., 0., 1.),
        axis,
        cosine_direction_in_z_hemisphere(seed),
    ));
}


/**
 * Create a random point that lies within the unit circle.
 *
 * @arg seed: The random seed.
 *
 * @returns: A random point, (radius, angle) in the unit circle.
 */
fn uniform_point_in_unit_circle(seed: vec2<f32>) -> vec2<f32>
{
    return vec2<f32>(sqrt(random_f32(seed.x)), 2. * PI * random_f32(seed.y));
}
