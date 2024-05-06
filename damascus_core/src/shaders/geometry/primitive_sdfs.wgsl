// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

//
// Signed Distance Functions
//
// Many of the below sdfs are based on the work of Inigo Quilez
// https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
//

// const DIFFUSE_TRAP: u32 = 8192u;
// const SPECULAR_TRAP: u32 = 16384u;
// const EXTINCTION_TRAP: u32 = 32768u;
// const EMISSION_TRAP: u32 = 65536u;
// const SCATTERING_TRAP: u32 = 131072u;


const CAPPED_CONE: u32 = 0u;
const CAPPED_TORUS: u32 = 1u;
const CAPSULE: u32 = 2u;
const CONE: u32 = 3u;
const CUT_SPHERE: u32 = 4u;
const CYLINDER: u32 = 5u;
const DEATH_STAR: u32 = 6u;
const ELLIPSOID: u32 = 7u;
const HEXAGONAL_PRISM: u32 = 8u;
const HOLLOW_SPHERE: u32 = 9u;
const INFINITE_CONE: u32 = 10u;
const INFINITE_CYLINDER: u32 = 11u;
const LINK: u32 = 12u;
const MANDELBOX: u32 = 13u;
const MANDELBULB: u32 = 14u;
const OCTAHEDRON: u32 = 15u;
const PLANE: u32 = 16u;
const RECTANGULAR_PRISM: u32 = 17u;
const RECTANGULAR_PRISM_FRAME: u32 = 18u;
const RHOMBUS: u32 = 19u;
const ROUNDED_CONE: u32 = 20u;
const SOLID_ANGLE: u32 = 21u;
const SPHERE: u32 = 22u;
const TORUS: u32 = 23u;
const TRIANGULAR_PRISM: u32 = 24u;


/**
 * Compute the min distance from a point to a circle.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the circle.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_circle(position: vec2f, radius: f32) -> f32 {
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
fn distance_to_sphere(position: vec3f, radius: f32) -> f32 {
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
fn distance_to_ellipsoid(position: vec3f, radii: vec3f) -> f32 {
    // Components of this vector that are < 1 are inside the ellipse
    // when projected onto the plane the respective axis is normal to
    var transformed_position: vec3f = position / radii;

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
    position: vec3f,
    radius: f32,
    cut_height: f32,
) -> f32 {
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    position: vec3f,
    radius: f32,
    cut_height: f32,
    thickness: f32,
) -> f32 {
    var half_thickness: f32 = thickness / 2.;

    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    position: vec3f,
    additive_sphere_radius: f32,
    subtractive_sphere_radius: f32,
    subtractive_sphere_height: f32,
) -> f32 {
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    position: vec3f,
    radius: f32,
    angle: f32,
) -> f32 {
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    position: vec3f,
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
    position: vec3f,
    width: f32,
    height: f32,
    depth: f32,
    thickness: f32,
) -> f32 {
    var prism_to_position = abs(position) - vec3(width, height, depth) / 2.;
    var inner_reflected: vec3f = abs(prism_to_position + thickness) - thickness;

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
    position: vec3f,
    width: f32,
    height: f32,
    depth: f32,
    corner_radius: f32,
) -> f32 {
    var abs_position: vec3f = abs(position);
    var half_width_height = vec2(width, height) / 2.;

    var s: vec2f = half_width_height * (half_width_height - 2. * abs_position.xy);
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
fn distance_to_triangular_prism(position: vec3f, base: f32, depth: f32) -> f32 {
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
    position: vec3f,
    radius: f32,
    height: f32,
) -> f32 {
    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2f = abs(cartesian_to_cylindrical(position));
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
fn distance_to_infinite_cylinder(position: vec3f, radius: f32) -> f32 {
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
fn distance_to_plane(position: vec3f, normal: vec3f) -> f32 {
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
    position: vec3f,
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
fn distance_to_cone(position: vec3f, angle: f32, height: f32) -> f32 {
    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

    // The cylindrical coordinates of the edge of the cone base
    var cylindrical_bound = vec2(abs(height * tan(angle)), height);

    // Vector from the top surface of the cone to the position given
    var cone_top_to_position: vec2f = cylindrical_position - cylindrical_bound * vec2(
        saturate_f32(cylindrical_position.x / cylindrical_bound.x),
        1.,
    );
    // Vector from the edge of the cone to the position given
    var cone_edge_to_position: vec2f = (
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
fn distance_to_infinite_cone(position: vec3f, angle: f32) -> f32 {
    // The normalized cylindrical coordinates of the edge of the cone base
    var cone_edge_direction: vec2f = vec2(sin(angle), cos(angle));

    // Cylindrical coordinates (r, h), ignoring the angle due to symmetry
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    position: vec3f,
    height: f32,
    lower_radius: f32,
    upper_radius: f32,
) -> f32 {
    var half_height: f32 = height / 2.;
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
    var cone_edge_to_position: vec2f = (
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
    position: vec3f,
    height: f32,
    lower_radius: f32,
    upper_radius: f32,
) -> f32 {
    var cylindrical_position: vec2f = cartesian_to_cylindrical(position);

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
fn distance_to_torus(position: vec3f, ring_radius: f32, tube_radius: f32) -> f32 {
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
    position: vec3f,
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
    position: vec3f,
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
fn distance_to_hexagonal_prism(position: vec3f, height: f32, depth: f32) -> f32 {
    // precomputed -cos(-PI / 6.), -sin(-PI / 6.), -tan(-PI / 6.)
    var cos_sin_tan = vec3(-0.86602540378, 0.5, 0.57735026919);
    var half_height: f32 = height / 2.;

    var abs_position: vec3f = abs(position);
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
fn distance_to_octahedron(position: vec3f, radial_extent: f32) -> f32 {
    var abs_position: vec3f = abs(position);

    var position_sum_to_extent: f32 = dot(abs_position, vec3(1.)) - radial_extent;

    var three_position: vec3f = 3. * abs_position;
    var change_of_axes: vec3f;
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
    position: vec3f,
    power: f32,
    iterations: u32,
    max_square_radius: f32,
    trap_colour: ptr<function, vec3f>,
) -> f32 {
    var current_position: vec3f = position;
    var radius_squared: f32 = dot2_vec3f(current_position);

    var abs_position: vec3f = abs(current_position);
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
    position: vec3f,
    power: f32,
    iterations: u32,
    max_square_radius: f32,
) -> f32 {
    var current_position: vec3f = position;
    var radius_squared: f32 = dot2_vec3f(current_position);

    var abs_position: vec3f = abs(current_position);

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


fn box_fold(position: vec3f, folding_limit: vec3f) -> vec3f {
    return clamp(position, -folding_limit, folding_limit) * 2. - position;
}


fn sphere_fold(
    position: vec4f,
    radius_squared: f32,
    min_square_radius: f32,
) -> vec4f {
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
    position: vec3f,
    scale: f32,
    iterations: i32,
    min_square_radius: f32,
    folding_limit: f32,
    trap_colour: ptr<function, vec3f>,
) -> f32 {
    var scale_vector = vec4(scale, scale, scale, abs(scale)) / min_square_radius;
    var initial_position = vec4(position, 1.);
    var current_position: vec4f = initial_position;

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
    position: vec3f,
    scale: f32,
    iterations: i32,
    min_square_radius: f32,
    folding_limit: f32,
) -> f32 {
    var scale_vector = vec4(scale, scale, scale, abs(scale)) / min_square_radius;
    var initial_position = vec4(position, 1.);
    var current_position: vec4f = initial_position;

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
