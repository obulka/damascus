// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


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

@group(1) @binding(1)
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
            *distance_to_light = _render_parameters.max_distance;
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
            _render_parameters.hit_tolerance,
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
    var pixel_footprint: f32 = _render_parameters.hit_tolerance;

    var position: vec3<f32> = ray_origin;
    while (
        distance_travelled < distance_to_shade_point
        && iterations < _render_parameters.max_ray_steps / 2u
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
        pixel_footprint += step_distance * _render_parameters.hit_tolerance;
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
    var pixel_footprint: f32 = _render_parameters.hit_tolerance;
    var position: vec3<f32> = ray_origin;
    while (
        distance_travelled < distance_to_shade_point
        && iterations < _render_parameters.max_ray_steps / 2u
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
        pixel_footprint += step_distance * _render_parameters.hit_tolerance;
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

    if light_id < _scene_parameters.num_non_physical_lights
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

    for (var light_id=0u; light_id < _scene_parameters.num_lights; light_id++) {
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
