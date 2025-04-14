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
use serde_hashkey::{Key, OrderedFloatPolicy};

use damascus_core::{
    geometry::{
        camera::{Camera, Std430GPUCamera},
        primitive::{Primitive, Std430GPUPrimitive},
    },
    lights::{Light, Lights, Std430GPULight},
    materials::{Material, ProceduralTexture, Std430GPUMaterial},
    render_passes::{
        ray_marcher::{
            GPURayMarcher, RayMarcher, RayMarcherRenderState, Std430GPURayMarcher,
            Std430GPURayMarcherRenderState,
        },
        resources::RenderResources,
    },
    scene::{Scene, Std430GPUSceneParameters},
    shaders::{
        self,
        ray_marcher::{RayMarcherCompilerSettings, RayMarcherPreprocessorDirectives},
    },
    DualDevice,
};

use super::{settings::SceneViewSettings, RenderResources, View};

use crate::MAX_TEXTURE_DIMENSION;

struct SceneViewCallback<'a> {
    buffer_data: Vec<BufferData<'a>>,
}

impl egui_wgpu::CallbackTrait for SceneViewCallback<'a> {
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
            resources.write_bind_groups(queue, data.clone());
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

pub struct SceneView {
    pub render_passes: Vec<RenderPasses>,
    pub stats_text: String,
    disabled: bool,
    camera_controls_enabled: bool,
}

impl Default for SceneView {
    fn default() -> Self {
        Self {
            render_passes: vec![RenderPasses::RayMarcher {
                pass: RayMarcher::new(),
            }],
            stats_text: String::new(),
            disabled: true,
            camera_controls_enabled: true,
        }
    }
}

impl View for SceneView {
    fn render_passes(&self) -> &Vec<RenderPasses> {
        &self.render_passes
    }

    fn render_passes_mut(&mut self) -> &mut Vec<RenderPasses> {
        &mut self.render_passes
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

        if let Some(final_pass) = (*self.render_passes_mut()).last_mut() {
            match final_pass {
                RenderPasses::RayMarcher { pass } => {
                    pass.render_data.resolution =
                        glam::UVec2::new(rect.width() as u32, rect.height() as u32)
                            .min(glam::UVec2::splat(MAX_TEXTURE_DIMENSION));

                    self.stats_text = format!(
                        "{:} paths per pixel @ {:.2} fps @ {:.0}x{:.0}",
                        pass.paths_rendered_per_pixel,
                        pass.frame_counter().fps,
                        rect.max.x - rect.min.x,
                        rect.max.y - rect.min.y
                    );

                    if self.disabled {
                        self.stats_text += " - viewer disabled, activate a node to enable it";
                        return None;
                    }

                    ui.ctx().request_repaint();

                    if ui.ctx().memory(|memory| memory.focused().is_none())
                        && ui.input(|input| input.key_pressed(egui::Key::Space))
                    {
                        self.toggle_play_pause();
                    }

                    self.update_camera(ui, &rect, &response);

                    if let Some(render_resource) = render_state
                        .renderer
                        .write()
                        .callback_resources
                        .get_mut::<RenderResources>()
                    {
                        let _data_changed: bool = self.update_if_hash_changed(render_state);
                    }

                    let mut paths_rendered: u32 = 0;
                    if self.paused() {
                        pass.frame_counter_mut().reset();
                    } else {
                        pass.frame_counter_mut().tick();
                        paths_rendered = 1;
                    }
                }
                _ => {}
            }
        }

        let callback = Some(egui_wgpu::Callback::new_paint_callback(
            rect,
            SceneViewCallback {
                render_parameters: self.renderer().as_std430(),
                scene_parameters: self
                    .renderer()
                    .scene
                    .scene_parameters(settings.max_primitives, settings.max_lights),
                render_state: self.render_state.as_std430(),
                render_camera: self.renderer().scene.render_camera.as_std430(),
                primitives: self
                    .renderer
                    .scene
                    .create_gpu_primitives(settings.max_primitives),
                lights: self.renderer.scene.create_gpu_lights(settings.max_lights),
                atmosphere: self.renderer().scene.atmosphere(),
                emissive_primitive_indices: self
                    .renderer
                    .scene
                    .emissive_primitive_indices(settings.max_primitives),
            },
        ));

        self.render_state.paths_rendered_per_pixel += paths_rendered;

        callback
    }
}

impl SceneView {
    pub fn disable_camera_controls(&mut self) {
        self.camera_controls_enabled = false;
    }

    pub fn enable_camera_controls(&mut self) {
        self.camera_controls_enabled = true;
    }

    pub fn set_renderer_to_default_with_camera(&mut self, camera: Camera) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene.render_camera = camera;
        self.renderer_mut().scene.primitives = vec![Primitive::default()];
        self.enable_camera_controls();
    }

    pub fn set_renderer_to_default_with_lights(&mut self, lights: Vec<Light>) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene.lights = lights;
        self.renderer_mut().scene.primitives = vec![Primitive::default()];
        self.enable_camera_controls();
    }

    pub fn set_renderer_to_default_with_atmosphere(&mut self, atmosphere: Material) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene.clear_primitives();
        self.renderer_mut().scene.clear_lights();
        self.renderer_mut().scene.atmosphere = atmosphere;
        self.enable_camera_controls();
    }

    pub fn set_renderer_to_default_with_procedural_texture(&mut self, texture: ProceduralTexture) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene.clear_primitives();
        self.renderer_mut().scene.clear_lights();
        self.renderer_mut().scene.atmosphere = Material::default();
        self.renderer_mut().scene.atmosphere.diffuse_colour_texture = texture;
        self.enable_camera_controls();
    }

    pub fn set_renderer_to_default_with_primitives(&mut self, primitives: Vec<Primitive>) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene.primitives = primitives;
        self.renderer_mut().scene.lights = vec![Light {
            light_type: Lights::AmbientOcclusion,
            ..Default::default()
        }];
        self.enable_camera_controls();
    }

    pub fn set_renderer_to_default_with_scene(&mut self, scene: Scene) {
        self.renderer_mut().reset_render_parameters();
        self.renderer_mut().scene = scene;
        self.disable_camera_controls();
    }

    pub fn set_renderer(&mut self, renderer: RayMarcher) {
        *self.renderer_mut() = renderer;
        self.disable_camera_controls();
    }

    fn update_camera(&mut self, ui: &egui::Ui, rect: &egui::Rect, response: &egui::Response) {
        self.renderer_mut().scene.render_camera.aspect_ratio = rect.width() / rect.height();
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
        self.renderer_mut().scene.render_camera.world_matrix *= camera_transform;
    }
}
