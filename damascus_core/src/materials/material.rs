use glam::Vec3;

#[derive(Default)]
pub struct Material {
    pub diffuse: f32,
    pub diffuse_colour: Vec3,
    pub specular: f32,
    pub specular_roughness: f32,
    pub specular_colour: Vec3,
    pub transmissive: f32,
    pub transmissive_roughness: f32,
    pub transmissive_colour: Vec3,
    pub emissive: f32,
    pub emissive_colour: Vec3,
    pub refractive_index: f32,
    pub scattering_coefficient: f32,
    pub scattering_colour: Vec3,
}
