// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
};

use damascus_core::render_passes::{
    resources::{RenderResource, RenderResources},
    RenderPass, RenderPasses,
};

use crate::{icons::Icons, MAX_TEXTURE_DIMENSION};

struct ViewportCallback {
    buffer_data: Vec<BufferData>,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();

        for data in &self.buffer_data {
            resources.write_bind_groups(queue, data);
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = resources.get().unwrap();
        if let Some(resource) = resources.resources.last() {
            resource.paint(render_pass);
        }
    }
}

pub struct Viewport {
    pub render_passes: Vec<RenderPasses>,
    pub stats_text: String,
    disabled: bool,
    camera_controls_enabled: bool,
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let mut view = Self::default();
        view.reconstruct_render_resources(render_state);
        view
    }

    pub fn render_passes(&self) -> &Vec<RenderPasses> {
        &self.render_passes
    }

    pub fn render_passes_mut(&mut self) -> &mut Vec<RenderPasses> {
        &mut self.render_passes
    }

    pub fn disable(&mut self) {
        self.pause();
        self.disabled = true;
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn disabled(&mut self) -> bool {
        self.disabled
    }

    pub fn pause(&mut self) {
        self.render_state.paused = true;
    }

    pub fn play(&mut self) {
        if !self.disabled {
            self.render_state.paused = false;
        }
    }

    pub fn paused(&self) -> bool {
        self.render_state.paused
    }

    pub fn disable_camera_controls(&mut self) {
        self.camera_controls_enabled = false;
    }

    pub fn enable_camera_controls(&mut self) {
        self.camera_controls_enabled = true;
    }

    pub fn enabled(&mut self) -> bool {
        !self.disabled()
    }

    pub fn toggle_play_pause(&mut self) {
        if self.disabled() {
            return;
        }
        if self.paused() {
            self.play();
        } else {
            self.pause();
        }
    }

    pub fn enable_and_play(&mut self) {
        self.enable();
        self.play();
    }

    pub fn reset(&mut self) {
        self.render_passes_mut()
            .iter_mut()
            .map(|render_pass| render_pass.reset())
            .collect()
    }

    pub fn set_final_pass(&mut self, render_pass: RenderPasses) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            *final_pass = render_pass;
        } else {
            self.render_passes_mut().push(render_pass);
        }
    }

    pub fn view_camera(&mut self, camera: Camera) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene.render_camera = camera;
                    pass.render_data.scene.primitives = vec![Primitive::default()];
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_lights(&mut self, lights: Vec<Light>) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene.lights = lights;
                    pass.render_data.scene.primitives = vec![Primitive::default()];
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_atmosphere(&mut self, atmosphere: Material) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene.clear_primitives();
                    pass.render_data.scene.clear_lights();
                    pass.render_data.scene.atmosphere = atmosphere;
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_procedural_texture(&mut self, texture: ProceduralTexture) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene.clear_primitives();
                    pass.render_data.scene.clear_lights();
                    pass.render_data.scene.atmosphere = Material::default();
                    pass.render_data.scene.atmosphere.diffuse_colour_texture = texture;
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_primitives(&mut self, primitives: Vec<Primitive>) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene.primitives = primitives;
                    pass.render_data.scene.lights = vec![Light {
                        light_type: Lights::AmbientOcclusion,
                        ..Default::default()
                    }];
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_scene(&mut self, scene: Scene) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_parameters();
                    pass.render_data.scene = scene;
                    self.disable_camera_controls();
                }
                _ => {}
            }
        }
    }

    fn update_3d_camera(
        ui: &egui::Ui,
        rect: &egui::Rect,
        response: &egui::Response,
        camera: &mut Camera,
    ) {
        camera.aspect_ratio = rect.width() / rect.height();
        if !self.camera_controls_enabled {
            return;
        }
        // Allow some basic camera movement
        let camera_transform = if response.dragged_by(egui::PointerButton::Secondary) {
            glam::Mat4::from_quat(glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                0.00015 * response.drag_delta().y,
                0.00015 * response.drag_delta().x,
                0.,
            ))
        } else {
            glam::Mat4::from_translation(glam::Vec3::new(
                -0.0015 * response.drag_delta().x,
                0.0015 * response.drag_delta().y,
                if response.hovered() {
                    -0.015 * ui.input(|input| input.smooth_scroll_delta.y)
                } else {
                    0.
                },
            ))
        };
        camera.world_matrix *= camera_transform;
    }

    fn update_camera(&mut self, ui: &egui::Ui, rect: &egui::Rect, response: &egui::Response) {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    Self::update_3d_camera(
                        ui,
                        rect,
                        response,
                        &mut pass.render_data.scene.render_camera,
                    );
                }
                RenderPasses::TextureViewer { pass } => {
                    if !self.camera_controls_enabled {
                        return;
                    }
                    let drag_delta: egui::Vec2 = response.drag_delta();
                    pass.pan += glam::Vec2::new(drag_delta.x, -drag_delta.y) * pass.zoom;
                    if response.hovered() {
                        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                        if scroll_delta != 0.0 {
                            let cursor_pos_egui: egui::Vec2 = ui.ctx().input(|i| {
                                i.pointer.hover_pos().unwrap_or(rect.size().to_pos2() * 0.5)
                                    - rect.min
                            });
                            let cursor_pos = glam::Vec2::new(
                                cursor_pos_egui.x - rect.width() * 0.5,
                                rect.height() * 0.5 - cursor_pos_egui.y,
                            );

                            let hovered_image_pixel_before: glam::Vec2 =
                                cursor_pos * pass.zoom - pass.pan;

                            pass.zoom /= (scroll_delta * 0.002).exp();

                            let hovered_image_pixel: glam::Vec2 = cursor_pos * pass.zoom - pass.pan;

                            pass.pan += hovered_image_pixel - hovered_image_pixel_before;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        render_state: &egui_wgpu::RenderState,
        available_size: egui::Vec2,
    ) -> Option<epaint::PaintCallback> {
        let (rect, response) = ui.allocate_at_least(available_size, egui::Sense::drag());
        let resolution = glam::UVec2::new(rect.width() as u32, rect.height() as u32)
            .min(glam::UVec2::splat(MAX_TEXTURE_DIMENSION));

        self.stats_text = format!("{:.0}x{:.0}", resolution.x, resolution.y);

        if self.disabled {
            serecompile_if_preprocessor_directives_changedlf.stats_text +=
                " - viewer disabled, activate a node to enable it";
            return None;
        }

        if ui.ctx().memory(|memory| memory.focused().is_none())
            && ui.input(|input| input.key_pressed(egui::Key::Space))
        {
            self.toggle_play_pause();
        }

        self.update_camera(ui, &rect, &response);

        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.resolution = resolution;

                    self.stats_text = format!(
                        "{:} paths per pixel @ {:.2} fps @ ",
                        pass.paths_rendered_per_pixel,
                        pass.frame_counter().fps,
                    ) + self.stats_text;

                    ui.ctx().request_repaint();

                    if self.paused() {
                        pass.frame_counter_mut().reset();
                    } else {
                        pass.frame_counter_mut().tick();
                    }
                }
                RenderPasses::TextureViewer { pass } => {
                    pass.render_data.resolution = resolution;

                    self.stats_text =
                        format!("{:.2} fps @ ", pass.frame_counter().fps,) + self.stats_text;

                    if self.paused() {
                        pass.frame_counter_mut().reset();
                    } else {
                        pass.frame_counter_mut().tick();
                        ui.ctx().request_repaint();
                    }
                }
                _ => {}
            }
        }

        Some(egui_wgpu::Callback::new_paint_callback(
            rect,
            ViewportCallback {
                buffer_data: self
                    .render_passes()
                    .iter()
                    .map(|render_pass| {
                        if let Some(render_resource) = render_state
                            .renderer
                            .write()
                            .callback_resources
                            .get_mut::<RenderResources>()
                        {
                            render_pass.update_if_hash_changed(
                                &render_state.device,
                                render_state.target_format.into(),
                                render_resource,
                            );
                        }

                        let buffer_data: BufferData = render_pass.buffer_data();

                        if !self.paused() {
                            match render_pass {
                                RenderPasses::RayMarcher { pass } => {
                                    pass.paths_rendered_per_pixel += 1;
                                }
                                _ => {}
                            }
                        }

                        buffer_data
                    })
                    .collect(),
            },
        ))
    }

    pub fn recompile_if_preprocessor_directives_changed(
        &mut self,
        render_state: &egui_wgpu::RenderState,
    ) {
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
                    render_pass.recompile_if_preprocessor_directives_changed(
                        &render_state.device,
                        render_state.target_format.into(),
                        render_resource,
                    )
                })
                .collect()
        }
    }

    pub fn recompile_shaders(&mut self, render_state: &egui_wgpu::RenderState) {
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

    pub fn reconstruct_render_resources(&mut self, render_state: &egui_wgpu::RenderState) {
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

    pub fn update_if_hash_changed(&mut self, render_state: &egui_wgpu::RenderState) -> bool {
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

    fn show_top_bar(&mut self, _render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::TextureViewer { pass } => {
                    ui.horizontal(|ui| {
                        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
                            match final_pass {
                                RenderPasses::TextureViewer { pass } => {
                                    ui.add(egui::Button::new("f/4").stroke(egui::Stroke::NONE))
                                        .on_hover_text("The gain to apply upon display.");
                                    ui.add(
                                        egui::Slider::new(&mut pass.grade.viewer_gain, 0.0..=64.)
                                            .clamping(egui::SliderClamping::Never)
                                            .logarithmic(true)
                                            .smallest_positive(0.01),
                                    );
                                    ui.add(egui::Button::new("γ").stroke(egui::Stroke::NONE))
                                        .on_hover_text("The gamma to apply upon display.");
                                    ui.add(
                                        egui::Slider::new(&mut pass.grade.viewer_gamma, 0.0..=64.)
                                            .clamping(egui::SliderClamping::Never)
                                            .logarithmic(true)
                                            .smallest_positive(0.01),
                                    );
                                }
                                _ => {}
                            }
                        }
                    });
                }
                _ => {}
            }
        }
        false
    }

    fn show_bottom_bar(
        &mut self,
        render_state: &egui_wgpu::RenderState,
        ui: &mut egui::Ui,
    ) -> bool {
        self.show_restart_pause_play_buttons(render_state, ui);
        ui.add(egui::Label::new(&self.stats_text).truncate());
        false
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

                let mut reconstruct_render_resource: bool = self.show_top_bar(render_state, ui);
                let paint_callback = self.show_frame(render_state, ui);
                reconstruct_render_resource |= self.show_bottom_bar(render_state, ui);

                if !reconstruct_render_resource {
                    if let Some(callback) = paint_callback {
                        ui.painter().add(callback);
                    }
                }
            });

        if reconstruct_render_resource {
            self.reconstruct_render_resource(render_state);
        }
    }
}
