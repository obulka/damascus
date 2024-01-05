use crevice::std140::AsStd140;
use glam::Vec3;
use rand::random;

use crate::{
    renderers::AOVs,
    scene::{Scene, Std140SceneParameters},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd140)]
pub struct GPURayMarcher {
    paths_per_pixel: u32,
    roulette: u32,
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: Vec3,
    dynamic_level_of_detail: u32,
    max_light_sampling_bounces: u32,
    sample_hdri: u32,
    sample_all_lights: u32,
    light_sampling_bias: f32,
    secondary_sampling: u32,
    // TODO add scattering material
    hdri_offset_angle: f32,
    // TODO precomputed irradiance
    // TODO variance & adaptive sampling
    output_aov: u32,
    // TODO resolution
    latlong: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd140)]
pub struct RenderParameters {
    ray_marcher: Std140GPURayMarcher,
    scene: Std140SceneParameters,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RayMarcher {
    pub scene: Scene,
    pub paths_per_pixel: u32,
    pub roulette: bool,
    pub max_distance: f32,
    pub max_ray_steps: u32,
    pub max_bounces: u32,
    pub hit_tolerance: f32,
    pub shadow_bias: f32,
    pub max_brightness: f32,
    pub seeds: Vec3,
    pub dynamic_level_of_detail: bool,
    pub max_light_sampling_bounces: u32,
    pub sample_hdri: bool,
    pub sample_all_lights: bool,
    pub light_sampling_bias: f32,
    pub secondary_sampling: bool,
    // TODO add scattering material
    pub hdri_offset_angle: f32,
    // TODO precomputed irradiance
    // TODO variance & adaptive sampling
    pub output_aov: AOVs,
    // TODO resolution
    pub latlong: bool,
}

impl Default for RayMarcher {
    fn default() -> Self {
        RayMarcher {
            scene: Scene::default(),
            paths_per_pixel: 1,
            roulette: true,
            max_distance: 100.0,
            max_ray_steps: 1000,
            max_bounces: 1,
            hit_tolerance: 0.0001,
            shadow_bias: 1.0,
            max_brightness: 999999999.9,
            seeds: Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()),
            dynamic_level_of_detail: true,
            max_light_sampling_bounces: 7,
            sample_hdri: false,
            sample_all_lights: true,
            light_sampling_bias: 1.0,
            secondary_sampling: false,
            hdri_offset_angle: 0.0,
            output_aov: AOVs::default(),
            latlong: false,
        }
    }
}

impl RayMarcher {
    pub fn to_gpu(&self) -> Std140GPURayMarcher {
        GPURayMarcher {
            paths_per_pixel: self.paths_per_pixel.max(1),
            roulette: self.roulette as u32,
            max_distance: self.max_distance,
            max_ray_steps: self.max_ray_steps,
            max_bounces: self.max_bounces.max(0),
            hit_tolerance: self.hit_tolerance.max(0.),
            shadow_bias: self.shadow_bias,
            max_brightness: self.max_brightness,
            seeds: self.seeds,
            dynamic_level_of_detail: self.dynamic_level_of_detail as u32,
            max_light_sampling_bounces: self.max_light_sampling_bounces,
            sample_hdri: self.sample_hdri as u32,
            sample_all_lights: self.sample_all_lights as u32,
            light_sampling_bias: self.light_sampling_bias,
            secondary_sampling: self.secondary_sampling as u32,
            hdri_offset_angle: self.hdri_offset_angle,
            output_aov: self.output_aov as u32,
            latlong: self.latlong as u32,
        }
        .as_std140()
    }

    pub fn as_render_parameters(&self) -> Std140RenderParameters {
        RenderParameters {
            ray_marcher: self.to_gpu(),
            scene: self.scene.create_scene_parameters(),
        }
        .as_std140()
    }
}
