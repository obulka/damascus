// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

const DIRECTIONAL: u32 = 0u;
const POINT: u32 = 1u;
const AMBIENT: u32 = 2u;
const AMBIENT_OCCLUSION: u32 = 3u;


struct Light {
    light_type: u32,
    dimensional_data: vec3f,
    intensity: f32,
    falloff: u32,
    colour: vec3f,
    shadow_hardness: f32,
    soften_shadows: u32,
}


struct Lights {
    lights: array<Light>,
}


@group(STORAGE_BIND_GROUP) @binding(1)
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
    emittance: vec3f,
    throughput: vec3f,
    pdf_0: f32,
    pdf_1: f32,
) -> vec3f {
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
fn sample_lights_pdf(num_lights: f32) -> f32 {
    return 1. / num_lights;
}


/**
 * Get the probability distribution function for equi-angular sampling.
 *
 * @arg step_distance: A uniform step distance along the ray.
 * @arg max_ray_distance: The maximum distance the ray can travel.
 * @arg ray: The ray.
 * @arg light_position: The position of the light.
 * @arg distance: The equi-angular distance along the ray.
 *
 * @returns: The probability distribution function.
 */
fn sample_equiangular_pdf(
    step_distance: f32,
    max_ray_distance: f32,
    light_position: vec3f,
    ray: ptr<function, Ray>,
    distance: ptr<function, f32>,
) -> f32 {
    // Get the coordinate of the closest point to the light along an
    // infinite ray
    var delta: f32 = dot(light_position - (*ray).origin, (*ray).direction);

    // Get distance this point is from light
    var distance_to_light: f32 = length(
        (*ray).origin + delta * (*ray).direction - light_position,
    );

    if (distance_to_light == 0.) {
        *distance = 0.;
        return 1.;
    }

    // Get the angle of the endpoints
    var theta_a: f32 = atan2(-delta, distance_to_light);
    var theta_b: f32 = atan2(max_ray_distance - delta, distance_to_light);

    // Take a sample
    var sample: f32 = distance_to_light * tan(mix(theta_a, theta_b, step_distance));

    *distance = delta + sample;

    if (theta_a != theta_b) {
        return (
            distance_to_light
            / ((theta_b - theta_a) * (distance_to_light * distance_to_light + sample * sample))
        );
    }

    return 1.;
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
    ray_origin: vec3f,
    surface_normal: vec3f,
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
    ray_origin: vec3f,
    ray_direction: vec3f,
    distance_to_shade_point: f32,
    hardness: f32,
) -> f32 {
    var distance_travelled: f32 = 0.;
    var shadow_intensity: f32 = 1.;
    var last_step_distance: f32 = 3.40282346638528859812e38; // FLT_MAX

    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_parameters.hit_tolerance;

    var position: vec3f = ray_origin;
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
    ray_origin: vec3f,
    ray_direction: vec3f,
    distance_to_shade_point: f32,
) -> f32 {
    var distance_travelled: f32 = 0.;
    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_parameters.hit_tolerance;
    var position: vec3f = ray_origin;
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
 * @arg light_index: The index of the chosen light to sample.
 * @arg surface_position: The point on the surface to compute the
 *     light intensity at.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg light_direction: The direction from the surface to the light.
 *
 * @returns: The colour of the sampled light.
 */
fn sample_non_physical_light(
    light_index: u32,
    surface_position: vec3f,
    surface_normal: vec3f,
    light_geometry_factor: ptr<function, f32>,
) -> vec3f {
    // Read the light properties
    var light: Light = _lights.lights[light_index];

    switch light.light_type {
#ifdef EnableDirectionalLights
        case DIRECTIONAL {
            var light_direction: vec3f = normalize(-light.dimensional_data);
            *light_geometry_factor = saturate_f32(dot(light_direction, surface_normal));

            var shadow_intensity_at_position: f32;
#ifdef EnableSoftShadows
            if bool(light.soften_shadows) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    _render_parameters.max_distance,
                    light.shadow_hardness,
                );
            } else {
#endif
                shadow_intensity_at_position = sample_shadow(
                    surface_position,
                    light_direction,
                    _render_parameters.max_distance,
                );
#ifdef EnableSoftShadows
            }
#endif

            return light.colour * light.intensity * shadow_intensity_at_position;
        }
#endif
#ifdef EnablePointLights
        case POINT {
            var light_direction: vec3f = light.dimensional_data - surface_position;
            var distance_to_light: f32 = length(light_direction);
            light_direction = normalize(light_direction);
            *light_geometry_factor = saturate_f32(dot(light_direction, surface_normal));

            var shadow_intensity_at_position: f32;
#ifdef EnableSoftShadows
            if bool(light.soften_shadows) {
                shadow_intensity_at_position = sample_soft_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                    light.shadow_hardness,
                );
            } else {
#endif
                shadow_intensity_at_position = sample_shadow(
                    surface_position,
                    light_direction,
                    distance_to_light,
                );
#ifdef EnableSoftShadows
            }
#endif

            return (
                light.colour
                * light_intensity(&light, distance_to_light)
                * shadow_intensity_at_position
            );
        }
#endif
        case AMBIENT, default {
            // Simply return the colour intensity.
            return light.intensity * light.colour;
        }
#ifdef EnableAmbientOcclusion
        case AMBIENT_OCCLUSION {
            return light.colour * sample_ambient_occlusion(
                surface_position,
                surface_normal,
                light.intensity,
                u32(light.dimensional_data.x)
            );
        }
#endif
    }
}


