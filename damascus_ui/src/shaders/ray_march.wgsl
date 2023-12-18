// Copyright 2022 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE.md file that should have been included as part
// of this package.


//
// Ray Marching shader
//


// TODO: separate into files and use import statements


struct SceneParameters {
    num_primitives: u32,
    num_lights: u32,
}


struct RayMarcherParameters {
    paths_per_pixel: u32,
    roulette: u32,
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: vec3<f32>,
    enable_depth_of_field: u32,
    dynamic_level_of_detail: u32,
    max_light_sampling_bounces: u32,
    sample_hdri: u32,
    sample_all_lights: u32,
    light_sampling_bias: f32,
    secondary_sampling: u32,
    hdri_offset_angle: f32,
    output_aov: u32,
    latlong: u32,
}


struct RenderParameters {
    ray_marcher: RayMarcherParameters,
    scene: SceneParameters,
}


// Global render settings
@group(0) @binding(0)
var<uniform> _render_params: RenderParameters;


// math.wgsl

let PI: f32 = 3.141592653589793;
let TWO_PI: f32 = 6.28318530718;


// wish we could overload functions
fn max_component_vec2f(vector_: vec2<f32>) -> f32 {
    return max(vector_.x, vector_.y);
}


fn max_component_vec3f(vector_: vec3<f32>) -> f32 {
    return max(vector_.x, max(vector_.y, vector_.z));
}


// fn max_component_vec4f(vector_: vec4<f32>) -> f32 {
//     return max(vector_.x, max(vector_.y, max(vector_.z, vector_.w)));
// }


// fn min_component_vec2f(vector_: vec2<f32>) -> f32 {
//     return min(vector_.x, vector_.y);
// }


// fn min_component_vec3f(vector_: vec3<f32>) -> f32 {
//     return min(vector_.x, min(vector_.y, vector_.z));
// }


// fn min_component_vec4f(vector_: vec4<f32>) -> f32 {
//     return min(vector_.x, min(vector_.y, min(vector_.z, vector_.w)));
// }


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
    if (length(perpendicular_vector) > 0.001) {
        return normalize(perpendicular_vector);
    }
    // If the vectors are too closely aligned use any perpendicular axis
    perpendicular_vector = cross(vec3(0., 1., 0.), vector_1);
    if (length(perpendicular_vector) > 0.001) {
        return normalize(perpendicular_vector);
    }
    perpendicular_vector = cross(vec3(1., 0., 0.), vector_1);
    if (length(perpendicular_vector) > 0.001) {
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
    if (angle == 0.) {
        return vector_to_align;
    }
    var rotation_axis: vec3<f32> = normal(unaligned_axis, alignment_direction);

    return axis_angle_rotation_matrix(rotation_axis, angle) * vector_to_align;
}

// random.wgsl


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

// geometry/sdfs.wgsl
// Copyright 2022 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE.md file that should have been included as part
// of this package.

//
// Signed Distance Functions
//
// Many of the below sdfs are based on the work of Inigo Quilez
// https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
//

// const SPHERE: u32 = 0u;
// const ELLIPSOID: u32 = 1u;
// const CUT_SPHERE: u32 = 2u;
// const HOLLOW_SPHERE: u32 = 3u;
// const DEATH_STAR: u32 = 4u;
// const SOLID_ANGLE: u32 = 5u;
// const RECTANGULAR_PRISM: u32 = 6u;
// const RECTANGULAR_PRISM_FRAME: u32 = 7u;
// const RHOMBUS: u32 = 8u;
// const TRIANGULAR_PRISM: u32 = 9u;
// const CYLINDER: u32 = 10u;
// const INFINITE_CYLINDER: u32 = 11u;
// const PLANE: u32 = 12u;
// const CAPSULE: u32 = 13u;
// const CONE: u32 = 14u;
// const INFINITE_CONE: u32 = 15u;
// const CAPPED_CONE: u32 = 16u;
// const ROUNDED_CONE: u32 = 17u;
// const TORUS: u32 = 18u;
// const CAPPED_TORUS: u32 = 19u;
// const LINK: u32 = 20u;
// const HEXAGONAL_PRISM: u32 = 21u;
// const OCTAHEDRON: u32 = 22u;
// const MANDELBULB: u32 = 23u;
// const MANDELBOX: u32 = 24u;

let DIFFUSE_TRAP: u32 = 8192u;
let SPECULAR_TRAP: u32 = 16384u;
let EXTINCTION_TRAP: u32 = 32768u;
let EMISSION_TRAP: u32 = 65536u;
let SCATTERING_TRAP: u32 = 131072u;


