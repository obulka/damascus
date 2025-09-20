// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;
use serde_hashkey::{to_key_with_ordered_float, Error, Key, OrderedFloatPolicy, Result};
use wgpu;

use super::{
    resources::{BufferDescriptor, StorageTextureView},
    FrameCounter, RenderPass, RenderPassHashes,
};

use crate::{
    scene::Scene,
    shaders::{
        ray_marcher::{
            all_directives_for_light, all_directives_for_material, all_directives_for_primitive,
            all_directives_for_ray_marcher, directives_for_light, directives_for_material,
            directives_for_primitive, directives_for_ray_marcher, RayMarcherPreprocessorDirectives,
            RAY_MARCHER_FRAGMENT_SHADER, RAY_MARCHER_VERTEX_SHADER,
        },
        ShaderSource,
    },
    textures::AOVs,
    DualDevice,
};

pub const MAX_TEXTURE_DIMENSION: u32 = 8192; // TODO get rid of this

// A change in the data within this struct will trigger the pass to
// recompile
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherCompilationData {
    pub enable_dynamic_recompilation_for_materials: bool,
    pub enable_dynamic_recompilation_for_primitives: bool,
    pub enable_dynamic_recompilation_for_ray_marcher: bool,
    pub enable_dynamic_recompilation_for_lights: bool,
}

impl Default for RayMarcherCompilationData {
    fn default() -> Self {
        Self {
            enable_dynamic_recompilation_for_materials: true,
            enable_dynamic_recompilation_for_primitives: true,
            enable_dynamic_recompilation_for_ray_marcher: true,
            enable_dynamic_recompilation_for_lights: true,
        }
    }
}

