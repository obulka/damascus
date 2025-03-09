// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{borrow::Cow, collections::HashSet, ops::BitOr, sync::Arc, time::SystemTime};

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
    wgpu::util::DeviceExt,
};
use glam;
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use damascus_core::{
    geometry::{
        camera::{Camera, Std430GPUCamera},
        primitive::{Primitive, Std430GPUPrimitive},
    },
    lights::{Light, Lights, Std430GPULight},
    materials::{Material, ProceduralTexture, Std430GPUMaterial},
    renderers::ray_marcher::{
        GPURayMarcher, RayMarcher, RenderState, Std430GPURayMarcher, Std430GPURenderState,
    },
    scene::{Scene, Std430GPUSceneParameters},
    shaders::{
        self,
        ray_marcher::{RayMarcherCompilerSettings, RayMarcherPreprocessorDirectives},
    },
    DualDevice,
};

use super::{
    binding_resources::{Buffer, StorageTextureView},
    RayMarcherViewSettings, View,
};

use crate::MAX_TEXTURE_DIMENSION;

pub struct RayMarcherView {
    pub renderer: RayMarcher,
    pub frames_to_update_fps: u32,
    pub stats_text: String,
    disabled: bool,
    camera_controls_enabled: bool,
    render_state: RenderState,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
}

impl Default for RayMarcherView {
    fn default() -> Self {
        Self {
            renderer: RayMarcher::default(),
            frames_to_update_fps: 10,
            stats_text: String::new(),
            disabled: true,
            camera_controls_enabled: true,
            render_state: RenderState::default(),
            recompile_hash: Key::<OrderedFloatPolicy>::Unit,
            reconstruct_hash: Key::<OrderedFloatPolicy>::Unit,
        }
    }
}

