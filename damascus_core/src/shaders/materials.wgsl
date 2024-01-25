
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
