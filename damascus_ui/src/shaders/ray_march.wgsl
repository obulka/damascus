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
    // enable_depth_of_field: bool,
    // aperture: f32,
    world_matrix: mat4x4<f32>,
    inverse_projection_matrix: mat4x4<f32>,
}


@group(1) @binding(0)
var<uniform> _render_camera: Camera;

// geometry/geometry.wgsl
// #include "material.wgsl"


struct Transform {
    position: vec3<f32>,
    rotation: vec3<f32>,
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
    primitives: array<Primitive>,
}

@group(2) @binding(0)
var<storage, read> _primitives: Primitives;

// ray_march.wgsl


struct VertexOut {
    @location(0) ray_direction: vec3<f32>,
    @location(1) ray_origin: vec3<f32>,
    @builtin(position) uv_position: vec4<f32>,
}


struct Uniforms {
    @size(16) angle: f32, // pad to 16 bytes
}


@group(0) @binding(0)
var<uniform> _uniforms: Uniforms;


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
    out.uv_position.x = out.uv_position.x * cos(_uniforms.angle);

    out.ray_origin = vec3<f32>(
        _render_camera.world_matrix[0][3],
        _render_camera.world_matrix[1][3],
        _render_camera.world_matrix[2][3]
    );

    var direction: vec4<f32> = (
        _render_camera.inverse_projection_matrix
        * vec4<f32>(v_positions[vertex_index], 0.0, 1.0)
    );
    direction = (
        _render_camera.world_matrix
        * vec4<f32>(direction.xyz, 0.0)
    );

    out.ray_direction = normalize(direction.xyz);

    return out;
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.ray_direction, 1.0);
}