/**
 * Compute the min distance from a point to a circle.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the circle.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_circle(position: vec2<f32>, radius: f32) -> f32 {
    return length(position) - radius;
}


/**
 * Compute the min distance from a point to a sphere.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the sphere.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_sphere(position: vec3<f32>, radius: f32) -> f32 {
    return length(position) - radius;
}


/**
 * Compute the inexact min distance from a point to an ellipsoid.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radii: The radius along the x, y, and z axes of the ellipsoid.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_ellipsoid(position: vec3<f32>, radii: vec3<f32>) -> f32 {
    // Components of this vector that are < 1 are inside the ellipse
    // when projected onto the plane the respective axis is normal to
    var scaled_position: vec3<f32> = position / radii;

    // If this length is < 1 we are inside the ellipsoid
    var scaled_length: f32 = length(scaled_position);

    return scaled_length * (scaled_length - 1.) / length(scaled_position / radii);
}


/**
 * Compute the min distance from a point to a cut sphere.
 * The cut surface faces up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the sphere.
 * @arg cut_height: The cut_height (y-axis) below which the sphere is
 *     culled.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_cut_sphere(
    position: vec3<f32>,
    radius: f32,
    cut_height: f32,
) -> f32 {
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // The radius of the circle made by slicing the sphere
    var cut_radius_squared: f32 = radius * radius - cut_height * cut_height;
    var cut_radius: f32 = sqrt(cut_radius_squared);

    // When the cut_height is positive, if we are outside an infinite
    // cone with its tip at the origin, opening through the edge of
    // the cut surface, then the nearest point will be on the
    // spherical surface. If the cut_height is negative, we must be
    // below the portion of the cone that is below the y-axis, but we
    // must also be below a curved boundary separating the regions where
    // the flat and spherical surfaces are closest
    var nearest_is_spherical: f32 = max(
        cut_radius_squared * (radius - cut_height + 2. * cylindrical_position.y)
            - (radius + cut_height) * cylindrical_position.x * cylindrical_position.x,
        cut_radius * cylindrical_position.y - cut_height * cylindrical_position.x,
    );

    if (nearest_is_spherical < 0.)
    {
        // Closest point is on the surface of the sphere
        return length(cylindrical_position) - radius;
    }
    else if (cylindrical_position.x < cut_radius)
    {
        // Closest point is within the cut surface
        return -cut_height + cylindrical_position.y;
    }
    else
    {
        // Closest point is on the edge of the cut surface
        return length(cylindrical_position - vec2(cut_radius, cut_height));
    }
}


/**
 * Compute the min distance from a point to a hollow sphere.
 * The hollowed opening points up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the sphere.
 * @arg cut_height: The cut_height (y-axis) at which an opening is
 *     created.
 * @arg thickness: The thickness of the walls of the hollow sphere.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_hollow_sphere(
    position: vec3<f32>,
    radius: f32,
    cut_height: f32,
    thickness: f32,
) -> f32 {
    var half_thickness: f32 = thickness / 2.;

    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    var cut_radius: f32 = sqrt(radius * radius - cut_height * cut_height);

    if (cut_height * cylindrical_position.x < cut_radius * cylindrical_position.y)
    {
        // Closest point is on the rim
        return length(
            cylindrical_position
            - vec2(cut_radius, cut_height)
        ) - half_thickness;
    }
    // Closest point is on the spherical surface
    return abs(length(cylindrical_position) - radius) - half_thickness;
}


/**
 * Compute the min distance from a point to a death star.
 * The hollowed opening points up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg additive_sphere_radius: The radius of the sphere that remains solid.
 * @arg subtractive_sphere_radius: The radius of the sphere that is cut from
 *     the solid.
 *
 * @arg subtractive_sphere_height: The height (y-axis) of the center of
 *     the sphere that is cut from the solid, above additive_sphere_radius +
 *     subtractive_sphere_radius, the result will be a standard sphere of
 *     radius additive_sphere_radius.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_death_star(
    position: vec3<f32>,
    additive_sphere_radius: f32,
    subtractive_sphere_radius: f32,
    subtractive_sphere_height: f32,
) -> f32 {
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    var additive_sphere_radius_squared: f32 = additive_sphere_radius * additive_sphere_radius;

    var cut_height: f32 = (
        additive_sphere_radius_squared
        - (
            subtractive_sphere_radius * subtractive_sphere_radius
            - subtractive_sphere_height * subtractive_sphere_height
        )
    ) / (2. * subtractive_sphere_height);

    var cut_radius: f32 = sqrt(additive_sphere_radius_squared - cut_height * cut_height);

    if (
        subtractive_sphere_height * positive_part_f32(cut_radius - cylindrical_position.x)
        < cylindrical_position.y * cut_radius - cylindrical_position.x * cut_height
    ) {
        // Closest point is on the rim
        return length(cylindrical_position - vec2(cut_radius, cut_height));
    }
    return max(
        // Closest point to the solid sphere
        length(cylindrical_position) - additive_sphere_radius,
        // Closest point to the hollowed portion
        subtractive_sphere_radius - length(
            cylindrical_position - vec2(0., subtractive_sphere_height)
        ),
    );
}


/**
 * Compute the min distance from a point to a solid angle.
 * The conical shape has its tip at the origin and opens up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the sphere to cut the angle out of.
 * @arg angle: The angle between the edge of the solid angle and the
 *     y-axis on [0-PI] measured between the y-axis and wall of the
 *     solid angle.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_solid_angle(
    position: vec3<f32>,
    radius: f32,
    angle: f32,
) -> f32 {
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // The direction from the tip of the conical portion to where it
    // meets the sphere
    var cone_edge_direction = vec2(sin(angle), cos(angle));

    // Distance to the sphere we cut the cone out of
    var distance_to_sphere: f32 = length(cylindrical_position) - radius;
    var distance_to_cone: f32 = length(
        cylindrical_position - cone_edge_direction * clamp(
            dot(cylindrical_position, cone_edge_direction),
            0.,
            radius,
        )
    );
    var inside: f32 = sign(
        cone_edge_direction.y * cylindrical_position.x
        - cone_edge_direction.x * cylindrical_position.y
    );

    return max(distance_to_sphere, inside * distance_to_cone);
}


/**
 * Compute the min distance from a point to a rectangular prism.
 * Centered at the origin.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg width: The width (x) of the prism.
 * @arg height: The height (y) of the prism.
 * @arg depth: The depth (z) of the prism.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_rectangular_prism(
    position: vec3<f32>,
    width: f32,
    height: f32,
    depth: f32,
) -> f32 {
    // Only look at positive quadrant, using symmetry
    var prism_to_position = abs(position) - vec3(width, height, depth) / 2.;
    // Clamp the components that are inside the prism to the surface
    // before getting the distance
    return sdf_length_vec3f(prism_to_position);
}


/**
 * Compute the min distance from a point to the frame of a
 * rectangular prism.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg width:  The width (x) of the frame.
 * @arg height:  The height (y) of the frame.
 * @arg depth:  The depth (z) of the frame.
 * @arg thickness:  The thickness of the frame.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_rectangular_prism_frame(
    position: vec3<f32>,
    width: f32,
    height: f32,
    depth: f32,
    thickness: f32,
) -> f32 {
    var prism_to_position = abs(position) - vec3(width, height, depth) / 2.;
    var inner_reflected: vec3<f32> = abs(prism_to_position + thickness) - thickness;

    return min(
        sdf_length_vec3f(vec3(prism_to_position.x, inner_reflected.yz)),
        min(
            sdf_length_vec3f(vec3(inner_reflected.x, prism_to_position.y, inner_reflected.z)),
            sdf_length_vec3f(vec3(inner_reflected.xy, prism_to_position.z)),
        ),
    );
}


/**
 * Compute the min distance from a point to a rhombus.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg width:  The width (x) of the rhombus.
 * @arg height:  The height (y) of the rhombus.
 * @arg depth:  The depth (z) of the rhombus, this the extruded
 *     dimension, or thickness.
 * @arg corner_radius:  The radius of the corners of the rhombus'
 *     xy-plane parallel face.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_rhombus(
    position: vec3<f32>,
    width: f32,
    height: f32,
    depth: f32,
    corner_radius: f32,
) -> f32 {
    var abs_position: vec3<f32> = abs(position);
    var half_width_height = vec2(width, height) / 2.;

    var s: vec2<f32> = half_width_height * (half_width_height - 2. * abs_position.xy);
    var f: f32 = clamp((s.x - s.y) / dot2_vec2f(half_width_height), -1., 1.);

    var inside: f32 = sign(
        dot(abs_position.xy, half_width_height.yx) - half_width_height.x * half_width_height.y,
    );

    var rhombus_to_position = vec2(
        inside * length(
            abs_position.xy - 0.5 * half_width_height * vec2(1. - f, 1. + f)
        ) - corner_radius,
        // Closest point along z-axis only depends on the thickness of
        // the extrusion
        abs_position.z - depth / 2.
    );

    return sdf_length_vec2f(rhombus_to_position);
}


/**
 * Compute the min distance from a point to a triangular prism.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg base: The equalateral triangles edge length (xy-plane).
 * @arg depth: The depth (z-axis) of the prism.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_triangular_prism(position: vec3<f32>, base: f32, depth: f32) -> f32 {
    // 0.28867513459f = tan(PI / 6.) / 2., converts base length
    // to the min distance from centroid to edge of triangle

    // 0.86602540378f = cos(PI / 6.) = base / height
    // 0.5f = sin(PI / 6.) = base / (2 * base)

    return max(
        abs(position.z) - depth,
        max(
            abs(position.x) * 0.86602540378 + position.y * 0.5,
            -position.y,
        ) - 0.28867513459 * base
    );
}


/**
 * Compute the min distance from a point to a cylinder
 * Symmetric about the xz-plane.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius (xz-plane) of the cylinder.
 * @arg height: The height (y-axis) of the cylinder.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_cylinder(
    position: vec3<f32>,
    radius: f32,
    height: f32,
) -> f32 {
    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2<f32> = abs(cartesian_to_cylindrical(position));
    var cylinder_to_position = cylindrical_position - vec2(radius, height / 2.);

    return sdf_length_vec2f(cylinder_to_position);
}


/**
 * Compute the min distance from a point to an infinite cylinder
 * (y-axis aligned).
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius (xz-plane) of the cylinder.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_infinite_cylinder(position: vec3<f32>, radius: f32) -> f32 {
    return distance_to_circle(position.xz, radius);
}


/**
 * Compute the min distance from a point to a plane.
 * Anything underneath the plane, as defined by the normal direction
 * pointing above, will be considered inside.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg normal: The normal direction of the plane.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_plane(position: vec3<f32>, normal: vec3<f32>) -> f32 {
    return dot(position, normal);
}


/**
 * Compute the min distance from a point to a capsule.
 * Oriented along the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the capsule.
 * @arg negative_height: The distance along the negative y-axis before
 *     entering the dome.
 * @arg positive_height: The distance along the positive y-axis before
 *     entering the dome.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_capsule(
    position: vec3<f32>,
    radius: f32,
    negative_height: f32,
    positive_height: f32,
) -> f32 {
    return length(vec3(
        position.x,
        position.y - clamp(position.y, -negative_height, positive_height),
        position.z,
    )) - radius;
}


/**
 * Compute the min distance from a point to a cone
 * (y-axis aligned). The tip of the cone is at the origin, and it opens
 * up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg angle: The angle between the tip and base of the cone [0-PI/2)
 *     measured between the y-axis and wall of the cone.
 * @arg height: The height (y-axis) of the cone. Cannot be 0.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_cone(position: vec3<f32>, angle: f32, height: f32) -> f32 {
    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // The cylindrical coordinates of the edge of the cone base
    var cylindrical_bound = vec2(abs(height * tan(angle)), height);

    // Vector from the top surface of the cone to the position given
    var cone_top_to_position: vec2<f32> = cylindrical_position - cylindrical_bound * vec2(
        saturate_f32(cylindrical_position.x / cylindrical_bound.x),
        1.,
    );
    // Vector from the edge of the cone to the position given
    var cone_edge_to_position: vec2<f32> = (
        cylindrical_position - cylindrical_bound * saturate_f32(
            dot(cylindrical_position, cylindrical_bound)
            / dot2_vec2f(cylindrical_bound),
        )
    );

    var height_sign: f32 = sign(height);

    // -1 if the position is inside the cone, +1 if it is outside
    var inside: f32 = sign(max(
        height_sign * (
            cylindrical_position.x * height
            - cylindrical_position.y * cylindrical_bound.x
        ),
        height_sign * (cylindrical_position.y - height),
    ));
    // The distance is the minimum between the distance to the edge and
    // the distance to the base
    return inside * min_length_vec2f(cone_edge_to_position, cone_top_to_position);
}


/**
 * Compute the min distance from a point to an infinite cone
 * (y-axis aligned). The tip of the cone is at the origin, and it opens
 * up the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg angle: The angle between the tip and base of the cone [0-PI/2)
 *     measured between the y-axis and wall of the cone.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_infinite_cone(position: vec3<f32>, angle: f32) -> f32 {
    // The normalized cylindrical coordinates of the edge of the cone base
    var cone_edge_direction: vec2<f32> = vec2(sin(angle), cos(angle));

    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // -1 if the position is inside the cone, +1 if it is outside
    var inside: f32 = sign(
        cylindrical_position.x * cone_edge_direction.y
        - cylindrical_position.y * cone_edge_direction.x,
    );

    // The shortest path is always to the cones edge, or tip if we are
    // below it. The dot product projects the position onto the cone
    // edge, and taking the positive part clamps the cone above the
    // xz-plane
    return inside * length(
        cylindrical_position - cone_edge_direction * positive_part_f32(
            dot(cylindrical_position, cone_edge_direction),
        ),
    );
}


/**
 * Compute the min distance from a point to a capped cone.
 * Oriented along the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg height: The height (y-axis) of the cone, centered at the origin
 *     Cannot be 0.
 * @arg lower_radius: The radius of the cone at y = -height/2.
 * @arg upper_radius: The radius of the cone at y = height/2.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_capped_cone(
    position: vec3<f32>,
    height: f32,
    lower_radius: f32,
    upper_radius: f32,
) -> f32 {
    var half_height: f32 = height / 2.;
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // The 'corners' are the apparent corners when the shape is
    // projected onto the xy-plane
    var upper_corner = vec2(upper_radius, half_height);
    var lower_to_upper_corner = vec2(upper_radius - lower_radius, height);

    var cone_top_or_bottom_to_position = vec2(
        cylindrical_position.x - min(
            cylindrical_position.x,
            select(upper_radius, lower_radius, cylindrical_position.y < 0.),
        ),
        abs(cylindrical_position.y) - half_height,
    );
    var cone_edge_to_position: vec2<f32> = (
        cylindrical_position
        - upper_corner
        + lower_to_upper_corner * saturate_f32(
            dot(upper_corner - cylindrical_position, lower_to_upper_corner)
            / dot2_vec2f(lower_to_upper_corner)
        )
    );

    var inside: f32 = select(
        1.,
        -1.,
        cone_edge_to_position.x < 0. && cone_top_or_bottom_to_position.y < 0.,
    );
    return inside * min_length_vec2f(cone_top_or_bottom_to_position, cone_edge_to_position);
}


/**
 * Compute the min distance from a point to a rounded cone.
 * Oriented along the y-axis.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg height: The distance (y-axis) between the centers of the lower
 *     and upper spheres which, when connected, form the rounded cone.
 * @arg lower_radius: The radius of the sphere at y = 0.
 * @arg upper_radius: The radius of the sphere at y = height.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_rounded_cone(
    position: vec3<f32>,
    height: f32,
    lower_radius: f32,
    upper_radius: f32,
) -> f32 {
    var cylindrical_position: vec2<f32> = cartesian_to_cylindrical(position);

    // Get the unit vector that is normal to the conical surface in 2D
    var parallel_x: f32 = (upper_radius - lower_radius) / height;
    var parallel_y: f32 = sqrt(1. - parallel_x * parallel_x);
    var parallel = vec2(parallel_x, parallel_y);

    var position_projected_on_cone: f32 = dot(cylindrical_position, parallel);

    if (position_projected_on_cone < 0.)
    {
        // Closest point is on the lower sphere
        return length(cylindrical_position) - lower_radius;
    }
    else if (position_projected_on_cone > parallel_y * height)
    {
        // Closest point is on the upper sphere
        return length(cylindrical_position - vec2(0., height)) - upper_radius;
    }

    // Closest point is on the conical surface, so project the position
    // onto the cone's normal direction, then offset it by the lower radius
    return dot(cylindrical_position, vec2(parallel_y, -parallel_x)) - lower_radius;
}


/**
 * Compute the min distance from a point to a torus.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg ring_radius: The radius (xy-plane) of the ring of the torus.
 * @arg tube_radius: The radius of the tube of the torus.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_torus(position: vec3<f32>, ring_radius: f32, tube_radius: f32) -> f32 {
    return distance_to_circle(
        vec2(distance_to_circle(position.xy, ring_radius), position.z),
        tube_radius,
    );
}


/**
 * Compute the min distance from a point to a capped torus.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg ring_radius: The radius (xy-plane) of the ring of the torus.
 * @arg tube_radius: The radius of the tube of the torus.
 * @arg cap_angle: The angle (xy-plane, symmetric about y-axis) to cap
 *     at, in the range (0-PI).
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_capped_torus(
    position: vec3<f32>,
    ring_radius: f32,
    tube_radius: f32,
    cap_angle: f32,
) -> f32 {
    var cap_direction = vec2(sin(cap_angle), cos(cap_angle));
    var abs_x_position = vec3(abs(position.x), position.yz);

    var cap_factor: f32;
    if (cap_direction.y * abs_x_position.x > cap_direction.x * abs_x_position.y) {
        // project position on xy-plane onto the direction we are capping at
        cap_factor = dot(abs_x_position.xy, cap_direction.xy);
    }
    else {
        // distance to z-axis from position
        cap_factor = length(abs_x_position.xy);
    }

    return sqrt(
        dot2_vec3f(abs_x_position)
        + ring_radius * ring_radius
        - 2. * ring_radius * cap_factor
    ) - tube_radius;
}


/**
 * Compute the min distance from a point to a chain link.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg ring_radius: The radius (xy-plane) of the ring of the torus that
 *     will be stretched to create the link.
 * @arg tube_radius: The radius of the tube that makes the link.
 * @arg height: The height (y-axis) to elongate the torus.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_link(
    position: vec3<f32>,
    ring_radius: f32,
    tube_radius: f32,
    height: f32,
) -> f32 {
    var height_difference: f32 = abs(position.y) - height / 2.;

    var distance_in_xy_plane: f32 = distance_to_circle(
        vec2(position.x, positive_part_f32(height_difference)),
        ring_radius,
    );
    return distance_to_circle(
        vec2(distance_in_xy_plane, position.z),
        tube_radius,
    );
}


/**
 * Compute the min distance from a point to a hexagonal prism.
 * The hexagonal face is parallel to the xy-plane, centered at the
 * origin.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg height: The height (y) of the prism.
 * @arg depth: The depth (z) of the prism.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_hexagonal_prism(position: vec3<f32>, height: f32, depth: f32) -> f32 {
    // precomputed -cos(-PI / 6.), -sin(-PI / 6.), -tan(-PI / 6.)
    var cos_sin_tan = vec3(-0.86602540378, 0.5, 0.57735026919);
    var half_height: f32 = height / 2.;

    var abs_position: vec3<f32> = abs(position);
    abs_position += vec3(
        2. * cos_sin_tan.xy * negative_part_f32(dot(cos_sin_tan.xy, abs_position.xy)),
        0.,
    );

    // Radial distance in xy-plane, and the distance along the z-axis
    var radial_and_z_distance = vec2(
        sign(abs_position.y - half_height) * length(
            abs_position.xy
            - vec2(
                clamp(abs_position.x, -cos_sin_tan.z * half_height, cos_sin_tan.z * half_height),
                half_height,
            ),
        ),
        abs_position.z - depth / 2.,
    );

    // Return the positive distance if we are outside, negative if we are inside
    return sdf_length_vec2f(radial_and_z_distance);
}


/**
 * Compute the min distance from a point to a octahedron.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radial_extent: The maximum distance along the x, y, and z axes.
 *     ie. The vertices are at +/-radial_extent on the x, y, and z axes.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_octahedron(position: vec3<f32>, radial_extent: f32) -> f32 {
    var abs_position: vec3<f32> = abs(position);

    var position_sum_to_extent: f32 = dot(abs_position, vec3(1.)) - radial_extent;

    var three_position: vec3<f32> = 3. * abs_position;
    var change_of_axes: vec3<f32>;
    if (three_position.x < position_sum_to_extent)
    {
        change_of_axes = abs_position;
    }
    else if (three_position.y < position_sum_to_extent)
    {
        change_of_axes = abs_position.yzx;
    }
    else if (three_position.z < position_sum_to_extent)
    {
        change_of_axes = abs_position.zxy;
    }
    else
    {
        return position_sum_to_extent * 0.57735027;
    }

    var surface: f32 = clamp(
        0.5 * (change_of_axes.z - change_of_axes.y + radial_extent),
        0.,
        radial_extent,
    );

    return length(vec3(
        change_of_axes.x,
        change_of_axes.y - radial_extent + surface,
        change_of_axes.z - surface,
    ));
}


/**
 * Compute the min distance from a point to a mandelbulb.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg power: One greater than the axes of symmetry in the xy-plane.
 * @arg iterations: The number of iterations to compute, the higher this
 *     is the slower it will be to compute, but the deeper the fractal
 *     will have detail.
 * @arg max_square_radius: When the square radius has reached this length,
 *     stop iterating.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_mandelbulb(
    position: vec3<f32>,
    power: f32,
    iterations: i32,
    max_square_radius: f32,
    trap_colour: ptr<function, vec3<f32>>,
) -> f32 {
    var current_position: vec3<f32> = position;
    var radius_squared: f32 = dot2_vec3f(current_position);

    var abs_position: vec3<f32> = abs(current_position);
    *trap_colour = abs_position;

    var dradius: f32 = 1.;
    for (var iteration=0; iteration < iterations; iteration++)
    {
        dradius = power * pow(radius_squared, (power - 1.) / 2.) * dradius + 1.;

        var current_radius: f32 = length(current_position);
        var theta: f32 = power * acos(current_position.z / current_radius);
        var phi: f32 = power * atan2(current_position.y, current_position.x);

        current_position = position + pow(current_radius, power) * vec3(
            sin(theta) * cos(phi),
            sin(theta) * sin(phi),
            cos(theta),
        );

        abs_position = abs(current_position);
        *trap_colour = min(*trap_colour, abs_position);

        radius_squared = dot2_vec3f(current_position);
        if(radius_squared > max_square_radius) {
            break;
        }
    }

    *trap_colour = saturate_vec3f(*trap_colour);

    return 0.25 * log(radius_squared) * sqrt(radius_squared) / dradius;
}


fn box_fold(position: vec3<f32>, folding_limit: vec3<f32>) -> vec3<f32> {
    return clamp(position, -folding_limit, folding_limit) * 2. - position;
}


fn sphere_fold(
    position: vec4<f32>,
    radius_squared: f32,
    min_square_radius: f32,
) -> vec4<f32> {
    return position * saturate_f32(
        max(min_square_radius / radius_squared, min_square_radius),
    );
}


/**
 * Compute the min distance from a point to a mandelbox.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg scale:
 * @arg iterations: The number of iterations to compute, the higher this
 *     is the slower it will be to compute, but the deeper the fractal
 *     will have detail.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_mandelbox(
    position: vec3<f32>,
    scale: f32,
    iterations: i32,
    min_square_radius: f32,
    folding_limit: f32,
    trap_colour: ptr<function, vec3<f32>>,
) -> f32 {
    var scale_vector = vec4(scale, scale, scale, abs(scale)) / min_square_radius;
    var initial_position = vec4(position, 1.);
    var current_position: vec4<f32> = initial_position;

    var folding_limit_vec3f = vec3(folding_limit);

    for (var iteration=0; iteration < iterations; iteration++)
    {
        var folded_position = box_fold(current_position.xyz, folding_limit_vec3f);

        var radius_squared: f32 = dot2_vec3f(folded_position);
        current_position = sphere_fold(
            vec4(folded_position, current_position.w),
            radius_squared,
            min_square_radius
        );

        current_position = scale_vector * current_position + initial_position;
        *trap_colour = min(*trap_colour, abs(current_position.xyz));
    }

    *trap_colour = saturate_vec3f(*trap_colour);

    return (
        length(current_position.xyz - abs(scale - 1.)) / current_position.w
        - pow(abs(scale), f32(1 - iterations))
    );
}

// materials/material.wgsl


struct Material {
    diffuse_probability: f32,
    diffuse_colour: vec3<f32>,
    specular_probability: f32,
    specular_roughness: f32,
    specular_colour: vec3<f32>,
    transmissive_probability: f32,
    transmissive_roughness: f32,
    transmissive_colour: vec3<f32>,
    emissive_probability: f32,
    emissive_colour: vec3<f32>,
    refractive_index: f32,
    scattering_coefficient: f32,
    scattering_colour: vec3<f32>,
}


/**
 * Perform material sampling.
 *
 * @arg seed: The seed to use in randomization.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the material of.
 * @arg incident_direction: The incoming ray direction.
 * @arg offset: The amount to offset the ray in order to escape the
 *     surface.
 * @arg material: The material properties of the surface.
 * @arg position: The position on the surface to sample the
 *     material of.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the material of.
 * @arg outgoing_direction: The direction the ray will travel after
 *     sampling the material.
 * @arg light_pdf: The PDF of the material we are sampling from the
 *     perspective of the light we will be sampling.
 *
 * @returns: The material PDF.
 */
