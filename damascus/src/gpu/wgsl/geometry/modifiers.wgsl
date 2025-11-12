// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


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
const ENABLE_ORBIT_TRAP_COLOUR: u32 = 1024u;

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

#ifdef EnableFiniteRepetition

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

#endif

#ifdef EnableInfiniteRepetition

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

#endif

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
#ifdef EnableHollowing
    return select(
        distance,
        abs(distance) - (*primitive).wall_thickness,
        bool((*primitive).modifiers & HOLLOW),
    ) - (*primitive).edge_radius;
#else
    return distance - (*primitive).edge_radius;
#endif
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
 * Mirror/elongate/repeat primitive at a position.
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
#elifdef EnableFiniteRepetition
        position,
#else
        position;
#endif
#ifdef EnableFiniteRepetition
        mirrored_finite_repetition(
            position,
            primitive,
        ),
        bool((*primitive).modifiers & FINITE_REPETITION),
    );
#endif

#ifdef EnableElongation
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
#endif

#ifdef EnableMirroring
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
#else
    return transformed_position;
#endif
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
) -> MaterialSample {
    var procedural_texture_seed = vec4(
        position,
        length((*primitive).dimensional_data),
    );

    var material_sample = MaterialSample(
        _materials[(*primitive).material_id].diffuse_colour,
        _materials[(*primitive).material_id].specular_probability,
        _materials[(*primitive).material_id].specular_colour,
        _materials[(*primitive).material_id].specular_roughness,
        _materials[(*primitive).material_id].extinction_colour,
        _materials[(*primitive).material_id].transmissive_probability,
        _materials[(*primitive).material_id].emissive_colour,
        _materials[(*primitive).material_id].transmissive_roughness,
        _materials[(*primitive).material_id].scattering_colour,
        _materials[(*primitive).material_id].refractive_index,
    );

#ifdef EnableDiffuseColourTexture
    material_sample.diffuse_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        material_sample.diffuse_colour,
        _materials[(*primitive).material_id].diffuse_colour_texture,
    );
#endif
#ifdef EnableSpecularProbabilityTexture
    material_sample.specular_probability = procedurally_texture_f32(
        procedural_texture_seed,
        material_sample.specular_probability,
        _materials[(*primitive).material_id].specular_probability_texture,
    );
#endif
#ifdef EnableSpecularRoughnessTexture
    material_sample.specular_roughness = procedurally_texture_f32(
        procedural_texture_seed,
        material_sample.specular_roughness,
        _materials[(*primitive).material_id].specular_roughness_texture,
    );
#endif
#ifdef EnableSpecularColourTexture
    material_sample.specular_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        material_sample.specular_colour,
        _materials[(*primitive).material_id].specular_colour_texture,
    );
#endif
#ifdef EnableTransmissiveProbabilityTexture
    material_sample.transmissive_probability = procedurally_texture_f32(
        procedural_texture_seed,
        material_sample.transmissive_probability,
        _materials[(*primitive).material_id].transmissive_probability_texture,
    );
#endif
#ifdef EnableTransmissiveRoughnessTexture
    material_sample.transmissive_roughness = procedurally_texture_f32(
        procedural_texture_seed,
        material_sample.transmissive_roughness,
        _materials[(*primitive).material_id].transmissive_roughness_texture,
    );
#endif
#ifdef EnableEmissiveColourTexture
    material_sample.emissive_colour = procedurally_texture_vec3f(
        procedural_texture_seed,
        material_sample.emissive_colour,
        _materials[(*primitive).material_id].emissive_colour_texture,
    );
#endif
#ifdef EnableRefractiveIndexTexture
    material_sample.refractive_index = procedurally_texture_f32(
        procedural_texture_seed,
        material_sample.refractive_index,
        _materials[(*primitive).material_id].refractive_index_texture,
    );
#endif

    return material_sample;
}

#ifdef EnableOrbitTrapColour

fn apply_orbit_trap_colour(
    orbit_trap_colour: vec3f,
    primitive: ptr<function, Primitive>,
    material: ptr<function, MaterialSample>,
) {
    // TODO this has not been tested vs select
    if (
        !bool((*primitive).modifiers & ENABLE_ORBIT_TRAP_COLOUR)
        || (*primitive).shape != MANDELBOX
        && (*primitive).shape != MANDELBULB
    ) {
        return;
    }

    material.diffuse_colour *= orbit_trap_colour;
    material.specular_colour *= orbit_trap_colour;
    material.emissive_colour *= orbit_trap_colour;
}

#endif

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
#ifdef EnableOrbitTrapColour
    orbit_trap_colour: ptr<function, vec3f>,
#endif
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
            distance = distance_to_mandelbox(
                position,
                (*primitive).dimensional_data.x,
                i32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
                (*primitive).dimensional_data.w,
#ifdef EnableOrbitTrapColour
                orbit_trap_colour,
#endif
            );
        }
