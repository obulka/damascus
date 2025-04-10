// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::{egui, egui_wgpu};

mod settings;
pub mod views;

pub use settings::ViewportSettings;
pub use views::Views;

use views::{CompositorView, RayMarcherView, View};

use crate::MAX_TEXTURE_DIMENSION;

pub struct Viewport {
    pub settings: ViewportSettings,
    pub view: Views,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: ViewportSettings,
    ) -> Self {
        if let Some(wgpu_render_state) = &creation_context.wgpu_render_state {
            return Self {
                settings: settings,
                view: Views::new(&wgpu_render_state, &settings),
            };
        }

        Self {
            settings: settings,
            view: Views::Error {
                error: anyhow::Error::msg("Failed to create new viewport"),
            },
        }
    }

    pub fn switch_to_ray_marcher_view(&mut self, render_state: &egui_wgpu::RenderState) {
        if matches!(self.view, Views::RayMarcher { .. }) {
            return;
        }

        let mut view = RayMarcherView::new(
            render_state,
            &self.settings.ray_marcher_view,
            &self.settings.compiler_settings.ray_marcher,
        );
        view.enable_and_play();

        self.view = Views::RayMarcher { view };
    }

    pub fn switch_to_compositor_view(&mut self, render_state: &egui_wgpu::RenderState) {
        if matches!(self.view, Views::Compositor { .. }) {
            return;
        }

        let mut view = CompositorView::new(
            render_state,
            &self.settings.compositor_view,
            &self.settings.compiler_settings.compositor,
        );
        view.enable();

        self.view = Views::Compositor { view };
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
    ) {
        self.view.recompile_if_preprocessor_directives_changed(
            render_state,
            &self.settings.compiler_settings,
        )
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
                reconstruct_render_resource = self.view.show(render_state, ui, &mut self.settings);
            });

        if reconstruct_render_resource {
            self.reconstruct_render_resource(render_state);
        }
    }
}
