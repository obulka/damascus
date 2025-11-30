// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{borrow::Cow, fmt::Debug, ops::Range};

use crevice::std430;
use glam::UVec2;
use macro_rules_attribute::derive;
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};
use slotmap::SlotMap;
use wgpu::{self, util::DeviceExt};

use crate::{
    DualDevice, EnumTraits,
    geometry::vertex::Vertex,
    gpu::{
        PreprocessorDirectives, ShaderSource,
        resources::{
            BindGroups, BindingResource, Buffer, BufferBindGroup, BufferData, BufferDescriptor,
            RenderResource, StorageTextureView, StorageTextureViewBindGroup, TextureView,
            TextureViewBindGroup,
        },
        scene::GPUScene,
    },
    time::FrameCounter,
};

pub mod checkerboard;
pub mod grade;
pub mod noise;
pub mod ray_marcher;
pub mod read;
pub mod view;

use checkerboard::Checkerboard;
use grade::Grade;
use noise::Noise;
use ray_marcher::RayMarcher;
use read::TextureReader;
use view::TextureViewer;

slotmap::new_key_type! { pub struct TextureEvaluatorId; }

#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureEvaluatorHashes {
    pub reset: Key<OrderedFloatPolicy>,
    pub recompile: Key<OrderedFloatPolicy>,
    pub reconstruct: Key<OrderedFloatPolicy>,
}

impl Default for TextureEvaluatorHashes {
    fn default() -> Self {
        Self {
            reset: Key::<OrderedFloatPolicy>::Unit,
            recompile: Key::<OrderedFloatPolicy>::Unit,
            reconstruct: Key::<OrderedFloatPolicy>::Unit,
        }
    }
}

impl PartialEq for TextureEvaluatorHashes {
    fn eq(&self, other: &Self) -> bool {
        self.reset == other.reset
            && self.recompile == other.recompile
            && self.reconstruct == other.reconstruct
    }
}

pub trait TextureEvaluator:
    Debug + Default + Clone + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    fn hashes(&self) -> &TextureEvaluatorHashes;

    fn hashes_mut(&mut self) -> &mut TextureEvaluatorHashes;

    fn frame_counter(&self) -> &FrameCounter;

    fn frame_counter_mut(&mut self) -> &mut FrameCounter;

    fn label(&self) -> String {
        String::new()
    }

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().reset)
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

    /// TODO do not like the finalize/finalized hack to differentiate
    fn finalize(mut self) -> Self {
        self.update_reset_hash();
        self
    }

    fn input_texture_views(&self) -> Vec<TextureView> {
        vec![]
    }

    fn with_input_texture_views(self, _input_texture_views: Vec<TextureView>) -> Self {
        self
    }

    fn output_mip_level_count(&self) -> u32 {
        1
    }

    fn output_sample_count(&self) -> u32 {
        1
    }

    fn output_texture_format(&self) -> wgpu::TextureFormat {
        // TODO make this a match on the Texture type
        wgpu::TextureFormat::Rgba32Float
    }

    fn output_texture_view_dimension(&self) -> wgpu::TextureViewDimension {
        wgpu::TextureViewDimension::D2
    }

    fn output_texture_dimensions(&self) -> Option<wgpu::Extent3d> {
        None
    }

    fn output_texture_view_descriptor(&self) -> wgpu::TextureViewDescriptor<'_> {
        wgpu::TextureViewDescriptor::default()
    }

    fn output_texture_view_visibility(&self) -> wgpu::ShaderStages {
        wgpu::ShaderStages::VERTEX_FRAGMENT
    }

    fn output_texture_descriptor(&self) -> Option<wgpu::TextureDescriptor<'_>> {
        if let Some(dimensions) = self.output_texture_dimensions() {
            Some(wgpu::TextureDescriptor {
                label: None,
                size: dimensions,
                mip_level_count: self.output_mip_level_count(),
                sample_count: self.output_sample_count(),
                dimension: self
                    .output_texture_view_dimension()
                    .compatible_texture_dimension(),
                format: self.output_texture_format(),
                usage: wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
        } else {
            None
        }
    }

    fn create_output_texture_view(&self, device: &wgpu::Device) -> Option<TextureView> {
        if let Some(texture_descriptor) = self.output_texture_descriptor() {
            Some(TextureView {
                texture_view: device
                    .create_texture(&texture_descriptor)
                    .create_view(&self.output_texture_view_descriptor()),
                data: vec![].into(),
                visibility: self.output_texture_view_visibility(),
                view_dimension: self.output_texture_view_dimension(),
                format: texture_descriptor.format,
            })
        } else {
            None
        }
    }

    fn set_output_texture_view(&mut self, output_texture_view: TextureView) {
        if let Some(output_texture_view_mut) = self.output_texture_view_mut() {
            *output_texture_view_mut = output_texture_view;
        }
    }

    fn initialize_output_texture_view(&mut self, device: &wgpu::Device) {
        if let Some(output_texture_view) = self.create_output_texture_view(device) {
            self.set_output_texture_view(output_texture_view)
        }
    }

    fn output_texture_view(&self) -> Option<&TextureView> {
        None
    }

    fn output_texture_view_mut(&mut self) -> Option<&mut TextureView> {
        None
    }

    fn evaluate_texture(&mut self, device: &wgpu::Device) {
        self.initialize_output_texture_view(device);
    }
}