#endif
#ifdef EnableMandelbulb
        case MANDELBULB {
            distance = distance_to_mandelbulb(
                position,
                (*primitive).dimensional_data.x,
                u32((*primitive).dimensional_data.y),
                (*primitive).dimensional_data.z,
#ifdef EnableOrbitTrapColour
                orbit_trap_colour,
#endif
            );
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
    material: ptr<function, MaterialSample>,
) -> f32 {
    var transformed_position: vec3f = rotate_translate_position(position, primitive);
    *material = texture_primitive(transformed_position, primitive);
    transformed_position = transform_position(
        transformed_position,
        primitive,
    ) / (*primitive).transform.uniform_scale;

#ifdef EnableOrbitTrapColour
    var orbit_trap_colour = vec3(1.);
#endif
    var distance: f32 = distance_to_transformed_primitive(
        transformed_position,
        primitive,
#ifdef EnableOrbitTrapColour
        &orbit_trap_colour,
#endif
    );

#ifdef EnableOrbitTrapColour
    apply_orbit_trap_colour(orbit_trap_colour, primitive, material);
#endif

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

#ifdef EnableOrbitTrapColour
    var orbit_trap_colour = vec3(1.);
#endif
    var distance: f32 = distance_to_transformed_primitive(
        transformed_position,
        primitive,
#ifdef EnableOrbitTrapColour
        &orbit_trap_colour,
#endif
    );

    return modify_distance(distance * (*primitive).transform.uniform_scale, primitive);
}


fn mix_material_samples(
    material_0: ptr<function, MaterialSample>,
    material_1: ptr<function, MaterialSample>,
    smoothing: f32,
) {
    material_0.diffuse_colour = mix(
        material_0.diffuse_colour,
        material_1.diffuse_colour,
        smoothing,
    );
    material_0.specular_probability = mix(
        material_0.specular_probability,
        material_1.specular_probability,
        smoothing,
    );
    material_0.specular_roughness = mix(
        material_0.specular_roughness,
        material_1.specular_roughness,
        smoothing,
    );
    material_0.specular_colour = mix(
        material_0.specular_colour,
        material_1.specular_colour,
        smoothing,
    );
    material_0.transmissive_probability = mix(
        material_0.transmissive_probability,
        material_1.transmissive_probability,
        smoothing,
    );
    material_0.transmissive_roughness = mix(
        material_0.transmissive_roughness,
        material_1.transmissive_roughness,
        smoothing,
    );
    material_0.extinction_colour = mix(
        material_0.extinction_colour,
        material_1.extinction_colour,
        smoothing,
    );
    material_0.emissive_colour = mix(
        material_0.emissive_colour,
        material_1.emissive_colour,
        smoothing,
    );
    material_0.refractive_index = mix(
        material_0.refractive_index,
        material_1.refractive_index,
        smoothing,
    );
    material_0.scattering_colour = mix(
        material_0.scattering_colour,
        material_1.scattering_colour,
        smoothing,
    );
    *material_1 = *material_0;
}


fn select_primitive(
    primitive_0: ptr<function, Primitive>,
    primitive_1: ptr<function, Primitive>,
    choice: bool,
) {
    (*primitive_0).id = select(
        (*primitive_0).id,
        (*primitive_1).id,
        choice,
    );
    (*primitive_1).id = (*primitive_0).id;
    (*primitive_0).material_id = select(
        (*primitive_0).material_id,
        (*primitive_1).material_id,
        choice,
    );
    (*primitive_1).material_id = (*primitive_0).material_id;
}


fn select_material_sample(
    material_0: ptr<function, MaterialSample>,
    material_1: ptr<function, MaterialSample>,
    choice: bool,
) {
    material_0.diffuse_colour = select(
        material_0.diffuse_colour,
        material_1.diffuse_colour,
        choice,
    );
    material_0.specular_probability = select(
        material_0.specular_probability,
        material_1.specular_probability,
        choice,
    );
    material_0.specular_roughness = select(
        material_0.specular_roughness,
        material_1.specular_roughness,
        choice,
    );
    material_0.specular_colour = select(
        material_0.specular_colour,
        material_1.specular_colour,
        choice,
    );
    material_0.transmissive_probability = select(
        material_0.transmissive_probability,
        material_1.transmissive_probability,
        choice,
    );
    material_0.transmissive_roughness = select(
        material_0.transmissive_roughness,
        material_1.transmissive_roughness,
        choice,
    );
    material_0.extinction_colour = select(
        material_0.extinction_colour,
        material_1.extinction_colour,
        choice,
    );
    material_0.emissive_colour = select(
        material_0.emissive_colour,
        material_1.emissive_colour,
        choice,
    );
    material_0.refractive_index = select(
        material_0.refractive_index,
        material_1.refractive_index,
        choice,
    );
    material_0.scattering_colour = select(
        material_0.scattering_colour,
        material_1.scattering_colour,
        choice,
    );
}


fn blend_primitives(
    distance_to_parent: f32,
    distance_to_child: f32,
    parent: ptr<function, Primitive>,
    child: ptr<function, Primitive>,
    parent_material: ptr<function, MaterialSample>,
    child_material: ptr<function, MaterialSample>,
) -> f32 {
    // TODO test if we can change this to blend_material_samples
    // and then just call this and blend_distances together
    // without losing performance
    switch (*parent).modifiers & BLEND_TYPE_MASK {
#ifdef EnablePrimitiveBlendSubtraction
        case SUBTRACTION {
            var smoothing: f32 = saturate_f32(
                0.5
                - 0.5
                * (distance_to_parent + distance_to_child)
                / (*parent).blend_strength
            );
            mix_material_samples(parent_material, child_material, smoothing);
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
            mix_material_samples(child_material, parent_material, smoothing);
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
            mix_material_samples(child_material, parent_material, smoothing);
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
