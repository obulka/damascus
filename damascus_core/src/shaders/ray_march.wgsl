// Copyright 2022 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE.md file that should have been included as part
// of this package.


//
// Ray Marching shader
//

#include Ray
#include Math
#include Random
#include PrimitiveSDFs
#include Materials
#include Primitive
#include PrimitiveModifiers
#include RenderParameters
#include SceneSDFs
#include Normals
#include Lights
#include Camera
#include AOVs
#include VertexShader


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
    offset: f32,
    distance_since_last_bounce: f32,
    intersection_position: vec3<f32>,
    surface_normal: vec3<f32>,
    previous_material_pdf: f32,
    sample_all_lights: bool,
    ray: ptr<function, Ray>,
    primitive: ptr<function, Primitive>,
    nested_dielectrics: ptr<function, NestedDielectrics>,
) -> f32 {
    (*ray).origin = intersection_position;

    sample_equiangular(
        distance_since_last_bounce,
        ray,
        nested_dielectrics,
    );

    var material_brdf: vec3<f32>;
    var light_sampling_material_pdf: f32;
    var material_pdf: f32 = sample_material(
        seed,
        surface_normal,
        offset,
        primitive,
        nested_dielectrics,
        ray,
        &material_brdf,
        &light_sampling_material_pdf,
    );

    if (
        _scene_parameters.num_lights > 0u
        && _render_parameters.max_light_sampling_bounces > 0u
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
        sample_lights_pdf(f32(_scene_parameters.num_lights), visible_surface_area),
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
    var nested_dielectrics: NestedDielectrics;
    push_dielectric(dielectric_from_atmosphere(), &nested_dielectrics);

    var path_seed: vec3<f32> = seed;
    var roulette = bool(_render_parameters.roulette);
    var dynamic_level_of_detail = bool(_render_parameters.dynamic_level_of_detail);

    var sample_all_lights = bool(_render_parameters.sample_all_lights);

    var distance_travelled: f32 = 0.;
    var distance_since_last_bounce = 0.;

    var last_step_distance: f32 = 1.;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = _render_parameters.hit_tolerance;

    var previous_material_pdf: f32 = 1.;

    // Data for the next ray
    var position_on_ray: vec3<f32> = (*ray).origin;

    // March the ray
    while (
        distance_travelled < _render_parameters.max_distance
        && iterations < _render_parameters.max_ray_steps
        && sum_component_vec3f((*ray).throughput) > pixel_footprint
        && length((*ray).colour) < _render_parameters.max_brightness
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

            var nearest_primitive: Primitive;
            find_nearest_primitive(
                position_on_ray,
                pixel_footprint,
                &nearest_primitive,
            );

            // Early exit for the various AOVs that are not 'beauty'
            if exit_early_with_aov {
                early_exit_aovs(
                    _render_parameters.output_aov,
                    intersection_position,
                    intersection_position, // TODO world to local
                    surface_normal,
                    nearest_primitive.id,
                    ray,
                );
                return;
            }

            previous_material_pdf = material_interaction(
                path_seed,
                2. * pixel_footprint * _render_parameters.shadow_bias,
                distance_since_last_bounce,
                intersection_position,
                surface_normal,
                previous_material_pdf,
                sample_all_lights,
                ray,
                &nearest_primitive,
                &nested_dielectrics,
            );

            // Exit if we have reached the bounce limit or with a random chance
            var rng: f32 = vec3f_to_random_f32(path_seed);
            var exit_probability: f32 = max_component_vec3f((*ray).throughput);
            if (
                bounces >= _render_parameters.max_bounces
                || (roulette && exit_probability <= rng)
            ) {
                final_aovs(
                    _render_parameters.output_aov,
                    bounces,
                    iterations,
                    distance_travelled,
                    ray,
                );
                return;
            } else if roulette {
                // Account for the lost intensity from the early exits
                (*ray).throughput /= vec3(exit_probability);
            }

            distance_since_last_bounce = 0.;
            // Reset the pixel footprint so multiple reflections don't reduce precision
            // If this isn't done artifacts can appear after refraction/reflection
            pixel_footprint = _render_parameters.hit_tolerance;

            // Update the random seed for the next iteration
            path_seed = random_vec3f(path_seed.zxy + seed);
        }
        pixel_footprint += select(
            0.,
            _render_parameters.hit_tolerance * step_distance,
            dynamic_level_of_detail && !hit_object,
        );

        last_step_distance = signed_step_distance;
        iterations++;
    }

    var corrected_distance = (
        distance_since_last_bounce
        + _render_parameters.max_distance
        - distance_travelled
    );

    ray_miss_aovs(
        _render_parameters.output_aov,
        bounces,
        iterations,
        corrected_distance,
        ray,
        &nested_dielectrics,
    );
}


@group(2) @binding(0)
var _progressive_rendering_texture: texture_storage_2d<f32, read_write>;


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var frag_coord_seed = vec3(vec2f_to_random_f32(in.frag_coordinate.xy));
    var seed = random_vec3f(_render_parameters.seeds + frag_coord_seed);


    var progressive_rendering_texture_dimensions: vec2<u32> = textureDimensions(
        _progressive_rendering_texture,
    );


    var pixel_colour = vec3(0.);
    if _render_parameters.paths_per_pixel == 1 {
        var current_pixel_indices: vec2<u32> = (
            (1. + in.uv_coordinate)
            * progressive_rendering_texture_dimensions
            / 2.
        );
        pixel_colour = textureLoad
            _progressive_rendering_texture,
            current_pixel_indices,
        );
        
    }


    var exit_early_with_aov: bool = (
        _render_parameters.output_aov > BEAUTY_AOV
        && _render_parameters.output_aov < STATS_AOV
    );

    for (var path=1u; path <= _render_parameters.paths_per_pixel; path++) {
        var ray: Ray = create_render_camera_ray(seed, in.uv_coordinate);

        march_path(seed, exit_early_with_aov, &ray);

        pixel_colour += ray.colour;

        seed = random_vec3f(seed.yzx + frag_coord_seed);
    }

    return vec4(pixel_colour, 1.) / f32(_render_parameters.paths_per_pixel);
}