fn sample_material(
    seed: vec3<f32>,
    surface_normal: vec3<f32>,
    incident_direction: vec3<f32>,
    offset: f32,
    material: ptr<function, Material>,
    position: ptr<function, vec3<f32>>,
    material_brdf: ptr<function, vec3<f32>>,
    outgoing_direction: ptr<function, vec3<f32>>,
    light_pdf: ptr<function, f32>,
) -> f32 {
    var diffuse_direction: vec3<f32> = cosine_direction_in_hemisphere(
        seed.xy,
        surface_normal,
    );

    var specular_probability: f32 = (*material).specular_probability;
    var transmissive_probability: f32 = (*material).transmissive_probability;

    // TODO fresnel

    var rng: f32 = vec3f_to_random_f32(seed);
    var material_pdf: f32;
    if ((*material).specular_probability > 0. && rng <= specular_probability) {
        // Specular bounce
        var ideal_specular_direction: vec3<f32> = reflect(
            incident_direction,
            surface_normal,
        );

        *outgoing_direction = normalize(mix(
            ideal_specular_direction,
            diffuse_direction,
            (*material).specular_roughness * (*material).specular_roughness,
        ));

        // Offset the point so that it doesn't get trapped on the surface.
        *position += offset * surface_normal;

        *material_brdf = (*material).specular_colour;

        var probability_over_pi = (*material).specular_probability / PI;
        *light_pdf = 0.;
        return probability_over_pi * dot(ideal_specular_direction, *outgoing_direction);
    }
    else if (
        (*material).transmissive_probability > 0.
        && rng <= transmissive_probability
    ) {
        // Transmissive bounce
        return 1.;
    }
    else {
        // Diffuse bounce
        *outgoing_direction = diffuse_direction;

        // Offset the point so that it doesn't get trapped on the surface.
        *position += offset * surface_normal;

        *material_brdf = (*material).diffuse_colour;

        var probability_over_pi = (*material).diffuse_probability / PI;
        *light_pdf = probability_over_pi;
        return probability_over_pi * dot(diffuse_direction, surface_normal);
    }
}