fn sample_physical_light(
    seed: vec3f,
    light_index: u32,
    surface_position: vec3f,
    surface_normal: vec3f,
    light_geometry_factor: ptr<function, f32>,
    light_sampling_pdf: ptr<function, f32>,
) -> vec3f {
    var emissive_primitive: Primitive = _primitives.primitives[_emissive_indices[light_index]];
    var light_position: vec3f = emissive_primitive.transform.translation;
    var radius: f32 = length(
        emissive_primitive.transform.uniform_scale * emissive_primitive.dimensional_data,
    );

    var light_normal: vec3f = cosine_direction_in_hemisphere(
        seed.xy,
        normalize(surface_position - light_position),
    );
    var light_direction: vec3f = light_position - surface_position + light_normal * length(
        emissive_primitive.transform.uniform_scale * emissive_primitive.dimensional_data,
    );
    var distance_to_light: f32 = length(light_direction);
    light_direction = normalize(mix(
        normalize(light_direction),
        cosine_direction_in_hemisphere(seed.y + seed.zx, surface_normal),
        _render_parameters.light_sampling_bias,
    ));

    *light_geometry_factor = dot(surface_normal, light_direction);
    if *light_geometry_factor <= 0. {
        *light_geometry_factor = 0.;
        return vec3f();
    }

    var distance_travelled: f32 = 0.;
    var iterations: u32 = 0u;
    var pixel_footprint: f32 = _render_parameters.hit_tolerance;
    var position: vec3f = surface_position;
    while (
        distance_travelled < distance_to_light
        && iterations < _render_parameters.max_ray_steps / 2u
    ) {
        var step_distance: f32 = abs(signed_distance_to_scene(
            position,
            pixel_footprint,
        ));

        if step_distance < pixel_footprint {
            var nearest_primitive: Primitive;
            find_nearest_primitive(
                position,
                pixel_footprint,
                &nearest_primitive,
            );
            *light_geometry_factor /=  max(
                distance_travelled * distance_travelled,
                pixel_footprint,
            );
            var radius: f32 = length(
                nearest_primitive.transform.uniform_scale * nearest_primitive.dimensional_data,
            );
            *light_sampling_pdf /= 2.0f * PI * radius * radius;
            return nearest_primitive.material.emissive_colour;
        }

        position += light_direction * step_distance;
        distance_travelled += step_distance;
        pixel_footprint += step_distance * _render_parameters.hit_tolerance;
        iterations++;
    }

    return vec3f();
}


/**
 * Perform direct illumination light sampling.
 *
 * @arg seed: The seed to use in randomization.
 * @arg ray: The material sampling ray that will leave the surface.
 * @arg surface_normal: The normal to the surface at the position we
 *     are sampling the illumination of.
 * @arg material_brdf: The BRDF of the surface at the position we
 *     are sampling the illumination of.
 * @arg material_pdf: The PDF of the material we are sampling the
 *     direct illumination of.
 *
 * @returns: The colour of the sampled light.
 */
fn light_sampling(
    seed: vec3f,
    ray: ptr<function, Ray>,
    surface_normal: vec3f,
    material_brdf: vec3f,
    material_pdf: f32,
) -> vec3f {
    var light_id = u32(
        f32(_scene_parameters.num_lights)
        * vec3f_to_random_f32(seed * vec3(4.82314437644, 1.9647352337074, 5.084007537183))
    );
    var light_sampling_pdf: f32 = 1. / f32(_scene_parameters.num_lights);
    var light_colour = vec3(0.);
    var light_geometry_factor: f32 = 1.;

#ifdef EnablePhysicalLights
    if (light_id < _scene_parameters.num_non_physical_lights) {
#endif
        light_colour = sample_non_physical_light(
            light_id,
            (*ray).origin,
            surface_normal,
            &light_geometry_factor,
        );
#ifdef EnablePhysicalLights
    } else {
        light_colour = sample_physical_light(
            seed,
            min(
                light_id - _scene_parameters.num_non_physical_lights,
                _scene_parameters.num_lights - _scene_parameters.num_non_physical_lights - 1u,
            ),
            (*ray).origin,
            surface_normal,
            &light_geometry_factor,
            &light_sampling_pdf,
        );
    }
#endif

    return multiple_importance_sample(
        light_colour,
        (*ray).throughput * material_brdf * light_geometry_factor / light_sampling_pdf,
        light_sampling_pdf,
        material_pdf * light_geometry_factor
    );
}
