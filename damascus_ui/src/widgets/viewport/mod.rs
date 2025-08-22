// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
};
use glam::UVec2;

use damascus_core::{
    camera::Camera,
    geometry::primitive::Primitive,
    lights::Light,
    materials::{Material, ProceduralTexture},
    render_passes::{
        resources::{BufferData, RenderResource, RenderResources},
        RenderPass, RenderPasses,
    },
    scene::Scene,
};

use crate::{icons::Icons, MAX_TEXTURE_DIMENSION};

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ViewportState {
    pub embedded: bool,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self { embedded: true }
    }
}

struct ViewportCallback {
    buffer_data: Vec<BufferData>,
}

impl ViewportCallback {
    fn render_pass_descriptor(&self, resource: &RenderResource) -> wgpu::RenderPassDescriptor<'_> {
        // wgpu::RenderPassDescriptor {
        //     label: "render to texture",
        //     color_attachments: &'a [Some(wgpu::RenderPassColorAttachment {
        //         view: &'tex TextureView,
        //         depth_slice: None, // TODO support 3d textures
        //         resolve_target: Option<&'tex TextureView>,
        //         ops: Operations<Color>,
        //     })],
        //     depth_stencil_attachment: None, // TODO support depth buffer
        //     timestamp_writes: None, // TODO support timestamp queries
        //     occlusion_query_set: None, // TODO support occlusion culling
        // }
        wgpu::RenderPassDescriptor::default()
    }
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();

        for (data, render_resource) in self.buffer_data.iter().zip(&resources.resources) {
            render_resource.write_bind_groups(queue, data);
        }

        if resources.resources.len() > 0 {
            resources.resources[..resources.resources.len() - 1]
                .iter()
                .for_each(|resource: &RenderResource| {
                    resource.paint(
                        &mut encoder.begin_render_pass(&self.render_pass_descriptor(resource)),
                    );
                });
        }

        vec![]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Viewport {
    pub render_passes: Vec<RenderPasses>,
    pub stats_text: String,
    pub state: ViewportState,
    disabled: bool,
    camera_controls_enabled: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            render_passes: vec![],
            stats_text: String::new(),
            state: ViewportState::default(),
            disabled: true,
            camera_controls_enabled: true,
        }
    }
}

impl Viewport {
    pub const ICON_SIZE: f32 = 25.;

    pub fn new(viewport_state: ViewportState, render_state: &egui_wgpu::RenderState) -> Self {
        let mut view = Self {
            state: viewport_state,
            ..Default::default()
        };
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
        for render_pass in self.render_passes_mut() {
            render_pass.frame_counter_mut().pause()
        }
    }

    pub fn play(&mut self) {
        if self.disabled {
            return;
        }

        for render_pass in self.render_passes_mut() {
            render_pass.frame_counter_mut().play()
        }
    }