// geometry/modifications.wgsl

// geometry/geometry.wgsl
// #include "material.wgsl"


let MAX_PRIMITIVES: u32 = 512u; // const not supported in the current version


struct Transform {
    translation: vec3<f32>,
    inverse_rotation: mat3x3<f32>,
    uniform_scale: f32,
}


struct Primitive {
    shape: u32,
    transform: Transform, // Could we just make this a world matrix?
    material: Material,
    modifiers: u32,
    blend_strength: f32,
    num_children: u32,
    custom_data: vec4<f32>,
}


struct Primitives {
    primitives: array<Primitive, MAX_PRIMITIVES>,
}


@group(2) @binding(0)
var<storage, read> _primitives: Primitives;



/**
 * Transform a ray's location.
 *
 * @arg ray_origin: The location the ray originates from.
 * @arg position: The amount to translate the ray.
 * @arg rotation: The amount to rotate the ray (radians).
 * @arg modifications: The modifications to perform.
 *     Each bit will enable a modification:
 *         bit 0: finite repetition
 *         bit 1: infinite repetition
 *         bit 2: elongation
 *         bit 3: mirror x
 *         bit 4: mirror y
 *         bit 5: mirror z
 * @arg repetition: The values to use when repeating the ray.
 * @arg elongation: The values to use when elongating the ray.
 *
 * @returns: The transformed ray origin.
 */
fn transform_ray(ray_origin: vec3<f32>, transform: Transform) -> vec3<f32> {
    var transformed_ray: vec3<f32> = (
        transform.inverse_rotation
        * (ray_origin - transform.translation)
    );
    // perform_shape_modification(
    //     modifications,
    //     repetition,
    //     elongation,
    //     transformed_ray
    // );

    return transformed_ray;
}


