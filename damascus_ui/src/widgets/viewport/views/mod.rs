// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use eframe::{egui, egui_wgpu, epaint};

use damascus_core::{renderers::Renderer, shaders::ray_marcher::RayMarcherCompilerSettings};

use super::settings::{RayMarcherViewSettings, ViewportSettings};

use crate::icons::Icons;

mod ray_marcher_view;

pub use ray_marcher_view::RayMarcherView;

pub trait View<R: Renderer<G, S>, G: Copy + Clone + AsStd430<Output = S>, S>: Default {
    const ICON_SIZE: f32 = 25.;

    fn renderer(&self) -> &R;

    fn renderer_mut(&mut self) -> &mut R;

    fn set_recompile_hash(&mut self);

    fn set_reconstruct_hash(&mut self, settings: &ViewportSettings);

    /// Create an instance of this render pipeline
    fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Option<Self> {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash();
        pipeline.set_reconstruct_hash(settings);

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        Self::construct_pipeline(
            &mut pipeline,
            creation_context.wgpu_render_state.as_ref()?,
            settings,
        );

        Some(pipeline)
    }

    /// Construict all uniform/storage/texture buffers and RenderResources
    fn construct_pipeline(
        &mut self,
        _wgpu_render_state: &egui_wgpu::RenderState,
        _settings: &ViewportSettings,
    );

    /// Construict all uniform/storage/texture buffers and RenderResources
    fn reconstruct_pipeline(
        &mut self,
        wgpu_render_state: &egui_wgpu::RenderState,
        settings: &ViewportSettings,
    ) {
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .clear();
        self.reset();
        self.construct_pipeline(wgpu_render_state, settings);
    }

    fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState);

    fn update_preprocessor_directives(&mut self, settings: &RayMarcherCompilerSettings) -> bool;

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
        settings: &ViewportSettings,
    ) -> Option<epaint::PaintCallback>;

    fn show_frame(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
        settings: &ViewportSettings,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let mut available_size: egui::Vec2 = ui.available_size().round();
            available_size.y -= Self::controls_height(ui.style());
            paint_callback = self.custom_painting(ui, frame, available_size, settings);
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
        settings: &ViewportSettings,
    ) -> bool {
        let paint_callback = self.show_frame(frame, ui, settings);
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
    Error,
}

impl Views {
    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Self {
        if let Some(view) = RayMarcherView::new(creation_context, settings) {
            return Self::RayMarcher { view: view };
        }
        Self::Error
    }

    pub fn reconstruct_pipeline(&mut self, frame: &eframe::Frame, settings: &ViewportSettings) {
        if let Some(wgpu_render_state) = frame.wgpu_render_state() {
            match self {
                Self::RayMarcher { view } => view.reconstruct_pipeline(wgpu_render_state, settings),
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
        settings: &RayMarcherCompilerSettings,
    ) -> bool {
        match self {
            Self::RayMarcher { view } => view.update_preprocessor_directives(settings),
            _ => false,
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        frame: &mut eframe::Frame,
        settings: &RayMarcherCompilerSettings,
    ) {
        if self.update_preprocessor_directives(settings) {
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
            Self::RayMarcher { view } => view.show(frame, ui, settings),
            _ => false,
        }
    }
}
