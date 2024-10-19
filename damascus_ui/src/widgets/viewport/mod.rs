// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::{egui, epaint};

use damascus_core::{
    geometry::{camera::Camera, Primitive},
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture},
    renderers::RayMarcher,
    scene::Scene,
};

mod settings;
mod viewport_2d;
mod viewport_3d;

pub use settings::{
    CompilerSettings, PipelineSettings2D, PipelineSettings3D, ViewportActiveState, ViewportSettings,
};

pub use viewport_2d::Viewport2d;
pub use viewport_3d::Viewport3d;

use crate::{icons::Icons, MAX_TEXTURE_DIMENSION};

pub struct Viewport {
    pub settings: ViewportSettings,
    viewport_2d: Option<Viewport2d>,
    viewport_3d: Option<Viewport3d>,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: ViewportSettings,
    ) -> Self {
        Self {
            settings: settings,
            viewport_2d: Viewport2d::new(creation_context, &settings),
            viewport_3d: Viewport3d::new(creation_context, &settings),
        }
    }

    pub fn set_active_state(&mut self, new_state: ViewportActiveState) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => match new_state {
                ViewportActiveState::Viewport3D => {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        if let Some(viewport_2d) = &mut self.viewport_2d {
                            if viewport_3d.enabled() {
                                viewport_2d.enable();
                            } else {
                                viewport_2d.disable();
                            }
                        }
                    }
                }
                ViewportActiveState::SeparateWindows => {}
                _ => {}
            },
            ViewportActiveState::Viewport3D => match new_state {
                ViewportActiveState::Viewport2D => {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        if let Some(viewport_2d) = &mut self.viewport_2d {
                            if viewport_2d.enabled() {
                                viewport_3d.enable();
                            } else {
                                viewport_3d.disable();
                            }
                        }
                    }
                }
                ViewportActiveState::SeparateWindows => {}
                _ => {}
            },
            ViewportActiveState::SeparateWindows => match new_state {
                ViewportActiveState::Viewport2D => {}
                ViewportActiveState::Viewport3D => {}
                _ => {}
            },
        }

        self.settings.active_state = new_state;
    }

    pub fn reconstruct_2d_render_pipeline(&mut self, frame: &eframe::Frame) {
        if let Some(viewport) = &mut self.viewport_2d {
            if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                wgpu_render_state
                    .renderer
                    .write()
                    .callback_resources
                    .clear();
                viewport.construct_render_pipeline(
                    wgpu_render_state,
                    &self.settings.pipeline_settings_2d,
                );
            }
        }
    }

    pub fn reconstruct_3d_render_pipeline(&mut self, frame: &eframe::Frame) {
        if let Some(viewport) = &mut self.viewport_3d {
            if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                wgpu_render_state
                    .renderer
                    .write()
                    .callback_resources
                    .clear();
                viewport.construct_render_pipeline(
                    wgpu_render_state,
                    &self.settings.pipeline_settings_3d,
                );
            }
        }
    }

    pub fn reconstruct_active_render_pipelines(&mut self, frame: &eframe::Frame) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.reconstruct_2d_render_pipeline(frame);
            }
            ViewportActiveState::Viewport3D => {
                self.reconstruct_3d_render_pipeline(frame);
            }
            ViewportActiveState::SeparateWindows => {
                self.reconstruct_2d_render_pipeline(frame);
                self.reconstruct_3d_render_pipeline(frame);
            }
        }
    }

    pub fn recompile_2d_shader(&mut self, frame: &eframe::Frame) {
        if let Some(viewport) = &mut self.viewport_2d {
            if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                viewport.recompile_shader(wgpu_render_state);
            }
        }
    }

    pub fn recompile_3d_shader(&mut self, frame: &eframe::Frame) {
        if let Some(viewport) = &mut self.viewport_3d {
            if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                viewport.recompile_shader(wgpu_render_state);
            }
        }
    }

    pub fn recompile_active_shaders(&mut self, frame: &eframe::Frame) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.recompile_2d_shader(frame);
            }
            ViewportActiveState::Viewport3D => {
                self.recompile_3d_shader(frame);
            }
            ViewportActiveState::SeparateWindows => {
                self.recompile_2d_shader(frame);
                self.recompile_3d_shader(frame);
            }
        }
    }

    pub fn update_2d_preprocessor_directives(&mut self) -> bool {
        if let Some(viewport) = &mut self.viewport_2d {
            return viewport.update_preprocessor_directives(&self.settings.compiler_settings);
        }
        false
    }

    pub fn update_3d_preprocessor_directives(&mut self) -> bool {
        if let Some(viewport) = &mut self.viewport_3d {
            return viewport.update_preprocessor_directives(&self.settings.compiler_settings);
        }
        false
    }

    pub fn update_active_preprocessor_directives(&mut self) -> bool {
        let mut has_changed = false;
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                has_changed |= self.update_2d_preprocessor_directives();
            }
            ViewportActiveState::Viewport3D => {
                has_changed |= self.update_3d_preprocessor_directives();
            }
            ViewportActiveState::SeparateWindows => {
                has_changed |= self.update_2d_preprocessor_directives();
                has_changed |= self.update_3d_preprocessor_directives();
            }
        }
        has_changed
    }

    pub fn recompile_if_2d_preprocessor_directives_changed(&mut self, frame: &mut eframe::Frame) {
        if self.update_2d_preprocessor_directives() {
            self.recompile_2d_shader(frame);
        }
    }

    pub fn recompile_if_3d_preprocessor_directives_changed(&mut self, frame: &mut eframe::Frame) {
        if self.update_2d_preprocessor_directives() {
            self.recompile_3d_shader(frame);
        }
    }

    pub fn recompile_if_active_preprocessor_directives_changed(
        &mut self,
        frame: &mut eframe::Frame,
    ) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.recompile_if_2d_preprocessor_directives_changed(frame);
            }
            ViewportActiveState::Viewport3D => {
                self.recompile_if_3d_preprocessor_directives_changed(frame);
            }
            ViewportActiveState::SeparateWindows => {
                self.recompile_if_2d_preprocessor_directives_changed(frame);
                self.recompile_if_3d_preprocessor_directives_changed(frame);
            }
        }
    }

    pub fn enable_and_play_2d(&mut self) {
        if let Some(viewport) = &mut self.viewport_2d {
            viewport.enable();
            viewport.play();
        }
    }

    pub fn enable_and_play_3d(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.enable();
            viewport.play();
        }
    }

    pub fn enable_and_play_active(&mut self) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.enable_and_play_2d();
            }
            ViewportActiveState::Viewport3D => {
                self.enable_and_play_3d();
            }
            ViewportActiveState::SeparateWindows => {
                self.enable_and_play_2d();
                self.enable_and_play_3d();
            }
        }
    }

    pub fn enable_2d(&mut self) {
        if let Some(viewport) = &mut self.viewport_2d {
            viewport.enable();
        }
    }

    pub fn enable_3d(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.enable();
        }
    }

    pub fn enable_active(&mut self) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.enable_2d();
            }
            ViewportActiveState::Viewport3D => {
                self.enable_3d();
            }
            ViewportActiveState::SeparateWindows => {
                self.enable_2d();
                self.enable_3d();
            }
        }
    }

    pub fn disable_2d(&mut self) {
        if let Some(viewport) = &mut self.viewport_2d {
            viewport.disable();
        }
    }

    pub fn disable_3d(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.disable();
        }
    }

    pub fn disable_active(&mut self) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.disable_2d();
            }
            ViewportActiveState::Viewport3D => {
                self.disable_3d();
            }
            ViewportActiveState::SeparateWindows => {
                self.disable_2d();
                self.disable_3d();
            }
        }
    }

    pub fn default_renderer_with_camera(&mut self, camera: Camera) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene.render_camera = camera;
            viewport.renderer.scene.primitives = vec![Primitive::default()];
            viewport.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_lights(&mut self, lights: Vec<Light>) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene.lights = lights;
            viewport.renderer.scene.primitives = vec![Primitive::default()];
            viewport.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_atmosphere(&mut self, atmosphere: Material) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene.clear_primitives();
            viewport.renderer.scene.clear_lights();
            viewport.renderer.scene.atmosphere = atmosphere;
            viewport.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_texture(&mut self, texture: ProceduralTexture) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene.clear_primitives();
            viewport.renderer.scene.clear_lights();
            viewport.renderer.scene.atmosphere = Material::default();
            viewport.renderer.scene.atmosphere.diffuse_colour_texture = texture;
            viewport.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_primitives(&mut self, primitives: Vec<Primitive>) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene.primitives = primitives;
            viewport.renderer.scene.lights = vec![Light {
                light_type: Lights::AmbientOcclusion,
                ..Default::default()
            }];
            viewport.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_scene(&mut self, scene: Scene) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer.reset_render_parameters();
            viewport.renderer.scene = scene;
            viewport.disable_camera_controls();
        }
    }

    pub fn set_3d_renderer(&mut self, renderer: RayMarcher) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.renderer = renderer;
            viewport.disable_camera_controls();
        }
    }

    pub fn set_2d_renderer(&mut self, renderer: RayMarcher) {
        if let Some(viewport) = &mut self.viewport_2d {
            viewport.renderer = renderer;
            viewport.disable_camera_controls();
        }
    }

    pub fn set_active_renderer(&mut self, renderer: RayMarcher) {
        match self.settings.active_state {
            ViewportActiveState::Viewport2D => {
                self.set_2d_renderer(renderer);
            }
            ViewportActiveState::Viewport3D => {
                self.set_3d_renderer(renderer);
            }
            ViewportActiveState::SeparateWindows => {
                let cloned_renderer = renderer.clone();
                self.set_2d_renderer(renderer);
                self.set_3d_renderer(cloned_renderer);
            }
        }
    }

    fn controls_height(style: &egui::Style) -> f32 {
        (Self::ICON_SIZE + style.spacing.button_padding.y + style.spacing.item_spacing.y) * 2. + 1.
    }

    fn show_2d_frame(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            if let Some(viewport) = &mut self.viewport_2d {
                let mut available_size: egui::Vec2 = ui.available_size().round();
                available_size.y -= Viewport::controls_height(ui.style());
                paint_callback =
                    viewport.custom_painting(ui, frame, available_size, &self.settings);
            }
        });
        paint_callback
    }

    fn show_3d_frame(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            if let Some(viewport) = &mut self.viewport_3d {
                let mut available_size: egui::Vec2 = ui.available_size().round();
                available_size.y -= Viewport::controls_height(ui.style());
                paint_callback =
                    viewport.custom_painting(ui, frame, available_size, &self.settings);
            }
        });
        paint_callback
    }

    fn show_2d_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
        let mut switch_to_3d = false;
        if let Some(viewport) = &mut self.viewport_2d {
            ui.horizontal(|ui| {
                let tooltip: &str;
                let pause_icon = egui::Image::new(if viewport.paused() {
                    tooltip = "start the render";
                    Icons::Play.source()
                } else {
                    tooltip = "pause the render";
                    Icons::Pause.source()
                })
                .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
                if ui
                    .add_enabled(viewport.enabled(), egui::ImageButton::new(pause_icon))
                    .on_hover_text(tooltip)
                    .clicked()
                {
                    viewport.toggle_play_pause();
                }
                if ui.button("3D").clicked() {
                    switch_to_3d = true;
                }
            });

            ui.add(egui::Label::new(&viewport.stats_text).truncate(true));
        }
        if switch_to_3d {
            self.set_active_state(ViewportActiveState::Viewport3D);
        }
        switch_to_3d
    }

    fn show_3d_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
        let mut switch_to_2d = false;
        if let Some(viewport) = &mut self.viewport_3d {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        viewport.enabled(),
                        egui::ImageButton::new(
                            egui::Image::new(Icons::Refresh.source())
                                .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE)),
                        ),
                    )
                    .on_hover_text("restart the render")
                    .clicked()
                {
                    if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                        viewport.recompile_shader(wgpu_render_state);
                    }
                }

                let tooltip: &str;
                let pause_icon = egui::Image::new(if viewport.paused() {
                    tooltip = "start the render";
                    Icons::Play.source()
                } else {
                    tooltip = "pause the render";
                    Icons::Pause.source()
                })
                .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
                if ui
                    .add_enabled(viewport.enabled(), egui::ImageButton::new(pause_icon))
                    .on_hover_text(tooltip)
                    .clicked()
                {
                    viewport.toggle_play_pause();
                }
                if ui.button("2D").clicked() {
                    switch_to_2d = true;
                }
            });

            ui.add(egui::Label::new(&viewport.stats_text).truncate(true));
        }
        if switch_to_2d {
            self.set_active_state(ViewportActiveState::Viewport2D);
        }
        switch_to_2d
    }

    fn set_button_backgrounds_transparent(ui: &mut egui::Ui) {
        let style: &mut egui::Style = ui.style_mut();
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
    }

    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let screen_size: egui::Vec2 = ctx.input(|input| input.screen_rect.size());
        let mut reconstruct_active_pipeline = false;
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
                Viewport::set_button_backgrounds_transparent(ui);

                let mut paint_callback = None;
                match self.settings.active_state {
                    ViewportActiveState::Viewport2D => {
                        paint_callback = self.show_2d_frame(frame, ui);
                        reconstruct_active_pipeline |= self.show_2d_controls(frame, ui);
                    }
                    ViewportActiveState::Viewport3D => {
                        paint_callback = self.show_3d_frame(frame, ui);
                        reconstruct_active_pipeline |= self.show_3d_controls(frame, ui);
                    }
                    _ => {}
                }
                if !reconstruct_active_pipeline {
                    if let Some(callback) = paint_callback {
                        ui.painter().add(callback);
                    }
                }
            });

        if reconstruct_active_pipeline {
            self.reconstruct_active_render_pipelines(frame);
        }
    }
}
