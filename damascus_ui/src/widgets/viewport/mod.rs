// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::{egui, epaint};

use damascus_core::{
    geometry::{camera::Camera, Primitive},
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture},
    scene::Scene,
};

pub mod render_pipeline;
mod settings;

use render_pipeline::RenderPipeline;
pub use settings::ViewportSettings;

// pub use texture_pipeline::TexturePipeline;
pub use render_pipeline::RenderPipelines;

use crate::{icons::Icons, MAX_TEXTURE_DIMENSION};

pub struct Viewport {
    pub settings: ViewportSettings,
    render_pipeline: RenderPipelines,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: ViewportSettings,
    ) -> Self {
        Self {
            settings: settings,
            render_pipeline: RenderPipelines::new(creation_context, &settings),
        }
    }

    pub fn default_renderer_with_camera(&mut self, camera: Camera) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene.render_camera = camera;
            render_pipeline.renderer.scene.primitives = vec![Primitive::default()];
            render_pipeline.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_lights(&mut self, lights: Vec<Light>) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene.lights = lights;
            render_pipeline.renderer.scene.primitives = vec![Primitive::default()];
            render_pipeline.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_atmosphere(&mut self, atmosphere: Material) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene.clear_primitives();
            render_pipeline.renderer.scene.clear_lights();
            render_pipeline.renderer.scene.atmosphere = atmosphere;
            render_pipeline.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_texture(&mut self, texture: ProceduralTexture) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene.clear_primitives();
            render_pipeline.renderer.scene.clear_lights();
            render_pipeline.renderer.scene.atmosphere = Material::default();
            render_pipeline
                .renderer
                .scene
                .atmosphere
                .diffuse_colour_texture = texture;
            render_pipeline.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_primitives(&mut self, primitives: Vec<Primitive>) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene.primitives = primitives;
            render_pipeline.renderer.scene.lights = vec![Light {
                light_type: Lights::AmbientOcclusion,
                ..Default::default()
            }];
            render_pipeline.enable_camera_controls();
        }
    }

    pub fn default_renderer_with_scene(&mut self, scene: Scene) {
        if let Some(render_pipeline) = &mut self.render_pipeline {
            render_pipeline.renderer.reset_render_parameters();
            render_pipeline.renderer.scene = scene;
            render_pipeline.disable_camera_controls();
        }
    }

    fn controls_height(style: &egui::Style) -> f32 {
        (Self::ICON_SIZE + style.spacing.button_padding.y + style.spacing.item_spacing.y) * 2. + 1.
    }

    // fn show_frame(
    //     &mut self,
    //     frame: &mut eframe::Frame,
    //     ui: &mut egui::Ui,
    // ) -> Option<epaint::PaintCallback> {
    //     let mut paint_callback = None;
    //     egui::Frame::canvas(ui.style()).show(ui, |ui| {
    //         if let Some(render_pipeline) = &mut self.render_pipeline {
    //             let mut available_size: egui::Vec2 = ui.available_size().round();
    //             available_size.y -= Viewport::controls_height(ui.style());
    //             paint_callback =
    //                 render_pipeline.custom_painting(ui, frame, available_size, &self.settings);
    //         }
    //     });
    //     paint_callback
    // }

    fn show_frame(
        &mut self,
        frame: &mut eframe::Frame,
        ui: &mut egui::Ui,
    ) -> Option<epaint::PaintCallback> {
        let mut paint_callback = None;
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            if let Some(render_pipeline) = &mut self.render_pipeline {
                let mut available_size: egui::Vec2 = ui.available_size().round();
                available_size.y -= Viewport::controls_height(ui.style());
                paint_callback =
                    render_pipeline.custom_painting(ui, frame, available_size, &self.settings);
            }
        });
        paint_callback
    }

    // fn show_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
    //     let mut switch_to = false;
    //     if let Some(render_pipeline) = &mut self.render_pipeline {
    //         ui.horizontal(|ui| {
    //             let tooltip: &str;
    //             let pause_icon = egui::Image::new(if render_pipeline.paused() {
    //                 tooltip = "start the render";
    //                 Icons::Play.source()
    //             } else {
    //                 tooltip = "pause the render";
    //                 Icons::Pause.source()
    //             })
    //             .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
    //             if ui
    //                 .add_enabled(render_pipeline.enabled(), egui::ImageButton::new(pause_icon))
    //                 .on_hover_text(tooltip)
    //                 .clicked()
    //             {
    //                 render_pipeline.toggle_play_pause();
    //             }
    //             if ui.button("3D").clicked() {
    //                 switch_to = true;
    //             }
    //         });

    //         ui.add(egui::Label::new(&render_pipeline.stats_text).truncate(true));
    //     }
    //     if switch_to {
    //         self.set_active_state(ViewportActiveState::Viewport3D);
    //     }
    //     switch_to
    // }

    fn show_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
        let mut reconstruct_required = false;
        if let Some(render_pipeline) = &mut self.render_pipeline {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        render_pipeline.enabled(),
                        egui::ImageButton::new(
                            egui::Image::new(Icons::Refresh.source())
                                .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE)),
                        ),
                    )
                    .on_hover_text("restart the render")
                    .clicked()
                {
                    if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                        render_pipeline.recompile_shader(wgpu_render_state);
                    }
                }

                let tooltip: &str;
                let pause_icon = egui::Image::new(if render_pipeline.paused() {
                    tooltip = "start the render";
                    Icons::Play.source()
                } else {
                    tooltip = "pause the render";
                    Icons::Pause.source()
                })
                .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
                if ui
                    .add_enabled(
                        render_pipeline.enabled(),
                        egui::ImageButton::new(pause_icon),
                    )
                    .on_hover_text(tooltip)
                    .clicked()
                {
                    render_pipeline.toggle_play_pause();
                }
                // if ui.button("2D").clicked() {
                //     reconstruct_required = true;
                // }
            });

            ui.add(egui::Label::new(&render_pipeline.stats_text).truncate(true));
        }

        reconstruct_required
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
                Viewport::set_button_backgrounds_transparent(ui);

                let paint_callback = self.show_frame(frame, ui);
                reconstruct_pipeline |= self.show_controls(frame, ui);
                if !reconstruct_pipeline {
                    if let Some(callback) = paint_callback {
                        ui.painter().add(callback);
                    }
                }
            });

        if reconstruct_pipeline {
            self.reconstruct_render_pipeline(frame);
        }
    }
}