/**
 * Compute the min distance from a point to a geometric object.
 *
 * @arg position: The point to get the distance to, from the primitive.
 * @arg primitive: The primitive to get the distance to.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_primitive(
    position: vec3<f32>,
    primitive: ptr<function, Primitive>,
) -> f32 {
    var scaled_position = position / (*primitive).transform.uniform_scale;

    var distance: f32;
    switch (*primitive).shape {
        case 1u { // would be nice if const existed in this version :(
            distance = distance_to_ellipsoid(scaled_position,(*primitive).custom_data.xyz);
        }
        case 2u {
            distance = distance_to_cut_sphere(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
            );
        }
        case 3u {
            distance = distance_to_hollow_sphere(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 4u {
            distance = distance_to_death_star(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 5u {
            distance = distance_to_solid_angle(
                scaled_position,
                (*primitive).custom_data.x,
                radians((*primitive).custom_data.y),
            );
        }
        case 6u {
            distance = distance_to_rectangular_prism(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 7u {
            distance = distance_to_rectangular_prism_frame(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
                (*primitive).custom_data.w,
            );
        }
        case 8u {
            distance = distance_to_rhombus(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
                (*primitive).custom_data.w,
            );
        }
        case 9u {
            distance = distance_to_triangular_prism(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
            );
        }
        case 10u {
            distance = distance_to_cylinder(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
            );
        }
        case 11u {
            distance = distance_to_infinite_cylinder(
                scaled_position,
                (*primitive).custom_data.x,
            );
        }
        case 12u {
            distance = distance_to_plane(
                scaled_position,
                normalize((*primitive).custom_data.xyz),
            );
        }
        case 13u {
            distance = distance_to_capsule(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 14u {
            distance = distance_to_cone(
                scaled_position,
                radians((*primitive).custom_data.x),
                (*primitive).custom_data.y,
            );
        }
        case 15u {
            distance = distance_to_infinite_cone(
                scaled_position,
                radians((*primitive).custom_data.x),
            );
        }
        case 16u {
            distance = distance_to_capped_cone(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 17u {
            distance = distance_to_rounded_cone(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 18u {
            distance = distance_to_torus(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
            );
        }
        case 19u {
            distance = distance_to_capped_torus(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                radians((*primitive).custom_data.z),
            );
        }
        case 20u {
            distance = distance_to_link(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
                (*primitive).custom_data.z,
            );
        }
        case 21u {
            distance = distance_to_hexagonal_prism(
                scaled_position,
                (*primitive).custom_data.x,
                (*primitive).custom_data.y,
            );
        }
        case 22u {
            distance = distance_to_octahedron(
                scaled_position,
                (*primitive).custom_data.x,
            );
        }
        case 23u {
            var colour = vec3(1.);
            distance = distance_to_mandelbulb(
                scaled_position,
                (*primitive).custom_data.x,
                i32((*primitive).custom_data.y),
                (*primitive).custom_data.z,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
        case 24u {
            var colour = vec3(1.);
            distance = distance_to_mandelbox(
                scaled_position,
                (*primitive).custom_data.x,
                i32((*primitive).custom_data.y),
                (*primitive).custom_data.z,
                (*primitive).custom_data.w,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
        default { // cannot use default w/ other clauses, maybe version too old
            distance = distance_to_sphere(scaled_position, (*primitive).custom_data.x);
        }
    }

    return distance * (*primitive).transform.uniform_scale;

    // TODO
    // return perform_distance_modification(
    //     distance * uniform_scale,
    //     primitive,
    // );
}


fn min_distance_to_primitive(
    ray_origin: vec3<f32>,
    pixel_footprint: f32,
    material: ptr<function, Material>,
) -> f32 {
    var min_distance: f32 = _render_params.ray_marcher.max_distance;

    for (
        var primitive_index = 0u;
        primitive_index < min(_render_params.scene.num_primitives, MAX_PRIMITIVES);
        primitive_index++
    ) {
        var primitive: Primitive = _primitives.primitives[primitive_index];

        var transformed_ray: vec3<f32> = transform_ray(ray_origin, primitive.transform);
        var distance_to_current: f32 = distance_to_primitive(
            transformed_ray,
            &primitive,
        );

        if (abs(distance_to_current) < abs(min_distance)) {
            min_distance = distance_to_current;
            *material = primitive.material;
        }
    }

    return min_distance;
}


/**
 * Estimate the surface normal at the closest point on the closest
 * object to a point.
 *
 * @arg position: The point near which to get the surface normal
 * @arg pixel_footprint: A value proportional to the amount of world
 *     space that fills a pixel, like the distance from camera.
 *
 * @returns: The normalized surface normal.
 */
fn estimate_surface_normal(position: vec3<f32>, pixel_footprint: f32) -> vec3<f32> {
    var material: Material;
    var normal_offset = vec2(0.5773, -0.5773);
    return normalize(
        normal_offset.xyy * min_distance_to_primitive(
            position + normal_offset.xyy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
            &material,
        )
        + normal_offset.yyx * min_distance_to_primitive(
            position + normal_offset.yyx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
            &material,
        )
        + normal_offset.yxy * min_distance_to_primitive(
            position + normal_offset.yxy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
            &material,
        )
        + normal_offset.xxx * min_distance_to_primitive(
            position + normal_offset.xxx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
            &material,
        )
    );
}

// lights.wgsl


let MAX_LIGHTS: u32 = 512u; // const not supported in the current version


struct Light {
    light_type: u32,
    dimensional_data: vec3<f32>,
    intensity: f32,
    falloff: f32,
    colour: vec3<f32>,
    shadow_hardness: f32,
    soften_shadows: u32,
}

struct Lights {
    lights: array<Light, MAX_LIGHTS>,
}

@group(3) @binding(0)
var<storage, read> _lights: Lights;

/**
 * Perform multiple importance sampling by combining probability
 * distribution functions.
 *
 * @arg emittance: The emissive values of the surface.
 * @arg throughput: The throughput of the ray.
 * @arg pdf_0: The first PDF.
 * @arg pdf_1: The second PDF.
 *
 * @returns: The multiple importance sampled colour.
 */
fn multiple_importance_sample(
    emittance: vec3<f32>,
    throughput: vec3<f32>,
    pdf_0: f32,
    pdf_1: f32,
) -> vec3<f32> {
    return emittance * throughput * balance_heuristic(pdf_0, pdf_1);
}


/**
 * Get the probability distribution function for the lights in the
 * scene.
 *
 * @arg num_lights: The number of lights in the scene.
 * @arg visible_surface_area: The surface area that is visible to the
 *     position we are sampling from.
 *
 * @returns: The probability distribution function.
 */
fn sample_lights_pdf(num_lights: f32, visible_surface_area: f32) -> f32 {
    if (visible_surface_area == 0.) {
        return 1. / num_lights;
    }
    else {
        return 1. / num_lights / visible_surface_area;
    }
}


/**
 * Get the direction, and distance of a directional light.
 *
 * @arg direction: The direction the light is travelling.
 * @arg light_direction: Will store the direction to the light.
 * @arg distance_to_light: Will store the distance to the light.
 * @arg visible_surface_area: The surface area that is visible to the
 *     position we are sampling from.
 */
fn directional_light_data(
    light: ptr<function, Light>,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
    visible_surface_area: ptr<function, f32>,
) {
    *visible_surface_area = TWO_PI;
    *distance_to_light = _render_params.ray_marcher.max_distance;
    *light_direction = normalize(-(*light).dimensional_data);
}


/**
 * Get the direction, and distance of a point light.
 *
 * @arg point_on_surface: The point on the surface to compute the
 *     light intensity at.
 * @arg position: The position of the light.
 * @arg surface_position: Will store the direction to the light.
 * @arg distance_to_light: Will store the distance to the light.
 * @arg visible_surface_area: The surface area that is visible to the
 *     position we are sampling from.
 */
fn point_light_data(
    light: ptr<function, Light>,
    surface_position: vec3<f32>,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
    visible_surface_area: ptr<function, f32>,
) {
    *visible_surface_area = 0.;
    *light_direction = (*light).dimensional_data - surface_position;
    *distance_to_light = length(*light_direction);
    *light_direction = normalize(*light_direction);
}


/**
 * Sample the data of a particular artificial light in the scene.
 * Artificial lights are any that are passed into the 'lights'
 * input.
 *
 * @arg surface_position: The position on the surface to sample the
 *     data of.
 * @arg light_index: The index of the chosen light in the lights
 *     texture.
 * @arg num_non_physical_lights: The number of lights in the scene.
 * @arg light_direction: The direction from the surface to the light.
 * @arg distance_to_light: The distance to the light's surface.
 *
 * @returns: The PDF of the light.
 */
