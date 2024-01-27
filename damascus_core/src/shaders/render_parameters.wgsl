
struct SceneParameters {
    atmosphere: Material,
    num_primitives: u32,
    num_lights: u32,
    num_non_physical_lights: u32,
}


struct RayMarcherParameters {
    paths_per_pixel: u32,
    roulette: u32,
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: vec3<f32>,
    dynamic_level_of_detail: u32,
    max_light_sampling_bounces: u32,
    sample_hdri: u32,
    sample_all_lights: u32,
    light_sampling_bias: f32,
    secondary_sampling: u32,
    hdri_offset_angle: f32,
    output_aov: u32,
    latlong: u32,
}


struct RenderParameters {
    ray_marcher: RayMarcherParameters,
    scene: SceneParameters,
}


// Global render settings
@group(0) @binding(0)
var<storage, read> _render_params: RenderParameters;
