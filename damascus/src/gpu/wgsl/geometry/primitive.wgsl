// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.


struct Transform {
    translation: vec3f,
    uniform_scale: f32,
    inverse_rotation: mat3x3f,
}


struct Primitive {
    id: u32,
    material_id: u32,
    num_descendants: u32,
    shape: u32,
    modifiers: u32,
    negative_repetitions: vec3f,
    blend_strength: f32,
    positive_repetitions: vec3f,
    wall_thickness: f32,
    spacing: vec3f,
    edge_radius: f32,
    elongation: vec3f,
    dimensional_data: vec4f,
    transform: Transform,
}


@group(STORAGE_BIND_GROUP) @binding(PRIMITIVES_BINDING)
var<storage, read> _primitives: array<Primitive>;


@group(STORAGE_BIND_GROUP) @binding(EMISSIVE_INDICES_BINDING)
var<storage, read> _emissive_indices: array<u32>;


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
            + _primitives[prospective_parent_id - 1u].num_descendants
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