pub trait GPUTextureEvaluator<Directives: PreprocessorDirectives>:
    TextureEvaluator + ShaderSource<Directives>
{
    fn create_recompilation_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().recompile)
    }

    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.hashes().reconstruct)
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

    fn new() -> Self {
        let mut texture_evaluator = Self::default();
        texture_evaluator.update_hashes();
        texture_evaluator
    }

    fn finalized(mut self) -> Self {
        self.update_hashes();
        self
    }

    fn reconstruct_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
    ) -> Option<RenderResource> {
        if self.update_reconstruction_hash() {
            if self.dynamic_recompilation_enabled() {
                self.update_directives();
            }
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

    fn index_count(&self) -> Range<u32> {
        0..4
    }

    fn index_buffer_data(&self) -> BufferDescriptor {
        BufferDescriptor {
            data: bytemuck::cast_slice(Vertex::quad_corner_indices_2d().as_slice()).to_vec(),
            usage: wgpu::BufferUsages::INDEX,
            visibility: wgpu::ShaderStages::NONE,
        }
    }

    fn base_vertex(&self) -> i32 {
        0
    }

    fn instance_count(&self) -> Range<u32> {
        0..1
    }

    fn vertex_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![BufferDescriptor {
            data: bytemuck::cast_slice(Vertex::quad_corner_vertices_2d().as_slice()).to_vec(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            visibility: wgpu::ShaderStages::VERTEX,
        }]
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![]
    }

    fn storage_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![]
    }

    fn buffer_data(&self) -> BufferData {
        BufferData {
            vertex: self.vertex_buffer_data(),
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
                buffers: &[],
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

        let index_buffer: Buffer = self.create_index_buffer(device);
        let vertex_buffers: Vec<Buffer> = self.create_vertex_buffers(device);
        let uniform_buffers: Vec<Buffer> = self.create_uniform_buffers(device);
        let storage_buffers: Vec<Buffer> = self.create_storage_buffers(device);
        let texture_views: Vec<TextureView> = self.create_texture_views(device);
        let storage_texture_views: Vec<StorageTextureView> =
            self.create_storage_texture_views(device);

        let vertex_bind_group: BufferBindGroup =
            self.create_vertex_bind_group(device, vertex_buffers);

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
            vertex_bind_group,
            uniform_bind_group,
            storage_bind_group,
            texture_bind_group,
            storage_texture_bind_group,
        };

        let render_pipeline: wgpu::RenderPipeline =
            self.render_pipeline(device, target_state, &bind_groups);

        RenderResource {
            render_pipeline: render_pipeline,
            index_buffer: index_buffer,
            bind_groups: bind_groups,
            index_count: self.index_count(),
            base_vertex: self.base_vertex(),
            instance_count: self.instance_count(),
        }
    }

    fn create_index_buffer(&self, device: &wgpu::Device) -> Buffer {
        let buffer_descriptor: BufferDescriptor = self.index_buffer_data();
        Buffer {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(self.label() + " index buffer")),
                contents: buffer_descriptor.data.as_slice(),
                usage: buffer_descriptor.usage,
            }),
            visibility: buffer_descriptor.visibility,
        }
    }

    fn create_vertex_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        self.vertex_buffer_data()
            .into_iter()
            .map(|buffer_descriptor| Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&(self.label() + " vertex buffer")),
                    contents: buffer_descriptor.data.as_slice(),
                    usage: buffer_descriptor.usage,
                }),
                visibility: buffer_descriptor.visibility,
            })
            .collect()
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

    fn vertex_bind_group_layout_entry(binding: u32, buffer: &Buffer) -> wgpu::BindGroupLayoutEntry {
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

    fn create_vertex_bind_group(
        &self,
        device: &wgpu::Device,
        buffers: Vec<Buffer>,
    ) -> BufferBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::vertex_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&(self.label() + " vertex bind group layout")),
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
            label: Some(&(self.label() + " vertex bind group")),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        BufferBindGroup {
            bind_group,
            bind_group_layout,
            buffers,
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
            let mut render_resource = self.render_resource(&device, texture_view.format.into());

            self.update_if_hash_changed(&device, texture_view.format.into(), &mut render_resource);

            let buffer_data: BufferData = self.buffer_data();

            self.frame_counter_mut().tick();

            // Write that data to the bind groups

            render_resource.write_bind_groups(&queue, &buffer_data);

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

            render_resource.paint(&mut encoder.begin_render_pass(&render_pass_desc));

            self.set_output_texture_view(texture_view);
        }
    }
}

