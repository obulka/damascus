// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


const PI: f32 = 3.141592653589793;
const TWO_PI: f32 = 6.28318530718;


// wish we could overload functions
fn max_component_vec2f(vector_: vec2<f32>) -> f32 {
    return max(vector_.x, vector_.y);
}


fn max_component_vec3f(vector_: vec3<f32>) -> f32 {
    return max(vector_.x, max(vector_.y, vector_.z));
}


/**
 * The positive part of the vector. Ie. any negative values will be 0.
 *
 * @arg value: The value.
 *
 * @returns: The positive part of the value.
 */
fn positive_part_f32(value: f32) -> f32 {
    return max(value, 0.);
}


/**
 * The positive part of the vector. Ie. any negative values will be 0.
 *
 * @arg vector: The vector.
 *
 * @returns: The positive part of the vector.
 */
fn positive_part_vec2f(value: vec2<f32>) -> vec2<f32> {
    return max(value, vec2(0.));
}


/**
 * The positive part of the vector. Ie. any negative values will be 0.
 *
 * @arg vector: The vector.
 *
 * @returns: The positive part of the vector.
 */
fn positive_part_vec3f(value: vec3<f32>) -> vec3<f32> {
    return max(value, vec3(0.));
}


/**
 * The negative part of the vector. Ie. any positive values will be 0,
 * and the negative values will be positive.
 *
 * @arg value: The value.
 *
 * @returns: The negative part of the value.
 */
fn negative_part_f32(value: f32) -> f32 {
    return -min(value, 0.);
}


/**
 * Sum the components of a vector.
 *
 * @arg vector_: The vector to sum the components of.
 *
 * @returns: The sum of the components.
 */
fn element_sum_vec3f(vector_: vec3<f32>) -> f32 {
    return vector_.x + vector_.y + vector_.z;
}


/**
 * Sum the components of a vector.
 *
 * @arg vector_: The vector to sum the components of.
 *
 * @returns: The sum of the components.
 */
fn element_sum_vec4f(vector_: vec4<f32>) -> f32 {
    return vector_.x + vector_.y + vector_.z + vector_.w;
}


/**
 * Convert a cartesion vector to cylindrical, without worrying about
 * the angle.
 *
 * @returns: Cylindrical coordinates symmetric about the y-axis.
 */
fn cartesian_to_cylindrical(coordinates: vec3<f32>) -> vec2<f32> {
    return vec2(length(coordinates.xz), coordinates.y);
}


/**
 * Dot product of a vector with itself.
 *
 * @arg vector_: The vector to take the dot product of.
 *
 * @returns: The dot product.
 */
fn dot2_vec2f(vector_: vec2<f32>) -> f32 {
    return dot(vector_, vector_);
}


/**
 * Dot product of a vector with itself.
 *
 * @arg vector_: The vector to take the dot product of.
 *
 * @returns: The dot product.
 */
fn dot2_vec3f(vector_: vec3<f32>) -> f32 {
    return dot(vector_, vector_);
}


/**
 * Get the length of the shorter of two vectors.
 *
 * @arg vector_0: The first vector to get the length of if it is the
 *     shortest option
 * @arg vector_1: The second vector to get the length of if it is the
 *     shortest option
 *
 * @returns: The shorter of the two lengths
 */
fn min_length_vec2f(vector_0: vec2<f32>, vector_1: vec2<f32>) -> f32 {
    return sqrt(min(dot2_vec2f(vector_0), dot2_vec2f(vector_1)));
}


/**
 * Saturate a value ie. clamp between 0 and 1
 *
 * Note: This should be a builtin function but I guess the wgsl version
 *     is old.
 *
 * @arg value: The value to saturate.
 *
 * @returns: The clamped value
 */
fn saturate_f32(value: f32) -> f32 {
    return clamp(value, 0., 1.);
}


/**
 * Saturate a value ie. clamp between 0 and 1
 *
 * Note: This should be a builtin function but I guess the wgsl version
 *     is old.
 *
 * @arg value: The value to saturate.
 *
 * @returns: The clamped value
 */
fn saturate_vec3f(value: vec3<f32>) -> vec3<f32> {
    return clamp(value, vec3(0.), vec3(1.));
}


/**
 * Compute the signed distance along a vector
 *
 * @arg vector_: A vector from a point to the nearest surface of an
 *     object.
 *
 * @returns: The signed length of the vector.
 */
fn sdf_length_vec2f(vector_: vec2<f32>) -> f32 {
    return (
        length(positive_part_vec2f(vector_))
        - negative_part_f32(max_component_vec2f(vector_))
    );
}


/**
 * Compute the signed distance along a vector
 *
 * @arg vector_: A vector from a point to the nearest surface of an
 *     object.
 *
 * @returns: The signed length of the vector.
 */
fn sdf_length_vec3f(vector_: vec3<f32>) -> f32 {
    return (
        length(positive_part_vec3f(vector_))
        - negative_part_f32(max_component_vec3f(vector_))
    );
}


/**
 * Combine two PDFs in an optimal manner.
 *
 * @arg pdf_0: The first PDF.
 * @arg pdf_1: The second PDF.
 *
 * @returns: The combined PDF.
 */
fn balance_heuristic(pdf_0: f32, pdf_1: f32) -> f32 {
    return pdf_0 / (pdf_0 + pdf_1);
}


fn power_of_u32(base: f32, exponent: u32) -> f32 {
    var base_: f32 = base;
    var exponent_: u32 = exponent;
    var result: f32 = 1.;
    loop {
        if bool(exponent_ & 1u) {
            result *= base_;
        }
        exponent_ >>= 1u;
        if !bool(exponent_) {
            break;
        }
        base_ *= base_;
    }

    return result;
}


/**
 * Convert a spherical unit vector (unit radius) to cartesion.
 *
 * @arg angles: The spherical angles in radians.
 *
 * @returns: The equivalent cartesion vector.
 */
fn spherical_unit_vector_to_cartesion(angles: vec2<f32>) -> vec3<f32> {
    var sin_phi: f32 = sin(angles.y);
    return vec3(
        cos(angles.x) * sin_phi,
        cos(angles.y),
        sin(angles.x) * sin_phi,
    );
}


/**
 * Convert the uv coordinate in a latlong image to angles.
 *
 * @arg uv_coordinate: The uv coordinate.
 *
 * @returns: The equivalent angles in radians.
 */
fn uv_coordinate_to_angles(uv_coordinate: vec2<f32>) -> vec2<f32> {
    return vec2(
        (uv_coordinate.x + 1.) * PI,
        (1. - uv_coordinate.y) * PI / 2.,
    );
}
