// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use itertools::izip;
use std::{borrow::Cow, collections::HashSet};

use crevice::std430::AsStd430;
use eframe::{
    egui,
    egui_wgpu::{
        self,
        wgpu::{self, util::DeviceExt},
    },
    epaint,
};
use serde_hashkey::{to_key_with_ordered_float, Error, Key, OrderedFloatPolicy, Result};

use damascus_core::{
    renderers::Renderer,
    shaders::{CompilerSettings, PreprocessorDirectives},
    textures::{Std430GPUVertex, Vertex},
    DualDevice, Settings,
};

use super::settings::{self, ViewportCompilerSettings, ViewportSettings};

use crate::icons::Icons;

mod compositor_view;
mod ray_marcher_view;
pub mod resources;

pub use compositor_view::CompositorView;
pub use ray_marcher_view::RayMarcherView;
use resources::{
    BindGroups, BindingResource, Buffer, BufferBindGroup, RenderResource, RenderResources,
    StorageTextureView, StorageTextureViewBindGroup, TextureView, TextureViewBindGroup,
    VertexBuffer,
};

pub trait View<
    R: Renderer<G, S>,
    G: Copy + Clone + AsStd430<Output = S>,
    S,
    C: CompilerSettings<D, R, G, S>,
    D: PreprocessorDirectives,
    V: Settings,
