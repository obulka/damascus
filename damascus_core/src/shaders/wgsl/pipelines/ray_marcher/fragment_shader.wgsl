// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


//
// Ray Marching shader
//

#include RayMarcherConstants
#include Ray
#include Math
#include Random
#include PrimitiveSDFs
#include Texture
#include ProceduralTexture
#include Material
#include Primitive
#include PrimitiveModifiers
#include RayMarcherRenderParameters
#include SceneSDFs
#include Normals
#include Lights
#include Camera
#include AOVs


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
    seed: vec3f,
    offset: f32,
    distance_since_last_bounce: f32,
    intersection_position: vec3f,
    surface_normal: vec3f,
    previous_material_pdf: f32,
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

    var material_brdf: vec3f;
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

    var material_geometry_factor: f32 = 1.;
    if (light_sampling_material_pdf > 0. && _scene_parameters.num_lights > 0u) {
        // Perform MIS light sampling
        (*ray).colour += light_sampling(
            seed,
            ray,
            surface_normal,
            material_brdf,
            light_sampling_material_pdf,
        );
        material_geometry_factor = saturate_f32(dot((*ray).direction, surface_normal));
    }

    (*ray).colour += multiple_importance_sample(
        (*primitive).material.emissive_colour,
        (*ray).throughput,
        previous_material_pdf,
        sample_lights_pdf(f32(_scene_parameters.num_lights)),
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
fn march_path(seed: vec3f, ray: ptr<function, Ray>) {
    var nested_dielectrics: NestedDielectrics;
    push_dielectric(dielectric_from_atmosphere(), &nested_dielectrics);

#ifdef EnableAOVs
    var exit_early_with_aov: bool = (
        _render_parameters.output_aov > BEAUTY_AOV
        && _render_parameters.output_aov < STATS_AOV
    );
#endif

    var path_seed: vec3f = seed;
    var dynamic_level_of_detail = bool(_render_parameters.flags & DYNAMIC_LEVEL_OF_DETAIL);

    var distance_travelled: f32 = 0.;
    var distance_since_last_bounce = 0.;

    var last_step_distance: f32 = 1.;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = _render_parameters.hit_tolerance;

    var previous_material_pdf: f32 = 1.;

    // Data for the next ray
    var position_on_ray: vec3f = (*ray).origin;

    // March the ray
    while (
        distance_travelled < _render_parameters.max_distance
        && iterations < _render_parameters.max_ray_steps
        && element_sum_vec3f((*ray).throughput) > pixel_footprint
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
            var surface_normal: vec3f = sign(last_step_distance) * estimate_surface_normal(
                intersection_position,
                pixel_footprint,
            );

            var nearest_primitive: Primitive;
            find_nearest_primitive(
                position_on_ray,
                pixel_footprint,
                &nearest_primitive,
            );

#ifdef EnableAOVs
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
#endif
            previous_material_pdf = material_interaction(
                path_seed,
                2. * pixel_footprint * _render_parameters.shadow_bias,
                distance_since_last_bounce,
                intersection_position,
                surface_normal,
                previous_material_pdf,
                ray,
                &nearest_primitive,
                &nested_dielectrics,
            );

            // Exit if we have reached the bounce limit or with a random chance
            var rng: f32 = vec3f_to_random_f32(path_seed);
            var exit_probability: f32 = max_component_vec3f((*ray).throughput);
            if (bounces >= _render_parameters.max_bounces || exit_probability <= rng) {
#ifdef EnableAOVs
                final_aovs(
                    _render_parameters.output_aov,
                    bounces,
                    iterations,
                    distance_travelled,
                    ray,
                );
#endif
                return;
            }

            // Account for the lost intensity from the early exits
            (*ray).throughput /= exit_probability;

            distance_since_last_bounce = 0.;
            // Reset the pixel footprint so multiple reflections don't reduce precision
            // If this isn't done artifacts can appear after refraction/reflection
            pixel_footprint = _render_parameters.hit_tolerance;

            // Update the random seed for the next iteration
            path_seed = (
                vec3(5771.878299824461, 8245.463474397617, 3274.701002467521)
                * random_vec3f(path_seed.zxy + f32(bounces))
            );
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


@group(STORAGE_TEXTURE_BIND_GROUP) @binding(PROGRESSIVE_RENDERING_TEXTURE_BINDING)
var _progressive_rendering_texture: texture_storage_2d<rgba32float, read_write>;


struct FragmentInput {
    @location(TEXTURE_UV_LOCATION) uv_coordinate: vec4f,
    @builtin(position) frag_coordinate: vec4f, // pixel centers
}


@fragment
fn fs_main(in: FragmentInput) -> @location(PIXEL_COLOUR_LOCATION) vec4f {
    // Use the UV coordinates and resolution to get texture coordinates
    var current_pixel_indices: vec2f = uv_to_pixels(
        in.uv_coordinate.xy,
        _render_camera.sensor_resolution,
    );
    var texture_coordinates = vec2u(current_pixel_indices);

    // Load the current state of the progressive render, unless this is
    // the first path, in which case initialise as black
    var pixel_colour: vec4f = select(
        vec4f(),
        textureLoad(_progressive_rendering_texture, texture_coordinates),
        _render_state.paths_rendered_per_pixel > 0.,
    );

    // If the render is paused just return the current texture value
    if bool(_render_state.flags & PAUSED) {
        return pixel_colour;
    }

    // Create a random seed which will be different for each pixel
    var frag_coord_seed = vec3(vec2f_to_random_f32(in.frag_coordinate.xy));
    var seed = vec3(2214.2410943055584, 5844.16158969744, 6821.991985188833)
        * random_vec3f(
            _render_parameters.seeds
            + frag_coord_seed
            + _render_state.paths_rendered_per_pixel
        ) + vec3(3553.392716193805, 7251.898513581492, 1848.9387464811002)
        * vec2f_to_random_f32(current_pixel_indices);

    // Get modified UV coordinates with a random offset from the original
    // without straying outside the bounds of the current pixel. This
    // provides antialiasing for free
    var uv_coordinates: vec2f = pixels_to_uv(
        // Add a random offset to the uv_coordinates for anti-aliasing 
        current_pixel_indices + random_vec2f(seed.xy),
        _render_camera.sensor_resolution,
    );

    // Create and march a ray
    var ray: Ray = create_render_camera_ray(seed.zx, uv_coordinates);
    march_path(seed, &ray);

    // Read, update, and store the current value for our pixel
    // so that the render can be done progressively
    pixel_colour = (
        _render_state.paths_rendered_per_pixel * pixel_colour
        + vec4(ray.colour, 1.)
    ) / (_render_state.paths_rendered_per_pixel + 1.);
    textureStore(_progressive_rendering_texture, texture_coordinates, pixel_colour);

    return pixel_colour;
}
