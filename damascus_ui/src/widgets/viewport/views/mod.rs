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
    render_passes::{
        resources::{RenderResource, RenderResources},
        RenderPass, RenderPasses,
    },
    renderers::Renderer,
    shaders::{CompilerSettings, PreprocessorDirectives},
    textures::{Std430GPUVertex, Vertex},
    DualDevice, Settings,
};

use super::settings::{self, ViewportCompilerSettings, ViewportSettings};

use crate::icons::Icons;

mod scene_view;
mod texture_view;

pub use scene_view::SceneView;
pub use texture_view::TextureView;

pub trait View: Default {
    const ICON_SIZE: f32 = 25.;

    fn render_passes(&self) -> &Vec<RenderPasses>;

    fn render_passes_mut(&mut self) -> &mut Vec<RenderPasses>;

    fn disable(&mut self) {}

    fn enable(&mut self) {}

    fn disabled(&mut self) -> bool {
        false
    }

    fn pause(&mut self);

    fn play(&mut self);

    fn paused(&self) -> bool;

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        render_state: &egui_wgpu::RenderState,
        available_size: egui::Vec2,
    ) -> Option<epaint::PaintCallback>;

    fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let mut view = Self::default();
        view.reconstruct_render_resources(render_state);
        view
    }

    fn enabled(&mut self) -> bool {
        !self.disabled()
    }

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

    fn reset(&mut self) {
        self.render_passes_mut()
            .iter_mut()
            .map(|render_pass| render_pass.reset())
            .collect()
    }

    fn recompile_shaders(&mut self, render_state: &egui_wgpu::RenderState) {
        if let Some(render_resources) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            self.render_passes_mut()
                .iter_mut()
                .zip(&mut render_resources.resources)
                .map(|(render_pass, render_resource)| {
                    render_pass.recompile_shader(
                        &render_state.device,
                        render_state.target_format.into(),
                        render_resource,
                    )
                })
                .collect()
        }
    }

    fn reconstruct_render_resources(&mut self, render_state: &egui_wgpu::RenderState) {
        render_state.renderer.write().callback_resources.clear();

        let mut render_resources = RenderResources::new(
            self.render_passes_mut()
                .iter_mut()
                .map(|render_pass| {
                    render_pass
                        .render_resource(&render_state.device, render_state.target_format.into())
                })
                .collect(),
        );

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(render_resources);
    }

    fn update_if_hash_changed(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        let mut updated = false;
        if let Some(render_resources) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            self.render_passes_mut()
                .iter_mut()
                .zip(&mut render_resources.resources)
                .map(|(render_pass, render_resource)| {
                    updated |= render_pass.update_if_hash_changed(
                        &render_state.device,
                        render_state.target_format.into(),
                        render_resource,
                    )
                })
                .collect()
        }
        updated
    }

    fn show_frame(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let mut available_size: egui::Vec2 = ui.available_size().round();
            available_size.y -= Self::controls_height(ui.style());
            if available_size.x > 0. && available_size.y > 0. {
                paint_callback = self.custom_painting(ui, render_state, available_size);
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

    fn show_top_bar(&mut self, _render_state: &egui_wgpu::RenderState, _ui: &mut egui::Ui) -> bool {
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

    fn show(&mut self, render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        let mut reconstruct_render_resource: bool = self.show_top_bar(render_state, ui);
        let paint_callback = self.show_frame(render_state, ui);
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
    Texture { view: TextureView },
    Scene { view: SceneView },
    Error { error: anyhow::Error },
}

impl Views {
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        Self::Scene {
            view: SceneView::new(render_state),
        }
    }

    pub fn reset(&mut self) {
        match self {
            Self::Scene { view } => view.reset(),
            Self::Texture { view } => view.reset(),
            _ => {}
        }
    }

    pub fn recompile_shaders(&mut self, render_state: &egui_wgpu::RenderState) {
        match self {
            Self::Scene { view } => view.recompile_shader(render_state),
            Self::Texture { view } => view.recompile_shader(render_state),
            _ => {}
        }
    }

    pub fn reconstruct_render_resources(&mut self, render_state: &egui_wgpu::RenderState) {
        match self {
            Self::Scene { view } => view.reconstruct_render_resources(render_state),
            Self::Texture { view } => view.reconstruct_render_resources(render_state),
            _ => {}
        }
    }

    pub fn update_if_hash_changed(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
        match self {
            Self::Scene { view } => view.update_if_hash_changed(render_state),
            Self::Texture { view } => view.update_if_hash_changed(render_state),
            _ => false,
        }
    }

    pub fn disable(&mut self) {
        match self {
            Self::Scene { view } => view.disable(),
            Self::Texture { view } => view.disable(),
            _ => {}
        }
    }

    pub fn enable(&mut self) {
        match self {
            Self::Scene { view } => view.enable(),
            Self::Texture { view } => view.enable(),
            _ => {}
        }
    }

    pub fn enabled(&mut self) -> bool {
        match self {
            Self::Scene { view } => view.enabled(),
            Self::Texture { view } => view.enabled(),
            _ => !self.disabled(),
        }
    }

    pub fn disabled(&mut self) -> bool {
        match self {
            Self::Scene { view } => view.disabled(),
            Self::Texture { view } => view.disabled(),
            _ => false,
        }
    }

    pub fn pause(&mut self) {
        match self {
            Self::Scene { view } => view.pause(),
            Self::Texture { view } => view.pause(),
            _ => {}
        }
    }

    pub fn play(&mut self) {
        match self {
            Self::Scene { view } => view.play(),
            Self::Texture { view } => view.play(),
            _ => {}
        }
    }

    pub fn toggle_play_pause(&mut self) {
        match self {
            Self::Scene { view } => view.toggle_play_pause(),
            Self::Texture { view } => view.toggle_play_pause(),
            _ => {}
        }
    }

    pub fn paused(&self) -> bool {
        match self {
            Self::Scene { view } => view.paused(),
            Self::Texture { view } => view.paused(),
            _ => false,
        }
    }

    pub fn enable_and_play(&mut self) {
        match self {
            Self::Scene { view } => view.enable_and_play(),
            Self::Texture { view } => view.enable_and_play(),
            _ => {}
        }
    }

    pub fn show(&mut self, render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        match self {
            Self::Scene { view } => view.show(render_state, ui),
            Self::Texture { view } => view.show(render_state, ui),
            _ => false,
        }
    }
}