>: Default
{
    const ICON_SIZE: f32 = 25.;

    fn renderer(&self) -> &R;

    fn renderer_mut(&mut self) -> &mut R;

    fn create_recompile_hash(
        &self,
        _compiler_settings: &C,
    ) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(self.renderer())
    }

    fn recompile_hash(&self) -> &Key<OrderedFloatPolicy>;

    fn recompile_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy>;

    fn create_reconstruct_hash(&self, settings: &V) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&settings)
    }

    fn reconstruct_hash(&self) -> &Key<OrderedFloatPolicy>;

    fn reconstruct_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy>;

    fn set_recompile_hash(&mut self, settings: &C) -> bool {
        let mut hash_changed = false;
        if let Ok(recompile_hash) = self.create_recompile_hash(settings) {
            if recompile_hash != *self.recompile_hash() {
                *self.recompile_hash_mut() = recompile_hash;
                hash_changed = true;
            }
        }

        (settings.dynamic_recompilation_enabled() && self.update_preprocessor_directives(&settings))
            || hash_changed
    }

    fn set_reconstruct_hash(&mut self, settings: &V) -> bool {
        if let Ok(reconstruct_hash) = self.create_reconstruct_hash(&settings) {
            if reconstruct_hash != *self.reconstruct_hash() {
                *self.reconstruct_hash_mut() = reconstruct_hash;
                return true;
            }
        }
        false
    }

    fn set_renderer(&mut self, renderer: R) {
        *self.renderer_mut() = renderer;
    }

    fn new(render_state: &egui_wgpu::RenderState, settings: &V, compiler_settings: &C) -> Self {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash(compiler_settings);
        pipeline.set_reconstruct_hash(settings);

        Self::construct_pipeline(&mut pipeline, render_state, settings);

        pipeline
    }

    fn current_preprocessor_directives(&self) -> &HashSet<D>;

    fn current_preprocessor_directives_mut(&mut self) -> &mut HashSet<D>;

    fn update_preprocessor_directives(&mut self, settings: &C) -> bool {
        let new_directives = settings.directives(self.renderer());
        let current_directives = self.current_preprocessor_directives_mut();

        // Check if the directives have changed and store them if they have
        if new_directives == *current_directives {
            return false;
        }
        *current_directives = new_directives;
        true
    }

    fn fragment_shaders(&self) -> Vec<String> {
        vec![]
    }

    fn vertex_shaders(&self) -> Vec<String> {
        vec![]
    }

    fn vertices(&self) -> Vec<Vec<Std430GPUVertex>> {
        vec![vec![
            Vertex::new(1., 1.).as_std430(),
            Vertex::new(-1., 1.).as_std430(),
            Vertex::new(1., -1.).as_std430(),
            Vertex::new(-1., -1.).as_std430(),
        ]]
    }

    fn create_render_pipelines(
        &self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        bind_groups: &Vec<BindGroups>,
    ) -> Vec<wgpu::RenderPipeline> {
        izip!(
            self.create_vertex_buffer_layouts(),
            bind_groups,
            self.vertex_shaders(),
            self.fragment_shaders()
        )
        .map(
            |(vertex_buffer_layouts, bind_group, vertex_shader_source, fragment_shader_source)| {
                let pipeline_layout: wgpu::PipelineLayout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("pipeline layout"),
                        bind_group_layouts: &bind_group.bind_group_layouts(),
                        push_constant_ranges: &[],
                    });

                let vertex_shader: wgpu::ShaderModule =
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("vertex shader"),
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&vertex_shader_source))
                            .into(),
                    });

                let fragment_shader: wgpu::ShaderModule =
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("fragment shader"),
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&fragment_shader_source))
                            .into(),
                    });

                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("render pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vertex_shader,
                        entry_point: Some("vs_main"),
                        buffers: &vertex_buffer_layouts,
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
            },
        )
        .collect()
    }

    fn construct_pipeline(&mut self, render_state: &egui_wgpu::RenderState, settings: &V) {
        let device = &render_state.device;

        let vertex_buffers: Vec<Vec<VertexBuffer>> = self.create_vertex_buffers(device, &settings);
        let uniform_buffers: Vec<Vec<Buffer>> = self.create_uniform_buffers(device, &settings);
        let storage_buffers: Vec<Vec<Buffer>> = self.create_storage_buffers(device, &settings);
        let texture_views: Vec<Vec<TextureView>> = self.create_texture_views(device);
        let storage_texture_views: Vec<Vec<StorageTextureView>> =
            self.create_storage_texture_views(device);

        let bind_groups: Vec<BindGroups> = izip!(
            uniform_buffers,
            storage_buffers,
            texture_views,
            storage_texture_views,
        )
        .map(
            |(uniform_buffers, storage_buffers, texture_views, storage_texture_views)| {
                let mut uniform_bind_group: Option<BufferBindGroup> = None;
                if !uniform_buffers.is_empty() {
                    let (bind_group_layout, bind_group) =
                        Self::create_uniform_binding(device, &uniform_buffers);
                    uniform_bind_group = Some(BufferBindGroup {
                        bind_group: bind_group,
                        bind_group_layout: bind_group_layout,
                        buffers: uniform_buffers,
                    });
                }

                let mut storage_bind_group: Option<BufferBindGroup> = None;
                if !storage_buffers.is_empty() {
                    let (bind_group_layout, bind_group) =
                        Self::create_storage_binding(device, &storage_buffers);
                    storage_bind_group = Some(BufferBindGroup {
                        bind_group: bind_group,
                        bind_group_layout: bind_group_layout,
                        buffers: storage_buffers,
                    });
                }

                let mut texture_bind_group: Option<TextureViewBindGroup> = None;
                if !texture_views.is_empty() {
                    let (bind_group_layout, bind_group) =
                        Self::create_texture_binding(device, &texture_views);

                    texture_bind_group = Some(TextureViewBindGroup {
                        bind_group: bind_group,
                        bind_group_layout: bind_group_layout,
                        texture_views: texture_views,
                    });
                }

                let mut storage_texture_bind_group: Option<StorageTextureViewBindGroup> = None;
                if !storage_texture_views.is_empty() {
                    let (bind_group_layout, bind_group) =
                        Self::create_storage_texture_binding(device, &storage_texture_views);

                    storage_texture_bind_group = Some(StorageTextureViewBindGroup {
                        bind_group: bind_group,
                        bind_group_layout: bind_group_layout,
                        storage_texture_views: storage_texture_views,
                    });
                }

                BindGroups {
                    uniform_bind_group,
                    storage_bind_group,
                    texture_bind_group,
                    storage_texture_bind_group,
                }
            },
        )
        .collect();

        let render_pipelines: Vec<wgpu::RenderPipeline> =
            self.create_render_pipelines(device, render_state.target_format, &bind_groups);

        let render_resources = RenderResources {
            resources: izip!(vertex_buffers, bind_groups, render_pipelines)
                .map(
                    |(vertex_buffers, bind_groups, render_pipeline)| RenderResource {
                        render_pipeline,
                        vertex_buffers,
                        bind_groups,
                    },
                )
                .collect(),
        };

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(render_resources);
    }

    fn reconstruct_pipeline(&mut self, render_state: &egui_wgpu::RenderState, settings: &V) {
        render_state.renderer.write().callback_resources.clear();
        self.reset();
        self.construct_pipeline(render_state, settings);
    }

    fn reconstruct_if_hash_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        settings: &V,
    ) -> bool {
        if self.set_reconstruct_hash(settings) {
            self.reconstruct_pipeline(render_state, settings);
            return true;
        }
        false
    }

    fn recompile_shader(&mut self, render_state: &egui_wgpu::RenderState) {
        if let Some(render_resources) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            self.reset();

            let device = &render_state.device;

            let render_pipelines: Vec<wgpu::RenderPipeline> = self.create_render_pipelines(
                device,
                render_state.target_format,
                &render_resources.bind_groups(),
            );

            // Create the updated pipeline
            for (resource, render_pipeline) in
                render_resources.resources.iter_mut().zip(render_pipelines)
            {
                resource.render_pipeline = render_pipeline;
            }
        }
    }

    fn recompile_if_hash_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        compiler_settings: &C,
    ) -> bool {
        if self.set_recompile_hash(compiler_settings) {
            self.recompile_shader(render_state);
            return true;
        }
        false
    }

    fn create_vertex_buffers(
        &mut self,
        device: &wgpu::Device,
        _settings: &V,
    ) -> Vec<Vec<VertexBuffer>> {
        let mut vertices = Vec::<Vec<VertexBuffer>>::new();
        for vertex in self.vertices().iter() {
            let vertex_count: u32 = vertex.len() as u32;
            vertices.push(vec![VertexBuffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vertex buffer"),
                    contents: bytemuck::cast_slice(vertex),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
                vertex_count: vertex_count,
            }]);
        }
        vertices
    }

    fn create_vertex_buffer_layouts(&self) -> Vec<Vec<wgpu::VertexBufferLayout<'_>>> {
        vec![vec![wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Std430GPUVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }]]
    }

    fn create_uniform_buffers(&self, _device: &wgpu::Device, _settings: &V) -> Vec<Vec<Buffer>> {
        vec![]
    }

    fn create_storage_buffers(&self, _device: &wgpu::Device, _settings: &V) -> Vec<Vec<Buffer>> {
        vec![]
    }

    fn create_texture_views(&self, _device: &wgpu::Device) -> Vec<Vec<TextureView>> {
        vec![]
    }

    fn create_storage_texture_views(&self, _device: &wgpu::Device) -> Vec<Vec<StorageTextureView>> {
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

    fn create_uniform_binding(
        device: &wgpu::Device,
        buffers: &Vec<Buffer>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::uniform_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let uniform_bind_group_layout: wgpu::BindGroupLayout =
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

        let uniform_bind_group: wgpu::BindGroup =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("uniform bind group"),
                layout: &uniform_bind_group_layout,
                entries: &bind_group_entries,
            });

        (uniform_bind_group_layout, uniform_bind_group)
    }

    fn create_storage_binding(
        device: &wgpu::Device,
        buffers: &Vec<Buffer>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| Self::storage_bind_group_layout_entry(binding as u32, &buffer))
            .collect();

        let storage_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let storage_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("storage bind group"),
            layout: &storage_bind_group_layout,
            entries: &bind_group_entries,
        });

        (storage_bind_group_layout, storage_bind_group)
    }

    fn create_texture_binding(
        device: &wgpu::Device,
        texture_views: &Vec<TextureView>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = texture_views
            .iter()
            .enumerate()
            .map(|(binding, texture_view)| {
                Self::texture_bind_group_layout_entry(binding as u32, &texture_view)
            })
            .collect();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture bind group"),
            layout: &texture_bind_group_layout,
            entries: &bind_group_entries,
        });

        (texture_bind_group_layout, texture_bind_group)
    }

    fn create_storage_texture_binding(
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

        let storage_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let storage_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("storage texture bind group"),
            layout: &storage_texture_bind_group_layout,
            entries: &bind_group_entries,
        });

        (
            storage_texture_bind_group_layout,
            storage_texture_bind_group,
        )
    }

    fn disable(&mut self) {}

    fn enable(&mut self) {}

    fn disabled(&mut self) -> bool {
        false
    }

    fn enabled(&mut self) -> bool {
        !self.disabled()
    }

    fn pause(&mut self);

    fn play(&mut self);

    fn paused(&self) -> bool;

    fn toggle_play_pause(&mut self) {
        if self.disabled() {
            return;
        }
        if self.paused() {
            self.play();
        } else {
            self.pause();
        }
    }

    fn enable_and_play(&mut self) {
        self.enable();
        self.play();
    }

    fn reset(&mut self) {}

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        render_state: &egui_wgpu::RenderState,
        available_size: egui::Vec2,
        settings: &V,
        compiler_settings: &C,
    ) -> Option<epaint::PaintCallback>;

    fn show_frame(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
        settings: &V,
        compiler_settings: &C,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let mut available_size: egui::Vec2 = ui.available_size().round();
            available_size.y -= Self::controls_height(ui.style());
            if available_size.x > 0. && available_size.y > 0. {
                paint_callback = self.custom_painting(
                    ui,
                    render_state,
                    available_size,
                    settings,
                    compiler_settings,
                );
            }
        });
        paint_callback
    }

    fn controls_height(style: &egui::Style) -> f32 {
        (Self::ICON_SIZE + style.spacing.button_padding.y + style.spacing.item_spacing.y) * 2. + 1.
    }

    fn show_restart_pause_play_buttons(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
    ) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    self.enabled(),
                    egui::ImageButton::new(
                        egui::Image::new(Icons::Refresh.source())
                            .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE)),
                    ),
                )
                .on_hover_text("restart the render")
                .clicked()
            {
                self.recompile_shader(render_state);
            }

            let tooltip: &str;
            let pause_icon = egui::Image::new(if self.paused() {
                tooltip = "start the render";
                Icons::Play.source()
            } else {
                tooltip = "pause the render";
                Icons::Pause.source()
            })
            .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
            if ui
                .add_enabled(self.enabled(), egui::ImageButton::new(pause_icon))
                .on_hover_text(tooltip)
                .clicked()
            {
                self.toggle_play_pause();
            }
        });
    }

    fn show_top_bar(
        &mut self,
        _render_state: &egui_wgpu::RenderState,
        _ui: &mut egui::Ui,
        _settings: &mut V,
    ) -> bool {
        false
    }

    fn show_bottom_bar(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
    ) -> bool {
        self.show_restart_pause_play_buttons(render_state, ui);
        false
    }

    fn show(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
        settings: &mut V,
        compiler_settings: &mut C,
    ) -> bool {
        let mut reconstruct_pipeline: bool = self.show_top_bar(render_state, ui, settings);
        let paint_callback = self.show_frame(render_state, ui, settings, compiler_settings);
        reconstruct_pipeline |= self.show_bottom_bar(render_state, ui);

        if !reconstruct_pipeline {
            if let Some(callback) = paint_callback {
                ui.painter().add(callback);
            }
        }

        reconstruct_pipeline
    }
}