    pub fn paused(&self) -> bool {
        if let Some(render_pass) = self.render_passes().first() {
            return render_pass.frame_counter().paused;
        }
        true
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

    pub fn update_render_passes(
        &mut self,
        new_render_passes: Vec<RenderPasses>,
        render_state: &egui_wgpu::RenderState,
    ) {
        if self.render_passes().is_empty() || self.render_passes().len() != new_render_passes.len()
        {
            *self.render_passes_mut() = new_render_passes;
            self.reconstruct_render_resources(render_state);
            return;
        }

        let mut reconstruct = false;
        for (current_passes, new_passes) in
            self.render_passes_mut().iter_mut().zip(new_render_passes)
        {
            if !current_passes.variant_matches(&new_passes) {
                *current_passes = new_passes;
                reconstruct = true;
                continue;
            }

            match (current_passes, new_passes) {
                (
                    RenderPasses::RayMarcher { pass: current_pass },
                    RenderPasses::RayMarcher { pass: mut new_pass },
                ) => {
                    // TODO these wont need to affect the hash once we are rendering
                    // to a fixed size texture
                    new_pass.render_data.scene.render_camera.sensor_resolution = current_pass
                        .render_data
                        .scene
                        .render_camera
                        .sensor_resolution;
                    new_pass.update_hashes();
                    if current_pass.hashes() == new_pass.hashes() {
                        continue;
                    }
                    current_pass.render_data = new_pass.render_data;
                    current_pass.compilation_data = new_pass.compilation_data;
                }
                (
                    RenderPasses::TextureViewer { pass: current_pass },
                    RenderPasses::TextureViewer { pass: new_pass },
                ) => {
                    if current_pass.hashes() == new_pass.hashes() {
                        continue;
                    }
                    current_pass.render_data = new_pass.render_data;
                    current_pass.construction_data = new_pass.construction_data;
                }
                _ => {}
            }
        }
        if reconstruct {
            self.reconstruct_render_resources(render_state);
        }
    }

    pub fn set_final_pass(&mut self, render_pass: RenderPasses) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            *final_pass = render_pass;
        } else {
            self.render_passes_mut().push(render_pass);
        }
    }

    pub fn view_camera(&mut self, camera: Camera, render_state: &egui_wgpu::RenderState) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene.render_camera = camera;
                    pass.render_data.scene.primitives = vec![Primitive::default()];
                }
                _ => {
                    self.update_render_passes(
                        vec![RenderPasses::default_pass_for_camera(camera)],
                        render_state,
                    );
                }
            }
        } else {
            self.update_render_passes(
                vec![RenderPasses::default_pass_for_camera(camera)],
                render_state,
            );
        }
        self.enable_camera_controls();
    }

    pub fn view_lights(&mut self, lights: Vec<Light>, render_state: &egui_wgpu::RenderState) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene.lights = lights;
                    pass.render_data.scene.primitives = vec![Primitive::default()];
                }
                _ => {
                    self.update_render_passes(
                        vec![RenderPasses::default_pass_for_lights(lights)],
                        render_state,
                    );
                }
            }
        } else {
            self.update_render_passes(
                vec![RenderPasses::default_pass_for_lights(lights)],
                render_state,
            );
        }
        self.enable_camera_controls();
    }

    pub fn view_atmosphere(&mut self, atmosphere: Material, render_state: &egui_wgpu::RenderState) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene.clear_primitives();
                    pass.render_data.scene.clear_lights();
                    pass.render_data.scene.atmosphere = atmosphere;
                    self.enable_camera_controls();
                }
                _ => {
                    self.update_render_passes(
                        vec![RenderPasses::default_pass_for_material(atmosphere)],
                        render_state,
                    );
                }
            }
        }
        self.update_render_passes(
            vec![RenderPasses::default_pass_for_material(atmosphere)],
            render_state,
        );
    }

    pub fn view_procedural_texture(
        &mut self,
        texture: ProceduralTexture,
        _render_state: &egui_wgpu::RenderState,
    ) {
        // TODO this can be removed once materials take real textures instead of procedural ones
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene.clear_primitives();
                    pass.render_data.scene.clear_lights();
                    pass.render_data.scene.atmosphere = Material::default();
                    pass.render_data.scene.atmosphere.emissive_intensity = 1.0;
                    pass.render_data.scene.atmosphere.emissive_colour_texture = texture;
                    self.enable_camera_controls();
                }
                _ => {}
            }
        }
    }

    pub fn view_primitives(
        &mut self,
        primitives: Vec<Primitive>,
        render_state: &egui_wgpu::RenderState,
    ) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene.primitives = primitives;
                }
                _ => {
                    self.update_render_passes(
                        vec![RenderPasses::default_pass_for_primitives(primitives)],
                        render_state,
                    );
                }
            }
        } else {
            self.update_render_passes(
                vec![RenderPasses::default_pass_for_primitives(primitives)],
                render_state,
            );
        }
        self.enable_camera_controls();
    }

    pub fn view_scene(&mut self, scene: Scene, render_state: &egui_wgpu::RenderState) {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.reset_render_data();
                    pass.render_data.scene = scene;
                }
                _ => {
                    self.update_render_passes(
                        vec![RenderPasses::default_pass_for_scene(scene)],
                        render_state,
                    );
                }
            }
        } else {
            self.update_render_passes(
                vec![RenderPasses::default_pass_for_scene(scene)],
                render_state,
            );
        }
        self.disable_camera_controls();
    }

    fn update_3d_camera(
        ui: &egui::Ui,
        rect: &egui::Rect,
        response: &egui::Response,
        camera_controls_disabled: bool,
        camera: &mut Camera,
    ) {
        camera.sensor_resolution = UVec2::new(rect.width() as u32, rect.height() as u32);
        if camera_controls_disabled {
            return;
        }
        // Allow some basic camera movement
        let camera_transform = if response.dragged_by(egui::PointerButton::Secondary) {
            glam::Mat4::from_quat(glam::Quat::from_euler(
                glam::EulerRot::XYZ,
                0.0015 * response.drag_delta().y,
                0.0015 * response.drag_delta().x,
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
        camera.camera_to_world *= camera_transform;
    }

    fn update_camera(&mut self, ui: &egui::Ui, rect: &egui::Rect, response: &egui::Response) {
        let camera_controls_disabled = !self.camera_controls_enabled;
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { ref mut pass } => {
                    Self::update_3d_camera(
                        ui,
                        rect,
                        response,
                        camera_controls_disabled,
                        &mut pass.render_data.scene.render_camera,
                    );
                }
                RenderPasses::TextureViewer { pass } => {
                    if camera_controls_disabled {
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
            self.stats_text += " - viewer disabled, activate a node to enable it";
            return None;
        }

        if ui.ctx().memory(|memory| memory.focused().is_none())
            && ui.input(|input| input.key_pressed(egui::Key::Space))
        {
            self.toggle_play_pause();
        }

        self.update_camera(ui, &rect, &response);

        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.scene.render_camera.sensor_resolution = resolution;

                    self.stats_text = format!(
                        "{:} paths per pixel @ {:.2} fps @ ",
                        pass.frame_counter().frame,
                        pass.frame_counter().fps,
                    ) + &self.stats_text;

                    if !self.paused() {
                        ui.ctx().request_repaint();
                    }
                }
                RenderPasses::TextureViewer { pass } => {
                    pass.render_data.resolution = resolution;

                    self.stats_text =
                        format!("{:.2} fps @ ", pass.frame_counter().fps) + &self.stats_text;

                    if !self.paused() {
                        ui.ctx().request_repaint();
                    }
                }
            }
        }

        if let Some(render_resources) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            return Some(egui_wgpu::Callback::new_paint_callback(
                rect,
                ViewportCallback {
                    buffer_data: self
                        .render_passes_mut()
                        .iter_mut()
                        .zip(&mut render_resources.resources)
                        .map(|(render_pass, render_resource)| {
                            let buffer_data: BufferData = render_pass.buffer_data(
                                &render_state.device,
                                render_state.target_format.into(),
                                render_resource,
                            );

                            // TODO these frame counters will not be much different/correct
                            // with multiple passes
                            render_pass.frame_counter_mut().tick();

                            buffer_data
                        })
                        .collect(),
                },
            ));
        }

        None
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
            for (render_pass, render_resource) in self
                .render_passes_mut()
                .iter_mut()
                .zip(&mut render_resources.resources)
            {
                render_pass.recompile_if_preprocessor_directives_changed(
                    &render_state.device,
                    render_state.target_format.into(),
                    render_resource,
                );
            }
        }
    }

    pub fn recompile_shaders(&mut self, render_state: &egui_wgpu::RenderState) {
        if let Some(render_resources) = render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            for (render_pass, render_resource) in self
                .render_passes_mut()
                .iter_mut()
                .zip(&mut render_resources.resources)
            {
                render_pass.recompile_shader(
                    &render_state.device,
                    render_state.target_format.into(),
                    render_resource,
                )
            }
        }
    }

    pub fn reconstruct_render_resources(&mut self, render_state: &egui_wgpu::RenderState) {
        render_state.renderer.write().callback_resources.clear();
        render_state
            .renderer
            .write()
            .callback_resources
            .insert(RenderResources::new(
                self.render_passes_mut()
                    .iter_mut()
                    .map(|render_pass| {
                        render_pass.render_resource(
                            &render_state.device,
                            render_state.target_format.into(),
                        )
                    })
                    .collect(),
            ));
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

    fn show_restart_pause_play_buttons(&mut self, ui: &mut egui::Ui) {
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

    fn show_top_bar(&mut self, ui: &mut egui::Ui) -> bool {
        if let Some(final_pass) = self.render_passes_mut().last_mut() {
            match final_pass {
                RenderPasses::TextureViewer { pass } => {
                    ui.horizontal(|ui| {
                        ui.add(egui::Button::new("f/4").stroke(egui::Stroke::NONE))
                            .on_hover_text("The gain to apply upon display.");
                        ui.add(
                            egui::Slider::new(&mut pass.grade.gain, 0.0..=64.)
                                .clamping(egui::SliderClamping::Never)
                                .logarithmic(true)
                                .smallest_positive(0.01),
                        );
                        ui.add(egui::Button::new("γ").stroke(egui::Stroke::NONE))
                            .on_hover_text("The gamma to apply upon display.");
                        ui.add(
                            egui::Slider::new(&mut pass.grade.gamma, 0.0..=64.)
                                .clamping(egui::SliderClamping::Never)
                                .logarithmic(true)
                                .smallest_positive(0.01),
                        );
                    });
                }
                _ => {}
            }
        }
        false
    }

    fn show_bottom_bar(&mut self, ui: &mut egui::Ui) -> bool {
        self.show_restart_pause_play_buttons(ui);
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
        let mut reconstruct_render_resources = false;
        if self.state.embedded {
            let mut embedded = self.state.embedded;
            egui::Window::new("viewer")
                .open(&mut embedded)
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

                    reconstruct_render_resources |= self.show_top_bar(ui);
                    let paint_callback = self.show_frame(render_state, ui);
                    reconstruct_render_resources |= self.show_bottom_bar(ui);

                    if !reconstruct_render_resources {
                        if let Some(callback) = paint_callback {
                            ui.painter().add(callback);
                        }
                    }
                });
            self.state.embedded = embedded;
            if reconstruct_render_resources {
                self.reconstruct_render_resources(render_state);
            }
        } else {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("damscus viewport"),
                egui::ViewportBuilder::default()
                    .with_title("damscus viewport")
                    .with_inner_size([720., 405.])
                    .with_resizable(true),
                move |ctx, class| {
                    if ctx.input(|i| i.viewport().close_requested())
                        || class != egui::ViewportClass::Immediate
                    {
                        self.state.embedded = true;
                    } else {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            Self::set_button_backgrounds_transparent(ui);

                            reconstruct_render_resources |= self.show_top_bar(ui);
                            let paint_callback = self.show_frame(render_state, ui);
                            reconstruct_render_resources |= self.show_bottom_bar(ui);

                            if !reconstruct_render_resources {
                                if let Some(callback) = paint_callback {
                                    ui.painter().add(callback);
                                }
                            }
                        });
                        if reconstruct_render_resources {
                            self.reconstruct_render_resources(render_state);
                        }
                    }
                },
            );
        }
    }
}