fn sample_non_physical_light_data(
    surface_position: vec3<f32>,
    light_index: u32,
    num_non_physical_lights: u32,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
) -> f32 {
    var visible_surface_area: f32 = 1.;
    var light: Light = _lights.lights[light_index];

    switch light.light_type {
        case 0u {
            directional_light_data(
                &light,
                light_direction,
                distance_to_light,
                &visible_surface_area,
            );
        }
        case 1u {
            point_light_data(
                &light,
                surface_position,
                light_direction,
                distance_to_light,
                &visible_surface_area,
            );
        }
        default {}
    }

    return sample_lights_pdf(f32(max(1u, num_non_physical_lights)), visible_surface_area);
}


/**
 * Sample the data of a particular light in the scene.
 *
 * @arg seed: The seed to use in randomization.
 * @arg position: The position on the surface to sample the data of.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the data of.
 * @arg num_non_physical_lights: The number of lights in the scene.
 * @arg light_id: The index of the chosen light to sample.
 * @arg light_direction: The direction from the surface to the light.
 * @arg distance_to_light: The distance to the light's surface.
 *
 * @returns: The PDF of the light.
 */
fn sample_light_data(
    seed: vec3<f32>,
    position: vec3<f32>,
    surface_normal: vec3<f32>,
    num_non_physical_lights: u32,
    light_id: u32,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
) -> f32 {
    // if (light_id < _lightTextureWidth)
    // {
    return sample_non_physical_light_data(
        position,
        light_id,
        num_non_physical_lights,
        light_direction,
        distance_to_light,
    );
    // }
    // else if (light_id - _lightTextureWidth < numEmissive)
    // {
    //     return samplePhysicalLightData(
    //         seed,
    //         position,
    //         emissiveIndices[light_id - _lightTextureWidth],
    //         numLights,
    //         light_direction,
    //         distance_to_light
    //     );
    // }

    // hdriLightData(
    //     seed * RAND_CONST_1,
    //     surface_normal,
    //     light_direction,
    //     distance_to_light
    // );
    // return sample_lights_pdf(max(1, numLights), 1.0f);
}


/**
 * Get the direction, distance, and intensity of a light.
 *
 * @arg intensity: The light intensity.
 * @arg falloff: The power of the falloff of the light.
 * @arg distance_to_light: The distance to the light.
 *
 * @returns: The light intensity.
 */
fn light_intensity(light: ptr<function, Light>, distance_to_light: f32) -> f32 {
    return (*light).intensity / pow(distance_to_light, (*light).falloff);
}


/**
 * Compute the ambient occlusion.
 *
 * @arg ray_origin: The origin of the ray.
 * @arg surface_normal: The normal to the surface.
 * @arg amount: The amount to scale the occlusion value by.
 * @arg iterations: The number of iterations to refine the
 *     occlusion.
 *
 * @returns: The occlusion value.
 */
fn sample_ambient_occlusion(
    ray_origin: vec3<f32>,
    surface_normal: vec3<f32>,
    amount: f32,
    iterations: u32,
) -> f32 {
    var material: Material;
    var occlusion: f32 = 0.;
    var occlusion_scale_factor: f32 = 1.;
    for (var iteration=0u; iteration < iterations; iteration++)
    {
        var step_distance: f32 = 0.001 + 0.15 * f32(iteration) / 4.;
        var distance_to_closest_object: f32 = abs(min_distance_to_primitive(
            ray_origin + step_distance * surface_normal,
            _render_params.ray_marcher.hit_tolerance,
            &material,
        ));
        occlusion += (step_distance - distance_to_closest_object) * occlusion_scale_factor;
        occlusion_scale_factor *= 0.95;
    }

    return (
        amount
        * saturate_f32(0.5 + 0.5 * surface_normal.y)  // ambient term
        * saturate_f32(1. - 1.5 * occlusion)          // occlusion term
    );
}


/**
 * Compute a soft shadow value.
 *
 * @arg ray_origin: The origin of the ray.
 * @arg ray_direction: The direction to cast the shadow ray.
 * @arg distance_to_shade_point: The maximum distance to check for
 *     a shadow casting object.
 * @arg hardness: The hardness of the shadow.
 *
 * @returns: The shadow intenstity.
 */
fn sample_soft_shadow(
    ray_origin: vec3<f32>,
    ray_direction: vec3<f32>,
    distance_to_shade_point: f32,
    hardness: f32,
) -> f32 {
    var material: Material;

    var distance_travelled: f32 = 0.;
    var shadow_intensity: f32 = 1.;
    var last_step_distance: f32 = 3.40282346638528859812e38; // FLT_MAX

    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_params.ray_marcher.hit_tolerance;

    var position: vec3<f32> = ray_origin;
    while (
        distance_travelled < distance_to_shade_point
        && iterations < _render_params.ray_marcher.max_ray_steps / 2u
    ) {
        var step_distance: f32 = abs(min_distance_to_primitive(
            position,
            pixel_footprint,
            &material,
        ));
        var step_distance_squared: f32 = step_distance * step_distance;
        var soft_offset: f32 = step_distance_squared / (2. * last_step_distance);
        shadow_intensity = min(
            shadow_intensity,
            hardness * sqrt(step_distance_squared - soft_offset * soft_offset)
            / positive_part_f32(distance_travelled - soft_offset),
        );

        if (step_distance < pixel_footprint) {
            shadow_intensity = saturate_f32(shadow_intensity);
            return shadow_intensity * shadow_intensity * (3. - 2. * shadow_intensity);
        }

        last_step_distance = step_distance;
        position += ray_direction * step_distance;
        distance_travelled += step_distance;
        pixel_footprint += step_distance * _render_params.ray_marcher.hit_tolerance;
        iterations++;
    }

    shadow_intensity = saturate_f32(shadow_intensity);
    return shadow_intensity * shadow_intensity * (3. - 2. * shadow_intensity);
}


/**
 * Compute a shadow value.
 *
 * @arg ray_origin: The origin of the ray.
 * @arg ray_direction: The direction to cast the shadow ray.
 * @arg distance_to_shade_point: The maximum distance to check for
 *     a shadow casting object.
 *
 * @returns: The shadow intenstity.
 */
fn sample_shadow(
    ray_origin: vec3<f32>,
    ray_direction: vec3<f32>,
    distance_to_shade_point: f32,
) -> f32 {
    var material: Material;

    var distance_travelled: f32 = 0.;
    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_params.ray_marcher.hit_tolerance;
    var position: vec3<f32> = ray_origin;
    while (
        distance_travelled < distance_to_shade_point
        && iterations < _render_params.ray_marcher.max_ray_steps / 2u
    ) {
        var step_distance: f32 = abs(min_distance_to_primitive(
            position,
            pixel_footprint,
            &material,
        ));

        if (step_distance < pixel_footprint) {
            return 0.;
        }

        position += ray_direction * step_distance;
        distance_travelled += step_distance;
        pixel_footprint += step_distance * _render_params.ray_marcher.hit_tolerance;
        iterations++;
    }

    return 1.;
}


/**
 * Perform direct illumination light sampling on a chosen artificial
 * light in the scene.
 *
 * @arg surface_position: The point on the surface to compute the
 *     light intensity at.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg light_direction: The direction from the surface to the light.
 * @arg distance_to_light: The distance to the light's surface.
 * @arg light_index: The index of the chosen light to sample.
 *
 * @returns: The colour of the sampled light.
 */
fn sample_non_physical_light(
    surface_position: vec3<f32>,
    surface_normal: vec3<f32>,
    light_direction: vec3<f32>,
    distance_to_light: f32,
    light_index: u32,
) -> vec3<f32> {
    // Read the light properties
    var light: Light = _lights.lights[light_index];

    var intensity: vec2<f32>;
    switch light.light_type {
        case 1u {
            var shadow_intensity_at_position: f32;
            if (bool(light.soften_shadows)) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                    light.shadow_hardness,
                );
            }
            else {
                shadow_intensity_at_position = sample_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                );
            }

            intensity = vec2(
                light_intensity(&light, distance_to_light),
                shadow_intensity_at_position,
            );
        }
        case 2u {
            // Ambient light, simply return the intensity.
            intensity = vec2(light.intensity, 1.);
        }
        case 3u {
            intensity = vec2(
                1.,
                sample_ambient_occlusion(
                    surface_position,
                    surface_normal,
                    light.intensity,
                    u32(light.dimensional_data.x)
                ),
            );
        }
        default {
            var shadow_intensity_at_position: f32;
            if (bool(light.soften_shadows)) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                    light.shadow_hardness,
                );
            }
            else {
                shadow_intensity_at_position = sample_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                );
            }

            intensity = vec2(light.intensity, shadow_intensity_at_position);
        }
    }

    return intensity.x * intensity.y * light.colour;
}


