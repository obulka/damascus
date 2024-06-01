// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use eframe::egui::{self, include_image};

use damascus_core::{
    geometry::{camera::Camera, Primitive},
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture},
    renderers::RayMarcher,
    scene::Scene,
};

mod viewport_3d;

pub use viewport_3d::Viewport3d;

use crate::MAX_TEXTURE_DIMENSION;

pub struct Viewport {
    viewport_3d: Option<Viewport3d>,
}

impl Viewport {
    pub fn new<'a>(creation_context: &'a eframe::CreationContext<'a>) -> Self {
        Self {
            viewport_3d: Viewport3d::new(creation_context),
        }
    }

    pub fn enable_and_play(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.enable();
            viewport.play();
        }
    }

    pub fn enable(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.enable();
        }
    }

    pub fn disable(&mut self) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.disable();
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

    pub fn show(&mut self, ctx: &egui::Context) {
        let screen_size: egui::Vec2 = ctx.input(|input| input.screen_rect.size());
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
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        viewport_3d.custom_painting(ui);
                    }
                });
                if let Some(viewport_3d) = &mut self.viewport_3d {
                    let style = ui.style_mut();
                    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                    style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let pause_icon = egui::Image::new(if viewport_3d.paused() {
                            include_image!("../../../assets/icons/play.svg")
                        } else {
                            include_image!("../../../assets/icons/pause.svg")
                        })
                        .fit_to_exact_size(egui::Vec2::splat(20.));
                        if ui
                            .add_enabled(viewport_3d.enabled(), egui::ImageButton::new(pause_icon))
                            .clicked()
                        {
                            viewport_3d.toggle_play_pause();
                        }
                    });

                    ui.add(egui::Label::new(&viewport_3d.stats_text).truncate(true));
                }
            });
    }
}
