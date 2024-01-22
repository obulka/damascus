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
    num_non_physical_lights: u32,
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


struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
    colour: vec3<f32>,
    throughput: vec3<f32>,
}


// math.wgsl

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

// const DIFFUSE_TRAP: u32 = 8192u;
// const SPECULAR_TRAP: u32 = 16384u;
// const EXTINCTION_TRAP: u32 = 32768u;
// const EMISSION_TRAP: u32 = 65536u;
// const SCATTERING_TRAP: u32 = 131072u;

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
    var transformed_position: vec3<f32> = position / radii;

    // If this length is < 1 we are inside the ellipsoid
    var scaled_length: f32 = length(transformed_position);

    return scaled_length * (scaled_length - 1.) / length(transformed_position / radii);
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

    if nearest_is_spherical < 0. {
        // Closest point is on the surface of the sphere
        return length(cylindrical_position) - radius;
    } else if cylindrical_position.x < cut_radius {
        // Closest point is within the cut surface
        return -cut_height + cylindrical_position.y;
    } else {
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

    return select(
        // Closest point is on the spherical surface
        abs(length(cylindrical_position) - radius) - half_thickness,
        // Closest point is on the rim
        length(cylindrical_position - vec2(cut_radius, cut_height)) - half_thickness,
        cut_height * cylindrical_position.x < cut_radius * cylindrical_position.y,
    );
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

    return select(
        max(
            // Closest point to the solid sphere
            length(cylindrical_position) - additive_sphere_radius,
            // Closest point to the hollowed portion
            subtractive_sphere_radius - length(
                cylindrical_position - vec2(0., subtractive_sphere_height)
            ),
        ),
        // Closest point is on the rim
        length(cylindrical_position - vec2(cut_radius, cut_height)),
        subtractive_sphere_height * positive_part_f32(cut_radius - cylindrical_position.x)
        < cylindrical_position.y * cut_radius - cylindrical_position.x * cut_height,
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
        dot(abs_position.xy, half_width_height.yx)
        - half_width_height.x * half_width_height.y,
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
    return inside * min_length_vec2f(
        cone_top_or_bottom_to_position,
        cone_edge_to_position,
    );
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

    if position_projected_on_cone < 0. {
        // Closest point is on the lower sphere
        return length(cylindrical_position) - lower_radius;
    } else if position_projected_on_cone > parallel_y * height {
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
    if cap_direction.y * abs_x_position.x > cap_direction.x * abs_x_position.y {
        // project position on xy-plane onto the direction we are capping at
        cap_factor = dot(abs_x_position.xy, cap_direction.xy);
    } else {
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
                clamp(
                    abs_position.x,
                    -cos_sin_tan.z * half_height,
                    cos_sin_tan.z * half_height,
                ),
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
    if three_position.x < position_sum_to_extent {
        change_of_axes = abs_position;
    } else if three_position.y < position_sum_to_extent {
        change_of_axes = abs_position.yzx;
    } else if three_position.z < position_sum_to_extent {
        change_of_axes = abs_position.zxy;
    } else {
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
fn distance_to_textured_mandelbulb(
    position: vec3<f32>,
    power: f32,
    iterations: u32,
    max_square_radius: f32,
    trap_colour: ptr<function, vec3<f32>>,
) -> f32 {
    var current_position: vec3<f32> = position;
    var radius_squared: f32 = dot2_vec3f(current_position);

    var abs_position: vec3<f32> = abs(current_position);
    *trap_colour = abs_position;

    var dradius: f32 = 1.;
    var iteration: u32 = 0u;
    loop {
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

        iteration++;
        if iteration >= iterations || radius_squared > max_square_radius {
            break;
        }
    }

    *trap_colour = saturate_vec3f(*trap_colour);

    return 0.25 * log(radius_squared) * sqrt(radius_squared) / dradius;
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
    iterations: u32,
    max_square_radius: f32,
) -> f32 {
    var current_position: vec3<f32> = position;
    var radius_squared: f32 = dot2_vec3f(current_position);

    var abs_position: vec3<f32> = abs(current_position);

    var dradius: f32 = 1.;
    var iteration: u32 = 0u;
    loop {
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

        radius_squared = dot2_vec3f(current_position);

        iteration++;
        if iteration >= iterations || radius_squared > max_square_radius {
            break;
        }
    }

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
fn distance_to_textured_mandelbox(
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
    }

    return (
        length(current_position.xyz - abs(scale - 1.)) / current_position.w
        - pow(abs(scale), f32(1 - iterations))
    );
}


// materials/material.wgsl

struct ProceduralTexture {
    texture_type: u32,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gamma: f32,
}

struct Material {
    diffuse_colour: vec3<f32>,
    diffuse_texture: ProceduralTexture,
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
 * Compute the schlick, simplified fresnel reflection coefficient.
 *
 * @arg incident_ray_direction: The incident direction.
 * @arg surface_normal_direction: The normal to the surface.
 * @arg incident_refractive_index: The refractive index the incident ray
 *     is travelling through.
 * @arg refracted_refractive_index: The refractive index the refracted ray
 *     will be travelling through.
 *
 * @returns: The reflection coefficient.
 */
fn schlick_reflection_coefficient(
    incident_ray_direction: vec3<f32>,
    surface_normal_direction: vec3<f32>,
    incident_refractive_index: f32,
    refracted_refractive_index: f32,
) -> f32 {
    var parallel_coefficient: f32 = (
        (incident_refractive_index - refracted_refractive_index)
        / (incident_refractive_index + refracted_refractive_index)
    );
    parallel_coefficient *= parallel_coefficient;

    var cos_x: f32 = -dot(surface_normal_direction, incident_ray_direction);
    if incident_refractive_index > refracted_refractive_index {
        var refractive_ratio: f32 = incident_refractive_index / refracted_refractive_index;
        var sin_transmitted_squared: f32 = refractive_ratio * refractive_ratio * (
            1. - cos_x * cos_x
        );
        if sin_transmitted_squared > 1. {
            return 1.;
        }
        cos_x = sqrt(1. - sin_transmitted_squared);
    }
    var one_minus_cos_x: f32 = 1. - cos_x;
    var one_minus_cos_x_squared: f32 = one_minus_cos_x * one_minus_cos_x;
    return (
        parallel_coefficient
        + (1. - parallel_coefficient)
        * one_minus_cos_x_squared
        * one_minus_cos_x_squared
        * one_minus_cos_x
    );
}


/**
 * Perform material sampling.
 *
 * @arg seed: The seed to use in randomization.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the material of.
 * @arg offset: The amount to offset the ray in order to escape the
 *     surface.
 * @arg material: The material properties of the surface.
 * @arg ray: The ray which has hit the surface with the above material.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the material of.
 * @arg light_sampling_pdf: The PDF of the material we are sampling from the
 *     perspective of the light we will be sampling.
 *
 * @returns: The material PDF.
 */
fn sample_material(
    seed: vec3<f32>,
    surface_normal: vec3<f32>,
    offset: f32,
    primitive: ptr<function, Primitive>,
    ray: ptr<function, Ray>,
    material_brdf: ptr<function, vec3<f32>>,
    light_sampling_pdf: ptr<function, f32>,
) -> f32 {
    var diffuse_direction: vec3<f32> = cosine_direction_in_hemisphere(
        seed.xy,
        surface_normal,
    );

    var specular_probability: f32 = (*primitive).material.specular_probability;
    var transmissive_probability: f32 = (*primitive).material.transmissive_probability;

    if specular_probability > 0. || transmissive_probability > 0. {
        // Adjust probabilities according to fresnel

        var incident_refractive_index: f32 = 1.; // TODO add nested dielectrics
        var refracted_refractive_index: f32 = (*primitive).material.refractive_index; // TODO ^

        // Compute the refraction values
        var reflectivity: f32 = schlick_reflection_coefficient(
            (*ray).direction,
            surface_normal,
            incident_refractive_index,
            refracted_refractive_index,
        );

        specular_probability = (
            (specular_probability + f32(transmissive_probability > 0.))
            * mix(specular_probability, 1., reflectivity)
        );

        transmissive_probability = (
            transmissive_probability * (1. - specular_probability)
            / (1. - (*primitive).material.specular_probability)
        );
    }

    // Interact with material according to the adjusted probabilities
    var rng: f32 = vec3f_to_random_f32(seed);
    var material_pdf: f32;
    if specular_probability > 0. && rng <= specular_probability {
        // Specular bounce
        var ideal_specular_direction: vec3<f32> = reflect(
            (*ray).direction,
            surface_normal,
        );

        (*ray).direction = normalize(mix(
            ideal_specular_direction,
            diffuse_direction,
            (*primitive).material.specular_roughness, // Assume roughness squared by CPU
        ));

        // Offset the point so that it doesn't get trapped on the surface.
        (*ray).origin += offset * surface_normal;

        *material_brdf = (*primitive).material.specular_colour;

        var probability_over_pi = specular_probability / PI;
        *light_sampling_pdf = 0.;
        return probability_over_pi * dot(ideal_specular_direction, (*ray).direction);
    } else if (
        (*primitive).material.transmissive_probability > 0.
        && rng <= transmissive_probability + specular_probability
    ) {
        // Transmissive bounce
        return 1.;
    } else {
        // Diffuse bounce
        (*ray).direction = diffuse_direction;

        // Offset the point so that it doesn't get trapped on the surface.
        (*ray).origin += offset * surface_normal;

        *material_brdf = (*primitive).material.diffuse_colour;

        var probability_over_pi = (1. - specular_probability - transmissive_probability) / PI;
        *light_sampling_pdf = probability_over_pi;
        return probability_over_pi * dot(diffuse_direction, surface_normal);
    }
}

// geometry/modifications.wgsl

// geometry/geometry.wgsl
// #include "material.wgsl"

const FINITE_REPETITION: u32 = 1u;
const INFINITE_REPETITION: u32 = 2u;
const ELONGATE: u32 = 4u;
const MIRROR_X: u32 = 8u;
const MIRROR_Y: u32 = 16u;
const MIRROR_Z: u32 = 32u;
const HOLLOW: u32 = 64u;
const BOUNDING_VOLUME: u32 = 4096u;

const MAX_PRIMITIVES: u32 = 512u; // const not supported in the current version


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
    negative_repetitions: vec3<f32>,
    positive_repetitions: vec3<f32>,
    spacing: vec3<f32>,
    blend_strength: f32,
    wall_thickness: f32,
    edge_radius: f32,
    elongation: vec3<f32>,
    num_descendants: u32,
    dimensional_data: vec4<f32>,
}


struct Primitives {
    primitives: array<Primitive, MAX_PRIMITIVES>,
}


@group(2) @binding(0)
var<storage, read> _primitives: Primitives;


/**
 * Finitely repeat an object in the positive quadrant.
 *
 * @arg position: The position of the ray.
 * @arg primitive: The primitive to repeat.
 *
 * @returns: The modified ray position that results in repetion.
 */
// fn symmetric_finite_repetition(
//     position: vec3<f32>,
//     primitive: ptr<function, Primitive>,
// ) -> vec3<f32> {
//     return (
//         position
//         - (*primitive).spacing
//         * clamp(
//             round(position / (*primitive).spacing),
//             -(*primitive).negative_repetitions,
//             (*primitive).positive_repetitions,
//         )
//     );
// }


/**
 * Finitely repeat an object in the positive quadrant.
 *
 * @arg position: The position of the ray.
 * @arg primitive: The primitive to repeat.
 *
 * @returns: The modified ray position that results in repetion.
 */
fn mirrored_finite_repetition(
    position: vec3<f32>,
    primitive: ptr<function, Primitive>,
) -> vec3<f32> {
    var space_partition_id: vec3<f32> = clamp(
        round(position / (*primitive).spacing),
        -(*primitive).negative_repetitions,
        (*primitive).positive_repetitions,
    );
    var repeated_position: vec3<f32> = position - (*primitive).spacing * space_partition_id;

    return select(
        -repeated_position,
        repeated_position,
        vec3<bool>(
            (i32(space_partition_id.x) & 1) == 0,
            (i32(space_partition_id.y) & 1) == 0,
            (i32(space_partition_id.z) & 1) == 0,
        ),
    );
}


/**
 * Infinitely repeat an object, mirroring with every repetition. By
 * mirroring we remove the constraint that the object must be symmetric
 * without repeating the distance check.
 *
 * @arg position: The position of the ray.
 * @arg primitive: The primitive to repeat.
 *
 * @returns: The modified ray position that results in repetion.
 */
fn mirrored_infinite_repetition(
    position: vec3<f32>,
    primitive: ptr<function, Primitive>,
) -> vec3<f32> {
    var space_partition_id: vec3<f32> = round(position / (*primitive).spacing);
    var repeated_position: vec3<f32> = position - (*primitive).spacing * space_partition_id;

    return select(
        -repeated_position,
        repeated_position,
        vec3<bool>(
            (i32(space_partition_id.x) & 1) == 0,
            (i32(space_partition_id.y) & 1) == 0,
            (i32(space_partition_id.z) & 1) == 0,
        ),
    );
}


/**
 * Modify the distance a ray has travelled, resulting in various
 * effects.
 *
 * @arg distance: The distance to the primitive without modification.
 * @arg primitive: The primitive to get the distance to.
 *
 * @returns: The modified distance to the primitive.
 */
fn modify_distance(distance: f32, primitive: ptr<function, Primitive>) -> f32 {
    return select(
        distance,
        abs(distance) - (*primitive).wall_thickness,
        bool((*primitive).modifiers & HOLLOW),
    ) - (*primitive).edge_radius;
}


/**
 * Transform a ray's location.
 *
 * @arg position: The location the ray originates from.
 * @arg primitive: The primitive which determines the transformation.
 *
 * @returns: The transformed ray origin.
 */
fn transform_position(
    position: vec3<f32>,
    primitive: ptr<function, Primitive>,
) -> vec3<f32> {
    var transformed_position: vec3<f32> = (
        (*primitive).transform.inverse_rotation
        * (position - (*primitive).transform.translation)
    );
    // Perform finite or infinite repetition if enabled
    transformed_position = select(
        select(
            transformed_position,
            mirrored_infinite_repetition(
                transformed_position,
                primitive,
            ),
            bool((*primitive).modifiers & INFINITE_REPETITION),
        ),
        mirrored_finite_repetition(
            transformed_position,
            primitive,
        ),
        bool((*primitive).modifiers & FINITE_REPETITION),
    );
    // Perform elongation if enabled
    transformed_position -= select(
        vec3(0.),
        clamp(
            transformed_position,
            -(*primitive).elongation,
            (*primitive).elongation,
        ),
        bool((*primitive).modifiers & ELONGATE),
    );
    // Perform mirroring if enabled
    return select(
        transformed_position,
        abs(transformed_position),
        vec3<bool>(
            bool((*primitive).modifiers & MIRROR_X),
            bool((*primitive).modifiers & MIRROR_Y),
            bool((*primitive).modifiers & MIRROR_Z),
        ),
    );
}


fn checkerboard(position: vec3<f32>) -> vec3<f32> {
    var square_signal: vec3<f32> = sign(fract(position * 0.5) - 0.5);
    return vec3(0.5 - 0.5 * square_signal.x * square_signal.y * square_signal.z);
}


fn procedurally_texture(
    position: vec3<f32>,
    colour: vec3<f32>,
    procedural_texture: ProceduralTexture,
) -> vec3<f32> {
    var textured_colour: vec3<f32> = colour;
    switch procedural_texture.texture_type {
        case 0u, default {
            return textured_colour;
        }
        case 1u {}
        case 2u {
            textured_colour *= checkerboard(position);
        }
    }
    return pow(
        (1. - procedural_texture.lift)
        * saturate_vec3f(textured_colour - procedural_texture.black_point)
        / (procedural_texture.white_point - procedural_texture.black_point)
        + procedural_texture.lift,
        vec3(1. / procedural_texture.gamma),
    );
}


/**
 * Compute the min distance from a point to a geometric object.
 *
 * @arg position: The point to get the distance to, from the primitive.
 * @arg primitive: The primitive to get the distance to.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_textured_primitive(
    position: vec3<f32>,
    primitive: ptr<function, Primitive>,
) -> f32 {
    var transformed_position: vec3<f32> = transform_position(
        position,
        primitive,
    ) / (*primitive).transform.uniform_scale;

    (*primitive).material.diffuse_colour = procedurally_texture(
        position,
        (*primitive).material.diffuse_colour,
        (*primitive).material.diffuse_texture,
    );

    var distance: f32;
    switch (*primitive).shape {
        case 0u, default { // cannot use const, maybe version too old
            distance = distance_to_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 1u {
            distance = distance_to_ellipsoid(
                transformed_position,
                (*primitive).dimensional_data.xyz,
            );
        }
        case 2u {
            distance = distance_to_cut_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 3u {
            distance = distance_to_hollow_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 4u {
            distance = distance_to_death_star(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 5u {
            distance = distance_to_solid_angle(
                transformed_position,
                (*primitive).dimensional_data.x,
                radians((*primitive).dimensional_data.y),
            );
        }
        case 6u {
            distance = distance_to_rectangular_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 7u {
            distance = distance_to_rectangular_prism_frame(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
        case 8u {
            distance = distance_to_rhombus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
        case 9u {
            distance = distance_to_triangular_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 10u {
            distance = distance_to_cylinder(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 11u {
            distance = distance_to_infinite_cylinder(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 12u {
            distance = distance_to_plane(
                transformed_position,
                normalize((*primitive).dimensional_data.xyz),
            );
        }
        case 13u {
            distance = distance_to_capsule(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 14u {
            distance = distance_to_cone(
                transformed_position,
                radians((*primitive).dimensional_data.x),
                (*primitive).dimensional_data.y,
            );
        }
        case 15u {
            distance = distance_to_infinite_cone(
                transformed_position,
                radians((*primitive).dimensional_data.x),
            );
        }
        case 16u {
            distance = distance_to_capped_cone(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 17u {
            distance = distance_to_rounded_cone(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 18u {
            distance = distance_to_torus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 19u {
            distance = distance_to_capped_torus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                radians((*primitive).dimensional_data.z),
            );
        }
        case 20u {
            distance = distance_to_link(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 21u {
            distance = distance_to_hexagonal_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 22u {
            distance = distance_to_octahedron(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 23u {
            var colour = vec3(1.);
            distance = distance_to_textured_mandelbulb(
                transformed_position,
                (*primitive).dimensional_data.x,
                u32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
        case 24u {
            var colour = vec3(1.);
            distance = distance_to_textured_mandelbox(
                transformed_position,
                (*primitive).dimensional_data.x,
                i32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
    }

    return modify_distance(distance * (*primitive).transform.uniform_scale, primitive);
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
    var transformed_position: vec3<f32> = transform_position(
        position,
        primitive,
    ) / (*primitive).transform.uniform_scale;

    var distance: f32;
    switch (*primitive).shape {
        case 0u, default { // cannot use const, maybe version too old
            distance = distance_to_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 1u {
            distance = distance_to_ellipsoid(
                transformed_position,
                (*primitive).dimensional_data.xyz,
            );
        }
        case 2u {
            distance = distance_to_cut_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 3u {
            distance = distance_to_hollow_sphere(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 4u {
            distance = distance_to_death_star(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 5u {
            distance = distance_to_solid_angle(
                transformed_position,
                (*primitive).dimensional_data.x,
                radians((*primitive).dimensional_data.y),
            );
        }
        case 6u {
            distance = distance_to_rectangular_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 7u {
            distance = distance_to_rectangular_prism_frame(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
        case 8u {
            distance = distance_to_rhombus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
        case 9u {
            distance = distance_to_triangular_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 10u {
            distance = distance_to_cylinder(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 11u {
            distance = distance_to_infinite_cylinder(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 12u {
            distance = distance_to_plane(
                transformed_position,
                normalize((*primitive).dimensional_data.xyz),
            );
        }
        case 13u {
            distance = distance_to_capsule(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 14u {
            distance = distance_to_cone(
                transformed_position,
                radians((*primitive).dimensional_data.x),
                (*primitive).dimensional_data.y,
            );
        }
        case 15u {
            distance = distance_to_infinite_cone(
                transformed_position,
                radians((*primitive).dimensional_data.x),
            );
        }
        case 16u {
            distance = distance_to_capped_cone(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 17u {
            distance = distance_to_rounded_cone(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 18u {
            distance = distance_to_torus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 19u {
            distance = distance_to_capped_torus(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                radians((*primitive).dimensional_data.z),
            );
        }
        case 20u {
            distance = distance_to_link(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
        case 21u {
            distance = distance_to_hexagonal_prism(
                transformed_position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
        case 22u {
            distance = distance_to_octahedron(
                transformed_position,
                (*primitive).dimensional_data.x,
            );
        }
        case 23u {
            distance = distance_to_mandelbulb(
                transformed_position,
                (*primitive).dimensional_data.x,
                u32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
            );
        }
        case 24u {
            distance = distance_to_mandelbox(
                transformed_position,
                (*primitive).dimensional_data.x,
                i32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
    }

    return modify_distance(distance * (*primitive).transform.uniform_scale, primitive);
}


fn mix_materials(
    primitive_0: ptr<function, Primitive>,
    primitive_1: ptr<function, Primitive>,
    smoothing: f32,
) {
    (*primitive_0).material.diffuse_colour = mix(
        (*primitive_0).material.diffuse_colour,
        (*primitive_1).material.diffuse_colour,
        smoothing,
    );
    (*primitive_0).material.specular_probability = mix(
        (*primitive_0).material.specular_probability,
        (*primitive_1).material.specular_probability,
        smoothing,
    );
    (*primitive_0).material.specular_roughness = mix(
        (*primitive_0).material.specular_roughness,
        (*primitive_1).material.specular_roughness,
        smoothing,
    );
    (*primitive_0).material.specular_colour = mix(
        (*primitive_0).material.specular_colour,
        (*primitive_1).material.specular_colour,
        smoothing,
    );
    (*primitive_0).material.transmissive_probability = mix(
        (*primitive_0).material.transmissive_probability,
        (*primitive_1).material.transmissive_probability,
        smoothing,
    );
    (*primitive_0).material.transmissive_roughness = mix(
        (*primitive_0).material.transmissive_roughness,
        (*primitive_1).material.transmissive_roughness,
        smoothing,
    );
    (*primitive_0).material.transmissive_colour = mix(
        (*primitive_0).material.transmissive_colour,
        (*primitive_1).material.transmissive_colour,
        smoothing,
    );
    (*primitive_0).material.emissive_probability = mix(
        (*primitive_0).material.emissive_probability,
        (*primitive_1).material.emissive_probability,
        smoothing,
    );
    (*primitive_0).material.emissive_colour = mix(
        (*primitive_0).material.emissive_colour,
        (*primitive_1).material.emissive_colour,
        smoothing,
    );
    (*primitive_0).material.refractive_index = mix(
        (*primitive_0).material.refractive_index,
        (*primitive_1).material.refractive_index,
        smoothing,
    );
    (*primitive_0).material.scattering_coefficient = mix(
        (*primitive_0).material.scattering_coefficient,
        (*primitive_1).material.scattering_coefficient,
        smoothing,
    );
    (*primitive_0).material.scattering_colour = mix(
        (*primitive_0).material.scattering_colour,
        (*primitive_1).material.scattering_colour,
        smoothing,
    );
    (*primitive_1).material = (*primitive_0).material;
}


fn select_material(
    primitive_0: ptr<function, Primitive>,
    primitive_1: ptr<function, Primitive>,
    choice: bool,
) {
    (*primitive_0).material.diffuse_colour = select(
        (*primitive_0).material.diffuse_colour,
        (*primitive_1).material.diffuse_colour,
        choice,
    );
    (*primitive_0).material.specular_probability = select(
        (*primitive_0).material.specular_probability,
        (*primitive_1).material.specular_probability,
        choice,
    );
    (*primitive_0).material.specular_roughness = select(
        (*primitive_0).material.specular_roughness,
        (*primitive_1).material.specular_roughness,
        choice,
    );
    (*primitive_0).material.specular_colour = select(
        (*primitive_0).material.specular_colour,
        (*primitive_1).material.specular_colour,
        choice,
    );
    (*primitive_0).material.transmissive_probability = select(
        (*primitive_0).material.transmissive_probability,
        (*primitive_1).material.transmissive_probability,
        choice,
    );
    (*primitive_0).material.transmissive_roughness = select(
        (*primitive_0).material.transmissive_roughness,
        (*primitive_1).material.transmissive_roughness,
        choice,
    );
    (*primitive_0).material.transmissive_colour = select(
        (*primitive_0).material.transmissive_colour,
        (*primitive_1).material.transmissive_colour,
        choice,
    );
    (*primitive_0).material.emissive_probability = select(
        (*primitive_0).material.emissive_probability,
        (*primitive_1).material.emissive_probability,
        choice,
    );
    (*primitive_0).material.emissive_colour = select(
        (*primitive_0).material.emissive_colour,
        (*primitive_1).material.emissive_colour,
        choice,
    );
    (*primitive_0).material.refractive_index = select(
        (*primitive_0).material.refractive_index,
        (*primitive_1).material.refractive_index,
        choice,
    );
    (*primitive_0).material.scattering_coefficient = select(
        (*primitive_0).material.scattering_coefficient,
        (*primitive_1).material.scattering_coefficient,
        choice,
    );
    (*primitive_0).material.scattering_colour = select(
        (*primitive_0).material.scattering_colour,
        (*primitive_1).material.scattering_colour,
        choice,
    );
    (*primitive_1).material = (*primitive_0).material;
}

fn blend_primitives(
    distance_to_parent: f32,
    distance_to_child: f32,
    parent: ptr<function, Primitive>,
    child: ptr<function, Primitive>,
) -> f32 {
    switch (*parent).modifiers & 3968u {
        case 128u {
            // Subtraction
            var negative_child_distance: f32 = -distance_to_child;
            var parent_closer_than_negative_child: bool = (
                negative_child_distance > distance_to_parent
            );
            select_material(parent, child, parent_closer_than_negative_child);
            return select(
                distance_to_parent,
                negative_child_distance,
                parent_closer_than_negative_child,
            );
        }
        case 256u {
            // Intersection
            var parent_closest: bool = distance_to_parent < distance_to_child;
            select_material(parent, child, parent_closest);
            return select(distance_to_parent, distance_to_child, parent_closest);
        }
        case 512u {
            // Smooth Union
            var smoothing: f32 = saturate_f32(
                0.5
                + 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            mix_materials(child, parent, smoothing);
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) - (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        case 1024u {
            // Smooth Subtraction
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_parent + distance_to_child)
                / (*parent).blend_strength
            );
            mix_materials(parent, child, smoothing);
            return mix(
                distance_to_parent,
                -distance_to_child,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        case 2048u {
            // Smooth Intersection
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            mix_materials(child, parent, smoothing);
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        default {
            // Union
            var child_closest: bool = distance_to_child < distance_to_parent;
            select_material(parent, child, child_closest);
            return select(distance_to_parent, distance_to_child, child_closest);
        }
    }
}


fn blend_distances(
    distance_to_parent: f32,
    distance_to_child: f32,
    parent: ptr<function, Primitive>,
) -> f32 {
    switch (*parent).modifiers & 3968u {
        case 128u {
            // Subtraction
            return max(distance_to_parent, -distance_to_child);
        }
        case 256u {
            // Intersection
            return max(distance_to_parent, distance_to_child);
        }
        case 512u {
            // Smooth Union
            var smoothing: f32 = saturate_f32(
                0.5
                + 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) - (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        case 1024u {
            // Smooth Subtraction
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_parent + distance_to_child)
                / (*parent).blend_strength
            );
            return mix(
                distance_to_parent,
                -distance_to_child,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        case 2048u {
            // Smooth Intersection
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
        default {
            // Union
            return min(distance_to_parent, distance_to_child);
        }
    }
}


fn distance_to_descendants_with_primitive(
    position: vec3<f32>,
    hit_tolerance: f32,
    earliest_ancestor_index: u32,
    family: ptr<function, Primitive>,
) -> f32 {
    // Get the distance to the topmost primitive
    var distance_to_family: f32 = distance_to_textured_primitive(position, family);

    // Check if the topmost primmitive is a bounding volume
    var family_is_bounded: bool = bool((*family).modifiers & BOUNDING_VOLUME);
    // And if we are outside that bounding volume if so
    var out_of_familys_boundary: bool = (
        family_is_bounded && distance_to_family > hit_tolerance
    );

    // If we are inside the bounding volume we don't want the initial distance
    // to be to the boundary, so set it to the maximum distance instead.
    distance_to_family = select(
        distance_to_family,
        _render_params.ray_marcher.max_distance,
        family_is_bounded && !out_of_familys_boundary,
    );

    // Track the number of descendants that should be, and have been, processed
    var num_descendants_to_process: u32 = (*family).num_descendants;
    // If are outside the boundary we want to return immediately
    // but failing the existing loop break conditions benchmarked faster
    // than adding an additional if statement, or adding
    // `out_of_familys_boundary` to the condition
    var descendants_processed: u32 = select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Track the index of the parent and current child we are processing
    var current_parent_index: u32 = earliest_ancestor_index;
    var child_index: u32 = current_parent_index + 1u + select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Allow us to set the next parent while we are looping rather than
    // searching for it after each level of children
    var next_parent_index: u32 = current_parent_index;
    var searching_for_next_parent: bool = true;

    // Create a primitive to re-use as our child
    var child: Primitive;

    // Process all immediate children breadth first
    // Then step into children and process all grandchildren breadth first
    // continuing until all relatives are processed
    loop {
        // If there are no more direct children
        if child_index > current_parent_index + (*family).num_descendants {
            // If all children & grandchildren have been processed, stop
            if num_descendants_to_process <= descendants_processed {
                break;
            }

            // Otherwise, continue until all grandchildren have been processed.
            // The next parent will either be the one we found at the current
            // depth, or at most the current child index
            current_parent_index = select(
                next_parent_index,
                child_index,
                searching_for_next_parent,
            );
            // Get the next parent and apply the current blended material
            *family = _primitives.primitives[current_parent_index];
            (*family).material = child.material;

            // Update the child index to point to the first child of the
            // new parent
            child_index = current_parent_index;

            // Reset this flag so we can find the next parent
            searching_for_next_parent = true;

            continue;
        }

        // Get and process the child, blending the material and distance
        // in the chosen manner
        child = _primitives.primitives[child_index];
        var distance_to_child: f32 = distance_to_textured_primitive(position, &child);

        var child_is_bounding_volume: bool = bool(child.modifiers & BOUNDING_VOLUME);
        var out_of_childs_boundary: bool = (
            child_is_bounding_volume
            && distance_to_child > hit_tolerance
        );

        // If this child has children record its index to use as the
        // next parent. This ensures the first, deepest child with
        // children is processed first
        var found_next_parent: bool = (
            searching_for_next_parent
            && child.num_descendants > 0u
            && !out_of_childs_boundary
        );
        next_parent_index = select(next_parent_index, child_index, found_next_parent);
        searching_for_next_parent = select(
            searching_for_next_parent,
            false,
            found_next_parent,
        );

        if out_of_childs_boundary {
            // If we are outside the childs boundary use the distance to the
            // boundary in a simple union with our current distance
            // and mark all children as processed
            descendants_processed += child.num_descendants;

            var child_closest: bool = distance_to_child < distance_to_family;
            distance_to_family = select(
                distance_to_family,
                distance_to_child,
                child_closest,
            );
            select_material(family, &child, child_closest);
        } else if !child_is_bounding_volume {
            // Otherwise, as long as the child isn't a bounding volume,
            // we can perform the normal blending operation
            distance_to_family = blend_primitives(
                distance_to_family,
                distance_to_child,
                family,
                &child,
            );
        }

        // Increment the counter tracking the number of children
        // processed so far
        descendants_processed++;
        // Skip the descendants of this child, for now
        child_index += child.num_descendants;

        continuing {
            // Continue to the next child
            child_index++;
        }
    }

    return distance_to_family;
}


fn signed_distance_to_scene_with_primitive(
    position: vec3<f32>,
    pixel_footprint: f32,
    closest_primitive: ptr<function, Primitive>,
) -> f32 {
    var distance_to_scene: f32 = _render_params.ray_marcher.max_distance;
    var primitive: Primitive;
    var primitives_processed = 0u;
    var hit_tolerance: f32 = _render_params.ray_marcher.hit_tolerance + pixel_footprint;
    while primitives_processed < _render_params.scene.num_primitives {
        primitive = _primitives.primitives[primitives_processed];
        var num_descendants: u32 = primitive.num_descendants;

        var signed_distance_field: f32 = distance_to_descendants_with_primitive(
            position,
            hit_tolerance,
            primitives_processed,
            &primitive,
        );

        var primitive_is_new_closest: bool = (
            abs(signed_distance_field) < abs(distance_to_scene)
        );
        distance_to_scene = select(
            distance_to_scene,
            signed_distance_field,
            primitive_is_new_closest,
        );
        select_material(
            closest_primitive,
            &primitive,
            primitive_is_new_closest,
        );

        // Skip all descendants, they were processed in the
        // `distance_to_descendants` function
        primitives_processed += num_descendants + 1u;
    }

    return distance_to_scene;
}


fn distance_to_descendants(
    position: vec3<f32>,
    hit_tolerance: f32,
    earliest_ancestor_index: u32,
    family: ptr<function, Primitive>,
) -> f32 {
    // Get the distance to the topmost primitive
    var distance_to_family: f32 = distance_to_primitive(position, family);

    // Check if the topmost primmitive is a bounding volume
    var family_is_bounded: bool = bool((*family).modifiers & BOUNDING_VOLUME);
    // And if we are outside that bounding volume if so
    var out_of_familys_boundary: bool = (
        family_is_bounded && distance_to_family > hit_tolerance
    );

    // If we are inside the bounding volume we don't want the initial distance
    // to be to the boundary, so set it to the maximum distance instead.
    distance_to_family = select(
        distance_to_family,
        _render_params.ray_marcher.max_distance,
        family_is_bounded && !out_of_familys_boundary,
    );

    // Track the number of descendants that should be, and have been, processed
    var num_descendants_to_process: u32 = (*family).num_descendants;
    // If are outside the boundary we want to return immediately
    // but failing the existing loop break conditions benchmarked faster
    // than adding an additional if statement, or adding
    // `out_of_familys_boundary` to the condition
    var descendants_processed: u32 = select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Track the index of the parent and current child we are processing
    var current_parent_index: u32 = earliest_ancestor_index;
    var child_index: u32 = current_parent_index + 1u + select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Allow us to set the next parent while we are looping rather than
    // searching for it after each level of children
    var next_parent_index: u32 = current_parent_index;
    var searching_for_next_parent: bool = true;

    // Create a primitive to re-use as our child
    var child: Primitive;

    // Process all immediate children breadth first
    // Then step into children and process all grandchildren breadth first
    // continuing until all relatives are processed
    loop {
        // If there are no more direct children
        if child_index > current_parent_index + (*family).num_descendants {
            // If all children & grandchildren have been processed, stop
            if num_descendants_to_process <= descendants_processed {
                break;
            }

            // Otherwise, continue until all grandchildren have been processed.
            // The next parent will either be the one we found at the current
            // depth, or at most the current child index
            current_parent_index = select(
                next_parent_index,
                child_index,
                searching_for_next_parent,
            );
            // Get the next parent and apply the current blended material
            *family = _primitives.primitives[current_parent_index];

            // Update the child index to point to the first child of the
            // new parent
            child_index = current_parent_index;

            // Reset this flag so we can find the next parent
            searching_for_next_parent = true;

            continue;
        }

        // Get and process the child, blending the material and distance
        // in the chosen manner
        child = _primitives.primitives[child_index];
        var distance_to_child: f32 = distance_to_primitive(position, &child);

        var child_is_bounding_volume: bool = bool(child.modifiers & BOUNDING_VOLUME);
        var out_of_childs_boundary: bool = (
            child_is_bounding_volume
            && distance_to_child > hit_tolerance
        );

        // If this child has children record its index to use as the
        // next parent. This ensures the first, deepest child with
        // children is processed first
        var found_next_parent: bool = (
            searching_for_next_parent
            && child.num_descendants > 0u
            && !out_of_childs_boundary
        );
        next_parent_index = select(next_parent_index, child_index, found_next_parent);
        searching_for_next_parent = select(
            searching_for_next_parent,
            false,
            found_next_parent,
        );

        if out_of_childs_boundary {
            // If we are outside the childs boundary use the distance to the
            // boundary in a simple union with our current distance
            // and mark all children as processed
            descendants_processed += child.num_descendants;

            var child_closest: bool = distance_to_child < distance_to_family;
            distance_to_family = select(
                distance_to_family,
                distance_to_child,
                child_closest,
            );
        } else if !child_is_bounding_volume {
            // Otherwise, as long as the child isn't a bounding volume,
            // we can perform the normal blending operation
            distance_to_family = blend_distances(
                distance_to_family,
                distance_to_child,
                family,
            );
        }

        // Increment the counter tracking the number of children
        // processed so far
        descendants_processed++;
        // Skip the descendants of this child, for now
        child_index += child.num_descendants;

        continuing {
            // Continue to the next child
            child_index++;
        }
    }

    return distance_to_family;
}


fn signed_distance_to_scene(
    position: vec3<f32>,
    pixel_footprint: f32,
) -> f32 {
    var distance_to_scene: f32 = _render_params.ray_marcher.max_distance;
    var primitive: Primitive;
    var primitives_processed = 0u;
    var hit_tolerance: f32 = _render_params.ray_marcher.hit_tolerance + pixel_footprint;
    while primitives_processed < _render_params.scene.num_primitives {
        primitive = _primitives.primitives[primitives_processed];
        var num_descendants: u32 = primitive.num_descendants;

        var signed_distance_field: f32 = distance_to_descendants(
            position,
            hit_tolerance,
            primitives_processed,
            &primitive,
        );

        var primitive_is_new_closest: bool = (
            abs(signed_distance_field) < abs(distance_to_scene)
        );
        distance_to_scene = select(
            distance_to_scene,
            signed_distance_field,
            primitive_is_new_closest,
        );

        // Skip all descendants, they were processed in the
        // `distance_to_descendants` function
        primitives_processed += num_descendants + 1u;
    }

    return distance_to_scene;
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
    var normal_offset = vec2(0.5773, -0.5773);
    return normalize(
        normal_offset.xyy * signed_distance_to_scene(
            position + normal_offset.xyy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.yyx * signed_distance_to_scene(
            position + normal_offset.yyx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.yxy * signed_distance_to_scene(
            position + normal_offset.yxy * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
        + normal_offset.xxx * signed_distance_to_scene(
            position + normal_offset.xxx * _render_params.ray_marcher.hit_tolerance,
            pixel_footprint,
        )
    );
}


// lights.wgsl


const MAX_LIGHTS: u32 = 512u;


struct Light {
    light_type: u32,
    dimensional_data: vec3<f32>,
    intensity: f32,
    falloff: u32,
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
    if visible_surface_area == 0. {
        return 1. / num_lights;
    } else {
        return 1. / num_lights / visible_surface_area;
    }
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
 * @arg light_direction: The direction from the surface to the light.
 * @arg distance_to_light: The distance to the light's surface.
 *
 * @returns: The PDF of the light.
 */
fn sample_non_physical_light_data(
    surface_position: vec3<f32>,
    light_index: u32,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
) -> f32 {
    var light: Light = _lights.lights[light_index];

    switch light.light_type {
        case 0u {
            // Directional light
            *distance_to_light = _render_params.ray_marcher.max_distance;
            *light_direction = normalize(-light.dimensional_data);
        }
        case 1u {
            // Point light
            *light_direction = light.dimensional_data - surface_position;
            *distance_to_light = length(*light_direction);
            *light_direction = normalize(*light_direction);
        }
        default {}
    }
    return 1.;
}


/**
 * Sample the data of a particular light in the scene.
 *
 * @arg seed: The seed to use in randomization.
 * @arg position: The position on the surface to sample the data of.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the data of.
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
    light_id: u32,
    light_direction: ptr<function, vec3<f32>>,
    distance_to_light: ptr<function, f32>,
) -> f32 {
    // if (light_id < _lightTextureWidth)
    // {
    return sample_non_physical_light_data(
        position,
        light_id,
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
    // return sample_lights_pdf(max(1, numLights), 1.);
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

    return (*light).intensity / power_of_u32(distance_to_light, (*light).falloff);
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
    var occlusion: f32 = 0.;
    var occlusion_scale_factor: f32 = 1.;
    for (var iteration=0u; iteration < iterations; iteration++)
    {
        var step_distance: f32 = 0.001 + 0.15 * f32(iteration) / 4.;
        var distance_to_closest_object: f32 = abs(signed_distance_to_scene(
            ray_origin + step_distance * surface_normal,
            _render_params.ray_marcher.hit_tolerance,
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
        var step_distance: f32 = abs(signed_distance_to_scene(
            position,
            pixel_footprint,
        ));
        var step_distance_squared: f32 = step_distance * step_distance;
        var soft_offset: f32 = step_distance_squared / (2. * last_step_distance);
        shadow_intensity = min(
            shadow_intensity,
            hardness * sqrt(step_distance_squared - soft_offset * soft_offset)
            / positive_part_f32(distance_travelled - soft_offset),
        );

        if step_distance < pixel_footprint {
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
    var distance_travelled: f32 = 0.;
    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_params.ray_marcher.hit_tolerance;
    var position: vec3<f32> = ray_origin;
    while (
        distance_travelled < distance_to_shade_point
        && iterations < _render_params.ray_marcher.max_ray_steps / 2u
    ) {
        var step_distance: f32 = abs(signed_distance_to_scene(
            position,
            pixel_footprint,
        ));

        if step_distance < pixel_footprint {
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
        case 0u {
            var shadow_intensity_at_position: f32;
            if bool(light.soften_shadows) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                    light.shadow_hardness,
                );
            } else {
                shadow_intensity_at_position = sample_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                );
            }

            intensity = vec2(light.intensity, shadow_intensity_at_position);
        }
        case 1u {
            var shadow_intensity_at_position: f32;
            if bool(light.soften_shadows) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                    light.shadow_hardness,
                );
            } else {
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
        case 2u, default {
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
 * @arg light_sampling_pdf: The PDF of the light we are sampling the
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
    light_sampling_pdf: f32,
    material_pdf: f32,
    light_id: u32,
) -> vec3<f32> {
    var light_colour = vec3(0.);
    var light_geometry_factor: f32 = 0.;

    if light_id < _render_params.scene.num_non_physical_lights
    {
        light_colour = sample_non_physical_light(
            position,
            surface_normal,
            light_direction,
            distance_to_light,
            light_id,
        );
        light_geometry_factor = saturate_f32(dot(light_direction, surface_normal));
    }
    // else if light_id - _lightTextureWidth - sampleHDRI < numEmissive {
    //     float actualDistance;
    //     float3 actualDirection = light_direction;
    //     float3 lightNormal;
    //     light_colour = marchPath(
    //         position,
    //         seed,
    //         sampleHDRI,
    //         distance_to_light * 2.,
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
        throughput * material_brdf * light_geometry_factor / light_sampling_pdf,
        light_sampling_pdf,
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
) -> vec3<f32> {
    var light_colour = vec3(0.);

    for (var light_id=0u; light_id < _render_params.scene.num_lights; light_id++) {
        var light_direction: vec3<f32> = surface_normal;
        var distance_to_light: f32 = 0.;

        var light_sampling_pdf: f32 = sample_light_data(
            seed * f32(light_id + 1u),
            position,
            surface_normal,
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
            light_sampling_pdf,
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
) -> vec3<f32> {
    if sample_all_lights {
        return sample_lights(
            seed,
            throughput,
            material_brdf,
            surface_normal,
            position,
            material_pdf,
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
 */
fn create_ray(uv_coordinate: vec4<f32>) -> Ray {
    return Ray(
        vec3(
            _render_camera.world_matrix[3][0],
            _render_camera.world_matrix[3][1],
            _render_camera.world_matrix[3][2],
        ),
        normalize((
            _render_camera.world_matrix
            * vec4(
                (_render_camera.inverse_projection_matrix * uv_coordinate).xyz,
                0.,
            )
        ).xyz),
        vec3(0.),
        vec3(1.),
    );
}


/**
 * Create a ray out of the camera. It will be either a standard ray,
 * a latlong ray, or a ray that will result in depth of field.
 *
 * @arg seed: The seed to use in randomization.
 * @arg uv_coordinate: The u, and v locations of the pixel.
 */
fn create_render_camera_ray(seed: vec3<f32>, uv_coordinate: vec4<f32>) -> Ray {
    // if (bool(_render_params.ray_marcher.latlong))
    // {
    //     // create_latlong_ray(
    //     //     uv_coordinate,
    //     //     ray_origin,
    //     //     ray_direction,
    //     // );
    // }
    // else if (bool(_render_camera.enable_depth_of_field))
    // {
    //     // create_ray_with_dof(
    //     //     uv_coordinate,
    //     //     seed,
    //     //     ray_origin,
    //     //     ray_direction,
    //     // );
    // }
    // else
    // {
    return create_ray(uv_coordinate);
    // }
}

// aovs.wgsl

const BEAUTY_AOV: u32 = 0u;
const STATS_AOV: u32 = 5u;


fn early_exit_aovs(
    aov_type: u32,
    world_position: vec3<f32>,
    local_position: vec3<f32>,
    surface_normal: vec3<f32>,
) -> vec3<f32> {
    switch aov_type {
        case 1u {
            return world_position;
        }
        case 2u {
            return local_position;
        }
        case 3u {
            return surface_normal;
        }
        case 4u {
            // Depth
            return vec3(abs(world_to_camera_space(world_position).z));
        }
        default {
            return vec3(-1.); // Invalid!!
        }
    }
}


fn final_aovs(
    aov_type: u32,
    bounces: u32,
    iterations: u32,
    distance_travelled: f32,
) -> vec3<f32> {
    switch aov_type {
        case 5u {
            return vec3(
                f32(bounces) / f32(_render_params.ray_marcher.max_bounces),
                f32(iterations) / f32(_render_params.ray_marcher.max_ray_steps),
                distance_travelled / _render_params.ray_marcher.max_distance,
            );
        }
        default {
            return vec3(-1.); // Invalid!!
        }
    }
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
 * @arg ray: The ray which will interact with the material.
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
    previous_material_pdf: f32,
    sample_all_lights: bool,
    ray: ptr<function, Ray>,
    primitive: ptr<function, Primitive>,
) -> f32 {
    (*ray).origin = intersection_position;

    var material_brdf: vec3<f32>;
    var light_sampling_material_pdf: f32;
    var material_pdf: f32 = sample_material(
        seed,
        surface_normal,
        offset,
        primitive,
        ray,
        &material_brdf,
        &light_sampling_material_pdf,
    );

    if (
        _render_params.scene.num_lights > 0u
        && _render_params.ray_marcher.max_light_sampling_bounces > 0u
        && light_sampling_material_pdf > 0.
    ) {
        // Perform MIS light sampling
        (*ray).colour += light_sampling(
            seed,
            (*ray).throughput,
            material_brdf,
            surface_normal,
            (*ray).origin,
            light_sampling_material_pdf,
            sample_all_lights,
        );
    }

    var material_geometry_factor: f32 = select(
        1.,
        saturate_f32(dot((*ray).direction, surface_normal)),
        light_sampling_material_pdf > 0.,
    );

    var radius: f32 = (*primitive).dimensional_data.x;
    var visible_surface_area: f32 = TWO_PI * radius * radius;

    (*ray).colour += multiple_importance_sample(
        (*primitive).material.emissive_colour * (*primitive).material.emissive_probability,
        (*ray).throughput,
        previous_material_pdf,
        sample_lights_pdf(f32(_render_params.scene.num_lights), visible_surface_area),
    );

    (*ray).throughput *= material_brdf * material_geometry_factor / material_pdf;

    return material_pdf;
}


/**
 * March a path through the scene.
 *
 * @arg seed: The seed to use in randomization.
 * @arg ray: The ray to march.
 *
 * @returns: The ray colour.
 */
fn march_path(seed: vec3<f32>, exit_early_with_aov: bool, ray: ptr<function, Ray>) {
    var path_seed: vec3<f32> = seed;
    var roulette = bool(_render_params.ray_marcher.roulette);
    var dynamic_level_of_detail = bool(_render_params.ray_marcher.dynamic_level_of_detail);

    var sample_all_lights = bool(_render_params.ray_marcher.sample_all_lights);

    var distance_travelled: f32 = 0.;
    var distance_since_last_bounce = 0.;

    var last_step_distance: f32 = 1.;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = _render_params.ray_marcher.hit_tolerance;

    var previous_material_pdf: f32 = 1.;

    // Data for the next ray
    var position_on_ray: vec3<f32> = (*ray).origin;

    // March the ray
    while (
        distance_travelled < _render_params.ray_marcher.max_distance
        && iterations < _render_params.ray_marcher.max_ray_steps
        && sum_component_vec3f((*ray).throughput) > pixel_footprint
        && length((*ray).colour) < _render_params.ray_marcher.max_brightness
    ) {
        position_on_ray = (*ray).origin + distance_since_last_bounce * (*ray).direction;

        // Keep the signed distance so we know whether or not we are inside the object
        var signed_step_distance = signed_distance_to_scene(
            position_on_ray,
            pixel_footprint,
        );

        // Take the absolute value, the true shortest distance to a surface
        var step_distance = abs(signed_step_distance);

        // Keep track of the distance the ray has travelled
        distance_travelled += step_distance;
        distance_since_last_bounce += step_distance;

        // Have we hit the nearest object?
        var hit_object: bool = step_distance < pixel_footprint;
        if hit_object {
            bounces++;
            var intersection_position = position_on_ray + step_distance * (*ray).direction;

            // The normal to the surface at that position
            var surface_normal: vec3<f32> = sign(last_step_distance) * estimate_surface_normal(
                intersection_position,
                pixel_footprint,
            );

            // Early exit for the various AOVs that are not 'beauty'
            if exit_early_with_aov {
                (*ray).colour = early_exit_aovs(
                    _render_params.ray_marcher.output_aov,
                    intersection_position,
                    intersection_position, // TODO world to local
                    surface_normal,
                );
                return;
            }

            var nearest_primitive: Primitive;
            // Keep the signed distance so we know whether or not we are inside the object
            signed_step_distance = signed_distance_to_scene_with_primitive(
                position_on_ray,
                pixel_footprint,
                &nearest_primitive,
            );

            previous_material_pdf = material_interaction(
                path_seed,
                step_distance,
                2. * pixel_footprint * _render_params.ray_marcher.shadow_bias,
                distance_since_last_bounce,
                intersection_position,
                surface_normal,
                previous_material_pdf,
                sample_all_lights,
                ray,
                &nearest_primitive,
            );

            // Exit if we have reached the bounce limit or with a random chance
            var rng: f32 = vec3f_to_random_f32(path_seed);
            var exit_probability: f32 = max_component_vec3f((*ray).throughput);
            if (
                bounces >= _render_params.ray_marcher.max_bounces
                || (roulette && exit_probability <= rng)
            ) {
                break;
            } else if roulette {
                // Account for the lost intensity from the early exits
                (*ray).throughput /= vec3(exit_probability);
            }

            distance_since_last_bounce = 0.;
            // Reset the pixel footprint so multiple reflections don't reduce precision
            pixel_footprint = _render_params.ray_marcher.hit_tolerance;

            // Update the random seed for the next iteration
            path_seed = random_vec3f(path_seed.zxy + seed);
        }
        pixel_footprint += select(
            0.,
            _render_params.ray_marcher.hit_tolerance * step_distance,
            dynamic_level_of_detail && !hit_object,
        );

        last_step_distance = signed_step_distance;
        iterations++;
    }

    (*ray).colour = select(
        (*ray).colour,
        final_aovs(
            _render_params.ray_marcher.output_aov,
            bounces,
            iterations,
            distance_travelled,
        ),
        _render_params.ray_marcher.output_aov > BEAUTY_AOV,
    );
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var frag_coord_seed = vec3(vec2f_to_random_f32(in.frag_coordinate.xy));
    var seed = random_vec3f(_render_params.ray_marcher.seeds + frag_coord_seed);
    var pixel_colour = vec3(0.);

    var exit_early_with_aov: bool = (
        _render_params.ray_marcher.output_aov > BEAUTY_AOV
        && _render_params.ray_marcher.output_aov < STATS_AOV
    );

    for (var path=1u; path <= _render_params.ray_marcher.paths_per_pixel; path++) {
        var ray: Ray = create_render_camera_ray(seed, in.uv_coordinate);

        march_path(seed, exit_early_with_aov, &ray);

        pixel_colour += ray.colour;

        seed = random_vec3f(seed.yzx + frag_coord_seed);
    }

    return vec4(pixel_colour, 1.) / f32(_render_params.ray_marcher.paths_per_pixel);
}
