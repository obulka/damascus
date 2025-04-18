// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{borrow::Cow, fmt::Debug, time::SystemTime};

use crevice::std430::AsStd430;
use serde_hashkey::{to_key_with_ordered_float, Error, Key, OrderedFloatPolicy, Result};
use wgpu::{self, util::DeviceExt};

use crate::{geometry, shaders};

// mod grade;
pub mod ray_marcher;
pub mod resources;
pub mod texture_viewer;

use ray_marcher::RayMarcher;
use resources::{
    BindGroups, BindingResource, Buffer, BufferBindGroup, BufferData, BufferDescriptor,
    RenderResource, StorageTextureView, StorageTextureViewBindGroup, TextureView,
    TextureViewBindGroup, VertexBuffer,
};
use texture_viewer::TextureViewer;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FrameCounter {
    pub first_frame: u32,
    pub frame: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub frames_to_update_fps: u32,
    pub paused: bool,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            first_frame: 0,
            frame: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            frames_to_update_fps: 10,
            paused: false,
        }
    }
}

impl FrameCounter {
    pub fn first_frame(mut self, first_frame: u32) -> Self {
        self.first_frame = first_frame;
        self
    }

    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }

    pub fn tick(&mut self) {
        if self.paused {
            self.update_frame_time();
            return;
        }

        if self.frames_since_first() % self.frames_to_update_fps == 0 {
            match SystemTime::now().duration_since(self.previous_frame_time) {
                Ok(frame_time) => {
                    self.fps = self.frames_to_update_fps as f32 / frame_time.as_secs_f32();
                }
                Err(_) => self.fps = 0.,
            }

            self.update_frame_time();
        }

        self.frame += 1;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn play(&mut self) {
        self.paused = false;
    }

    pub fn frames_since_first(&self) -> u32 {
        self.frame - self.first_frame
    }

    pub fn update_frame_time(&mut self) {
        self.previous_frame_time = SystemTime::now();
    }

    pub fn reset(&mut self) {
        self.update_frame_time();
        self.frame = self.first_frame;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RenderPassHashes {
    reset: Key<OrderedFloatPolicy>,
    recompile: Key<OrderedFloatPolicy>,
    reconstruct: Key<OrderedFloatPolicy>,
}

impl Default for RenderPassHashes {
    fn default() -> Self {
        Self {
            reset: Key::<OrderedFloatPolicy>::Unit,
            recompile: Key::<OrderedFloatPolicy>::Unit,
            reconstruct: Key::<OrderedFloatPolicy>::Unit,
        }
    }
}

pub trait RenderPass<
    Vertex: geometry::Vertex<GPUVertex, Std430GPUVertex>,
    GPUVertex: Copy + Clone + AsStd430<Output = Std430GPUVertex>,
    Std430GPUVertex: bytemuck::Pod,
    Directives: shaders::PreprocessorDirectives,
>:
    Debug
    + Default
    + Clone
    + serde::Serialize
    + for<'a> serde::Deserialize<'a>
    + shaders::ShaderSource<Directives>
{
    fn hashes(&self) -> &RenderPassHashes;

    fn hashes_mut(&mut self) -> &mut RenderPassHashes;

    fn frame_counter(&self) -> &FrameCounter;

    fn frame_counter_mut(&mut self) -> &mut FrameCounter;

    fn vertices(&self) -> Vec<Std430GPUVertex>;

    fn new() -> Self {
        let mut render_pass = Self::default();
        render_pass.update_hashes();
        render_pass
    }

    fn label(&self) -> String {
        String::new()
    }

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().reset)
    }

    fn create_recompilation_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().recompile)
    }

    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().reconstruct)
    }

    fn update_reset_hash(&mut self) -> bool {
        if let Ok(reset_hash) = self.create_reset_hash() {
            if reset_hash != self.hashes().reset {
                self.hashes_mut().reset = reset_hash;
                return true;
            }
        }
        false
    }

    fn update_recompilation_hash(&mut self) -> bool {
        let mut hash_changed = self.dynamic_recompilation_enabled() && self.update_directives();
        if let Ok(recompilation_hash) = self.create_recompilation_hash() {
            if recompilation_hash != self.hashes().recompile {
                self.hashes_mut().recompile = recompilation_hash;
                hash_changed = true;
            }
        }

        hash_changed
    }

    fn update_reconstruction_hash(&mut self) -> bool {
        if let Ok(reconstruction_hash) = self.create_reconstruction_hash() {
            if reconstruction_hash != self.hashes().reconstruct {
                self.hashes_mut().reconstruct = reconstruction_hash;
                return true;
            }
        }
        false
    }

    fn update_hashes(&mut self) -> bool {
        let mut hash_changed = false;
        hash_changed |= self.update_reset_hash();
        hash_changed |= self.update_recompilation_hash();
        hash_changed |= self.update_reconstruction_hash();
        hash_changed
    }

    fn reset(&mut self) {
        self.frame_counter_mut().reset();
    }

    fn reset_if_hash_changed(&mut self) -> bool {
        if self.update_reset_hash() {
            self.reset();
            return true;
        }
        false
    }

    fn reconstruct_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
    ) -> Option<RenderResource> {
        if self.update_reconstruction_hash() {
            return Some(self.render_resource(device, target_state));
        }
        None
    }

    fn recompile_if_preprocessor_directives_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        if self.dynamic_recompilation_enabled() && self.update_directives() {
            self.recompile_shader(device, target_state, render_resource);
            return true;
        }
        false
    }

    fn recompile_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        if self.update_recompilation_hash() {
            self.recompile_shader(device, target_state, render_resource);
            return true;
        }
        false
    }

    fn update_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        if let Some(new_resource) = self.reconstruct_if_hash_changed(device, target_state.clone()) {
            *render_resource = new_resource;
            self.update_recompilation_hash();
            self.update_reset_hash();
            return true;
        }

        if self.recompile_if_hash_changed(device, target_state, render_resource) {
            self.update_reset_hash();
            return true;
        }

        self.reset_if_hash_changed()
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![]
    }

    fn storage_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![]
    }

    fn buffer_data(&self) -> BufferData {
        BufferData {
            uniform: self.uniform_buffer_data(),
            storage: self.storage_buffer_data(),
        }
    }

    fn render_pipeline(
        &self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        bind_groups: &BindGroups,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout: wgpu::PipelineLayout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&(self.label() + " pipeline layout")),
                bind_group_layouts: &bind_groups.bind_group_layouts(),
                push_constant_ranges: &[],
            });

        let vertex_shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&(self.label() + " vertex shader")),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&self.vertex_shader())),
            });

        let fragment_shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&(self.label() + " fragment shader")),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&self.fragment_shader())),
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&(self.label() + " render pipeline")),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: Some("vs_main"),
                buffers: &[self.create_vertex_buffer_layout()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(target_state)],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    fn recompile_shader(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) {
        self.reset();
        let render_pipeline: wgpu::RenderPipeline =
            self.render_pipeline(device, target_state, &render_resource.bind_groups);
        render_resource.render_pipeline = render_pipeline;
    }

    fn render_resource(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
    ) -> RenderResource {
        self.reset();

        let vertex_buffer: VertexBuffer = self.create_vertex_buffer(device);
        let uniform_buffers: Vec<Buffer> = self.create_uniform_buffers(device);
        let storage_buffers: Vec<Buffer> = self.create_storage_buffers(device);
        let texture_views: Vec<TextureView> = self.create_texture_views(device);
        let storage_texture_views: Vec<StorageTextureView> =
            self.create_storage_texture_views(device);

        let mut uniform_bind_group: Option<BufferBindGroup> = None;
        if !uniform_buffers.is_empty() {
            uniform_bind_group = Some(self.create_uniform_bind_group(device, uniform_buffers));
        }

        let mut storage_bind_group: Option<BufferBindGroup> = None;
        if !storage_buffers.is_empty() {
            storage_bind_group = Some(self.create_storage_bind_group(device, storage_buffers));
        }

        let mut texture_bind_group: Option<TextureViewBindGroup> = None;
        if !texture_views.is_empty() {
            texture_bind_group = Some(self.create_texture_view_bind_group(device, texture_views));
        }

        let mut storage_texture_bind_group: Option<StorageTextureViewBindGroup> = None;
        if !storage_texture_views.is_empty() {
            storage_texture_bind_group =
                Some(self.create_storage_texture_view_bind_group(device, storage_texture_views));
        }

        let bind_groups = BindGroups {
            uniform_bind_group,
            storage_bind_group,
            texture_bind_group,
            storage_texture_bind_group,
        };

        let render_pipeline: wgpu::RenderPipeline =
            self.render_pipeline(device, target_state, &bind_groups);

        RenderResource {
            render_pipeline,
            vertex_buffer,
            bind_groups,
        }
    }

    fn create_vertex_buffer(&self, device: &wgpu::Device) -> VertexBuffer {
        let vertices: Vec<Std430GPUVertex> = self.vertices();
        let vertex_count: u32 = vertices.len() as u32;
        VertexBuffer {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(self.label() + " vertex buffer")),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: vertex_count,
        }
    }

    fn create_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Std430GPUVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Vertex::attr_array(),
        }
    }

    fn create_uniform_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        self.uniform_buffer_data()
            .into_iter()
            .map(|buffer_descriptor| Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&(self.label() + " uniform buffer")),
                    contents: buffer_descriptor.data.as_slice(),
                    usage: buffer_descriptor.usage,
                }),
                visibility: buffer_descriptor.visibility,
            })
            .collect()
    }

    fn create_storage_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        self.storage_buffer_data()
            .into_iter()
            .map(|buffer_descriptor| Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&(self.label() + " storage buffer")),
                    contents: buffer_descriptor.data.as_slice(),
                    usage: buffer_descriptor.usage,
                }),
                visibility: buffer_descriptor.visibility,
            })
            .collect()
    }

    fn create_texture_views(&self, _device: &wgpu::Device) -> Vec<TextureView> {
        vec![]
    }

    fn create_storage_texture_views(&self, _device: &wgpu::Device) -> Vec<StorageTextureView> {
        vec![]
    }

    fn uniform_bind_group_layout_entry(
        binding: u32,
        buffer: &Buffer,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: buffer.visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn storage_bind_group_layout_entry(
        binding: u32,
        buffer: &Buffer,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: buffer.visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn texture_bind_group_layout_entry(
        binding: u32,
        texture_view: &TextureView,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: texture_view.visibility,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                view_dimension: texture_view.view_dimension,
                multisampled: false,
            },
            count: None,
        }
    }

    fn storage_texture_bind_group_layout_entry(
        binding: u32,
        storage_texture_view: &StorageTextureView,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: storage_texture_view.visibility,
            ty: wgpu::BindingType::StorageTexture {
                access: storage_texture_view.access,
                format: storage_texture_view.format,
                view_dimension: storage_texture_view.view_dimension,
            },
            count: None,
        }
    }

    fn create_uniform_bind_group(
        &self,
        device: &wgpu::Device,
        buffers: Vec<Buffer>,
    ) -> BufferBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::uniform_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let bind_group_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&(self.label() + " uniform bind group layout")),
                entries: &bind_group_layout_entries,
            });

        let bind_group_entries: Vec<wgpu::BindGroupEntry<'_>> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_resource(),
            })
            .collect();

        let bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&(self.label() + " uniform bind group")),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        BufferBindGroup {
            bind_group,
            bind_group_layout,
            buffers,
        }
    }

    fn create_storage_bind_group(
        &self,
        device: &wgpu::Device,
        buffers: Vec<Buffer>,
    ) -> BufferBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::storage_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&(self.label() + " storage bind group layout")),
            entries: &bind_group_layout_entries,
        });

        let bind_group_entries: Vec<wgpu::BindGroupEntry<'_>> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_resource(),
            })
            .collect();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&(self.label() + " storage bind group")),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        BufferBindGroup {
            bind_group,
            bind_group_layout,
            buffers,
        }
    }

    fn create_texture_view_bind_group(
        &self,
        device: &wgpu::Device,
        texture_views: Vec<TextureView>,
    ) -> TextureViewBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = texture_views
            .iter()
            .enumerate()
            .map(|(binding, texture_view)| {
                Self::texture_bind_group_layout_entry(binding as u32, &texture_view)
            })
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&(self.label() + " texture bind group layout")),
            entries: &bind_group_layout_entries,
        });

        let bind_group_entries: Vec<wgpu::BindGroupEntry<'_>> = texture_views
            .iter()
            .enumerate()
            .map(|(binding, texture_view)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: texture_view.as_resource(),
            })
            .collect();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&(self.label() + " texture bind group")),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        TextureViewBindGroup {
            bind_group,
            bind_group_layout,
            texture_views,
        }
    }

    fn create_storage_texture_view_bind_group(
        &self,
        device: &wgpu::Device,
        storage_texture_views: Vec<StorageTextureView>,
    ) -> StorageTextureViewBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = storage_texture_views
            .iter()
            .enumerate()
            .map(|(binding, storage_texture_view)| {
                Self::storage_texture_bind_group_layout_entry(binding as u32, &storage_texture_view)
            })
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&(self.label() + " storage texture bind group layout")),
            entries: &bind_group_layout_entries,
        });

        let bind_group_entries: Vec<wgpu::BindGroupEntry<'_>> = storage_texture_views
            .iter()
            .enumerate()
            .map(|(binding, storage_texture_view)| wgpu::BindGroupEntry {
                binding: binding as u32,
                resource: storage_texture_view.as_resource(),
            })
            .collect();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&(self.label() + " storage texture bind group")),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        StorageTextureViewBindGroup {
            bind_group,
            bind_group_layout,
            storage_texture_views,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RenderPasses {
    RayMarcher { pass: RayMarcher },
    TextureViewer { pass: TextureViewer },
}

impl RenderPasses {
    pub fn new() -> Self {
        Self::TextureViewer {
            pass: TextureViewer::new(),
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::RayMarcher { pass } => pass.reset(),
            Self::TextureViewer { pass } => pass.reset(),
        }
    }

    pub fn frame_counter(&self) -> &FrameCounter {
        match self {
            Self::RayMarcher { pass } => pass.frame_counter(),
            Self::TextureViewer { pass } => pass.frame_counter(),
        }
    }

    pub fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        match self {
            Self::RayMarcher { pass } => pass.frame_counter_mut(),
            Self::TextureViewer { pass } => pass.frame_counter_mut(),
        }
    }

    pub fn render_resource(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
    ) -> RenderResource {
        match self {
            Self::RayMarcher { pass } => pass.render_resource(device, target_state),
            Self::TextureViewer { pass } => pass.render_resource(device, target_state),
        }
    }

    pub fn buffer_data(&self) -> BufferData {
        match self {
            Self::RayMarcher { pass } => pass.buffer_data(),
            Self::TextureViewer { pass } => pass.buffer_data(),
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        match self {
            Self::RayMarcher { pass } => pass.recompile_if_preprocessor_directives_changed(
                device,
                target_state,
                render_resource,
            ),
            Self::TextureViewer { pass } => pass.recompile_if_preprocessor_directives_changed(
                device,
                target_state,
                render_resource,
            ),
        }
    }

    pub fn recompile_shader(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) {
        match self {
            Self::RayMarcher { pass } => {
                pass.recompile_shader(device, target_state, render_resource)
            }
            Self::TextureViewer { pass } => {
                pass.recompile_shader(device, target_state, render_resource)
            }
        }
    }

    pub fn update_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        match self {
            Self::RayMarcher { pass } => {
                pass.update_if_hash_changed(device, target_state, render_resource)
            }
            Self::TextureViewer { pass } => {
                pass.update_if_hash_changed(device, target_state, render_resource)
            }
        }
    }
}
