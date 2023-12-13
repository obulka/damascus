// Copyright 2022 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE.md file that should have been included as part
// of this package.


//
// Ray Marching shader
//


// TODO: separate into files and use import statements


// math.wgsl

// wish we could overload functions
fn max_component_2(vector_: vec2<f32>) -> f32 {
    return max(vector_.x, vector_.y);
}

fn max_component_3(vector_: vec3<f32>) -> f32 {
    return max(vector_.x, max(vector_.y, vector_.z));
}

fn max_component_4(vector_: vec4<f32>) -> f32 {
    return max(vector_.x, max(vector_.y, max(vector_.z, vector_.w)));
}

fn min_component_2(vector_: vec2<f32>) -> f32 {
    return min(vector_.x, vector_.y);
}

fn min_component_3(vector_: vec3<f32>) -> f32 {
    return min(vector_.x, min(vector_.y, vector_.z));
}

fn min_component_4(vector_: vec4<f32>) -> f32 {
    return min(vector_.x, min(vector_.y, min(vector_.z, vector_.w)));
}


// random.wgsl


/**
 * Get a random value on the interval [0, 1].
 *
 * @arg seed: The random seed.
 *
 * @returns: A random value on the interval [0, 1].
 */
fn random(seed: f32) -> f32 {
    return fract(sin(seed * 91.3458) * 47453.5453);
}


// materials/material.wgsl


struct Material {
    diffuse: f32,
    diffuse_colour: vec3<f32>,
    specular: f32,
    specular_roughness: f32,
    specular_colour: vec3<f32>,
    transmissive: f32,
    transmissive_roughness: f32,
    transmissive_colour: vec3<f32>,
    emissive: f32,
    emissive_colour: vec3<f32>,
    refractive_index: f32,
    scattering_coefficient: f32,
    scattering_colour: vec3<f32>,
}


/**
 * Handle the interaction between a ray and the surface of a material.
 *
 * @arg step_distance: The last step size to be marched.
 * @arg pixel_footprint: A value proportional to the amount of world
 *     space that fills a pixel, like the distance from camera.
 * @arg distance: The distance travelled since the last bounce.
 * @arg intersection_position: The position at which the ray
 *     intersects the geometry.
 * @arg surface_normal: The surface normal at the intersection point.
 * @arg direction: The incoming ray direction.
 * @arg origin: The ray origin.
 * @arg ray_colour: The colour of the ray.
 * @arg throughput: The throughput of the ray.
 * @arg material: The material to interact with.
 */
fn material_interaction(
    step_distance: f32,
    pixel_footprint: f32,
    distance: f32,
    intersection_position: vec3<f32>,
    surface_normal: vec3<f32>,
    direction: ptr<function, vec3<f32>>,
    origin: ptr<function, vec3<f32>>,
    ray_colour: ptr<function, vec4<f32>>,
    throughput: ptr<function, vec4<f32>>,
    material: ptr<function, Material>,
) {
    *ray_colour = vec4<f32>((*material).diffuse_colour, 1.0);
}


// geometry/camera.wgsl
// #include "math.h"


struct Camera {
    enable_depth_of_field: u32, // bool isn't host-shareable?
    aperture: f32,
    world_matrix: mat4x4<f32>,
    inverse_projection_matrix: mat4x4<f32>,
}


@group(1) @binding(0)
var<uniform> _render_camera: Camera;

// geometry/geometry.wgsl
// #include "material.wgsl"


let MAX_PRIMITIVES: u32 = 512u; // const not supported in the current version


struct Transform {
    translation: vec3<f32>,
    inverse_rotation: mat3x3<f32>,
    scale: vec3<f32>,
    //skew: vec3<f32>,
}


struct Primitive {
    shape: u32,
    transform: Transform, // Could we just make this a world matrix?
    material: Material,
    modifiers: u32,
    blend_strength: f32,
    num_children: u32,
    custom_data: vec4<f32>,
}


struct Primitives {
    primitives: array<Primitive, MAX_PRIMITIVES>,
}


@group(2) @binding(0)
var<storage, read> _primitives: Primitives;


// lights.wgsl


let MAX_LIGHTS: u32 = 512u; // const not supported in the current version


struct Light {
    light_type: u32,
    dimensional_data: vec3<f32>,
    intensity: f32,
    falloff: f32,
    colour: vec3<f32>,
    shadow_hardness: f32,
    soften_shadows: u32,
}

struct Lights {
    lights: array<Light, MAX_LIGHTS>,
}

@group(3) @binding(0)
var<storage, read> _lights: Lights;