#[derive(Default, EnumTraits!)]
pub enum TextureEvaluators {
    #[default]
    White,
    Black,
    Checkerboard(Checkerboard),
    Constant,
    Grade(Grade),
    Noise(Noise),
    RayMarcher(RayMarcher),
    TextureViewer(TextureViewer),
    TextureReader(TextureReader),
}

impl DualDevice<UVec2, std430::UVec2> for TextureEvaluators {
    fn to_gpu(&self) -> UVec2 {
        UVec2::new(
            match self {
                Self::White => 1,
                Self::Black => 2,
                Self::Checkerboard(_) => 3,
                Self::Constant => 4,
                Self::Grade(_) => 5,
                Self::Noise(_) => 6,
                _ => 0,
            },
            0,
        )
    }
}

impl TextureEvaluators {
    pub fn new() -> Self {
        Self::TextureViewer(TextureViewer::new())
    }

    pub fn reset(&mut self) {
        match self {
            Self::Grade(grade) => grade.reset(),
            Self::RayMarcher(ray_marcher) => ray_marcher.reset(),
            Self::TextureViewer(texture_viewer) => texture_viewer.reset(),
            Self::TextureReader(texture_reader) => texture_reader.reset(),
            _ => {}
        }
    }

    pub fn frame_counter(&self) -> Option<&FrameCounter> {
        match self {
            Self::Grade(grade) => Some(grade.frame_counter()),
            Self::RayMarcher(ray_marcher) => Some(ray_marcher.frame_counter()),
            Self::TextureViewer(texture_viewer) => Some(texture_viewer.frame_counter()),
            Self::TextureReader(texture_reader) => Some(texture_reader.frame_counter()),
            _ => None,
        }
    }

    pub fn frame_counter_mut(&mut self) -> Option<&mut FrameCounter> {
        match self {
            Self::Grade(grade) => Some(grade.frame_counter_mut()),
            Self::RayMarcher(ray_marcher) => Some(ray_marcher.frame_counter_mut()),
            Self::TextureViewer(texture_viewer) => Some(texture_viewer.frame_counter_mut()),
            Self::TextureReader(texture_reader) => Some(texture_reader.frame_counter_mut()),
            _ => None,
        }
    }

