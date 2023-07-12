use glam::Vec3;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUMaterial {
    diffuse: f32,
    diffuse_colour: [f32; 3],
    specular: f32,
    specular_roughness: f32,
    specular_colour: [f32; 3],
    transmissive: f32,
    transmissive_roughness: f32,
    transmissive_colour: [f32; 3],
    emissive: f32,
    emissive_colour: [f32; 3],
    refractive_index: f32,
    scattering_coefficient: f32,
    scattering_colour: [f32; 3],
}

#[derive(Debug, Default)]
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

impl Material {
    pub fn to_gpu_material(&self) -> GPUMaterial {
        GPUMaterial {
            diffuse: self.diffuse,
            diffuse_colour: self.diffuse_colour.to_array(),
            specular: self.specular,
            specular_roughness: self.specular_roughness,
            specular_colour: self.specular_colour.to_array(),
            transmissive: self.transmissive,
            transmissive_roughness: self.transmissive_roughness,
            transmissive_colour: self.transmissive_colour.to_array(),
            emissive: self.emissive,
            emissive_colour: self.emissive_colour.to_array(),
            refractive_index: self.refractive_index,
            scattering_coefficient: self.scattering_coefficient,
            scattering_colour: self.scattering_colour.to_array(),
        }
    }
}
