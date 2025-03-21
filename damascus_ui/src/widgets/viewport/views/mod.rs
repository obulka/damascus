// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{borrow::Cow, collections::HashSet};

use crevice::std430::AsStd430;
use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
};

use damascus_core::{
    renderers::Renderer,
    shaders::{CompilerSettings, PreprocessorDirectives},
    Settings,
};

use super::settings::{self, ViewportCompilerSettings, ViewportSettings};

use crate::icons::Icons;

mod compositor_view;
mod ray_marcher_view;
pub mod resources;

pub use compositor_view::CompositorView;
pub use ray_marcher_view::RayMarcherView;
use resources::{
    BindingResource, Buffer, BufferBindGroup, RenderResources, StorageTextureView,
    StorageTextureViewBindGroup, TextureView, TextureViewBindGroup,
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

    fn set_recompile_hash(&mut self) -> bool;

    fn set_reconstruct_hash(&mut self, settings: &V) -> bool;

    fn set_renderer(&mut self, renderer: R) {
        *self.renderer_mut() = renderer;
    }

    fn new(render_state: &egui_wgpu::RenderState, settings: &V) -> Self {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash();
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

    fn get_shader(&self) -> String;

    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        bind_group_layouts: Vec<&wgpu::BindGroupLayout>,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("source shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&self.get_shader())).into(),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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

    fn construct_pipeline(&mut self, render_state: &egui_wgpu::RenderState, settings: &V) {
        let device = &render_state.device;

        let uniform_buffers: Vec<Buffer> = self.create_uniform_buffers(device, &settings);
        let storage_buffers: Vec<Buffer> = self.create_storage_buffers(device, &settings);
        let texture_views: Vec<TextureView> = self.create_texture_views(device);
        let storage_texture_views: Vec<StorageTextureView> =
            self.create_storage_texture_views(device);

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

        let mut render_resources = RenderResources {
            render_pipeline: None,
            uniform_bind_group: uniform_bind_group,
            storage_bind_group: storage_bind_group,
            texture_bind_group: texture_bind_group,
            storage_texture_bind_group: storage_texture_bind_group,
        };

        render_resources.render_pipeline = Some(self.create_render_pipeline(
            device,
            render_state.target_format,
            render_resources.bind_group_layouts(),
        ));

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

            // Create the updated pipeline
            render_resources.render_pipeline = Some(self.create_render_pipeline(
                device,
                render_state.target_format,
                render_resources.bind_group_layouts(),
            ));
        }
    }

    fn recompile_if_hash_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        compiler_settings: &C,
    ) -> bool {
        if self.set_recompile_hash() {
            self.reset();
            if compiler_settings.dynamic_recompilation_enabled()
                && self.update_preprocessor_directives(&compiler_settings)
            {
                self.recompile_shader(render_state);
            }
            return true;
        }
        false
    }

    fn create_uniform_buffers(&self, _device: &wgpu::Device, _settings: &V) -> Vec<Buffer> {
        vec![]
    }

    fn create_storage_buffers(&self, _device: &wgpu::Device, _settings: &V) -> Vec<Buffer> {
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

    fn show_controls(&mut self, render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        self.show_restart_pause_play_buttons(render_state, ui);
        false
    }

    fn show(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
        settings: &V,
        compiler_settings: &C,
    ) -> bool {
        let paint_callback = self.show_frame(render_state, ui, settings, compiler_settings);
        let reconstruct_pipeline = self.show_controls(render_state, ui);

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
            view: RayMarcherView::new(render_state, &settings.ray_marcher_view),
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
        settings: &ViewportSettings,
    ) -> bool {
        match self {
            Self::RayMarcher { view } => view.show(
                render_state,
                ui,
                &settings.ray_marcher_view,
                &settings.compiler_settings.ray_marcher,
            ),
            Self::Compositor { view } => view.show(
                render_state,
                ui,
                &settings.compositor_view,
                &settings.compiler_settings.compositor,
            ),
            _ => false,
        }
    }
}
