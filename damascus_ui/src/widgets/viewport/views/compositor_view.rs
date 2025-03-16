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
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use damascus_core::{
    renderers::compositor::{
        Compositor, CompositorRenderState, GPUCompositor, Std430GPUCompositor,
        Std430GPUCompositorRenderState,
    },
    shaders::{
        self,
        compositor::{CompositorCompilerSettings, CompositorPreprocessorDirectives},
    },
    textures::Texture,
    DualDevice,
};

use super::{
    resources::{Buffer, TextureView},
    settings::CompositorViewSettings,
    RenderResources, View,
};

use crate::MAX_TEXTURE_DIMENSION;

struct CompositorViewCallback {
    render_parameters: Std430GPUCompositor,
    render_state: Std430GPUCompositorRenderState,
}

impl egui_wgpu::CallbackTrait for CompositorViewCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &RenderResources = resources.get().unwrap();
        resources.prepare(
            device,
            queue,
            vec![
                bytemuck::cast_slice(&[self.render_parameters]),
                bytemuck::cast_slice(&[self.render_state]),
            ],
            vec![],
        );
        Vec::new()
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

pub struct CompositorView {
    pub renderer: Compositor,
    pub frames_to_update_fps: u32,
    pub stats_text: String,
    disabled: bool,
    camera_controls_enabled: bool,
    render_state: CompositorRenderState,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
    preprocessor_directives: HashSet<CompositorPreprocessorDirectives>,
}

impl Default for CompositorView {
    fn default() -> Self {
        Self {
            renderer: Compositor::default(),
            frames_to_update_fps: 10,
            stats_text: String::new(),
            disabled: true,
            camera_controls_enabled: true,
            render_state: CompositorRenderState::default(),
            recompile_hash: Key::<OrderedFloatPolicy>::Unit,
            reconstruct_hash: Key::<OrderedFloatPolicy>::Unit,
            preprocessor_directives: HashSet::<CompositorPreprocessorDirectives>::new(),
        }
    }
}

impl
    View<
        Compositor,
        GPUCompositor,
        Std430GPUCompositor,
        CompositorCompilerSettings,
        CompositorPreprocessorDirectives,
        CompositorViewSettings,
    > for CompositorView
{
    fn renderer(&self) -> &Compositor {
        &self.renderer
    }

    fn renderer_mut(&mut self) -> &mut Compositor {
        &mut self.renderer
    }

    fn set_recompile_hash(&mut self) -> bool {
        if let Ok(recompile_hash) = to_key_with_ordered_float(self.renderer()) {
            if recompile_hash != self.recompile_hash {
                self.recompile_hash = recompile_hash;
                return true;
            }
        }
        false
    }

    fn set_reconstruct_hash(&mut self, settings: &CompositorViewSettings) -> bool {
        if let Ok(reconstruct_hash) = to_key_with_ordered_float(&settings) {
            if reconstruct_hash != self.reconstruct_hash {
                self.reconstruct_hash = reconstruct_hash;
                return true;
            }
        }
        false
    }

    fn current_preprocessor_directives(&self) -> &HashSet<CompositorPreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_preprocessor_directives_mut(
        &mut self,
    ) -> &mut HashSet<CompositorPreprocessorDirectives> {
        &mut self.preprocessor_directives
    }

    fn get_shader(&self) -> String {
        shaders::compositor::compositor_shader(self.current_preprocessor_directives())
    }

    fn create_uniform_buffers(
        &self,
        device: &wgpu::Device,
        _settings: &CompositorViewSettings,
    ) -> Vec<Buffer> {
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("compositor render parameter buffer"),
                    contents: bytemuck::cast_slice(&[self.renderer().as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("compositor render progress buffer"),
                    contents: bytemuck::cast_slice(&[self.render_state.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
        let mut width: u32 = 10;
        let mut height: u32 = 10;
        let mut texture_data = Rgba32FImage::new(width, height);
        if let Ok(image) = ImageReader::open(&self.renderer().texture.filepath) {
            if let Ok(decoded_image) = image.decode() {
                texture_data = decoded_image.to_rgba32f();
                (width, height) = texture_data.dimensions();
            }
        }

        let size = wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: self.renderer().texture.layers,
        };
        let texture_descriptor = wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("compositor texture"),
            view_formats: &[],
        };
        let texture: wgpu::Texture = device.create_texture(&texture_descriptor);
        let texture_view: wgpu::TextureView = texture.create_view(&Default::default());
        vec![TextureView {
            texture: texture,
            texture_view: texture_view,
            texture_data: texture_data,
            visibility: wgpu::ShaderStages::FRAGMENT,
            view_dimension: wgpu::TextureViewDimension::D2,
            size: size,
        }]
    }

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

    fn reset(&mut self) {}

    fn show_controls(&mut self, render_state: &egui_wgpu::RenderState, ui: &mut egui::Ui) -> bool {
        self.show_restart_pause_play_buttons(render_state, ui);
        ui.add(egui::Label::new(&self.stats_text).truncate());
        false
    }

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        render_state: &egui_wgpu::RenderState,
        available_size: egui::Vec2,
        settings: &CompositorViewSettings,
        compiler_settings: &CompositorCompilerSettings,
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

        // ui.ctx().request_repaint();

        if ui.ctx().memory(|memory| memory.focused().is_none())
            && ui.input(|input| input.key_pressed(egui::Key::Space))
        {
            self.toggle_play_pause();
        }

        self.update_camera(ui, &rect, &response);

        let _data_changed: bool = self.reconstruct_if_hash_changed(render_state, settings)
            || self.recompile_if_hash_changed(render_state, compiler_settings);

        if self.paused() {
            self.render_state.previous_frame_time = SystemTime::now();
            self.render_state.frame_counter = 1;
        } else {
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
            CompositorViewCallback {
                render_parameters: self.renderer().as_std430(),
                render_state: self.render_state.as_std430(),
            },
        ));

        callback
    }
}

impl CompositorView {
    pub fn disable_camera_controls(&mut self) {
        self.camera_controls_enabled = false;
    }

    pub fn enable_camera_controls(&mut self) {
        self.camera_controls_enabled = true;
    }

    pub fn set_texture(
        &mut self,
        texture: Texture,
        render_state: &egui_wgpu::RenderState,
        settings: &CompositorViewSettings,
    ) {
        self.renderer_mut().texture = texture;
        self.reconstruct_pipeline(render_state, settings);
    }

    fn update_camera(&mut self, _ui: &egui::Ui, rect: &egui::Rect, _response: &egui::Response) {
        let _aspect_ratio = rect.width() / rect.height();
        if !self.camera_controls_enabled {
            return;
        }
        // TODO
    }
}
