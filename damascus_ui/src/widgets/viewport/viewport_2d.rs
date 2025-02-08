// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::borrow::Cow;
use std::collections::HashSet;
use std::ops::BitOr;
use std::sync::Arc;
use std::time::SystemTime;

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    epaint,
    wgpu::util::DeviceExt,
};
use glam;
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use damascus_core::{shaders, textures::Texture};

use super::{CompilerSettings, PipelineSettings2D, ViewportSettings};

use crate::MAX_TEXTURE_DIMENSION;

pub struct Viewport2d {
    pub texture: Texture,
    pub enable_frame_rate_overlay: bool,
    pub frames_to_update_fps: u32,
    pub stats_text: String,
    disabled: bool,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
}

impl Viewport2d {
    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Option<Self> {
        let texture = Texture::default();
        let recompile_hash = to_key_with_ordered_float(&texture).ok()?;
        let reconstruct_hash = to_key_with_ordered_float(&settings.pipeline_settings_2d).ok()?;
        let mut viewport2d = Self {
            texture: texture,
            enable_frame_rate_overlay: true,
            frames_to_update_fps: 10,
            stats_text: String::new(),
            disabled: true,
            recompile_hash: recompile_hash,
            reconstruct_hash: reconstruct_hash,
        };

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        Self::construct_render_pipeline(
            &mut viewport2d,
            creation_context.wgpu_render_state.as_ref()?,
            &settings.pipeline_settings_2d,
        );

        Some(viewport2d)
    }

    pub fn construct_render_pipeline(
        &mut self,
        wgpu_render_state: &egui_wgpu::RenderState,
        pipeline_settings_2d: &PipelineSettings2D,
    ) {
        self.reset_render();

        let device = &wgpu_render_state.device;

        // Uniforms
        let (uniform_bind_group_layout, uniform_bind_group) = Self::create_uniform_binding(device);

        // Storage
        let (storage_bind_group_layout, storage_bind_group) = Self::create_storage_binding(device);

        // Create the texture to render to and initialize from
        let texture_view = Self::create_progressive_rendering_texture(device);
        let (progressive_rendering_bind_group_layout, progressive_rendering_bind_group) =
            Self::create_progressive_rendering_binding(device, &texture_view);

        // Create the pipeline
        let render_pipeline = self.create_render_pipeline(
            device,
            wgpu_render_state.target_format,
            &uniform_bind_group_layout,
            &storage_bind_group_layout,
            &progressive_rendering_bind_group_layout,
        );

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(RenderResources {
                render_pipeline,
                uniform_bind_group,
                uniform_bind_group_layout,
                storage_bind_group,
                storage_bind_group_layout,
                progressive_rendering_bind_group,
                progressive_rendering_bind_group_layout,
            });
    }