impl
    View<
        RayMarcher,
        GPURayMarcher,
        Std430GPURayMarcher,
        RayMarcherCompilerSettings,
        RayMarcherPreprocessorDirectives,
        RayMarcherViewSettings,
    > for RayMarcherView
{
    fn renderer(&self) -> &RayMarcher {
        &self.renderer
    }

    fn renderer_mut(&mut self) -> &mut RayMarcher {
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

    fn set_reconstruct_hash(&mut self, settings: &RayMarcherViewSettings) -> bool {
        if let Ok(reconstruct_hash) = to_key_with_ordered_float(&settings) {
            if reconstruct_hash != self.reconstruct_hash {
                self.reconstruct_hash = reconstruct_hash;
                return true;
            }
        }
        false
    }

    /// Construict all uniform/storage/texture buffers and RenderResources
    fn construct_pipeline(
        &mut self,
        wgpu_render_state: &egui_wgpu::RenderState,
        settings: &RayMarcherViewSettings,
    ) {
        let device = &wgpu_render_state.device;

        // Uniforms
        let uniform_buffers: Vec<Buffer> = self.create_uniform_buffers(device, &settings);
        let (uniform_bind_group_layout, uniform_bind_group) =
            Self::create_uniform_binding(device, &uniform_buffers);

        // Storage
        let storage_buffers: Vec<Buffer> = self.create_storage_buffers(device, &settings);
        let (storage_bind_group_layout, storage_bind_group) =
            Self::create_storage_binding(device, &storage_buffers);

        // Create the texture to render to and initialize from
        let storage_texture_views: Vec<StorageTextureView> =
            Self::create_storage_texture_views(device);
        let (storage_texture_bind_group_layout, storage_texture_bind_group) =
            Self::create_storage_texture_binding(device, &storage_texture_views);

        // Create the pipeline
        let render_pipeline = self.create_render_pipeline(
            device,
            wgpu_render_state.target_format,
            &uniform_bind_group_layout,
            &storage_bind_group_layout,
            &storage_texture_bind_group_layout,
        );

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Viewport3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(RenderResources {
                render_pipeline,
                uniform_bind_group,
                uniform_bind_group_layout,
                uniform_buffers,
                storage_bind_group,
                storage_bind_group_layout,
                storage_buffers,
                storage_texture_bind_group,
                storage_texture_bind_group_layout,
            });
    }

    fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState) {
        if let Some(render_resources) = wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            self.reset();

            let device = &wgpu_render_state.device;

            // Create the updated pipeline
            render_resources.render_pipeline = self.create_render_pipeline(
                device,
                wgpu_render_state.target_format,
                &render_resources.uniform_bind_group_layout,
                &render_resources.storage_bind_group_layout,
                &render_resources.storage_texture_bind_group_layout,
            );
        }
    }

    fn current_preprocessor_directives(
        &mut self,
    ) -> &mut HashSet<RayMarcherPreprocessorDirectives> {
        &mut self.render_state.preprocessor_directives
    }

    fn create_uniform_buffers(
        &self,
        device: &Arc<wgpu::Device>,
        settings: &RayMarcherViewSettings,
    ) -> Vec<Buffer> {
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render parameter buffer"),
                    contents: bytemuck::cast_slice(&[self.renderer().as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher scene parameter buffer"),
                    contents: bytemuck::cast_slice(&[self
                        .renderer()
                        .scene
                        .scene_parameters(settings.max_primitives, settings.max_lights)]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render progress buffer"),
                    contents: bytemuck::cast_slice(&[self.render_state.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher camera buffer"),
                    contents: bytemuck::cast_slice(&[self
                        .renderer()
                        .scene
                        .render_camera
                        .as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT.bitor(wgpu::ShaderStages::VERTEX),
            },
        ]
    }

    fn create_storage_buffers(
        &self,
        device: &Arc<wgpu::Device>,
        settings: &RayMarcherViewSettings,
    ) -> Vec<Buffer> {
        let primitives: Vec<Std430GPUPrimitive> = self
            .renderer
            .scene
            .create_gpu_primitives(settings.max_primitives);
        let lights: Vec<Std430GPULight> =
            self.renderer.scene.create_gpu_lights(settings.max_lights);
        let emissive_primitive_indices: Vec<u32> = self
            .renderer
            .scene
            .emissive_primitive_indices(settings.max_primitives);
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher primitives buffer"),
                    contents: &[
                        bytemuck::cast_slice(primitives.as_slice()),
                        vec![
                            0;
                            (settings.max_primitives - primitives.len())
                                * size_of::<Std430GPUPrimitive>()
                        ]
                        .as_slice(),
                    ]
                    .concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher lights buffer"),
                    contents: &[
                        bytemuck::cast_slice(lights.as_slice()),
                        vec![0; (settings.max_lights - lights.len()) * size_of::<Std430GPULight>()]
                            .as_slice(),
                    ]
                    .concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher render globals buffer"),
                    contents: bytemuck::cast_slice(&[self.renderer().scene.atmosphere()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("ray marcher emissive primitive ids"),
                    contents: &[
                        bytemuck::cast_slice(emissive_primitive_indices.as_slice()),
                        vec![
                            0;
                            (settings.max_primitives - emissive_primitive_indices.len())
                                * size_of::<u32>()
                        ]
                        .as_slice(),
                    ]
                    .concat(),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_storage_texture_views(device: &Arc<wgpu::Device>) -> Vec<StorageTextureView> {
        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: MAX_TEXTURE_DIMENSION,
                height: MAX_TEXTURE_DIMENSION,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("ray marcher progressive rendering texture"),
            view_formats: &[],
        };

        vec![StorageTextureView {
            texture_view: device
                .create_texture(&texture_descriptor)
                .create_view(&Default::default()),
            visibility: wgpu::ShaderStages::FRAGMENT,
            access: wgpu::StorageTextureAccess::ReadWrite,
            format: texture_descriptor.format,
            view_dimension: wgpu::TextureViewDimension::D2,
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

    fn reset(&mut self) {
        self.render_state.paths_rendered_per_pixel = 0;
    }

    fn show_controls(&mut self, frame: &mut eframe::Frame, ui: &mut egui::Ui) -> bool {
        self.show_restart_pause_play_buttons(frame, ui);
        ui.add(egui::Label::new(&self.stats_text).truncate(true));
        false
    }

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        available_size: egui::Vec2,
        settings: &RayMarcherViewSettings,
        compiler_settings: &RayMarcherCompilerSettings,
    ) -> Option<epaint::PaintCallback> {
        let (rect, response) = ui.allocate_at_least(available_size, egui::Sense::drag());

        self.render_state.resolution = glam::UVec2::new(rect.width() as u32, rect.height() as u32)
            .min(glam::UVec2::splat(MAX_TEXTURE_DIMENSION));

        self.stats_text = format!(
            "{:} paths per pixel @ {:.2} fps @ {:.0}x{:.0}",
            self.render_state.paths_rendered_per_pixel,
            self.render_state.fps,
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

        let _data_changed: bool = self.reconstruct_if_hash_changed(frame, settings)
            || self.recompile_if_hash_changed(frame, compiler_settings);

        let mut paths_rendered: u32 = 0;

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

            paths_rendered = 1;
        }

        let callback = Some(egui_wgpu::Callback::new_paint_callback(
            rect,
            RayMarcherViewCallback {
                render_parameters: self.renderer().as_std430(),
                scene_parameters: self
                    .renderer()
                    .scene
                    .scene_parameters(settings.max_primitives, settings.max_lights),
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
                render_state: self.render_state.as_std430(),
            },
        ));

        self.render_state.paths_rendered_per_pixel += paths_rendered;

        callback
    }
}

impl RayMarcherView {
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

    pub fn set_renderer_to_default_with_texture(&mut self, texture: ProceduralTexture) {
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

    fn create_render_pipeline(
        &self,
        device: &Arc<wgpu::Device>,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        storage_bind_group_layout: &wgpu::BindGroupLayout,
        storage_texture_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ray marcher pipeline layout"),
            bind_group_layouts: &[
                uniform_bind_group_layout,
                storage_bind_group_layout,
                storage_texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ray marcher source shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                &shaders::ray_marcher::ray_march_shader(&self.render_state.preprocessor_directives),
            ))
            .into(),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ray marcher render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(texture_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
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

struct RayMarcherViewCallback {
    render_parameters: Std430GPURayMarcher,
    scene_parameters: Std430GPUSceneParameters,
    render_state: Std430GPURenderState,
    render_camera: Std430GPUCamera,
    primitives: Vec<Std430GPUPrimitive>,
    lights: Vec<Std430GPULight>,
    atmosphere: Std430GPUMaterial,
    emissive_primitive_indices: Vec<u32>,
}

impl egui_wgpu::CallbackTrait for RayMarcherViewCallback {
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
                bytemuck::cast_slice(&[self.scene_parameters]),
                bytemuck::cast_slice(&[self.render_state]),
                bytemuck::cast_slice(&[self.render_camera]),
            ],
            vec![
                bytemuck::cast_slice(self.primitives.as_slice()),
                bytemuck::cast_slice(self.lights.as_slice()),
                bytemuck::cast_slice(&[self.atmosphere]),
                bytemuck::cast_slice(self.emissive_primitive_indices.as_slice()),
            ],
        );
        Vec::new()
    }

    fn paint<'a>(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = resources.get().unwrap();
        resources.paint(render_pass);
    }
}

struct RenderResources {
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffers: Vec<Buffer>,
    storage_bind_group: wgpu::BindGroup,
    storage_bind_group_layout: wgpu::BindGroupLayout,
    storage_buffers: Vec<Buffer>,
    storage_texture_bind_group: wgpu::BindGroup,
    storage_texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniform_buffer_data: Vec<&[u8]>,
        storage_buffer_data: Vec<&[u8]>,
    ) {
        // Update our uniform buffer with the angle from the UI
        for (buffer, data) in self.uniform_buffers.iter().zip(uniform_buffer_data) {
            queue.write_buffer(&buffer.buffer, 0, data);
        }

        for (buffer, data) in self.storage_buffers.iter().zip(storage_buffer_data) {
            queue.write_buffer(&buffer.buffer, 0, data);
        }
    }

    fn paint<'render_pass>(&'render_pass self, render_pass: &mut wgpu::RenderPass<'render_pass>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.storage_bind_group, &[]);
        render_pass.set_bind_group(2, &self.storage_texture_bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }
}