/**
 * Perform direct illumination light sampling on a chosen light in
 * the scene.
 *
 * @arg seed: The seed to use in randomization.
 * @arg throughput: The throughput of the ray.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the illumination of.
 * @arg distance_to_light: The distance to the light's surface.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg position: The position on the surface to sample the
 *     illumination of.
 * @arg light_direction: The direction from the surface to the light.
 * @arg light_pdf: The PDF of the light we are sampling the
 *     direct illumination of.
 * @arg material_pdf: The PDF of the material we are sampling the
 *     direct illumination of.
 * @arg light_id: The index of the chosen light to sample.
 * @arg numEmissive: The number of emissive objects in the scene.
 * @arg sampleHDRI: Whether or not to sample the HDRI. If there are
 *     lights in the scene this will increase the noise, but will be
 *     more accurate.
 * @arg nestedDielectrics: The stack of dielectrics that we have
 *     entered without exiting.
 * @arg numNestedDielectrics: The number of dielectrics in the
 *     stack.
 *
 * @returns: The colour of the sampled light.
 */
fn sample_light(
    seed: vec3<f32>,
    throughput: vec3<f32>,
    material_brdf: vec3<f32>,
    distance_to_light: f32,
    surface_normal: vec3<f32>,
    position: vec3<f32>,
    light_direction: vec3<f32>,
    light_pdf: f32,
    material_pdf: f32,
    light_id: u32,
) -> vec3<f32> {
    // float4 light_colour = float4(0);
    // float light_geometry_factor;

    // if (light_id < _lightTextureWidth)
    // {
    var light_colour = sample_non_physical_light(
        position,
        surface_normal,
        light_direction,
        distance_to_light,
        light_id,
    );
    var light_geometry_factor: f32 = saturate_f32(dot(light_direction, surface_normal));
    // }
    // else if (light_id - _lightTextureWidth - sampleHDRI < numEmissive)
    // {
    //     float actualDistance;
    //     float3 actualDirection = light_direction;
    //     float3 lightNormal;
    //     light_colour = marchPath(
    //         position,
    //         seed,
    //         sampleHDRI,
    //         distance_to_light * 2.0f,
    //         nestedDielectrics,
    //         numNestedDielectrics,
    //         numEmissive,
    //         position + distance_to_light * light_direction,
    //         actualDistance,
    //         actualDirection,
    //         lightNormal
    //     );
    //     light_geometry_factor = geometryFactor(
    //         actualDirection,
    //         lightNormal,
    //         actualDistance
    //     );
    // }

    return multiple_importance_sample(
        light_colour,
        throughput * material_brdf * light_geometry_factor / light_pdf,
        light_pdf,
        material_pdf * light_geometry_factor
    );
}


/**
 * Perform direct illumination light sampling on every light in the
 * scene.
 *
 * @arg seed: The seed to use in randomization.
 * @arg throughput: The throughput of the ray.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the illumination of.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg position: The position on the surface to sample the
 *     illumination of.
 * @arg material_pdf: The PDF of the material we are sampling the
 *     direct illumination of.
 *
 * @returns: The colour of the sampled light.
 */
fn sample_lights(
    seed: vec3<f32>,
    throughput: vec3<f32>,
    material_brdf: vec3<f32>,
    surface_normal: vec3<f32>,
    position: vec3<f32>,
    material_pdf: f32,
    num_non_physical_lights: u32,
) -> vec3<f32> {
    var light_colour = vec3(0.);

    for (var light_id=0u; light_id < num_non_physical_lights; light_id++) {
        var light_direction: vec3<f32> = surface_normal;
        var distance_to_light: f32 = 0.;

        var light_pdf: f32 = sample_light_data(
            seed * f32(light_id + 1u),
            position,
            surface_normal,
            num_non_physical_lights,
            light_id,
            &light_direction,
            &distance_to_light,
        );

        light_colour += sample_light(
            seed * f32(light_id + 2u),
            throughput,
            material_brdf,
            distance_to_light,
            surface_normal,
            position,
            light_direction,
            light_pdf,
            material_pdf,
            light_id,
        );
    }

    return light_colour;
}


/**
 * Perform direct illumination light sampling.
 *
 * @arg seed: The seed to use in randomization.
 * @arg throughput: The throughput of the ray.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the illumination of.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg position: The position on the surface to sample the
 *     illumination of.
 * @arg material_pdf: The PDF of the material we are sampling the
 *     direct illumination of.
 * @arg sample_all_lights: Whether to sample all the lights or one
 *     random one.
 *
 * @returns: The colour of the sampled light.
 */
fn light_sampling(
    seed: vec3<f32>,
    throughput: vec3<f32>,
    material_brdf: vec3<f32>,
    surface_normal: vec3<f32>,
    position: vec3<f32>,
    material_pdf: f32,
    sample_all_lights: bool,
    num_non_physical_lights: u32,
) -> vec3<f32> {
    if (sample_all_lights) {
        return sample_lights(
            seed,
            throughput,
            material_brdf,
            surface_normal,
            position,
            material_pdf,
            num_non_physical_lights,
        );
    }

    return vec3(0.);
}


// geometry/camera.wgsl
// #include "math.h"


struct Camera {
    enable_depth_of_field: u32, // bool isn't host-shareable?
    aperture: f32,
    focal_distance: f32,
    world_matrix: mat4x4<f32>,
    inverse_world_matrix: mat4x4<f32>,
    inverse_projection_matrix: mat4x4<f32>,
}


@group(1) @binding(0)
var<uniform> _render_camera: Camera;


fn world_to_camera_space(world_position: vec3<f32>) -> vec3<f32> {
    return (
        _render_camera.inverse_world_matrix
        * vec4(world_position, 1.)
    ).xyz;
}


/**
 * Generate a ray out of a camera.
 *
 * @arg uv_coordinate: The UV position in the resulting image.
 * @arg ray_origin: Will store the origin of the ray.
 * @arg ray_direction: Will store the direction of the ray.
 */
fn create_ray(
    uv_coordinate: vec4<f32>,
    ray_origin: ptr<function, vec3<f32>>,
    ray_direction: ptr<function, vec3<f32>>,
) {
    *ray_origin = vec3(
        _render_camera.world_matrix[3][0],
        _render_camera.world_matrix[3][1],
        _render_camera.world_matrix[3][2],
    );

    var direction: vec4<f32> = (
        _render_camera.inverse_projection_matrix
        * uv_coordinate
    );
    direction = _render_camera.world_matrix * vec4(direction.xyz, 0.);

    *ray_direction = normalize(direction.xyz);
}


/**
 * Create a ray out of the camera. It will be either a standard ray,
 * a latlong ray, or a ray that will result in depth of field.
 *
 * @arg seed: The seed to use in randomization.
 * @arg uv_coordinate: The u, and v locations of the pixel.
 * @arg ray_origin: The location to store the origin of the new ray.
 * @arg ray_direction: The location to store the direction of the new
 *     ray.
 */
fn create_render_camera_ray(
    seed: vec3<f32>,
    uv_coordinate: vec4<f32>,
    ray_origin: ptr<function, vec3<f32>>,
    ray_direction: ptr<function, vec3<f32>>,
) {
    if (bool(_render_params.ray_marcher.latlong))
    {
        // create_latlong_ray(
        //     uv_coordinate,
        //     ray_origin,
        //     ray_direction,
        // );
    }
    else if (bool(_render_params.ray_marcher.enable_depth_of_field))
    {
        // create_ray_with_dof(
        //     uv_coordinate,
        //     seed,
        //     ray_origin,
        //     ray_direction,
        // );
    }
    else
    {
        create_ray(
            uv_coordinate,
            ray_origin,
            ray_direction,
        );
    }
}

// aovs.wgsl


let BEAUTY_AOV: u32 = 0u;
let WORLD_POSITION_AOV: u32 = 1u;
let LOCAL_POSITION_AOV: u32 = 2u;
let NORMALS_AOV: u32 = 3u;
let DEPTH_AOV: u32 = 4u;


fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3<f32>,
    local_position: vec3<f32>,
    surface_normal: vec3<f32>,
) -> vec4<f32> {
    if (aov_type == WORLD_POSITION_AOV) {
        return vec4(world_position, 1.);
    }
    else if (aov_type == LOCAL_POSITION_AOV) {
        return vec4(local_position, 1.);
    }
    else if (aov_type == NORMALS_AOV) {
        return vec4(surface_normal, 1.);
    }
    else if (aov_type == DEPTH_AOV) {
        return vec4(abs(world_to_camera_space(world_position).z));
    }
    return vec4(-1.); // Invalid!!
}

