// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


// Increasing OR decreasing this number seems to negatively affect performance
const NESTED_DIELECTRIC_DEPTH: u32 = 7u;


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
    extinction_colour: vec3<f32>,
    emissive_probability: f32,
    emissive_colour: vec3<f32>,
    refractive_index: f32,
    scattering_colour: vec3<f32>,
}


struct Dielectric {
    id: u32,
    refractive_index: f32,
    extinction_colour: vec3<f32>,
    scattering_colour: vec3<f32>,
}


struct NestedDielectrics {
    current_depth: u32,
    nested_dielectrics: array<Dielectric, NESTED_DIELECTRIC_DEPTH>,
}


// TODO this could be uniform but can't get the alignment right
@group(1) @binding(2)
var<storage, read> _atmosphere: Material;


fn dielectric_from_atmosphere() -> Dielectric {
    return Dielectric(
        0u,
        _atmosphere.refractive_index,
        _atmosphere.extinction_colour,
        _atmosphere.scattering_colour,
    );
}


fn dielectric_from_primitive(primitive: ptr<function, Primitive>) -> Dielectric {
    return Dielectric(
        (*primitive).id,
        (*primitive).material.refractive_index,
        (*primitive).material.extinction_colour,
        (*primitive).material.scattering_colour,
    );
}


fn push_dielectric(
    dielectric: Dielectric,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) {
    (*nested_dielectrics).nested_dielectrics[(*nested_dielectrics).current_depth] = dielectric;
    (*nested_dielectrics).current_depth = min(
        NESTED_DIELECTRIC_DEPTH,
        (*nested_dielectrics).current_depth + 1u,
    )
}


fn peek_dielectric(nested_dielectrics: ptr<function, NestedDielectrics>) -> Dielectric {
    return (*nested_dielectrics).nested_dielectrics[(*nested_dielectrics).current_depth - 1u];
}


fn pop_dielectric(nested_dielectrics: ptr<function, NestedDielectrics>) -> Dielectric {
    (*nested_dielectrics).current_depth = select(
        0u,
        (*nested_dielectrics).current_depth - 1u,
        (*nested_dielectrics).current_depth > 0u,
    )
    return (*nested_dielectrics).nested_dielectrics[(*nested_dielectrics).current_depth];
}

fn peek_previous_dielectric(nested_dielectrics: ptr<function, NestedDielectrics>) -> Dielectric {
    return (*nested_dielectrics).nested_dielectrics[(*nested_dielectrics).current_depth - 2u];
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
        if sin_transmitted_squared >= 1. {
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
    nested_dielectrics: ptr<function, NestedDielectrics>,
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
    var diffuse_probability: f32 =  1. - specular_probability - transmissive_probability;

    var incident_dielectric: Dielectric = peek_dielectric(nested_dielectrics);
    var is_exiting: bool = is_exiting_primitive(primitive, &incident_dielectric);

    var refracted_dielectric: Dielectric;
    if is_exiting {
        refracted_dielectric = peek_previous_dielectric(nested_dielectrics);
    } else {
        refracted_dielectric = dielectric_from_primitive(primitive);
    }

    // Compute reflectivity for fresnel
    var reflectivity: f32 = schlick_reflection_coefficient(
        (*ray).direction,
        surface_normal,
        incident_dielectric.refractive_index,
        refracted_dielectric.refractive_index,
    );

    // Adjust probabilities according to fresnel
    specular_probability = select(
        specular_probability,
        (specular_probability + f32(transmissive_probability > 0.)) * mix(
            specular_probability,
            1.,
            reflectivity,
        ),
        specular_probability > 0. || transmissive_probability > 0.,
    );
    transmissive_probability = select(
        transmissive_probability,
        (
            transmissive_probability * (1. - specular_probability)
            / (1. - (*primitive).material.specular_probability)
        ),
        (specular_probability > 0. || transmissive_probability > 0.)
        && (*primitive).material.specular_probability < 1.,
    );

    // Interact with material according to the adjusted probabilities
    var rng: f32 = vec3f_to_random_f32(seed);
    var material_pdf: f32;
    if (
        (*primitive).material.transmissive_probability > 0.
        && transmissive_probability > 0.
        && rng <= transmissive_probability
    ) {
        // Transmissive bounce
        var refractive_ratio: f32 = (
            incident_dielectric.refractive_index
            / refracted_dielectric.refractive_index
        );
        var cos_incident: f32 = -dot((*ray).direction, surface_normal);
        var sin_transmitted_squared: f32 = refractive_ratio * refractive_ratio * (
            1. - cos_incident * cos_incident
        );

        if sin_transmitted_squared < 1. {
            // Refract
            var cos_transmitted = sqrt(1. - sin_transmitted_squared);
            var ideal_refracted_direction: vec3<f32> = normalize(
                refractive_ratio * (*ray).direction
                + (refractive_ratio * cos_incident - cos_transmitted) * surface_normal
            );
            (*ray).direction = normalize(mix(
                ideal_refracted_direction,
                -diffuse_direction,
                (*primitive).material.transmissive_roughness, // Assume roughness squared by CPU
            ));

            // Offset the point so that it doesn't get trapped on the surface.
            (*ray).origin += offset * ((*ray).direction - surface_normal);

            *material_brdf = vec3(1.);

            if is_exiting {
                pop_dielectric(nested_dielectrics);
            } else {
                push_dielectric(refracted_dielectric, nested_dielectrics);
            }

            var probability_over_pi = transmissive_probability / PI;
            *light_sampling_pdf = 0.;
            return probability_over_pi * dot(ideal_refracted_direction, (*ray).direction);
        }

        // Reflect instead
        specular_probability = transmissive_probability;
    }
    if (
        diffuse_probability <= 0.
        || (specular_probability > 0. && rng <= specular_probability + transmissive_probability)
    ) {
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


fn checkerboard(seed: vec3<f32>) -> vec3<f32> {
    var square_signal: vec3<f32> = sign(fract(seed * 0.5) - 0.5);
    return vec3(0.5 - 0.5 * square_signal.x * square_signal.y * square_signal.z);
}


fn grade(
    lift: f32,
    black_point: f32,
    white_point: f32,
    gamma: f32,
    colour: vec3<f32>,
) -> vec3<f32> {
    return pow(
        (1. - lift) * saturate_vec3f(colour - black_point) / (white_point - black_point) + lift,
        vec3(1. / gamma),
    );
}


fn procedurally_texture(
    seed: vec3<f32>,
    colour: vec3<f32>,
    texture: ProceduralTexture,
) -> vec3<f32> {
    switch texture.texture_type {
        case 0u, default {
            // None
            return colour;
        }
        case 1u {
            // Grade
            return grade(
                texture.lift,
                texture.black_point,
                texture.white_point,
                texture.gamma,
                colour,
            );
        }
        case 2u {
            // Checkerboard
            return colour * grade(
                texture.lift,
                texture.black_point,
                texture.white_point,
                texture.gamma,
                checkerboard(seed),
            );
        }
    }
}


fn sample_equiangular(
    distance_since_last_bounce: f32,
    ray: ptr<function, Ray>,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) {
    var current_dielectric: Dielectric = peek_dielectric(nested_dielectrics);
    (*ray).throughput *= exp(
        -current_dielectric.extinction_colour * distance_since_last_bounce,
    );
}
