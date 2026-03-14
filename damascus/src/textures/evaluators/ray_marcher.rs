// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

use crevice::std430::AsStd430;
use glam::UVec2;
use macro_rules_attribute::derive;
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};
use wgpu;

use crate::{
    DualDevice, PreprocessorDirectivesBaseTraits,
    gpu::{
        ShaderSource,
        resources::{BufferData, BufferDescriptor, RenderResource, TextureView},
        scene::{GPUScene, ScenePreprocessorDirectives},
    },
    textures::{
        AOVs,
        evaluators::{GPUTextureEvaluator, TextureEvaluator, TextureEvaluatorHashes},
    },
    time::FrameCounter,
};

#[derive(Copy, Default, PreprocessorDirectivesBaseTraits!)]
pub enum RayMarcherPreprocessorDirectives {
    #[default]
    EnableAOVs,
    EnableLightSampling,
    SceneDirective(ScenePreprocessorDirectives),
}

impl Display for RayMarcherPreprocessorDirectives {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::SceneDirective(scene_directive) => {
                write!(formatter, "{:?}", scene_directive)
            }
            _ => {
                write!(formatter, "{:?}", self)
            }
        }
    }
}

impl From<ScenePreprocessorDirectives> for RayMarcherPreprocessorDirectives {
    fn from(scene_directive: ScenePreprocessorDirectives) -> Self {
        Self::SceneDirective(scene_directive)
    }
}

impl RayMarcherPreprocessorDirectives {
    pub fn all_directives_for_ray_marcher() -> HashSet<Self> {
        HashSet::<RayMarcherPreprocessorDirectives>::from([
            Self::EnableAOVs,
            Self::EnableLightSampling,
        ])
    }