// sdfs.wgsl


/**
 * Compute the min distance from a point to a sphere.
 *
 * @arg position: The point to get the distance to, from the object.
 * @arg radius: The radius of the sphere.
 *
 * @returns: The minimum distance from the point to the shape.
 */
fn distance_to_sphere(position: vec3<f32>, radius: f32) -> f32 {
    return length(position) - radius;
}


// modifications.wgsl


/**
 * Transform a ray's location.
 *
 * @arg rayOrigin: The location the ray originates from.
 * @arg position: The amount to translate the ray.
 * @arg rotation: The amount to rotate the ray (radians).
 * @arg modifications: The modifications to perform.
 *     Each bit will enable a modification:
 *         bit 0: finite repetition
 *         bit 1: infinite repetition
 *         bit 2: elongation
 *         bit 3: mirror x
 *         bit 4: mirror y
 *         bit 5: mirror z
 * @arg repetition: The values to use when repeating the ray.
 * @arg elongation: The values to use when elongating the ray.
 *
 * @returns: The transformed ray origin.
 */
fn transform_ray(ray_origin: vec3<f32>, transform: Transform) -> vec3<f32> {
    var transformed_ray: vec3<f32> = (
        transform.inverse_rotation
        * (ray_origin - transform.translation)
    );
    // performShapeModification(
    //     modifications,
    //     repetition,
    //     elongation,
    //     transformed_ray
    // );

    return transformed_ray;
}


// ray_march.wgsl


// TODO pass these as uniforms
let MAX_RAY_DISTANCE: f32 = 1000.0;
let HIT_TOLERANCE: f32 = 0.001;
let MAX_RAY_STEPS: u32 = 10000u;
let MAX_BRIGHTNESS: f32 = 100000.0;
let LEVEL_OF_DETAIL: bool = true;
let BOUNCES_PER_RAY: u32 = 5u;
let ROULETTE: bool = true;

// Constants
let NORMAL_OFFSET_0 = vec3<f32>(0.5773, -0.5773, -0.5773);
let NORMAL_OFFSET_1 = vec3<f32>(-0.5773, -0.5773, 0.5773);
let NORMAL_OFFSET_2 = vec3<f32>(-0.5773, 0.5773, -0.5773);
let NORMAL_OFFSET_3 = vec3<f32>(0.5773, 0.5773, 0.5773);


struct VertexOut {
    @location(0) ray_direction: vec3<f32>,
    @location(1) ray_origin: vec3<f32>,
    @builtin(position) uv_position: vec4<f32>,
}


struct RenderGlobals {
    num_primitives: u32,
    num_lights: u32,
}


@group(0) @binding(0)
var<uniform> _render_globals: RenderGlobals;


var<private> v_positions: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(-1.0, -1.0),
);


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var out: VertexOut;

    out.uv_position = vec4<f32>(v_positions[vertex_index], 0.0, 1.0);
    // out.uv_position.x = out.uv_position.x * cos(x); // TODO something similar to maintain aspect of render cam

    out.ray_origin = vec3<f32>(
        _render_camera.world_matrix[3][0],
        _render_camera.world_matrix[3][1],
        _render_camera.world_matrix[3][2]
    );

    var direction: vec4<f32> = (
        _render_camera.inverse_projection_matrix
        * vec4<f32>(v_positions[vertex_index], 0.0, 1.0)
    );
    direction = _render_camera.world_matrix * vec4<f32>(direction.xyz, 0.0);

    out.ray_direction = normalize(direction.xyz);

    return out;
}


fn min_distance_to_primitive(
    ray_origin: vec3<f32>,
    pixel_footprint: f32,
    material: ptr<function, Material>,
) -> f32 {
    var min_distance: f32 = MAX_RAY_DISTANCE;

    for (
        var primitive_index = 0u;
        primitive_index < min(_render_globals.num_primitives, MAX_PRIMITIVES);
        primitive_index++
    ) {
        var primitive: Primitive = _primitives.primitives[primitive_index];

        var transformed_ray: vec3<f32> = transform_ray(ray_origin, primitive.transform);
        var uniform_scale: f32 = length(primitive.transform.scale); // TODO do better
        var distance_to_current: f32 = distance_to_sphere( // TODO add other shapes
            transformed_ray / uniform_scale,
            primitive.custom_data.x,
        ) * uniform_scale;

        if (abs(distance_to_current) < abs(min_distance)) {
            min_distance = distance_to_current;
            *material = primitive.material;
        }
    }

    return min_distance;
}


