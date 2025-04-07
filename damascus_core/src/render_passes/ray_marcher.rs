// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::time::SystemTime;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2, Vec3};
use strum::{Display, EnumIter, EnumString};

use super::{RenderPass, TextureProcessingPass};

use crate::{
    scene::Scene,
    shaders::{
        self,
        ray_marcher::{
            all_directives_for_light, all_directives_for_material, all_directives_for_primitive,
            all_directives_for_ray_marcher, directives_for_light, directives_for_material,
            directives_for_primitive, directives_for_ray_marcher, RAY_MARCHER_FRAGMENT_SHADER,
            RAY_MARCHER_VERTEX_SHADER,
        },
        ShaderSource,
    },
    textures::AOVs,
    DualDevice, Hashable,
};

pub const MAX_TEXTURE_DIMENSION: u32 = 8192; // TODO get rid of this

// A change in the data within this struct will trigger the pass to
// recompile
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherCompilationOptions {
    pub enable_dynamic_recompilation_for_materials: bool,
    pub enable_dynamic_recompilation_for_primitives: bool,
    pub enable_dynamic_recompilation_for_ray_marcher: bool,
    pub enable_dynamic_recompilation_for_lights: bool,
}

impl Default for RayMarcherCompilationOptions {
    fn default() -> Self {
        Self {
            enable_dynamic_recompilation_for_materials: true,
            enable_dynamic_recompilation_for_primitives: true,
            enable_dynamic_recompilation_for_ray_marcher: true,
            enable_dynamic_recompilation_for_lights: true,
        }
    }
}

impl RayMarcherCompilationOptions {}

impl Hashable for RayMarcherCompilationOptions {}

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct RayMarcherPipelineOptions {
    pub num_primitives: usize,
    pub num_lights: usize,
}

impl Hashable for RayMarcherPipelineOptions {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcherRenderState {
    paths_rendered_per_pixel: f32,
    resolution: Vec2,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherRenderState {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub paths_rendered_per_pixel: u32,
    pub resolution: UVec2,
    pub paused: bool,
}

impl Default for RayMarcherRenderState {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            paths_rendered_per_pixel: 0,
            resolution: UVec2::ZERO,
            paused: true,
        }
    }
}

impl RayMarcherRenderState {}

