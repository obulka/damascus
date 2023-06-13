struct Camera {
    focal_length: f32,
    horizontal_aperture: f32,
    near_plane: f32,
    far_plane: f32,
    focal_distance: f32,
    f_stops: f32,
    world_matrix: mat4x4<f32>,
};


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
};


struct PrimitiveProperties {
    position: vec3<f32>,
    rotation: vec3<f32>,
    scale: vec3<f32>,
    modifiers: u32,
    blend_strength: f32,
    num_children: u32,
    material: Material,
};


struct Sphere {
    radius: f32,
    properties: PrimitiveProperties,
};
