// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::{egui, egui_wgpu};

pub mod views;

pub use views::Views;
use views::{SceneView, TextureView, View};

use crate::MAX_TEXTURE_DIMENSION;

pub struct Viewport {
    pub view: Views,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new<'a>(creation_context: &'a eframe::CreationContext<'a>) -> Self {
        if let Some(wgpu_render_state) = &creation_context.wgpu_render_state {
            return Self {
                view: Views::new(&wgpu_render_state),
            };
        }

        Self {
            view: Views::Error {
                error: anyhow::Error::msg("Failed to create new viewport"),
            },
        }
    }

    pub fn switch_to_ray_marcher_view(&mut self, render_state: &egui_wgpu::RenderState) {
        if matches!(self.view, Views::Scene { .. }) {
            return;
        }

        let mut view = SceneView::new(render_state);
        view.enable_and_play();

        self.view = Views::Scene { view };
    }

    pub fn switch_to_compositor_view(&mut self, render_state: &egui_wgpu::RenderState) {
        if matches!(self.view, Views::Texture { .. }) {
            return;
        }

        let mut view = TextureView::new(render_state);
        view.enable();

        self.view = Views::Texture { view };
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
    ) {
        self.view
            .recompile_if_preprocessor_directives_changed(render_state)
    }

    pub fn reconstruct_render_resource(&mut self, render_state: &egui_wgpu::RenderState) {
        self.view.reconstruct_render_resource(render_state);
    }

    fn set_button_backgrounds_transparent(ui: &mut egui::Ui) {
        let style: &mut egui::Style = ui.style_mut();
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
    }

    pub fn show(&mut self, ctx: &egui::Context, render_state: &egui_wgpu::RenderState) {
        let screen_size: egui::Vec2 = ctx.input(|input| input.screen_rect.size());
        let mut reconstruct_render_resource = false;
        egui::Window::new("viewer")
            .default_width(720.)
            .default_height(405.)
            .max_width(
                (screen_size.x * 0.9)
                    .round()
                    .min(MAX_TEXTURE_DIMENSION as f32),
            )
            .max_height(
                (screen_size.y * 0.9)
                    .round()
                    .min(MAX_TEXTURE_DIMENSION as f32),
            )
            .resizable(true)
            .movable(true)
            .constrain(true)
            .show(ctx, |ui| {
                Self::set_button_backgrounds_transparent(ui);
                reconstruct_render_resource = self.view.show(render_state, ui);
            });

        if reconstruct_render_resource {
            self.reconstruct_render_resource(render_state);
        }
    }
}