impl DualDevice<GPURayMarcherRenderState, Std430GPURayMarcherRenderState>
    for RayMarcherRenderState
{
    fn to_gpu(&self) -> GPURayMarcherRenderState {
        GPURayMarcherRenderState {
            paths_rendered_per_pixel: self.paths_rendered_per_pixel as f32,
            resolution: self.resolution.as_vec2(),
            flags: self.paused as u32,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcherRenderParameters {
    max_distance: f32,
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seeds: Vec3,
    equiangular_samples: u32,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    output_aov: u32,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherRenderParameters {
    pub scene: Scene,
    pub max_distance: f32,
    pub max_ray_steps: u32,
    pub max_bounces: u32,
    pub hit_tolerance: f32,
    pub shadow_bias: f32,
    pub max_brightness: f32,
    pub seeds: Vec3,
    pub dynamic_level_of_detail: bool,
    pub equiangular_samples: u32,
    pub max_light_sampling_bounces: u32,
    pub sample_atmosphere: bool,
    pub light_sampling_bias: f32,
    pub secondary_sampling: bool,
    pub output_aov: AOVs,
}

impl Default for RayMarcherRenderParameters {
    fn default() -> Self {
        RayMarcher {
            scene: Scene::default(),
            max_distance: 100.,
            max_ray_steps: 1000,
            max_bounces: 1,
            hit_tolerance: 0.0001,
            shadow_bias: 1.,
            max_brightness: 999999999.9,
            seeds: Vec3::new(1111., 2222., 3333.),
            dynamic_level_of_detail: true,
            equiangular_samples: 0,
            max_light_sampling_bounces: 1,
            sample_atmosphere: false,
            light_sampling_bias: 0.,
            secondary_sampling: false,
            output_aov: AOVs::default(),
        }
    }
}

impl RayMarcherRenderParameters {
    pub fn reset_render_parameters(&mut self) {
        let default_ray_marcher = Self::default();

        self.max_distance = default_ray_marcher.max_distance;
        self.max_ray_steps = default_ray_marcher.max_ray_steps;
        self.max_bounces = default_ray_marcher.max_bounces;
        self.hit_tolerance = default_ray_marcher.hit_tolerance;
        self.shadow_bias = default_ray_marcher.shadow_bias;
        self.max_brightness = default_ray_marcher.max_brightness;
        self.seeds = default_ray_marcher.seeds;
        self.dynamic_level_of_detail = default_ray_marcher.dynamic_level_of_detail;
        self.equiangular_samples = default_ray_marcher.equiangular_samples;
        self.max_light_sampling_bounces = default_ray_marcher.max_light_sampling_bounces;
        self.sample_atmosphere = default_ray_marcher.sample_atmosphere;
        self.light_sampling_bias = default_ray_marcher.light_sampling_bias;
        self.secondary_sampling = default_ray_marcher.secondary_sampling;
    }
}

impl DualDevice<GPURayMarcherRenderParameters, Std430GPURayMarcherRenderParameters>
    for RayMarcherRenderParameters
{
    fn to_gpu(&self) -> GPURayMarcherRenderParameters {
        GPURayMarcherRenderParameters {
            max_distance: self.max_distance.max(1e-8),
            max_ray_steps: self.max_ray_steps.max(1),
            max_bounces: self.max_bounces.max(1),
            hit_tolerance: self.hit_tolerance.max(0.),
            shadow_bias: self.shadow_bias,
            max_brightness: self.max_brightness,
            seeds: self.seeds,
            equiangular_samples: self.equiangular_samples,
            max_light_sampling_bounces: self.max_light_sampling_bounces,
            light_sampling_bias: self.light_sampling_bias * self.light_sampling_bias,
            output_aov: self.output_aov as u32,
            flags: self.dynamic_level_of_detail as u32
                | (self.sample_atmosphere as u32) << 1
                | (self.secondary_sampling as u32) << 2,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherPass {
    pub render_parameters: RayMarcherRenderParameters,
    pub compilation_options: RayMarcherCompilationOptions,
    pub render_state: RayMarcherRenderState,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
    preprocessor_directives: HashSet<RayMarcherPreprocessorDirectives>,
}

impl Default for RayMarcherPass {
    fn default() -> Self {
        Self {
            render_parameters: RayMarcherRenderParameters::default(),
            compilation_options: RayMarcherCompilationOptions::default(),
            render_state: RayMarcherRenderState::default(),
            recompile_hash: Key::<OrderedFloatPolicy>::Unit,
            reconstruct_hash: Key::<OrderedFloatPolicy>::Unit,
            preprocessor_directives: HashSet::<RayMarcherPreprocessorDirectives>::new(),
        }
    }
}

impl ShaderSource<RayMarcherPreprocessorDirectives> for RayMarcherPass {
    fn dynamic_directives(&self) -> HashSet<RayMarcherPreprocessorDirectives> {
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

        if !self
            .compilation_options()
            .enable_dynamic_recompilation_for_ray_marcher
        {
            preprocessor_directives.extend(all_directives_for_ray_marcher());
        } else {
            preprocessor_directives.extend(directives_for_ray_marcher(self));
        }

        if !self
            .compilation_options()
            .enable_dynamic_recompilation_for_primitives
        {
            preprocessor_directives.extend(all_directives_for_primitive());
        }

        if !self
            .compilation_options()
            .enable_dynamic_recompilation_for_materials
        {
            preprocessor_directives.extend(all_directives_for_material());
        } else {
            preprocessor_directives.extend(directives_for_material(&self.scene.atmosphere));
        }

        if !self
            .compilation_options()
            .enable_dynamic_recompilation_for_lights
        {
            preprocessor_directives.extend(all_directives_for_light());
        } else {
            for light in &self.scene.lights {
                preprocessor_directives.extend(directives_for_light(&light));
            }
        }

        if self
            .compilation_options()
            .enable_dynamic_recompilation_for_primitives
            || self.enable_dynamic_recompilation_for_materials
        {
            for primitive in &self.scene.primitives {
                if self
                    .compilation_options()
                    .enable_dynamic_recompilation_for_materials
                {
                    preprocessor_directives.extend(directives_for_material(&primitive.material));
                }
                if self
                    .compilation_options()
                    .enable_dynamic_recompilation_for_primitives
                {
                    preprocessor_directives.extend(directives_for_primitive(&primitive));
                }
            }
        }

        preprocessor_directives
    }

    fn vertex_shader_raw(&self) -> String {
        RAY_MARCHER_VERTEX_SHADER
    }

    fn fragment_shader_raw(&self) -> String {
        RAY_MARCHER_FRAGMENT_SHADER
    }

    fn current_directives(&self) -> &HashSet<Directives> {}

    fn current_directives_mut(&mut self) -> &mut HashSet<Directives>;

    fn dynamic_recompilation_enabled(&self) -> bool {
        let options = self.compilation_options();
        options.enable_dynamic_recompilation_for_primitives
            || options.enable_dynamic_recompilation_for_materials
            || options.enable_dynamic_recompilation_for_ray_marcher
            || options.enable_dynamic_recompilation_for_lights
    }
}

impl
    RenderPass<
        RayMarcherCompilationOptions,
        RayMarcherPipelineOptions,
        TextureVertex,
        GPUTextureVertex,
        Std430GPUTextureVertex,
    > for RayMarcherPass
{
    fn compilation_options(&self) -> &RayMarcherCompilationOptions {
        &self.compilation_options
    }

    fn pipeline_options(&self) -> &RayMarcherPipelineOptions {
        RayMarcherPipelineOptions {
            num_primitives: self.render_parameters.scene.primitives.len(),
            num_lights: self.render_parameters.scene.lights.len(),
        }
    }

    fn label(&self) -> String {
        "ray marcher"
    }

    fn recompile_hash(&self) -> &Key<OrderedFloatPolicy> {
        &self.recompile_hash
    }

    fn recompile_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy> {
        &mut self.recompile_hash
    }

    fn reconstruct_hash(&self) -> &Key<OrderedFloatPolicy> {
        &self.reconstruct_hash
    }

    fn reconstruct_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy> {
        &mut self.reconstruct_hash
    }

    fn create_uniform_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render parameter buffer"),
                    contents: bytemuck::cast_slice(&[self.render_parameters.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher scene parameter buffer"),
                    contents: bytemuck::cast_slice(&[self.render_parameters.scene.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render progress buffer"),
                    contents: bytemuck::cast_slice(&[self.render_state.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher camera buffer"),
                    contents: bytemuck::cast_slice(&[self
                        .render_parameters
                        .scene
                        .render_camera
                        .as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            },
        ]
    }

    fn create_storage_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        let primitives: Vec<Std430GPUPrimitive> =
            self.render_parameters.scene.create_gpu_primitives();
        let lights: Vec<Std430GPULight> = self.render_parameters.scene.create_gpu_lights();
        let emissive_primitive_indices: Vec<u32> =
            self.render_parameters.scene.emissive_primitive_indices();
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher primitives buffer"),
                    contents: &[bytemuck::cast_slice(primitives.as_slice())].concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher lights buffer"),
                    contents: &[bytemuck::cast_slice(lights.as_slice())].concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render globals buffer"),
                    contents: bytemuck::cast_slice(&[self.render_parameters.scene.atmosphere()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher emissive primitive ids"),
                    contents: &[bytemuck::cast_slice(emissive_primitive_indices.as_slice())]
                        .concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_storage_texture_views(&self, device: &wgpu::Device) -> Vec<StorageTextureView> {
        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: MAX_TEXTURE_DIMENSION,
                height: MAX_TEXTURE_DIMENSION,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("ray marcher progressive rendering texture"),
            view_formats: &[],
        };

        vec![StorageTextureView {
            texture_view: device
                .create_texture(&texture_descriptor)
                .create_view(&Default::default()),
            visibility: wgpu::ShaderStages::FRAGMENT,
            access: wgpu::StorageTextureAccess::ReadWrite,
            format: texture_descriptor.format,
            view_dimension: wgpu::TextureViewDimension::D2,
        }]
    }

    fn reset(&mut self) {
        self.render_state.paths_rendered_per_pixel = 0;
    }
}

impl TextureProcessingPass for RayMarcherPass {}