    pub fn directives_for_ray_marcher(ray_marcher: &RayMarcherRenderData) -> HashSet<Self> {
        let mut preprocessor_directives = HashSet::<Self>::new();

        if ray_marcher.output_aov > AOVs::Beauty {
            preprocessor_directives.insert(Self::EnableAOVs);
        }
        if ray_marcher.light_sampling {
            preprocessor_directives.insert(Self::EnableLightSampling);
        }

        preprocessor_directives
    }
}

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
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RayMarcherConstructionData {
    pub num_primitives: usize,
    pub num_lights: usize,
    pub num_emissive_primitives: usize,
    #[serde(skip_deserializing)]
    input_texture_views: Vec<TextureView>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
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
    pub gpu_scene: GPUScene,
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
            gpu_scene: GPUScene::default(),
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
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPURayMarcher {
    paths_rendered_per_pixel: u32,
    flags: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RayMarcher {
    subframe_counter: FrameCounter,
    render_data: RayMarcherRenderData,
    compilation_data: RayMarcherCompilationData,
    construction_data: RayMarcherConstructionData,
    hashes: TextureEvaluatorHashes,
    preprocessor_directives: HashSet<RayMarcherPreprocessorDirectives>,
    #[serde(skip)]
    output_texture_view: Option<TextureView>,
    #[serde(skip)]
    render_resource: Option<RenderResource>,
}

impl RayMarcher {
    pub fn with_gpu_scene(&self) -> &GPUScene {
        &self.render_data.gpu_scene
    }

    pub fn gpu_scene(mut self, gpu_scene: GPUScene) -> Self {
        self.render_data.gpu_scene = gpu_scene;
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

impl Default for RayMarcher {
    fn default() -> Self {
        Self {
            subframe_counter: FrameCounter::default(),
            render_data: RayMarcherRenderData::default(),
            compilation_data: RayMarcherCompilationData::default(),
            construction_data: RayMarcherConstructionData::default(),
            hashes: TextureEvaluatorHashes::default(),
            preprocessor_directives: HashSet::<RayMarcherPreprocessorDirectives>::new(),
            output_texture_view: None,
            render_resource: None,
        }
    }
}

impl TextureEvaluator for RayMarcher {
    fn label(&self) -> String {
        "ray marcher".to_owned()
    }

    fn reset(&mut self) {
        self.frame_counter_mut().reset();
        self.output_texture_view = None;
    }

    fn hashes(&self) -> &TextureEvaluatorHashes {
        &self.hashes
    }

    fn hashes_mut(&mut self) -> &mut TextureEvaluatorHashes {
        &mut self.hashes
    }

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.render_data)
    }

    fn frame_counter(&self) -> &FrameCounter {
        &self.subframe_counter
    }

    fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        &mut self.subframe_counter
    }

    fn input_texture_views(&self) -> Vec<TextureView> {
        self.construction_data.input_texture_views.clone()
    }

    fn with_input_texture_views(mut self, input_texture_views: Vec<TextureView>) -> Self {
        self.construction_data.input_texture_views = input_texture_views;
        self
    }

    fn output_texture_dimensions(&self) -> Option<wgpu::Extent3d> {
        let sensor_resolution: UVec2 = self.render_data.gpu_scene.cameras
            [self.render_data.gpu_scene.render_camera]
            .sensor_resolution;
        Some(wgpu::Extent3d {
            width: sensor_resolution.x,
            height: sensor_resolution.y,
            depth_or_array_layers: 1, // TODO could output AOVs to other layers
        })
    }

    fn set_output_texture_view(&mut self, output_texture_view: TextureView) {
        self.output_texture_view = Some(output_texture_view);
    }

    fn output_texture_view(&self) -> Option<&TextureView> {
        self.output_texture_view.as_ref()
    }

    fn output_texture_view_mut(&mut self) -> Option<&mut TextureView> {
        self.output_texture_view.as_mut()
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
            preprocessor_directives
                .extend(RayMarcherPreprocessorDirectives::all_directives_for_ray_marcher());
        } else {
            preprocessor_directives.extend(
                RayMarcherPreprocessorDirectives::directives_for_ray_marcher(&self.render_data),
            );
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_primitives
        {
            preprocessor_directives.extend(
                ScenePreprocessorDirectives::all_directives_for_primitive()
                    .iter()
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_materials
        {
            preprocessor_directives.extend(
                ScenePreprocessorDirectives::all_directives_for_material()
                    .iter()
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        } else {
            preprocessor_directives.extend(
                self.render_data
                    .gpu_scene
                    .preprocessor_directives
                    .intersection(&ScenePreprocessorDirectives::all_directives_for_material())
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        }

        if !self
            .compilation_data
            .enable_dynamic_recompilation_for_lights
        {
            preprocessor_directives.extend(
                ScenePreprocessorDirectives::all_directives_for_light()
                    .iter()
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        } else {
            preprocessor_directives.extend(
                self.render_data
                    .gpu_scene
                    .preprocessor_directives
                    .intersection(&ScenePreprocessorDirectives::all_directives_for_light())
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        }

        if self
            .compilation_data
            .enable_dynamic_recompilation_for_primitives
        {
            preprocessor_directives.extend(
                self.render_data
                    .gpu_scene
                    .preprocessor_directives
                    .intersection(&ScenePreprocessorDirectives::all_directives_for_primitive())
                    .map(|scene_directive| {
                        RayMarcherPreprocessorDirectives::from(*scene_directive)
                    }),
            );
        }

        preprocessor_directives
    }

    fn vertex_shader_raw(&self) -> &str {
        include_str!("../../gpu/wgsl/textures/evaluators/ray_marcher/vertex_shader.wgsl")
    }

    fn fragment_shader_raw(&self) -> &str {
        include_str!("../../gpu/wgsl/textures/evaluators/ray_marcher/fragment_shader.wgsl")
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

impl GPUTextureEvaluator<RayMarcherPreprocessorDirectives> for RayMarcher {
    fn create_recompilation_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.compilation_data)
    }

    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        self.construction_data.num_primitives = self.render_data.gpu_scene.primitives.len();
        self.construction_data.num_lights = self.render_data.gpu_scene.lights.len();
        self.construction_data.num_emissive_primitives =
            self.render_data.gpu_scene.emissive_primitive_indices.len();
        to_key_with_ordered_float(&self.construction_data)
    }

    fn render_resource(&self) -> &Option<RenderResource> {
        &self.render_resource
    }

    fn render_resource_mut(&mut self) -> &mut Option<RenderResource> {
        &mut self.render_resource
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.gpu_scene.array_lengths.as_std430()])
                    .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.gpu_scene.cameras
                    [self.render_data.gpu_scene.render_camera]
                    .as_std430()])
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.gpu_scene.materials
                    [self.render_data.gpu_scene.atmosphere]
                    .as_std430()])
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn storage_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .primitives
                        .iter()
                        .map(|primitive| primitive.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .lights
                        .iter()
                        .map(|light| light.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .emissive_primitive_indices
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .materials
                        .iter()
                        .map(|material| material.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .checkerboards
                        .iter()
                        .map(|checkerboard| checkerboard.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .noises
                        .iter()
                        .map(|noise| noise.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(
                    self.render_data
                        .gpu_scene
                        .grades
                        .iter()
                        .map(|grade| grade.as_std430())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
        if let Some(output_texture_view) = self.output_texture_view() {
            return vec![output_texture_view.clone()];
        } else if let Some(output_texture_view) = self.create_output_texture_view(device) {
            return vec![output_texture_view];
        }
        vec![]
    }

    fn evaluate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if let Some(texture_view) = self.create_output_texture_view(device)
            && texture_view
                .texture_view
                .texture()
                .usage()
                .contains(wgpu::TextureUsages::RENDER_ATTACHMENT)
        {
            let reset: bool = self.update_if_hash_changed(device);

            let buffer_data: BufferData = self.buffer_data();

            self.frame_counter_mut().tick();

            if !reset {
                // Write that data to the bind groups
                let texture_views = self.create_texture_views(device);

                let bind_group = self.create_texture_view_bind_group(device, texture_views);

                if let Some(render_resource) = self.render_resource_mut() {
                    if let Some(texture_bind_group) =
                        &mut render_resource.bind_groups.texture_bind_group
                    {
                        *texture_bind_group = bind_group;
                    }
                }
            }

            if let Some(render_resource) = self.render_resource_mut() {
                render_resource.write_bind_groups(queue, &buffer_data);
            }

            // Set up a render pass and paint to it

            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view.texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 0.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            };

            if let Some(render_resource) = self.render_resource_mut() {
                render_resource.paint(&mut encoder.begin_render_pass(&render_pass_desc));
            }

            self.set_output_texture_view(texture_view);
        }
    }
}
