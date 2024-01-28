
const FINITE_REPETITION: u32 = 1u;
const INFINITE_REPETITION: u32 = 2u;
const ELONGATE: u32 = 4u;
const MIRROR_X: u32 = 8u;
const MIRROR_Y: u32 = 16u;
const MIRROR_Z: u32 = 32u;
const HOLLOW: u32 = 64u;
const BOUNDING_VOLUME: u32 = 4096u;


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
    (*primitive_0).id = select(
        (*primitive_0).id,
        (*primitive_1).id,
        smoothing > 0.5,
    );
    (*primitive_1).id = (*primitive_0).id;
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
    (*primitive_0).id = select(
        (*primitive_0).id,
        (*primitive_1).id,
        choice,
    );
    (*primitive_1).id = (*primitive_0).id;
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
