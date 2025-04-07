// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use itertools::izip;
use std::{borrow::Cow, collections::HashSet};

use crevice::std430::AsStd430;
use serde_hashkey::{to_key_with_ordered_float, Error, Key, OrderedFloatPolicy, Result};
use wgpu;

use crate::{
    geometry,
    renderers::Renderer,
    shaders,
    textures::{
        texture_corner_vertices_2d, GPUTextureVertex, Std430GPUTextureVertex, TextureVertex,
    },
    DualDevice, Hashable,
};

// mod grade;
pub mod ray_marcher;
pub mod resources;
pub mod viewer;

use ray_marcher::RayMarcherPass;
use resources::{
    BindGroups, BindingResource, Buffer, BufferBindGroup, RenderResource, StorageTextureView,
    StorageTextureViewBindGroup, TextureView, TextureViewBindGroup, VertexBuffer,
};
// use viewer::ViewerPass;

pub trait RenderPass<
    CompilationOptions: Hashable,
    PipelineOptions: Hashable,
    Vertex: geometry::Vertex<GPUVertex, Std430GPUVertex>,
    GPUVertex: Copy + Clone + AsStd430<Output = Std430GPUVertex>,
    Std430GPUVertex,
>: Default + shaders::ShaderSource<Directives: shaders::PreprocessorDirectives>
{
    fn compilation_options(&self) -> &CompilationOptions;

    fn recompile_hash(&self) -> &Key<OrderedFloatPolicy>;

    fn recompile_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy>;

    fn pipeline_options(&self) -> &PipelineOptions;

    fn reconstruct_hash(&self) -> &Key<OrderedFloatPolicy>;

    fn reconstruct_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy>;

    fn vertices(&self) -> Vec<Std430GPUVertex>;

    fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let mut render_pass = Self::default();
        render_pass.update_recompile_hash();
        render_pass.update_reconstruct_hash();

        Self::construct_pipeline(&mut render_pass, render_state);

        render_pass
    }

    fn label(&self) -> String {
        ""
    }

    fn reset(&mut self) {}

    fn create_recompile_hash(&self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(self.compilation_options())
    }

    fn create_reconstruct_hash(&self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(self.pipeline_options())
    }

    fn update_recompile_hash(&mut self) -> bool {
        let mut hash_changed = false;
        if let Ok(recompile_hash) = self.create_recompile_hash() {
            if recompile_hash != *self.recompile_hash() {
                *self.recompile_hash_mut() = recompile_hash;
                hash_changed = true;
            }
        }

        self.update_directives() || hash_changed
    }

    fn update_reconstruct_hash(&mut self) -> bool {
        if let Ok(reconstruct_hash) = self.create_reconstruct_hash() {
            if reconstruct_hash != *self.reconstruct_hash() {
                *self.reconstruct_hash_mut() = reconstruct_hash;
                return true;
            }
        }
        false
    }

    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        bind_groups: &BindGroups,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout: wgpu::PipelineLayout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(self.label() + " pipeline layout"),
                bind_group_layouts: &bind_groups.bind_group_layouts(),
                push_constant_ranges: &[],
            });

        let vertex_shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("vertex shader"),
                source: self.vertex_shader(),
            });

        let fragment_shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("fragment shader"),
                source: self.fragment_shader(),
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: Some("vs_main"),
                buffers: &self.create_vertex_buffer_layout(),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(texture_format.into())],
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

    fn construct_pipeline(&mut self, render_state: &egui_wgpu::RenderState) {
        let device = &render_state.device;

        let vertex_buffer: VertexBuffer = self.create_vertex_buffer(device);
        let uniform_buffers: Vec<Buffer> = self.create_uniform_buffers(device);
        let storage_buffers: Vec<Buffer> = self.create_storage_buffers(device);
        let texture_views: Vec<TextureView> = self.create_texture_views(device);
        let storage_texture_views: Vec<StorageTextureView> =
            self.create_storage_texture_views(device);

        let mut uniform_bind_group: Option<BufferBindGroup> = None;
        if !uniform_buffers.is_empty() {
            uniform_bind_group = Some(Self::create_uniform_bind_group(device, uniform_buffers));
        }

        let mut storage_bind_group: Option<BufferBindGroup> = None;
        if !storage_buffers.is_empty() {
            storage_bind_group = Some(Self::create_storage_bind_group(device, storage_buffers));
        }

        let mut texture_bind_group: Option<TextureViewBindGroup> = None;
        if !texture_views.is_empty() {
            texture_bind_group = Some(Self::create_texture_view_bind_group(device, texture_views));
        }

        let mut storage_texture_bind_group: Option<StorageTextureViewBindGroup> = None;
        if !storage_texture_views.is_empty() {
            storage_texture_bind_group = Some(Self::create_storage_texture_view_bind_group(
                device,
                storage_texture_views,
            ));
        }

        let bind_groups = BindGroups {
            uniform_bind_group,
            storage_bind_group,
            texture_bind_group,
            storage_texture_bind_group,
        };

        let render_pipeline: wgpu::RenderPipeline =
            self.create_render_pipeline(device, render_state.target_format, &bind_groups);

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(RenderResource {
                render_pipeline,
                vertex_buffer,
                bind_groups,
            });
    }

    fn reconstruct_pipeline(&mut self, render_state: &egui_wgpu::RenderState) {
        render_state.renderer.write().callback_resources.clear();
        self.reset();
        self.construct_pipeline(render_state);
    }

    fn reconstruct_if_hash_changed(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        if self.update_reconstruct_hash() {
            self.reconstruct_pipeline(render_state);
            return true;
        }
        false
    }

    fn recompile_shader(&mut self, render_state: &egui_wgpu::RenderState) {
        if let Some(render_resource) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResource>()
        {
            self.reset();

            let device = &render_state.device;

            let render_pipeline: wgpu::RenderPipeline = self.create_render_pipeline(
                device,
                render_state.target_format,
                &render_resource.bind_groups(),
            );

            render_resource.render_pipeline = render_pipeline;
        }
    }

    fn recompile_if_hash_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        compiler_settings: &C,
    ) -> bool {
        if self.update_recompile_hash(compiler_settings) {
            self.recompile_shader(render_state);
            return true;
        }
        false
    }

    fn reconstruct_or_recompile_if_hash_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
    ) -> bool {
        self.reconstruct_if_hash_changed(render_state)
            || self.recompile_if_hash_changed(render_state)
    }

    fn create_vertex_buffer(&mut self, device: &wgpu::Device) -> VertexBuffer {
        let vertices: Vec<Std430GPUVertex> = self.vertices();
        let vertex_count: u32 = vertices.len() as u32;
        VertexBuffer {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: vertex_count,
        };
    }

    fn create_vertex_buffer_layout(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Std430GPUVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Vertex::attr_array(),
        }
    }

    fn create_uniform_buffers(&self, _device: &wgpu::Device) -> Vec<Buffer> {
        vec![]
    }

    fn create_storage_buffers(&self, _device: &wgpu::Device) -> Vec<Buffer> {
        vec![]
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

    fn create_uniform_bind_group(device: &wgpu::Device, buffers: Vec<Buffer>) -> BufferBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::uniform_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let bind_group_layout: wgpu::BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform bind group layout"),
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
            label: Some("uniform bind group"),
            layout: &bind_group_layout,
            entries: &bind_group_entries,
        });

        BufferBindGroup {
            bind_group,
            bind_group_layout,
            buffers,
        }
    }

    fn create_storage_bind_group(device: &wgpu::Device, buffers: Vec<Buffer>) -> BufferBindGroup {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::storage_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("storage bind group layout"),
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
            label: Some("storage bind group"),
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
            label: Some("texture bind group layout"),
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
            label: Some("texture bind group"),
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
        device: &wgpu::Device,
        storage_texture_views: &Vec<StorageTextureView>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = storage_texture_views
            .iter()
            .enumerate()
            .map(|(binding, storage_texture_view)| {
                Self::storage_texture_bind_group_layout_entry(binding as u32, &storage_texture_view)
            })
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("storage texture bind group layout"),
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
            label: Some("storage texture bind group"),
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

pub trait TextureProcessingPass:
    RenderPass<
    CompilationOptions: Hashable,
    PipelineOptions: Hashable,
    TextureVertex,
    GPUTextureVertex,
    Std430GPUTextureVertex,
>
{
    fn vertices(&self) -> Vec<Std430GPUVertex> {
        texture_corner_vertices_2d()
    }
}

// pub enum RenderPasses {
//     RayMarcher { pass: RayMarcherPass },
//     Viewer { pass: Viewer },
//     Error { error: anyhow::Error },
// }

// impl RenderPasses {
//     pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
//         Self::RayMarcher {
//             pass: RayMarcher::new(render_state),
//         }
//     }

//     pub fn reconstruct_pipeline(&mut self, render_state: &egui_wgpu::RenderState) {
//         match self {
//             Self::RayMarcher { pass } => pass.reconstruct_pipeline(render_state),
//             Self::Compositor { pass } => pass.reconstruct_pipeline(render_state),
//             _ => {}
//         }
//     }

//     pub fn recompile_shader(&mut self, render_state: &egui_wgpu::RenderState) {
//         match self {
//             Self::RayMarcher { pass } => pass.recompile_shader(render_state),
//             Self::Compositor { pass } => pass.recompile_shader(render_state),
//             _ => {}
//         }
//     }

//     pub fn update_directives(&mut self) -> bool {
//         match self {
//             Self::RayMarcher { pass } => pass.update_directives(),
//             Self::Compositor { pass } => pass.update_directives(),
//             _ => false,
//         }
//     }

//     pub fn recompile_if_preprocessor_directives_changed(
//         &mut self,
//         render_state: &egui_wgpu::RenderState,
//     ) {
//         if self.update_directives() {
//             self.recompile_shader(render_state);
//         }
//     }
// }