impl RayMarcherCompilationData {}

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RayMarcherConstructionData {
    pub num_primitives: usize,
    pub num_lights: usize,
    pub num_emissive_primitives: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcherRenderData {
    max_ray_steps: u32,
    max_bounces: u32,
    hit_tolerance: f32,
    shadow_bias: f32,
    max_brightness: f32,
    seed: u32,
    equiangular_samples: u32,
    max_light_sampling_bounces: u32,
    light_sampling_bias: f32,
    output_aov: u32,
    flags: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcherRenderData {
    pub scene: Scene,
    pub max_ray_steps: u32,
    pub max_bounces: u32,
    pub hit_tolerance: f32,
    pub shadow_bias: f32,
    pub max_brightness: f32,
    pub seed: u32,
    pub dynamic_level_of_detail: bool,
    pub equiangular_samples: u32,
    pub light_sampling: bool,
    pub max_light_sampling_bounces: u32,
    pub sample_atmosphere: bool,
    pub light_sampling_bias: f32,
    pub secondary_sampling: bool,
    pub output_aov: AOVs,
}

impl Default for RayMarcherRenderData {
    fn default() -> Self {
        Self {
            scene: Scene::default(),
            max_ray_steps: 1000,
            max_bounces: 1,
            hit_tolerance: 0.0001,
            shadow_bias: 1.,
            max_brightness: 999999999.9,
            seed: 42,
            dynamic_level_of_detail: true,
            equiangular_samples: 0,
            light_sampling: false,
            max_light_sampling_bounces: 1,
            sample_atmosphere: false,
            light_sampling_bias: 0.,
            secondary_sampling: false,
            output_aov: AOVs::default(),
        }
    }
}

impl RayMarcherRenderData {
    pub fn reset_render_data(&mut self) {
        let default_ray_marcher = Self::default();

        self.max_ray_steps = default_ray_marcher.max_ray_steps;
        self.max_bounces = default_ray_marcher.max_bounces;
        self.hit_tolerance = default_ray_marcher.hit_tolerance;
        self.shadow_bias = default_ray_marcher.shadow_bias;
        self.max_brightness = default_ray_marcher.max_brightness;
        self.seed = default_ray_marcher.seed;
        self.dynamic_level_of_detail = default_ray_marcher.dynamic_level_of_detail;
        self.equiangular_samples = default_ray_marcher.equiangular_samples;
        self.light_sampling = default_ray_marcher.light_sampling;
        self.max_light_sampling_bounces = default_ray_marcher.max_light_sampling_bounces;
        self.sample_atmosphere = default_ray_marcher.sample_atmosphere;
        self.light_sampling_bias = default_ray_marcher.light_sampling_bias;
        self.secondary_sampling = default_ray_marcher.secondary_sampling;
    }
}

impl DualDevice<GPURayMarcherRenderData, Std430GPURayMarcherRenderData> for RayMarcherRenderData {
    fn to_gpu(&self) -> GPURayMarcherRenderData {
        GPURayMarcherRenderData {
            max_ray_steps: self.max_ray_steps.max(1),
            max_bounces: self.max_bounces.max(1),
            hit_tolerance: self.hit_tolerance.max(0.),
            shadow_bias: self.shadow_bias,
            max_brightness: self.max_brightness,
            seed: self.seed,
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

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPURayMarcher {
    paths_rendered_per_pixel: u32,
    flags: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcher {
    pub render_data: RayMarcherRenderData,
    pub compilation_data: RayMarcherCompilationData,
    pub subframe_counter: FrameCounter,
    hashes: RenderPassHashes,
    preprocessor_directives: HashSet<RayMarcherPreprocessorDirectives>,
}

impl Default for RayMarcher {
    fn default() -> Self {
        Self {
            render_data: RayMarcherRenderData::default(),
            compilation_data: RayMarcherCompilationData::default(),
            subframe_counter: FrameCounter::default(),
            hashes: RenderPassHashes::default(),
            preprocessor_directives: HashSet::<RayMarcherPreprocessorDirectives>::new(),
        }
    }
}

impl DualDevice<GPURayMarcher, Std430GPURayMarcher> for RayMarcher {
    fn to_gpu(&self) -> GPURayMarcher {
        GPURayMarcher {
            paths_rendered_per_pixel: self.subframe_counter.frame,
            flags: self.subframe_counter.paused as u32,
        }
    }
}

impl ShaderSource<RayMarcherPreprocessorDirectives> for RayMarcher {
    fn dynamic_directives(&self) -> HashSet<RayMarcherPreprocessorDirectives> {
        let mut preprocessor_directives = HashSet::<RayMarcherPreprocessorDirectives>::new();

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_ray_marcher
        {
            preprocessor_directives.extend(all_directives_for_ray_marcher());
        } else {
            preprocessor_directives.extend(directives_for_ray_marcher(&self.render_data));
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_primitives
        {
            preprocessor_directives.extend(all_directives_for_primitive());
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_materials
        {
            preprocessor_directives.extend(all_directives_for_material());
        } else {
            preprocessor_directives
                .extend(directives_for_material(&self.render_data.scene.atmosphere));
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_lights
        {
            preprocessor_directives.extend(all_directives_for_light());
        } else {
            for light in &self.render_data.scene.lights {
                preprocessor_directives.extend(directives_for_light(&light));
            }
        }

        if self
            .compilation_data
            .enable_dynamic_recompilation_for_primitives
            || self
                .compilation_data
                .enable_dynamic_recompilation_for_materials
        {
            for primitive in &self.render_data.scene.primitives {
                if self
                    .compilation_data
                    .enable_dynamic_recompilation_for_materials
                {
                    preprocessor_directives.extend(directives_for_material(&primitive.material));
                }
                if self
                    .compilation_data
                    .enable_dynamic_recompilation_for_primitives
                {
                    preprocessor_directives.extend(directives_for_primitive(&primitive));
                }
            }
        }

        preprocessor_directives
    }

    fn vertex_shader_raw(&self) -> &str {
        RAY_MARCHER_VERTEX_SHADER
    }

    fn fragment_shader_raw(&self) -> &str {
        RAY_MARCHER_FRAGMENT_SHADER
    }

    fn current_directives(&self) -> &HashSet<RayMarcherPreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_directives_mut(&mut self) -> &mut HashSet<RayMarcherPreprocessorDirectives> {
        &mut self.preprocessor_directives
    }

    fn dynamic_recompilation_enabled(&self) -> bool {
        self.compilation_data
            .enable_dynamic_recompilation_for_primitives
            || self
                .compilation_data
                .enable_dynamic_recompilation_for_materials
            || self
                .compilation_data
                .enable_dynamic_recompilation_for_ray_marcher
            || self
                .compilation_data
                .enable_dynamic_recompilation_for_lights
    }
}

impl RenderPass<RayMarcherPreprocessorDirectives> for RayMarcher {
    fn label(&self) -> String {
        "ray marcher".to_owned()
    }

    fn hashes(&self) -> &RenderPassHashes {
        &self.hashes
    }

    fn hashes_mut(&mut self) -> &mut RenderPassHashes {
        &mut self.hashes
    }

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.render_data)
    }

    fn create_recompilation_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.compilation_data)
    }

    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&RayMarcherConstructionData {
            num_primitives: self.render_data.scene.primitives.len(),
            num_lights: self.render_data.scene.lights.len(),
            num_emissive_primitives: self.render_data.scene.num_emissive_primitives(),
        })
    }

    fn frame_counter(&self) -> &FrameCounter {
        &self.subframe_counter
    }

    fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        &mut self.subframe_counter
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.scene.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.scene.render_camera.as_std430()])
                    .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            },
        ]
    }

    fn storage_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data.scene.create_gpu_primitives().as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(self.render_data.scene.create_gpu_lights().as_slice())
                    .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.scene.atmosphere.as_std430()])
                    .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .scene
                        .emissive_primitive_indices()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
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
}

