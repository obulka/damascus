// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


const FINITE_REPETITION: u32 = 1u;
const INFINITE_REPETITION: u32 = 2u;
const ELONGATE: u32 = 4u;
const MIRROR_X: u32 = 8u;
const MIRROR_Y: u32 = 16u;
const MIRROR_Z: u32 = 32u;
const HOLLOW: u32 = 64u;
const SUBTRACTION: u32 = 128u;
const INTERSECTION: u32 = 256u;
const BLEND_TYPE_MASK: u32 = 384u;
const BOUNDING_VOLUME: u32 = 512u;


/**
 * Finitely repeat an object in the positive quadrant.
 *
 * @arg position: The position of the ray.
 * @arg primitive: The primitive to repeat.
 *
 * @returns: The modified ray position that results in repetion.
 */
// fn symmetric_finite_repetition(
//     position: vec3f,
//     primitive: ptr<function, Primitive>,
// ) -> vec3f {
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
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> vec3f {
    var space_partition_id: vec3f = clamp(
        round(position / (*primitive).spacing),
        -(*primitive).negative_repetitions,
        (*primitive).positive_repetitions,
    );
    var repeated_position: vec3f = position - (*primitive).spacing * space_partition_id;

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
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> vec3f {
    var space_partition_id: vec3f = round(position / (*primitive).spacing);
    var repeated_position: vec3f = position - (*primitive).spacing * space_partition_id;

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
 * Transform and rotate a position.
 *
 * @arg position: The location the ray originates from.
 * @arg primitive: The primitive which determines the transformation.
 *
 * @returns: The transformed position.
 */
fn rotate_translate_position(
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> vec3f {
    return (
        (*primitive).transform.inverse_rotation
        * (position - (*primitive).transform.translation)
    );
}


/**
 * Mirror/elongate/repeate primitive at a position.
 *
 * @arg position: The location the ray originates from.
 * @arg primitive: The primitive which determines the transformation.
 *
 * @returns: The transformed ray origin.
 */
fn transform_position(
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> vec3f {
    // Perform finite or infinite repetition if enabled
#ifdef EnableFiniteRepetition
    var transformed_position: vec3f = select(
#else
    var transformed_position: vec3f =
#endif
#ifdef EnableInfiniteRepetition
        select(
            position,
            mirrored_infinite_repetition(
                position,
                primitive,
            ),
            bool((*primitive).modifiers & INFINITE_REPETITION),
#ifdef EnableFiniteRepetition
        ),
#else
        );
#endif
#else
#ifdef EnableFiniteRepetition
        position,
#else
        position;
#endif
#endif
#ifdef EnableFiniteRepetition
        mirrored_finite_repetition(
            position,
            primitive,
        ),
        bool((*primitive).modifiers & FINITE_REPETITION),
    );
#endif

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
 * Modify the material of a primitive using its procedural textures.
 *
 * @arg position: The point to use as the seed.
 * @arg primitive: The primitive to modify.
 */
fn texture_primitive(
    position: vec3f,
    primitive: ptr<function, Primitive>,
) {
    var procedural_texture_seed = vec4(
        position,
        length((*primitive).dimensional_data),
    );
#ifdef EnableDiffuseColourTexture
    (*primitive).material.diffuse_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        (*primitive).material.diffuse_colour,
        (*primitive).material.diffuse_colour_texture,
    );
#endif
#ifdef EnableSpecularProbabilityTexture
    (*primitive).material.specular_probability = procedurally_texture_f32(
        procedural_texture_seed,
        (*primitive).material.specular_probability,
        (*primitive).material.specular_probability_texture,
    );
#endif
#ifdef EnableSpecularRoughnessTexture
    (*primitive).material.specular_roughness = procedurally_texture_f32(
        procedural_texture_seed,
        (*primitive).material.specular_roughness,
        (*primitive).material.specular_roughness_texture,
    );
#endif
#ifdef EnableSpecularColourTexture
    (*primitive).material.specular_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        (*primitive).material.specular_colour,
        (*primitive).material.specular_colour_texture,
    );
#endif
#ifdef EnableTransmissiveProbabilityTexture
    (*primitive).material.transmissive_probability = procedurally_texture_f32(
        procedural_texture_seed,
        (*primitive).material.transmissive_probability,
        (*primitive).material.transmissive_probability_texture,
    );
#endif
#ifdef EnableTransmissiveRoughnessTexture
    (*primitive).material.transmissive_roughness = procedurally_texture_f32(
        procedural_texture_seed,
        (*primitive).material.transmissive_roughness,
        (*primitive).material.transmissive_roughness_texture,
    );
#endif
#ifdef EnableEmissiveColourTexture
    (*primitive).material.emissive_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        (*primitive).material.emissive_colour,
        (*primitive).material.emissive_colour_texture,
    );
#endif
#ifdef EnableRefractiveIndexTexture
    (*primitive).material.refractive_index = procedurally_texture_f32(
        procedural_texture_seed,
        (*primitive).material.refractive_index,
        (*primitive).material.refractive_index_texture,
    );
#endif
}


/**
 * Compute the min distance from a point to a geometric object.
 *
 * @arg position: The point to get the distance to, from the primitive.
 * @arg primitive: The primitive to get the distance to.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_transformed_primitive(
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> f32 {
    var distance: f32;
    switch (*primitive).shape {
#ifdef EnableCappedCone
        case CAPPED_CONE {
            distance = distance_to_capped_cone(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableCappedTorus
        case CAPPED_TORUS {
            distance = distance_to_capped_torus(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                radians((*primitive).dimensional_data.z),
            );
        }
#endif
#ifdef EnableCapsule
        case CAPSULE {
            distance = distance_to_capsule(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableCone
        case CONE {
            distance = distance_to_cone(
                position,
                radians((*primitive).dimensional_data.x),
                (*primitive).dimensional_data.y,
            );
        }
#endif
#ifdef EnableCutSphere
        case CUT_SPHERE {
            distance = distance_to_cut_sphere(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
#endif
#ifdef EnableCylinder
        case CYLINDER {
            distance = distance_to_cylinder(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
#endif
#ifdef EnableDeathStar
        case DEATH_STAR {
            distance = distance_to_death_star(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableEllipsoid
        case ELLIPSOID {
            distance = distance_to_ellipsoid(
                position,
                (*primitive).dimensional_data.xyz,
            );
        }
#endif
#ifdef EnableHexagonalPrism
        case HEXAGONAL_PRISM {
            distance = distance_to_hexagonal_prism(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
#endif
#ifdef EnableHollowSphere
        case HOLLOW_SPHERE {
            distance = distance_to_hollow_sphere(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableInfiniteCone
        case INFINITE_CONE {
            distance = distance_to_infinite_cone(
                position,
                radians((*primitive).dimensional_data.x),
            );
        }
#endif
#ifdef EnableInfiniteCylinder
        case INFINITE_CYLINDER {
            distance = distance_to_infinite_cylinder(
                position,
                (*primitive).dimensional_data.x,
            );
        }
#endif
#ifdef EnableLink
        case LINK {
            distance = distance_to_link(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableMandelbox
        case MANDELBOX {
            var colour = vec3(1.);
            distance = distance_to_textured_mandelbox(
                position,
                (*primitive).dimensional_data.x,
                i32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
#endif
#ifdef EnableMandelbulb
        case MANDELBULB {
            var colour = vec3(1.);
            distance = distance_to_textured_mandelbulb(
                position,
                (*primitive).dimensional_data.x,
                u32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                &colour,
            );
            (*primitive).material.diffuse_colour *= colour; // TODO use modifiers
        }
#endif
#ifdef EnableOctahedron
        case OCTAHEDRON {
            distance = distance_to_octahedron(
                position,
                (*primitive).dimensional_data.x,
            );
        }
#endif
#ifdef EnablePlane
        case PLANE {
            distance = distance_to_plane(
                position,
                normalize((*primitive).dimensional_data.xyz),
            );
        }
#endif
#ifdef EnableRectangularPrism
        case RECTANGULAR_PRISM {
            distance = distance_to_rectangular_prism(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableRectangularPrismFrame
        case RECTANGULAR_PRISM_FRAME {
            distance = distance_to_rectangular_prism_frame(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
#endif
#ifdef EnableRhombus
        case RHOMBUS {
            distance = distance_to_rhombus(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
            );
        }
#endif
#ifdef EnableRoundedCone
        case ROUNDED_CONE {
            distance = distance_to_rounded_cone(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
                (*primitive).dimensional_data.z,
            );
        }
#endif
#ifdef EnableSolidAngle
        case SOLID_ANGLE {
            distance = distance_to_solid_angle(
                position,
                (*primitive).dimensional_data.x,
                radians((*primitive).dimensional_data.y),
            );
        }
#endif
        case SPHERE, default {
            distance = distance_to_sphere(
                position,
                (*primitive).dimensional_data.x,
            );
        }
#ifdef EnableTorus
        case TORUS {
            distance = distance_to_torus(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
#endif
#ifdef EnableTriangularPrism
        case TRIANGULAR_PRISM {
            distance = distance_to_triangular_prism(
                position,
                (*primitive).dimensional_data.x,
                (*primitive).dimensional_data.y,
            );
        }
#endif
    }

    return distance;
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
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> f32 {
    var transformed_position: vec3f = rotate_translate_position(position, primitive);
    texture_primitive(transformed_position, primitive);
    transformed_position = transform_position(
        transformed_position,
        primitive,
    ) / (*primitive).transform.uniform_scale;

    var distance: f32 = distance_to_transformed_primitive(
        transformed_position,
        primitive,
    );

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
    position: vec3f,
    primitive: ptr<function, Primitive>,
) -> f32 {
    var transformed_position: vec3f = transform_position(
        rotate_translate_position(
            position,
            primitive,
        ),
        primitive,
    ) / (*primitive).transform.uniform_scale;

    var distance: f32 = distance_to_transformed_primitive(
        transformed_position,
        primitive,
    );

    return modify_distance(distance * (*primitive).transform.uniform_scale, primitive);
}


fn mix_primitives(
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
    (*primitive_0).material.extinction_colour = mix(
        (*primitive_0).material.extinction_colour,
        (*primitive_1).material.extinction_colour,
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


fn select_primitive(
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
    (*primitive_0).material.extinction_colour = select(
        (*primitive_0).material.extinction_colour,
        (*primitive_1).material.extinction_colour,
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
    switch (*parent).modifiers & BLEND_TYPE_MASK {
#ifdef EnablePrimitiveBlendSubtraction
        case SUBTRACTION {
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_parent + distance_to_child)
                / (*parent).blend_strength
            );
            mix_primitives(parent, child, smoothing);
            return mix(
                distance_to_parent,
                -distance_to_child,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
#endif
#ifdef EnablePrimitiveBlendIntersection
        case INTERSECTION {
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            mix_primitives(child, parent, smoothing);
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) + (*parent).blend_strength * smoothing * (1. - smoothing);
        }
#endif
        default {
            // Union
            var smoothing: f32 = saturate_f32(
                0.5
                + 0.5
                * (distance_to_child - distance_to_parent)
                / (*parent).blend_strength
            );
            mix_primitives(child, parent, smoothing);
            return mix(
                distance_to_child,
                distance_to_parent,
                smoothing,
            ) - (*parent).blend_strength * smoothing * (1. - smoothing);
        }
    }
}


fn blend_distances(
    distance_to_parent: f32,
    distance_to_child: f32,
    parent: ptr<function, Primitive>,
) -> f32 {
    switch (*parent).modifiers & BLEND_TYPE_MASK {
#ifdef EnablePrimitiveBlendSubtraction
        case SUBTRACTION {
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
#endif
#ifdef EnablePrimitiveBlendIntersection
        case INTERSECTION {
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
#endif
        default {
            // Union
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
    }
}