    pub fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState) {
        if let Some(render_resources) = wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .get_mut::<RenderResources>()
        {
            self.reset_render();

            let device = &wgpu_render_state.device;

            // Create the updated pipeline
            render_resources.render_pipeline = self.create_render_pipeline(
                device,
                wgpu_render_state.target_format,
                &render_resources.uniform_bind_group_layout,
                &render_resources.storage_bind_group_layout,
                &render_resources.progressive_rendering_bind_group_layout,
            );
        }
    }

    pub fn disable(&mut self) {
        self.pause();
        self.disabled = true;
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn enabled(&mut self) -> bool {
        !self.disabled
    }

    pub fn pause(&mut self) {
        self.render_state.paused = true;
    }

    pub fn play(&mut self) {
        if !self.disabled {
            self.render_state.paused = false;
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if !self.disabled {
            self.render_state.paused = !self.render_state.paused;
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

    fn create_uniform_binding(
        device: &Arc<wgpu::Device>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 2d uniform bind group layout"),
                entries: &[
                    // wgpu::BindGroupLayoutEntry {
                    //     binding: 0,
                    //     visibility: wgpu::ShaderStages::FRAGMENT,
                    //     ty: wgpu::BindingType::Buffer {
                    //         ty: wgpu::BufferBindingType::Uniform,
                    //         has_dynamic_offset: false,
                    //         min_binding_size: None,
                    //     },
                    //     count: None,
                    // },
                ],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 2d uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                // wgpu::BindGroupEntry {
                //     binding: 0,
                //     resource: render_parameters_buffer.as_entire_binding(),
                // },
            ],
        });

        (uniform_bind_group_layout, uniform_bind_group)
    }

    fn create_storage_binding(
        device: &Arc<wgpu::Device>,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let storage_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 2d scene storage bind group layout"),
                entries: &[
                    // wgpu::BindGroupLayoutEntry {
                    //     binding: 0,
                    //     visibility: wgpu::ShaderStages::FRAGMENT,
                    //     ty: wgpu::BindingType::Buffer {
                    //         ty: wgpu::BufferBindingType::Storage { read_only: true },
                    //         has_dynamic_offset: false,
                    //         min_binding_size: None,
                    //     },
                    //     count: None,
                    // },

                ],
            });
        let storage_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 2d scene storage bind group"),
            layout: &storage_bind_group_layout,
            entries: &[
                // wgpu::BindGroupEntry {
                //     binding: 0,
                //     resource: primitives_buffer.as_entire_binding(),
                // },
            ],
        });

        (storage_bind_group_layout, storage_bind_group)
    }

    fn create_progressive_rendering_texture(device: &Arc<wgpu::Device>) -> wgpu::TextureView {
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
            label: Some("viewport 2d progressive rendering texture"),
            view_formats: &[],
        };
        let texture = device.create_texture(&texture_descriptor);
        texture.create_view(&Default::default())
    }

    fn create_progressive_rendering_binding(
        device: &Arc<wgpu::Device>,
        texture_view: &wgpu::TextureView,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let progressive_rendering_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 2d progressive rendering bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadWrite,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });

        let progressive_rendering_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("viewport 2d progressive rendering bind group"),
                layout: &progressive_rendering_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                }],
            });

        (
            progressive_rendering_bind_group_layout,
            progressive_rendering_bind_group,
        )
    }

    pub fn update_preprocessor_directives(&mut self, settings: &CompilerSettings) -> bool {
        let mut preprocessor_directives = HashSet::<shaders::PreprocessorDirectives>::new();

        if !settings.enable_dynamic_recompilation_for_ray_marcher {
            preprocessor_directives.extend(shaders::all_directives_for_ray_marcher());
        } else {
            preprocessor_directives.extend(shaders::directives_for_ray_marcher(&self.renderer));
        }

        if !settings.enable_dynamic_recompilation_for_primitives {
            preprocessor_directives.extend(shaders::all_directives_for_primitive());
        }

        if !settings.enable_dynamic_recompilation_for_materials {
            preprocessor_directives.extend(shaders::all_directives_for_material());
        } else {
            preprocessor_directives.extend(shaders::directives_for_material(
                &self.renderer.scene.atmosphere,
            ));
        }

        if !settings.enable_dynamic_recompilation_for_lights {
            preprocessor_directives.extend(shaders::all_directives_for_light());
        } else {
            for light in &self.renderer.scene.lights {
                preprocessor_directives.extend(shaders::directives_for_light(&light));
            }
        }

        if settings.enable_dynamic_recompilation_for_primitives
            || settings.enable_dynamic_recompilation_for_materials
        {
            for primitive in &self.renderer.scene.primitives {
                if settings.enable_dynamic_recompilation_for_materials {
                    preprocessor_directives
                        .extend(shaders::directives_for_material(&primitive.material));
                }
                if settings.enable_dynamic_recompilation_for_primitives {
                    preprocessor_directives.extend(shaders::directives_for_primitive(&primitive));
                }
            }
        }

        // Check if the directives have changed and store them if they have
        if preprocessor_directives == self.render_state.preprocessor_directives {
            return false;
        }
        self.render_state.preprocessor_directives = preprocessor_directives;
        true
    }

    fn create_render_pipeline(
        &self,
        device: &Arc<wgpu::Device>,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        storage_bind_group_layout: &wgpu::BindGroupLayout,
        progressive_rendering_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viewport 2d pipeline layout"),
            bind_group_layouts: &[
                uniform_bind_group_layout,
                storage_bind_group_layout,
                progressive_rendering_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport 2d source shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shaders::ray_march_shader(
                &self.render_state.preprocessor_directives,
            )))
            .into(),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewport 2d render pipeline"),
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

    pub fn reset_render(&mut self) {
        self.render_state.paths_rendered_per_pixel = 0;
    }

    fn update_camera(&mut self, ui: &egui::Ui, rect: &egui::Rect, response: &egui::Response) {
        self.renderer.scene.render_camera.aspect_ratio = rect.width() / rect.height();
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
        self.renderer.scene.render_camera.world_matrix *= camera_transform;
    }

    pub fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        available_size: egui::Vec2,
        settings: &ViewportSettings,
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

        // Check if the nodegraph has changed and reset the render if it has
        if let Ok(new_hash) = to_key_with_ordered_float(&settings.pipeline_settings_2d) {
            if new_hash != self.reconstruct_hash {
                self.reconstruct_hash = new_hash;
                if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                    wgpu_render_state
                        .renderer
                        .write()
                        .callback_resources
                        .clear();
                    self.construct_render_pipeline(
                        wgpu_render_state,
                        &settings.pipeline_settings_2d,
                    );
                }
            }
        } else {
            panic!("Cannot hash settings!")
        }

        if let Ok(new_hash) = to_key_with_ordered_float(&self.renderer) {
            if new_hash != self.recompile_hash {
                self.reset_render();
                self.recompile_hash = new_hash;
                if settings.compiler_settings.dynamic_recompilation_enabled()
                    && self.update_preprocessor_directives(&settings.compiler_settings)
                {
                    if let Some(wgpu_render_state) = frame.wgpu_render_state() {
                        self.recompile_shader(wgpu_render_state);
                    }
                }
            }
        } else {
            panic!("Cannot hash renderer!")
        }

        if self.render_state.paused {
            self.render_state.previous_frame_time = SystemTime::now();
            self.render_state.frame_counter = 1;
            return None;
        }

        if self.enable_frame_rate_overlay {
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

        self.render_state.paths_rendered_per_pixel += 1;

        Some(egui_wgpu::Callback::new_paint_callback(
            rect,
            Viewport2dCallback {
                // render_parameters: self.renderer.render_parameters(),
            },
        ))
    }
}

struct Viewport2dCallback {
    // render_parameters: Std430GPURayMarcher,
}

impl egui_wgpu::CallbackTrait for Viewport2dCallback {
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
            device, queue,
            // self.render_parameters,
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
    storage_bind_group: wgpu::BindGroup,
    storage_bind_group_layout: wgpu::BindGroupLayout,
    progressive_rendering_bind_group: wgpu::BindGroup,
    progressive_rendering_bind_group_layout: wgpu::BindGroupLayout,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        // render_parameters: Std430GPURayMarcher,
    ) {
        // Update our uniform buffer with the angle from the UI
        // queue.write_buffer(
        //     &self.render_parameters_buffer,
        //     0,
        //     bytemuck::cast_slice(&[render_parameters]),
        // );
    }

    fn paint<'render_pass>(&'render_pass self, render_pass: &mut wgpu::RenderPass<'render_pass>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.storage_bind_group, &[]);
        render_pass.set_bind_group(2, &self.progressive_rendering_bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }
}
