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

struct Camera {
    focal_length: f32,
    horizontal_aperture: f32,
    near_plane: f32,
    far_plane: f32,
    focal_distance: f32,
    f_stops: f32,
    world_matrix: mat4x4<f32>,
}


// geometry/geometry.wgsl
// #include material.wgsl


struct Transform {
    position: vec3<f32>,
    rotation: vec3<f32>,
    scale: vec3<f32>,
    //skew: vec3<f32>,
}


struct Primitive {
    transform: Transform,
    material: Material,
    modifiers: u32,
    blend_strength: f32,
    num_children: u32,
}


struct Sphere {
    radius: f32,
    primitive: Primitive,
}


// ray_march.wgsl


struct VertexOut {
    @location(0) ray_direction: vec4<f32>,
    @builtin(position) position: vec4<f32>,
}


struct Uniforms {
    @size(16) angle: f32, // pad to 16 bytes
}


@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


var<private> v_positions: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(-1.0, -1.0),
);


@vertex
fn vs_main(@builtin(vertex_index) v_idx: u32) -> VertexOut {
    var out: VertexOut;

    out.position = vec4<f32>(v_positions[v_idx], 0.0, 1.0);
    out.position.x = out.position.x * cos(uniforms.angle);
    out.ray_direction = out.position;

    return out;
}


@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.ray_direction;
}
