// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.
// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, time::SystemTime};

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
    wgpu::util::DeviceExt,
};
use glam;
use image::{ImageReader, Rgba32FImage};
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result};

use damascus_core::{
    render_passes::{
        resources::RenderResources,
        texture_viewer::{
            GPUTextureViewer, Std430GPUTextureViewer, Std430GPUTextureViewerRenderState,
            TextureViewer, TextureViewerRenderState,
        },
    },
    shaders::{self, texture_viewer::TextureViewerPreprocessorDirectives},
    textures::{Grade, Texture},
    DualDevice,
};

use super::View;

use crate::MAX_TEXTURE_DIMENSION;

struct TextureViewCallback {
    render_parameters: Std430GPUTextureViewer,
    render_state: Std430GPUTextureViewerRenderState,
}

impl egui_wgpu::CallbackTrait for TextureViewCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();
        resources.write_bind_groups(
            queue,
            vec![BufferData {
                uniform: vec![
                    bytemuck::cast_slice(&[self.render_parameters]),
                    bytemuck::cast_slice(&[self.render_state]),
                ],
                storage: vec![],
            }],
        );

        vec![]
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = resources.get().unwrap();
        resources.paint(render_pass);
    }
}

pub struct TextureView {
    pub render_passes: Vec<RenderPasses>,
    pub frames_to_update_fps: u32,
    pub stats_text: String,
    final_pass: RenderPasses,
    disabled: bool,
    camera_controls_enabled: bool,
}

impl Default for TextureView {
    fn default() -> Self {
        Self {
            render_passes: Vec::<RenderPasses>::new(),
            frames_to_update_fps: 10,
            stats_text: String::new(),
            final_pass: RenderPasses::TextureViewer {
                pass: TextureViewer::new(),
            },
            disabled: true,
            camera_controls_enabled: true,
        }
    }
}

impl View for TextureView {
    fn disable(&mut self) {
        self.pause();
        self.disabled = true;
    }

    fn enable(&mut self) {
        self.disabled = false;
    }

    fn disabled(&mut self) -> bool {
        self.disabled
    }

    fn pause(&mut self) {
        self.render_state.paused = true;
    }

    fn play(&mut self) {
        if !self.disabled {
            self.render_state.paused = false;
        }
    }

    fn paused(&self) -> bool {
        self.render_state.paused
    }

    fn show_top_bar(&mut self, _render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        ui.horizontal(|ui| {
            ui.add(egui::Button::new("f/4").stroke(egui::Stroke::NONE))
                .on_hover_text("The gain to apply upon display.");
            ui.add(
                egui::Slider::new(&mut settings.viewer_gain, 0.0..=64.)
                    .clamping(egui::SliderClamping::Never)
                    .logarithmic(true)
                    .smallest_positive(0.01),
            );
            ui.add(egui::Button::new("γ").stroke(egui::Stroke::NONE))
                .on_hover_text("The gamma to apply upon display.");
            ui.add(
                egui::Slider::new(&mut settings.viewer_gamma, 0.0..=64.)
                    .clamping(egui::SliderClamping::Never)
                    .logarithmic(true)
                    .smallest_positive(0.01),
            );
        });
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

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        render_state: &egui_wgpu::RenderState,
        available_size: egui::Vec2,
    ) -> Option<epaint::PaintCallback> {
        let (rect, response) = ui.allocate_at_least(available_size, egui::Sense::drag());

        self.render_state.resolution = glam::UVec2::new(rect.width() as u32, rect.height() as u32)
            .min(glam::UVec2::splat(MAX_TEXTURE_DIMENSION));

        self.stats_text = format!(
            "{:.2} fps @ {:.0}x{:.0}",
            self.render_state.fps,
            rect.max.x - rect.min.x,
            rect.max.y - rect.min.y
        );

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

        let _data_changed: bool = self.update_if_hash_changed(render_state);

        if self.paused() {
            self.render_state.previous_frame_time = SystemTime::now();
            self.render_state.frame_counter = 1;
        } else {
            ui.ctx().request_repaint();

            if self.render_state.frame_counter % self.frames_to_update_fps == 0 {
                match SystemTime::now().duration_since(self.render_state.previous_frame_time) {
                    Ok(frame_time) => {
                        self.render_state.fps =
                            self.frames_to_update_fps as f32 / frame_time.as_secs_f32();
                    }
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                }

                self.render_state.previous_frame_time = SystemTime::now();
                self.render_state.frame_counter = 1;
            } else {
                self.render_state.frame_counter += 1;
            }
        }

        let callback = Some(egui_wgpu::Callback::new_paint_callback(
            rect,
            TextureViewCallback {
                render_parameters: self.renderer().as_std430(),
                render_state: self.render_state.as_std430(),
            },
        ));

        callback
    }
}

impl TextureView {
    pub fn disable_camera_controls(&mut self) {
        self.camera_controls_enabled = false;
    }

    pub fn enable_camera_controls(&mut self) {
        self.camera_controls_enabled = true;
    }

    pub fn set_texture(&mut self, texture: Texture, render_state: &egui_wgpu::RenderState) {
        self.renderer_mut().texture = texture;
        self.reconstruct_pipeline(render_state);
    }

    fn update_camera(&mut self, ui: &egui::Ui, rect: &egui::Rect, response: &egui::Response) {
        if !self.camera_controls_enabled {
            return;
        }
        let drag_delta: egui::Vec2 = response.drag_delta();
        self.render_state.pan +=
            glam::Vec2::new(drag_delta.x, -drag_delta.y) * self.render_state.zoom;
        if response.hovered() {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                let cursor_pos_egui: egui::Vec2 = ui.ctx().input(|i| {
                    i.pointer.hover_pos().unwrap_or(rect.size().to_pos2() * 0.5) - rect.min
                });
                let cursor_pos = glam::Vec2::new(
                    cursor_pos_egui.x - rect.width() * 0.5,
                    rect.height() * 0.5 - cursor_pos_egui.y,
                );

                let hovered_image_pixel_before: glam::Vec2 =
                    cursor_pos * self.render_state.zoom - self.render_state.pan;

                self.render_state.zoom /= (scroll_delta * 0.002).exp();

                let hovered_image_pixel: glam::Vec2 =
                    cursor_pos * self.render_state.zoom - self.render_state.pan;

                self.render_state.pan += hovered_image_pixel - hovered_image_pixel_before;
            }
        }
    }
}