/**
 * Estimate the surface normal at the closest point on the closest
 * object to a point.
 *
 * @arg position: The point near which to get the surface normal
 * @arg pixel_footprint: A value proportional to the amount of world
 *     space that fills a pixel, like the distance from camera.
 *
 * @returns: The normalized surface normal.
 */
fn estimate_surface_normal(position: vec3<f32>, pixel_footprint: f32) -> vec3<f32> {
    var material: Material;
    return normalize(
        NORMAL_OFFSET_0 * min_distance_to_primitive(
            position + NORMAL_OFFSET_0 * HIT_TOLERANCE,
            pixel_footprint,
            &material,
        )
        + NORMAL_OFFSET_1 * min_distance_to_primitive(
            position + NORMAL_OFFSET_1 * HIT_TOLERANCE,
            pixel_footprint,
            &material,
        )
        + NORMAL_OFFSET_2 * min_distance_to_primitive(
            position + NORMAL_OFFSET_2 * HIT_TOLERANCE,
            pixel_footprint,
            &material,
        )
        + NORMAL_OFFSET_3 * min_distance_to_primitive(
            position + NORMAL_OFFSET_3 * HIT_TOLERANCE,
            pixel_footprint,
            &material,
        )
    );
}


/**
 * March a path through the scene.
 *
 * @arg ray_origin: The origin of the ray.
 * @arg ray_direction: The direction of the ray.
 *
 * @returns: The ray colour.
 */
fn march_path(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec4<f32> {
    var ray_colour = vec4<f32>(0.0);
    var throughput = vec4<f32>(1.0);

    var distance_travelled: f32 = 0.0;
    var distance_since_last_bounce = 0.0;

    var last_step_distance: f32 = 1.0;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = HIT_TOLERANCE;

    // Data for the next ray
    var origin: vec3<f32> = ray_origin;
    var position_on_ray: vec3<f32> = origin;
    var direction: vec3<f32> = ray_direction;

    // March the ray
    while (
        distance_travelled < MAX_RAY_DISTANCE
        && iterations < MAX_RAY_STEPS
        && (throughput.x + throughput.y + throughput.z + throughput.w) > HIT_TOLERANCE
        && length(ray_colour) < MAX_BRIGHTNESS
    ) {
        position_on_ray = origin + distance_since_last_bounce * direction;


        var nearest_material: Material;
        // Keep the signed distance so we know whether or not we are inside the object
        var signed_step_distance = min_distance_to_primitive(
            position_on_ray,
            pixel_footprint,
            &nearest_material,
        );

        // Take the absolute value, the true shortest distance to a surface
        var step_distance = abs(signed_step_distance);

        // Keep track of the distance the ray has travelled
        distance_travelled += step_distance;
        distance_since_last_bounce += step_distance;

        // Have we hit the nearest object?
        if (step_distance < pixel_footprint) {
            bounces++;
            var intersection_position = position_on_ray + step_distance * direction;

            // The normal to the surface at that position
            var surface_normal: vec3<f32> = sign(last_step_distance) * estimate_surface_normal(
                intersection_position,
                pixel_footprint,
            );

            // Early exit for the various AOVs that are not 'beauty'
            // if (_render_globals.output_aov > BEAUTY) {
            //     return aov_data(_render_globals.output_aov)
            // }

            material_interaction(
                step_distance,
                pixel_footprint,
                distance_since_last_bounce,
                intersection_position,
                surface_normal,
                &direction,
                &origin,
                &ray_colour,
                &throughput,
                &nearest_material,
            );

            // Exit if we have reached the bounce limit or with a random chance
            var rng: f32 = 0.0; // TODO add random functions
            var exit_probability: f32 = max_component_3(throughput.xyz);
            if (bounces > BOUNCES_PER_RAY || (ROULETTE && exit_probability <= rng)) {
                return ray_colour; // TODO object id in alpha after you can sample
            }
            else if (ROULETTE) {
                // Account for the lost intensity from the early exits
                throughput /= vec4<f32>(exit_probability);
            }

            distance_since_last_bounce = 0.0;
            // Reset the pixel footprint so multiple reflections don't reduce precision
            pixel_footprint = HIT_TOLERANCE;
        }
        else if (LEVEL_OF_DETAIL) {
            pixel_footprint += HIT_TOLERANCE * step_distance;
        }

        last_step_distance = signed_step_distance;
        iterations++;
    }

    return ray_colour;
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    var ray_colour = vec4<f32>(0.0);

    for (var path=1; path <= 1; path++) {
        ray_colour += march_path(in.ray_origin, in.ray_direction);
    }

    return ray_colour;
}
