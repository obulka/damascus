// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.
use eframe::{egui, egui_wgpu};

use damascus_core::{
    geometry::{camera::Camera, Primitive},
    lights::{Light, Lights},
    materials::{Material, ProceduralTexture},
    renderers::RayMarcher,
    scene::Scene,
};

mod settings;
mod viewport_3d;

pub use settings::ViewportSettings;
pub use viewport_3d::Viewport3d;

use crate::{icons::Icons, MAX_TEXTURE_DIMENSION};

pub struct Viewport {
    pub settings: ViewportSettings,
    viewport_3d: Option<Viewport3d>,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 20.;

    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: ViewportSettings,
    ) -> Self {
        Self {
            settings: settings,
            viewport_3d: Viewport3d::new(creation_context),
        }
    }

    pub fn reconstruct_render_pipeline(&mut self, wgpu_render_state: &egui_wgpu::RenderState) {
        if let Some(viewport) = &mut self.viewport_3d {
            wgpu_render_state
                .renderer
                .write()
                .callback_resources
                .clear();
            viewport.construct_render_pipeline(wgpu_render_state);
        }
    }

    pub fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState) {
        if let Some(viewport) = &mut self.viewport_3d {
            viewport.recompile_shader(wgpu_render_state);
        }
    }

    pub fn update_preprocessor_directives(&mut self) -> bool {
        if let Some(viewport) = &mut self.viewport_3d {
            return viewport.update_preprocessor_directives(&self.settings);
        }
        false
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

    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
                let style = ui.style_mut();
                style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.active.weak_bg_fill = egui::Color32::TRANSPARENT;

                let controls_height: f32 = (Self::ICON_SIZE
                    + style.spacing.button_padding.y
                    + style.spacing.item_spacing.y)
                    * 2.;

                egui::Frame::canvas(style).show(ui, |ui| {
                    if let Some(viewport_3d) = &mut self.viewport_3d {
                        let mut available_size: egui::Vec2 = ui.available_size().round();
                        available_size.y -= controls_height;
                        viewport_3d.custom_painting(ui, frame, available_size, &self.settings);
                    }
                });
                if let Some(viewport_3d) = &mut self.viewport_3d {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let pause_icon = egui::Image::new(if viewport_3d.paused() {
                            Icons::Play.source()
                        } else {
                            Icons::Pause.source()
                        })
                        .fit_to_exact_size(egui::Vec2::splat(Self::ICON_SIZE));
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
