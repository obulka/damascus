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
    return fract(sin(73.1 * seed + 91.3458) * 47453.5453);
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
        fract(sin(13.157 * seed.x + 71.743) * 7513.471),
        fract(sin(97.519 * seed.y + 113.591) * 47453.5453),
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
        fract(sin(75.19 * seed.x + 71.743) * 7513.471),
        fract(sin(15.73 * seed.y + 113.591) * 47453.553),
        fract(sin(7.37 * seed.z + 147.781) * 8769.132),
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
 * given axis, with a distribution that is cosine weighted.
 *
 * @arg seed: The random seed.
 * @arg axis: The axis to align the hemisphere with.
 *
 * @returns: A random unit vector.
 */
fn cosine_direction_in_hemisphere(seed: vec2<f32>, axis: vec3<f32>) -> vec3<f32> {
    var uniform_random_numbers: vec2<f32> = random_vec2f(seed);
    var r: f32 = sqrt(uniform_random_numbers.x);
    var angle: f32 = TWO_PI * uniform_random_numbers.y;

    var secondary_axis: vec3<f32> = select(
        vec3(1., 0., 0.),
        vec3(0., 1., 0.),
        abs(axis.x) > 1e-6,
    );
    var perpendicular_axis: vec3<f32> = normalize(cross(secondary_axis, axis));
    var basis_axis: vec3<f32> = cross(axis, perpendicular_axis);

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
fn uniform_point_in_unit_circle(seed: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(sqrt(random_f32(seed.x)), TWO_PI * random_f32(seed.y));
}
