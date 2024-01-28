
const MAX_PRIMITIVES: u32 = 512u; // const not supported in the current version


struct Transform {
    translation: vec3<f32>,
    inverse_rotation: mat3x3<f32>,
    uniform_scale: f32,
}


struct Primitive {
    id: u32,
    shape: u32,
    transform: Transform, // Could we just make this a world matrix?
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


fn is_parent_of(parent: ptr<function, Primitive>, child: ptr<function, Primitive>) -> bool {
    return (
        (*parent).id < (*child).id
        && (*parent).id + (*parent).num_descendants >= (*child).id
    );
}


fn is_child_of(child: ptr<function, Primitive>, parent: ptr<function, Primitive>) -> bool {
    return (
        (*parent).id < (*child).id
        && (*parent).id + (*parent).num_descendants >= (*child).id
    );
}