    pub fn render_resource(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
    ) -> Option<RenderResource> {
        match self {
            Self::Grade(grade) => Some(grade.render_resource(device, target_state)),
            Self::RayMarcher(ray_marcher) => {
                Some(ray_marcher.render_resource(device, target_state))
            }
            Self::TextureViewer(texture_viewer) => {
                Some(texture_viewer.render_resource(device, target_state))
            }
            _ => None,
        }
    }

    pub fn buffer_data(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> BufferData {
        self.update_if_hash_changed(device, target_state, render_resource);
        match self {
            Self::Grade(grade) => grade.buffer_data(),
            Self::RayMarcher(ray_marcher) => ray_marcher.buffer_data(),
            Self::TextureViewer(texture_viewer) => texture_viewer.buffer_data(),
            _ => BufferData::default(),
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        match self {
            Self::Grade(grade) => grade.recompile_if_preprocessor_directives_changed(
                device,
                target_state,
                render_resource,
            ),
            Self::RayMarcher(ray_marcher) => ray_marcher
                .recompile_if_preprocessor_directives_changed(
                    device,
                    target_state,
                    render_resource,
                ),
            Self::TextureViewer(texture_viewer) => texture_viewer
                .recompile_if_preprocessor_directives_changed(
                    device,
                    target_state,
                    render_resource,
                ),
            _ => false,
        }
    }

    pub fn recompile_shader(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) {
        match self {
            Self::Grade(grade) => grade.recompile_shader(device, target_state, render_resource),
            Self::RayMarcher(ray_marcher) => {
                ray_marcher.recompile_shader(device, target_state, render_resource)
            }
            Self::TextureViewer(texture_viewer) => {
                texture_viewer.recompile_shader(device, target_state, render_resource)
            }
            _ => {}
        }
    }

    pub fn update_if_hash_changed(
        &mut self,
        device: &wgpu::Device,
        target_state: wgpu::ColorTargetState,
        render_resource: &mut RenderResource,
    ) -> bool {
        match self {
            Self::Grade(grade) => {
                grade.update_if_hash_changed(device, target_state, render_resource)
            }
            Self::RayMarcher(ray_marcher) => {
                ray_marcher.update_if_hash_changed(device, target_state, render_resource)
            }
            Self::TextureViewer(texture_viewer) => {
                texture_viewer.update_if_hash_changed(device, target_state, render_resource)
            }
            Self::TextureReader(texture_reader) => texture_reader.reset_if_hash_changed(),
            _ => false,
        }
    }

    pub fn default_pass_for_scene(gpu_scene: GPUScene) -> Self {
        Self::RayMarcher(RayMarcher::default().gpu_scene(gpu_scene).finalized())
    }

    pub fn create_output_texture_view(&self, device: &wgpu::Device) -> Option<TextureView> {
        match self {
            Self::Grade(grade) => grade.create_output_texture_view(device),
            Self::RayMarcher(ray_marcher) => ray_marcher.create_output_texture_view(device),
            Self::TextureViewer(texture_viewer) => {
                texture_viewer.create_output_texture_view(device)
            }
            Self::TextureReader(texture_reader) => {
                texture_reader.create_output_texture_view(device)
            }
            _ => None,
        }
    }

    pub fn output_texture_view(&self) -> Option<&TextureView> {
        match self {
            Self::Grade(grade) => grade.output_texture_view(),
            Self::RayMarcher(ray_marcher) => ray_marcher.output_texture_view(),
            Self::TextureViewer(texture_viewer) => texture_viewer.output_texture_view(),
            Self::TextureReader(texture_reader) => texture_reader.output_texture_view(),
            _ => None,
        }
    }

    pub fn evaluate(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        match self {
            Self::Grade(grade) => grade.evaluate(device, queue, encoder),
            Self::RayMarcher(ray_marcher) => ray_marcher.evaluate(device, queue, encoder),
            Self::TextureViewer(texture_viewer) => texture_viewer.evaluate(device, queue, encoder),
            Self::TextureReader(texture_reader) => texture_reader.evaluate_texture(device),
            _ => {}
        }
    }
}

pub type TextureEvaluatorsMap = SlotMap<TextureEvaluatorId, TextureEvaluators>;
