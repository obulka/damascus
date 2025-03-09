// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, sync::Arc};

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

use super::settings::{RayMarcherViewSettings, ViewportCompilerSettings, ViewportSettings};

use crate::icons::Icons;

mod buffers;
mod ray_marcher_view;

pub use ray_marcher_view::RayMarcherView;

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

    fn new<'a>(creation_context: &'a eframe::CreationContext<'a>, settings: &V) -> Option<Self> {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash();
        pipeline.set_reconstruct_hash(settings);

        Self::construct_pipeline(
            &mut pipeline,
            creation_context.wgpu_render_state.as_ref()?,
            settings,
        );

        Some(pipeline)
    }

    fn current_preprocessor_directives(&mut self) -> &mut HashSet<D>;

    fn update_preprocessor_directives(&mut self, settings: &C) -> bool {
        let new_directives = settings.directives(self.renderer());
        let current_directives = self.current_preprocessor_directives();

        // Check if the directives have changed and store them if they have
        if new_directives == *current_directives {
            return false;
        }
        *current_directives = new_directives;
        true
    }

    fn construct_pipeline(&mut self, wgpu_render_state: &egui_wgpu::RenderState, settings: &V);

    fn reconstruct_pipeline(&mut self, wgpu_render_state: &egui_wgpu::RenderState, settings: &V) {
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .clear();
        self.reset();
        self.construct_pipeline(wgpu_render_state, settings);
    }

    fn reconstruct_if_hash_changed(&mut self, frame: &mut eframe::Frame, settings: &V) -> bool {
        if self.set_reconstruct_hash(settings) {
            if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                self.reconstruct_pipeline(wgpu_render_state, settings);
            }
            return true;
        }
        false
    }

    fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState);

    fn recompile_if_hash_changed(
        &mut self,
        frame: &mut eframe::Frame,
        compiler_settings: &C,
    ) -> bool {
        if self.set_recompile_hash() {
            self.reset();
            if compiler_settings.dynamic_recompilation_enabled()
                && self.update_preprocessor_directives(&compiler_settings)
            {
                if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                    self.recompile_shader(wgpu_render_state);
                }
            }
            return true;
        }
        false
    }

    fn create_uniform_buffers(
        &self,
        _device: &Arc<wgpu::Device>,
        _settings: &V,
    ) -> Vec<buffers::Buffer> {
        vec![]
    }

    fn create_storage_buffers(
        &self,
        _device: &Arc<wgpu::Device>,
        _settings: &V,
    ) -> Vec<buffers::Buffer> {
        vec![]
    }

    fn uniform_bind_group_layout_entry(
        binding: u32,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn create_uniform_binding(
        device: &Arc<wgpu::Device>,
        buffers: &Vec<buffers::Buffer>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| {
                Self::uniform_bind_group_layout_entry(binding as u32, buffer.visibility)
            })
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
                resource: buffer.buffer.as_entire_binding(),
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

    fn storage_bind_group_layout_entry(
        binding: u32,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn create_storage_binding(
        device: &Arc<wgpu::Device>,
        buffers: &Vec<buffers::Buffer>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout_entries: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| {
                Self::storage_bind_group_layout_entry(binding as u32, buffer.visibility)
            })
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
                resource: buffer.buffer.as_entire_binding(),
            })
            .collect();

        let storage_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ray marcher scene storage bind group"),
            layout: &storage_bind_group_layout,
            entries: &bind_group_entries,
        });

        (storage_bind_group_layout, storage_bind_group)
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
        frame: &mut eframe::Frame,
        available_size: egui::Vec2,
        settings: &V,
        compiler_settings: &C,
    ) -> Option<epaint::PaintCallback>;

    fn show_frame(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
        settings: &V,
        compiler_settings: &C,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let mut available_size: egui::Vec2 = ui.available_size().round();
            available_size.y -= Self::controls_height(ui.style());
            paint_callback =
                self.custom_painting(ui, frame, available_size, settings, compiler_settings);
        });
        paint_callback
    }

    fn controls_height(style: &egui::Style) -> f32 {
        (Self::ICON_SIZE + style.spacing.button_padding.y + style.spacing.item_spacing.y) * 2. + 1.
    }

    fn show_restart_pause_play_buttons(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
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
                if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                    self.recompile_shader(wgpu_render_state);
                }
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

    fn show_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
        self.show_restart_pause_play_buttons(frame, ui);
        false
    }

    fn show(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
        settings: &V,
        compiler_settings: &C,
    ) -> bool {
        let paint_callback = self.show_frame(frame, ui, settings, compiler_settings);
        let reconstruct_pipeline = self.show_controls(frame, ui);

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
    Texture,
    Error { error: anyhow::Error },
}

impl Views {
    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Self {
        if let Some(view) = RayMarcherView::new(creation_context, &settings.ray_marcher_view) {
            return Self::RayMarcher { view: view };
        }
        Self::Error {
            error: anyhow::Error::msg("Failed to create ray marcher view"),
        }
    }

    pub fn reconstruct_pipeline(&mut self, frame: &eframe::Frame, settings: &ViewportSettings) {
        if let Some(wgpu_render_state) = frame.wgpu_render_state() {
            match self {
                Self::RayMarcher { view } => {
                    view.reconstruct_pipeline(wgpu_render_state, &settings.ray_marcher_view)
                }
                _ => {}
            }
        }
    }

    pub fn recompile_shader(&mut self, frame: &eframe::Frame) {
        if let Some(wgpu_render_state) = frame.wgpu_render_state() {
            match self {
                Self::RayMarcher { view } => view.recompile_shader(wgpu_render_state),
                _ => {}
            }
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
            _ => false,
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        frame: &mut eframe::Frame,
        compiler_settings: &ViewportCompilerSettings,
    ) {
        if self.update_preprocessor_directives(compiler_settings) {
            self.recompile_shader(frame);
        }
    }

    pub fn disable(&mut self) {
        match self {
            Self::RayMarcher { view } => view.disable(),
            _ => {}
        }
    }

    pub fn enable(&mut self) {
        match self {
            Self::RayMarcher { view } => view.enable(),
            _ => {}
        }
    }

    pub fn enabled(&mut self) -> bool {
        match self {
            Self::RayMarcher { view } => view.enabled(),
            _ => !self.disabled(),
        }
    }

    pub fn disabled(&mut self) -> bool {
        match self {
            Self::RayMarcher { view } => view.disabled(),
            _ => false,
        }
    }

    pub fn pause(&mut self) {
        match self {
            Self::RayMarcher { view } => view.pause(),
            _ => {}
        }
    }

    pub fn play(&mut self) {
        match self {
            Self::RayMarcher { view } => view.play(),
            _ => {}
        }
    }

    pub fn toggle_play_pause(&mut self) {
        match self {
            Self::RayMarcher { view } => view.toggle_play_pause(),
            _ => {}
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            Self::RayMarcher { view } => view.paused(),
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::RayMarcher { view } => view.reset(),
            _ => {}
        }
    }

    pub fn enable_and_play(&mut self) {
        match self {
            Self::RayMarcher { view } => view.enable_and_play(),
            _ => {}
        }
    }

    pub fn show(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
        settings: &ViewportSettings,
    ) -> bool {
        match self {
            Self::RayMarcher { view } => view.show(
                frame,
                ui,
                &settings.ray_marcher_view,
                &settings.compiler_settings.ray_marcher,
            ),
            _ => false,
        }
    }
}