pub enum Views {
    RayMarcher { view: RayMarcherView },
    Compositor { view: CompositorView },
    Error { error: anyhow::Error },
}

impl Views {
    pub fn new(render_state: &egui_wgpu::RenderState, settings: &ViewportSettings) -> Self {
        Self::RayMarcher {
            view: RayMarcherView::new(
                render_state,
                &settings.ray_marcher_view,
                &settings.compiler_settings.ray_marcher,
            ),
        }
    }

    pub fn reconstruct_pipeline(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        settings: &ViewportSettings,
    ) {
        match self {
            Self::RayMarcher { view } => {
                view.reconstruct_pipeline(render_state, &settings.ray_marcher_view)
            }
            Self::Compositor { view } => {
                view.reconstruct_pipeline(render_state, &settings.compositor_view)
            }
            _ => {}
        }
    }

    pub fn recompile_shader(&mut self, render_state: &egui_wgpu::RenderState) {
        match self {
            Self::RayMarcher { view } => view.recompile_shader(render_state),
            Self::Compositor { view } => view.recompile_shader(render_state),
            _ => {}
        }
    }

    pub fn update_preprocessor_directives(
        &mut self,
        compiler_settings: &ViewportCompilerSettings,
    ) -> bool {
        match self {
            Self::RayMarcher { view } => {
                view.update_preprocessor_directives(&compiler_settings.ray_marcher)
            }
            Self::Compositor { view } => {
                view.update_preprocessor_directives(&compiler_settings.compositor)
            }
            _ => false,
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        compiler_settings: &ViewportCompilerSettings,
    ) {
        if self.update_preprocessor_directives(compiler_settings) {
            self.recompile_shader(render_state);
        }
    }

    pub fn disable(&mut self) {
        match self {
            Self::RayMarcher { view } => view.disable(),
            Self::Compositor { view } => view.disable(),
            _ => {}
        }
    }

    pub fn enable(&mut self) {
        match self {
            Self::RayMarcher { view } => view.enable(),
            Self::Compositor { view } => view.enable(),
            _ => {}
        }
    }

    pub fn enabled(&mut self) -> bool {
        match self {
            Self::RayMarcher { view } => view.enabled(),
            Self::Compositor { view } => view.enabled(),
            _ => !self.disabled(),
        }
    }

    pub fn disabled(&mut self) -> bool {
        match self {
            Self::RayMarcher { view } => view.disabled(),
            Self::Compositor { view } => view.disabled(),
            _ => false,
        }
    }

    pub fn pause(&mut self) {
        match self {
            Self::RayMarcher { view } => view.pause(),
            Self::Compositor { view } => view.pause(),
            _ => {}
        }
    }

    pub fn play(&mut self) {
        match self {
            Self::RayMarcher { view } => view.play(),
            Self::Compositor { view } => view.play(),
            _ => {}
        }
    }

    pub fn toggle_play_pause(&mut self) {
        match self {
            Self::RayMarcher { view } => view.toggle_play_pause(),
            Self::Compositor { view } => view.toggle_play_pause(),
            _ => {}
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            Self::RayMarcher { view } => view.paused(),
            Self::Compositor { view } => view.paused(),
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::RayMarcher { view } => view.reset(),
            Self::Compositor { view } => view.reset(),
            _ => {}
        }
    }

    pub fn enable_and_play(&mut self) {
        match self {
            Self::RayMarcher { view } => view.enable_and_play(),
            Self::Compositor { view } => view.enable_and_play(),
            _ => {}
        }
    }

    pub fn show(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
        settings: &mut ViewportSettings,
    ) -> bool {
        match self {
            Self::RayMarcher { view } => view.show(
                render_state,
                ui,
                &mut settings.ray_marcher_view,
                &mut settings.compiler_settings.ray_marcher,
            ),
            Self::Compositor { view } => view.show(
                render_state,
                ui,
                &mut settings.compositor_view,
                &mut settings.compiler_settings.compositor,
            ),
            _ => false,
        }
    }
}
