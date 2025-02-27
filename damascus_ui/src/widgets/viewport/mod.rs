// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui;

mod settings;
pub mod views;

pub use settings::ViewportSettings;
pub use views::Views;

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
        Self {
            settings: settings,
            view: Views::new(creation_context, &settings),
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(&mut self, frame: &mut eframe::Frame) {
        self.view
            .recompile_if_preprocessor_directives_changed(frame, &self.settings.compiler_settings)
    }

    pub fn reconstruct_pipeline(&mut self, frame: &eframe::Frame) {
        self.view.reconstruct_pipeline(frame, &self.settings);
    }

    fn set_button_backgrounds_transparent(ui: &mut egui::Ui) {
        let style: &mut egui::Style = ui.style_mut();
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
    }

    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let screen_size: egui::Vec2 = ctx.input(|input| input.screen_rect.size());
        let mut reconstruct_pipeline = false;
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
                reconstruct_pipeline = self.view.show(frame, ui, &self.settings);
            });

        if reconstruct_pipeline {
            self.reconstruct_pipeline(frame);
        }
    }
}
