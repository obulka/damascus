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
    render_passes::resources::{
        BindGroups, BindingResource, Buffer, BufferBindGroup, RenderResource, RenderResources,
        StorageTextureView, StorageTextureViewBindGroup, TextureView, TextureViewBindGroup,
        VertexBuffer,
    },
    renderers::Renderer,
    shaders::{CompilerSettings, PreprocessorDirectives},
    textures::{Std430GPUVertex, Vertex},
    DualDevice, Settings,
};

use super::settings::{self, ViewportCompilerSettings, ViewportSettings};

use crate::icons::Icons;

mod compositor_view;
mod ray_marcher_view;

pub use compositor_view::CompositorView;
pub use ray_marcher_view::RayMarcherView;

pub trait View: Default {
    const ICON_SIZE: f32 = 25.;

    fn new(render_state: &egui_wgpu::RenderState, settings: &V, compiler_settings: &C) -> Self {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash(compiler_settings);
        pipeline.set_reconstruct_hash(settings);

        Self::construct_pipeline(&mut pipeline, render_state, settings);

        pipeline
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
                self.reset();
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
        let mut reconstruct_render_resource: bool = self.show_top_bar(render_state, ui, settings);
        let paint_callback = self.show_frame(render_state, ui, settings, compiler_settings);
        reconstruct_render_resource |= self.show_bottom_bar(render_state, ui);

        if !reconstruct_render_resource {
            if let Some(callback) = paint_callback {
                ui.painter().add(callback);
            }
        }

        reconstruct_render_resource
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

    pub fn reconstruct_render_resource(&mut self, render_state: &egui_wgpu::RenderState) {
        render_state.renderer.write().callback_resources.clear();
        match self {
            Self::RayMarcher { view } => view.reconstruct_render_resource(
                &render_state.device,
                render_state.target_format.into(),
            ),
            Self::Compositor { view } => view.reconstruct_render_resource(
                &render_state.device,
                render_state.target_format.into(),
            ),
            _ => {}
        }

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(render_resources);
    }

    pub fn recompile_shader(&mut self, render_state: &egui_wgpu::RenderState) {
        if let Some(render_resource) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            match self {
                Self::RayMarcher { view } => view.recompile_shader(render_state),
                Self::Compositor { view } => view.recompile_shader(render_state),
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
