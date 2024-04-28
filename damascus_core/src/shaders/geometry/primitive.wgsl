// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


const MAX_PRIMITIVES: u32 = 512u;


struct Transform {
    translation: vec3<f32>,
    inverse_rotation: mat3x3<f32>,
    uniform_scale: f32,
}


struct Primitive {
    id: u32,
    shape: u32,
    transform: Transform,
    material: Material,
    modifiers: u32,
    negative_repetitions: vec3<f32>,
    positive_repetitions: vec3<f32>,
    spacing: vec3<f32>,
    blend_strength: f32,
    wall_thickness: f32,
    edge_radius: f32,
    elongation: vec3<f32>,
    num_descendants: u32,
    dimensional_data: vec4<f32>,
}


struct Primitives {
    primitives: array<Primitive, MAX_PRIMITIVES>,
}


@group(1) @binding(0)
var<storage, read> _primitives: Primitives;


struct EmissiveIndex {
    @size(16) index: u32,
}


@group(0) @binding(4)
var<uniform> _emissive_indices: array<EmissiveIndex, MAX_PRIMITIVES>;


fn is_parent_of(parent: ptr<function, Primitive>, prospective_child_id: u32) -> bool {
    return (
        (*parent).id < prospective_child_id
        && (*parent).id + (*parent).num_descendants >= prospective_child_id
    );
}


fn is_child_of(child: ptr<function, Primitive>, prospective_parent_id: u32) -> bool {
    return (
        prospective_parent_id < (*child).id
        && (
            prospective_parent_id
            + _primitives.primitives[prospective_parent_id - 1u].num_descendants
        ) >= (*child).id
    );
}


fn is_exiting_primitive(
    primitive: ptr<function, Primitive>,
    current_dielectric: ptr<function, Dielectric>,
) -> bool {
    return (
        (*current_dielectric).id > 0u
        && (
            (*current_dielectric).id == (*primitive).id
            || is_parent_of(primitive, (*current_dielectric).id)
            || is_child_of(primitive, (*current_dielectric).id)
        )
    );
}