// ray_march.wgsl


struct VertexOut {
    @location(0) uv_coordinate: vec4<f32>,
    @builtin(position) frag_coordinate: vec4<f32>,
}


var<private> v_positions: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    vec2<f32>(1., 1.),
    vec2<f32>(-1., 1.),
    vec2<f32>(1., -1.),
    vec2<f32>(-1., -1.),
);


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var out: VertexOut;
    out.frag_coordinate = vec4(v_positions[vertex_index], 0., 1.);
    out.uv_coordinate = vec4(v_positions[vertex_index], 0., 1.);

    return out;
}


/**
 * Handle the interaction between a ray and the surface of a material.
 *
 * @arg step_distance: The last step size to be marched.
 * @arg offset: The distance to offset the position in order to escape
 *     the surface.
 * @arg distance: The distance travelled since the last bounce.
 * @arg intersection_position: The position at which the ray
 *     intersects the geometry.
 * @arg surface_normal: The surface normal at the intersection point.
 * @arg num_lights: The number of lights in the scene.
 * @arg previous_material_pdf: The PDF of the last material interacted
 *     with.
 * @arg direction: The incoming ray direction.
 * @arg origin: The ray origin.
 * @arg ray_colour: The colour of the ray.
 * @arg throughput: The throughput of the ray.
 * @arg material: The material to interact with.
 *
 * @returns: The material pdf.
 */
fn material_interaction(
    seed: vec3<f32>,
    step_distance: f32,
    offset: f32,
    distance: f32,
    intersection_position: vec3<f32>,
    surface_normal: vec3<f32>,
    num_lights: f32,
    previous_material_pdf: f32,
    light_sampling_enabled: bool,
    sample_all_lights: bool,
    direction: ptr<function, vec3<f32>>,
    origin: ptr<function, vec3<f32>>,
    ray_colour: ptr<function, vec4<f32>>,
    throughput: ptr<function, vec3<f32>>,
    material: ptr<function, Material>,
) -> f32 {
    *origin = intersection_position;

    var material_brdf: vec3<f32>;
    var outgoing_direction: vec3<f32>;
    var materials_light_pdf: f32;
    var material_pdf: f32 = sample_material(
        seed,
        surface_normal,
        *direction,
        offset,
        material,
        origin,
        &material_brdf,
        direction,
        &materials_light_pdf,
    );

    if (light_sampling_enabled && materials_light_pdf > 0.) {
        var num_non_physical_lights = u32(num_lights);
        // Perform MIS light sampling
        *ray_colour += vec4(
            light_sampling(
                seed,
                *throughput,
                material_brdf,
                surface_normal,
                *origin + surface_normal * offset,
                materials_light_pdf,
                sample_all_lights,
                num_non_physical_lights,
            ),
            0.,
        );
    }

    var material_geometry_factor: f32;
    if (materials_light_pdf > 0.) {
        material_geometry_factor = saturate_f32(dot(outgoing_direction, surface_normal));
    }
    else {
        material_geometry_factor = 1.;
    }

    // TODO
    var radius: f32 = 1.;
    var visible_surface_area: f32 = TWO_PI * radius * radius;

    *ray_colour += vec4(
        multiple_importance_sample(
            (*material).emissive_colour * (*material).emissive_probability,
            *throughput,
            previous_material_pdf,
            sample_lights_pdf(num_lights, visible_surface_area),
        ),
        1.,
    );

    *throughput *= material_brdf * material_geometry_factor / material_pdf;

    return material_pdf;
}


/**
 * March a path through the scene.
 *
 * @arg ray_origin: The origin of the ray.
 * @arg ray_direction: The direction of the ray.
 *
 * @returns: The ray colour.
 */
fn march_path(
    ray_origin: vec3<f32>,
    ray_direction: vec3<f32>,
    seed: vec3<f32>,
) -> vec4<f32> {
    var path_seed: vec3<f32> = seed;
    var roulette = bool(_render_params.ray_marcher.roulette);
    var dynamic_level_of_detail = bool(_render_params.ray_marcher.dynamic_level_of_detail);

    var num_lights = f32(_render_params.scene.num_lights); // TODO Add emissive prims
    var light_sampling_enabled = (
        num_lights > 0.
        && _render_params.ray_marcher.max_light_sampling_bounces > 0u
    );
    var sample_all_lights = bool(_render_params.ray_marcher.sample_all_lights);

    var ray_colour = vec4(0.);
    var throughput = vec3(1.);

    var distance_travelled: f32 = 0.;
    var distance_since_last_bounce = 0.;

    var last_step_distance: f32 = 1.;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = _render_params.ray_marcher.hit_tolerance;

    var previous_material_pdf: f32 = 1.;

    // Data for the next ray
    var origin: vec3<f32> = ray_origin;
    var position_on_ray: vec3<f32> = origin;
    var direction: vec3<f32> = ray_direction;

    // March the ray
    while (
        distance_travelled < _render_params.ray_marcher.max_distance
        && iterations < _render_params.ray_marcher.max_ray_steps
        && sum_component_vec3f(throughput) > _render_params.ray_marcher.hit_tolerance
        && length(ray_colour) < _render_params.ray_marcher.max_brightness
    ) {
        position_on_ray = origin + distance_since_last_bounce * direction;


        var nearest_material: Material;
        // Keep the signed distance so we know whether or not we are inside the object
        var signed_step_distance = min_distance_to_primitive(
            position_on_ray,
            pixel_footprint,
            &nearest_material,
        );

        // Take the absolute value, the true shortest distance to a surface
        var step_distance = abs(signed_step_distance);

        // Keep track of the distance the ray has travelled
        distance_travelled += step_distance;
        distance_since_last_bounce += step_distance;

        // Have we hit the nearest object?
        if (step_distance < pixel_footprint) {
            bounces++;
            var intersection_position = position_on_ray + step_distance * direction;

            // The normal to the surface at that position
            var surface_normal: vec3<f32> = sign(last_step_distance) * estimate_surface_normal(
                intersection_position,
                pixel_footprint,
            );

            // Early exit for the various AOVs that are not 'beauty'
            if (_render_params.ray_marcher.output_aov > BEAUTY_AOV) {
                return early_exit_aovs(
                    _render_params.ray_marcher.output_aov,
                    intersection_position,
                    intersection_position, // TODO world to local
                    surface_normal,
                    // TODO object id
                );
            }

            previous_material_pdf = material_interaction(
                path_seed,
                step_distance,
                2. * pixel_footprint * _render_params.ray_marcher.shadow_bias,
                distance_since_last_bounce,
                intersection_position,
                surface_normal,
                num_lights,
                previous_material_pdf,
                light_sampling_enabled,
                sample_all_lights,
                &direction,
                &origin,
                &ray_colour,
                &throughput,
                &nearest_material,
            );

            // Exit if we have reached the bounce limit or with a random chance
            var rng: f32 = vec3f_to_random_f32(path_seed);
            var exit_probability: f32 = max_component_vec3f(throughput);
            if (
                bounces >= _render_params.ray_marcher.max_bounces
                || (roulette && exit_probability <= rng)
            ) {
                return ray_colour; // TODO object id in alpha after you can sample
            }
            else if (roulette) {
                // Account for the lost intensity from the early exits
                throughput /= vec3(exit_probability);
            }

            distance_since_last_bounce = 0.;
            // Reset the pixel footprint so multiple reflections don't reduce precision
            pixel_footprint = _render_params.ray_marcher.hit_tolerance;

            // Update the random seed for the next iteration
            path_seed = random_vec3f(path_seed.zxy + seed);
        }
        else if (dynamic_level_of_detail) {
            pixel_footprint += _render_params.ray_marcher.hit_tolerance * step_distance;
        }

        last_step_distance = signed_step_distance;
        iterations++;
    }

    return ray_colour;
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var frag_coord_seed = vec3(vec2f_to_random_f32(in.frag_coordinate.xy));
    var seed = random_vec3f(_render_params.ray_marcher.seeds + frag_coord_seed);
    var ray_colour = vec4(0.);

    var ray_origin: vec3<f32>;
    var ray_direction: vec3<f32>;
    for (var path=1u; path <= _render_params.ray_marcher.paths_per_pixel; path++) {
        create_render_camera_ray(
            seed,
            in.uv_coordinate,
            &ray_origin,
            &ray_direction,
        );

        ray_colour += march_path(
            ray_origin,
            ray_direction,
            seed,
        );

        seed = random_vec3f(seed.yzx + frag_coord_seed);
    }

    return ray_colour / f32(_render_params.ray_marcher.paths_per_pixel);
}
