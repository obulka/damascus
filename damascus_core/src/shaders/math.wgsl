
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
fn sum_component_vec3f(vector_: vec3<f32>) -> f32 {
    return vector_.x + vector_.y + vector_.z;
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


/**
 * Get a rotation matrix from an axis and an angle about that axis.
 *
 * @arg axis: The axis to rotate about.
 * @arg angle: The rotation angle in radians.
 * @arg out: The location to store the rotation matrix.
 */
fn axis_angle_rotation_matrix(axis: vec3<f32>, angle: f32) -> mat3x3<f32> {
    var cos_angle: f32 = cos(angle);
    var one_minus_cos_angle: f32 = 1. - cos_angle;
    var sin_angle: f32 = sin(angle);

    var axis_squared: vec3<f32> = axis * axis;

    var axis_xy: f32 = axis.x * axis.y * one_minus_cos_angle;
    var axis_xz: f32 = axis.x * axis.z * one_minus_cos_angle;
    var axis_yz: f32 = axis.y * axis.z * one_minus_cos_angle;

    var axis_sin_angle: vec3<f32> = axis * sin_angle;

    var rotation_matrix: mat3x3<f32>;
    rotation_matrix[0][0] = cos_angle + axis_squared.x * one_minus_cos_angle;
    rotation_matrix[1][0] = axis_xy - axis_sin_angle.z;
    rotation_matrix[2][0] = axis_xz + axis_sin_angle.y;
    rotation_matrix[0][1] = axis_xy + axis_sin_angle.z;
    rotation_matrix[1][1] = cos_angle + axis_squared.y * one_minus_cos_angle;
    rotation_matrix[2][1] = axis_yz - axis_sin_angle.x;
    rotation_matrix[0][2] = axis_xz - axis_sin_angle.y;
    rotation_matrix[1][2] = axis_yz + axis_sin_angle.x;
    rotation_matrix[2][2] = cos_angle + axis_squared.z * one_minus_cos_angle;

    return rotation_matrix;
}


/**
 * Get the angle between two vectors.
 *
 * @arg vector_0: The first vector.
 * @arg vector_1: The second vector.
 *
 * @returns: The angle.
 */
fn angle_between_vec3f(vector_0: vec3<f32>, vector_1: vec3<f32>) -> f32 {
    return acos(dot(vector_0, vector_1));
}


/**
 * Find an axis normal to both input vectors.
 *
 * @arg vector_0: The first vector.
 * @arg vector_1: The second vector.
 *
 * @returns: The angle.
 */
fn normal(vector_0: vec3<f32>, vector_1: vec3<f32>) -> vec3<f32> {
    var perpendicular_vector: vec3<f32> = cross(vector_0, vector_1);
    // If the two axes are too closely aligned it creates artifacts
    // so check the magnitude of the cross product before normalizing
    if length(perpendicular_vector) > 0.001 {
        return normalize(perpendicular_vector);
    }
    // If the vectors are too closely aligned use any perpendicular axis
    perpendicular_vector = cross(vec3(0., 1., 0.), vector_1);
    if length(perpendicular_vector) > 0.001 {
        return normalize(perpendicular_vector);
    }
    perpendicular_vector = cross(vec3(1., 0., 0.), vector_1);
    if length(perpendicular_vector) > 0.001 {
        return normalize(perpendicular_vector);
    }
    return normalize(cross(vec3(0., 0., 1.), vector_1));
}


/**
 * Align a vector that has been defined relative to an axis with another
 * axis. For example if a vector has been chosen randomly in a
 * particular hemisphere, rotate that hemisphere to align with a new
 * axis.
 *
 * @arg unaligned_axis: The axis, about which, the vector was defined.
 * @arg alignment_direction: The axis to align with.
 * @arg vector_to_align: The vector that was defined relative to
 *     unaligned_axis.
 *
 * @returns: The aligned vector.
 */
fn align_with_direction(
    unaligned_axis: vec3<f32>,
    alignment_direction: vec3<f32>,
    vector_to_align: vec3<f32>,
) -> vec3<f32> {
    var angle: f32 = angle_between_vec3f(unaligned_axis, alignment_direction);
    if angle == 0. {
        return vector_to_align;
    }
    var rotation_axis: vec3<f32> = normal(unaligned_axis, alignment_direction);

    return axis_angle_rotation_matrix(rotation_axis, angle) * vector_to_align;
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