impl RayMarcher {
    pub fn scene(mut self, scene: Scene) -> Self {
        self.render_data.scene = scene;
        self
    }

    pub fn max_ray_steps(mut self, max_ray_steps: u32) -> Self {
        self.render_data.max_ray_steps = max_ray_steps;
        self
    }

    pub fn max_bounces(mut self, max_bounces: u32) -> Self {
        self.render_data.max_bounces = max_bounces;
        self
    }

    pub fn hit_tolerance(mut self, hit_tolerance: f32) -> Self {
        self.render_data.hit_tolerance = hit_tolerance;
        self
    }

    pub fn shadow_bias(mut self, shadow_bias: f32) -> Self {
        self.render_data.shadow_bias = shadow_bias;
        self
    }

    pub fn max_brightness(mut self, max_brightness: f32) -> Self {
        self.render_data.max_brightness = max_brightness;
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.render_data.seed = seed;
        self
    }

    pub fn dynamic_level_of_detail(mut self, dynamic_level_of_detail: bool) -> Self {
        self.render_data.dynamic_level_of_detail = dynamic_level_of_detail;
        self
    }

    pub fn equiangular_samples(mut self, equiangular_samples: u32) -> Self {
        self.render_data.equiangular_samples = equiangular_samples;
        self
    }

    pub fn max_light_sampling_bounces(mut self, max_light_sampling_bounces: u32) -> Self {
        self.render_data.max_light_sampling_bounces = max_light_sampling_bounces;
        self
    }

    pub fn light_sampling(mut self, light_sampling: bool) -> Self {
        self.render_data.light_sampling = light_sampling;
        self
    }

    pub fn sample_atmosphere(mut self, sample_atmosphere: bool) -> Self {
        self.render_data.sample_atmosphere = sample_atmosphere;
        self
    }

    pub fn light_sampling_bias(mut self, light_sampling_bias: f32) -> Self {
        self.render_data.light_sampling_bias = light_sampling_bias;
        self
    }

    pub fn secondary_sampling(mut self, secondary_sampling: bool) -> Self {
        self.render_data.secondary_sampling = secondary_sampling;
        self
    }

    pub fn output_aov(mut self, output_aov: AOVs) -> Self {
        self.render_data.output_aov = output_aov;
        self
    }
}
