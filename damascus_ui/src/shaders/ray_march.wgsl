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


struct VertexOut {
    @location(0) ray_direction: vec3<f32>,
    @location(1) ray_origin: vec3<f32>,
    @builtin(position) uv_position: vec4<f32>,
}


struct RenderGlobals {
    num_primitives: u32,
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
        _render_camera.world_matrix[0][3],
        _render_camera.world_matrix[1][3],
        _render_camera.world_matrix[2][3]
    );

    var direction: vec4<f32> = (
        _render_camera.inverse_projection_matrix
        * vec4<f32>(v_positions[vertex_index], 0.0, 1.0)
    );
    direction = _render_camera.world_matrix * vec4<f32>(direction.xyz, 0.0);

    out.ray_direction = normalize(direction.xyz);

    return out;
}


fn min_distance_to_primitive(ray_origin: vec3<f32>, pixel_footprint: f32) -> f32 {
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
        }
    }

    return min_distance;
}


fn march_path(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec4<f32> {
    var ray_colour = vec4<f32>(0.0);
    var throughput = vec4<f32>(1.0);

    var distance_travelled: f32 = 0.0;
    var distance_since_last_bounce = 0.0;

    var last_step_distance: f32 = 1.0;

    var iterations: u32 = 0u;
    var bounces: u32 = 0u;

    var pixel_footprint: f32 = HIT_TOLERANCE;

    var origin: vec3<f32> = ray_origin;
    var position_on_ray: vec3<f32> = origin;
    var direction: vec3<f32> = ray_direction;

    while (
        distance_travelled < MAX_RAY_DISTANCE
        && iterations < MAX_RAY_STEPS
        && (throughput.x + throughput.y + throughput.z + throughput.w) > HIT_TOLERANCE
        && length(ray_colour) < MAX_BRIGHTNESS
    ) {
        position_on_ray = origin + distance_since_last_bounce * direction;

        var signed_step_distance = min_distance_to_primitive(
            position_on_ray,
            pixel_footprint,
        );

        var step_distance = abs(signed_step_distance);

        distance_travelled += step_distance;
        distance_since_last_bounce += step_distance;

        if (step_distance < pixel_footprint) {
            return throughput;

            // var intersection_position = position_on_ray + step_distance * direction;

            // distance_since_last_bounce = 0.0;
            // pixel_footprint = HIT_TOLERANCE;
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
