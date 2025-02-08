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

use damascus_core::{
    geometry::{camera::Std430GPUCamera, Std430GPUPrimitive},
    lights::Std430GPULight,
    materials::Std430GPUMaterial,
    renderers::ray_marcher::{RayMarcher, RenderState, Std430GPURayMarcher, Std430GPURenderState},
    scene::Std430GPUSceneParameters,
    shaders,
};

use super::{CompilerSettings, PipelineSettings3D, ViewportSettings};

use crate::MAX_TEXTURE_DIMENSION;

pub struct Viewport3d {
    pub renderer: RayMarcher,
    pub enable_frame_rate_overlay: bool,
    pub frames_to_update_fps: u32,
    pub stats_text: String,
    disabled: bool,
    camera_controls_enabled: bool,
    render_state: RenderState,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
}

impl Viewport3d {
    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Option<Self> {
        let renderer = RayMarcher::default();
        let recompile_hash = to_key_with_ordered_float(&renderer).ok()?;
        let reconstruct_hash = to_key_with_ordered_float(&settings.pipeline_settings_3d).ok()?;
        let mut viewport3d = Self {
            renderer: renderer,
            enable_frame_rate_overlay: true,
            frames_to_update_fps: 10,
            stats_text: String::new(),
            disabled: true,
            camera_controls_enabled: true,
            render_state: RenderState::default(),
            recompile_hash: recompile_hash,
            reconstruct_hash: reconstruct_hash,
        };

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        Self::construct_render_pipeline(
            &mut viewport3d,
            creation_context.wgpu_render_state.as_ref()?,
            &settings.pipeline_settings_3d,
        );

        Some(viewport3d)
    }

    pub fn construct_render_pipeline(
        &mut self,
        wgpu_render_state: &egui_wgpu::RenderState,
        pipeline_settings_3d: &PipelineSettings3D,
    ) {
        self.reset_render();

        let device = &wgpu_render_state.device;

        // Uniforms
        let (
            render_parameters_buffer,
            scene_parameters_buffer,
            render_state_buffer,
            render_camera_buffer,
        ) = self.create_uniform_buffers(device, pipeline_settings_3d);
        let (uniform_bind_group_layout, uniform_bind_group) = Self::create_uniform_binding(
            device,
            &render_parameters_buffer,
            &scene_parameters_buffer,
            &render_state_buffer,
            &render_camera_buffer,
        );

        // Storage
        let (
            primitives_buffer,
            lights_buffer,
            atmosphere_buffer,
            emissive_primitive_indices_buffer,
        ) = self.create_storage_buffers(device, pipeline_settings_3d);
        let (storage_bind_group_layout, storage_bind_group) = Self::create_storage_binding(
            device,
            &primitives_buffer,
            &lights_buffer,
            &atmosphere_buffer,
            &emissive_primitive_indices_buffer,
        );

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
                render_parameters_buffer,
                scene_parameters_buffer,
                render_state_buffer,
                render_camera_buffer,
                storage_bind_group,
                storage_bind_group_layout,
                primitives_buffer,
                lights_buffer,
                atmosphere_buffer,
                emissive_primitive_indices_buffer,
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

    fn create_uniform_buffers(
        &self,
        device: &Arc<wgpu::Device>,
        pipeline_settings_3d: &PipelineSettings3D,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let render_parameters_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d render parameter buffer"),
                contents: bytemuck::cast_slice(&[self.renderer.render_parameters()]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let scene_parameters_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d scene parameter buffer"),
                contents: bytemuck::cast_slice(&[self.renderer.scene.scene_parameters(
                    pipeline_settings_3d.max_primitives,
                    pipeline_settings_3d.max_lights,
                )]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let render_state_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d render progress buffer"),
            contents: bytemuck::cast_slice(&[self.render_state.as_std_430()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let render_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d camera buffer"),
            contents: bytemuck::cast_slice(&[self.renderer.scene.render_camera.as_std_430()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        (
            render_parameters_buffer,
            scene_parameters_buffer,
            render_state_buffer,
            render_camera_buffer,
        )
    }

    fn create_uniform_binding(
        device: &Arc<wgpu::Device>,
        render_parameters_buffer: &wgpu::Buffer,
        scene_parameters_buffer: &wgpu::Buffer,
        render_state_buffer: &wgpu::Buffer,
        render_camera_buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d uniform bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT.bitor(wgpu::ShaderStages::VERTEX),
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: render_parameters_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: scene_parameters_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: render_state_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: render_camera_buffer.as_entire_binding(),
                },
            ],
        });

        (uniform_bind_group_layout, uniform_bind_group)
    }

    fn create_storage_buffers(
        &self,
        device: &Arc<wgpu::Device>,
        pipeline_settings_3d: &PipelineSettings3D,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let primitives: Vec<Std430GPUPrimitive> = self
            .renderer
            .scene
            .create_gpu_primitives(pipeline_settings_3d.max_primitives);
        let lights: Vec<Std430GPULight> = self
            .renderer
            .scene
            .create_gpu_lights(pipeline_settings_3d.max_lights);
        let emissive_primitive_indices: Vec<u32> = self
            .renderer
            .scene
            .emissive_primitive_indices(pipeline_settings_3d.max_primitives);
        let primitives_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d primitives buffer"),
            contents: &[
                bytemuck::cast_slice(primitives.as_slice()),
                vec![
                    0;
                    (pipeline_settings_3d.max_primitives - primitives.len())
                        * size_of::<Std430GPUPrimitive>()
                ]
                .as_slice(),
            ]
            .concat(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let lights_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d lights buffer"),
            contents: &[
                bytemuck::cast_slice(lights.as_slice()),
                vec![
                    0;
                    (pipeline_settings_3d.max_lights - lights.len()) * size_of::<Std430GPULight>()
                ]
                .as_slice(),
            ]
            .concat(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let atmosphere_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d render globals buffer"),
            contents: bytemuck::cast_slice(&[self.renderer.scene.atmosphere()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let emissive_primitive_indices_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d emissive primitive ids"),
                contents: &[
                    bytemuck::cast_slice(emissive_primitive_indices.as_slice()),
                    vec![
                        0;
                        (pipeline_settings_3d.max_primitives - emissive_primitive_indices.len())
                            * size_of::<u32>()
                    ]
                    .as_slice(),
                ]
                .concat(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            });

        (
            primitives_buffer,
            lights_buffer,
            atmosphere_buffer,
            emissive_primitive_indices_buffer,
        )
    }

    fn create_storage_binding(
        device: &Arc<wgpu::Device>,
        primitives_buffer: &wgpu::Buffer,
        lights_buffer: &wgpu::Buffer,
        atmosphere_buffer: &wgpu::Buffer,
        emissive_primitive_indices_buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let storage_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d scene storage bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let storage_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d scene storage bind group"),
            layout: &storage_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: primitives_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: lights_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: atmosphere_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: emissive_primitive_indices_buffer.as_entire_binding(),
                },
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
            label: Some("viewport 3d progressive rendering texture"),
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
                label: Some("viewport 3d progressive rendering bind group layout"),
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
                label: Some("viewport 3d progressive rendering bind group"),
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
            label: Some("viewport 3d pipeline layout"),
            bind_group_layouts: &[
                uniform_bind_group_layout,
                storage_bind_group_layout,
                progressive_rendering_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport 3d source shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shaders::ray_march_shader(
                &self.render_state.preprocessor_directives,
            )))
            .into(),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewport 3d render pipeline"),
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
        if let Ok(new_hash) = to_key_with_ordered_float(&settings.pipeline_settings_3d) {
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
                        &settings.pipeline_settings_3d,
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
            Viewport3dCallback {
                render_parameters: self.renderer.render_parameters(),
                scene_parameters: self.renderer.scene.scene_parameters(
                    settings.pipeline_settings_3d.max_primitives,
                    settings.pipeline_settings_3d.max_lights,
                ),
                render_camera: self.renderer.scene.render_camera.as_std_430(),
                primitives: self
                    .renderer
                    .scene
                    .create_gpu_primitives(settings.pipeline_settings_3d.max_primitives),
                lights: self
                    .renderer
                    .scene
                    .create_gpu_lights(settings.pipeline_settings_3d.max_lights),
                atmosphere: self.renderer.scene.atmosphere(),
                emissive_primitive_indices: self
                    .renderer
                    .scene
                    .emissive_primitive_indices(settings.pipeline_settings_3d.max_primitives),
                render_state: self.render_state.as_std_430(),
            },
        ))
    }
}

struct Viewport3dCallback {
    render_parameters: Std430GPURayMarcher,
    scene_parameters: Std430GPUSceneParameters,
    render_camera: Std430GPUCamera,
    primitives: Vec<Std430GPUPrimitive>,
    lights: Vec<Std430GPULight>,
    atmosphere: Std430GPUMaterial,
    emissive_primitive_indices: Vec<u32>,
    render_state: Std430GPURenderState,
}

impl egui_wgpu::CallbackTrait for Viewport3dCallback {
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
            self.render_parameters,
            self.scene_parameters,
            self.render_camera,
            &self.primitives,
            &self.lights,
            self.atmosphere,
            &self.emissive_primitive_indices,
            self.render_state,
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
    render_parameters_buffer: wgpu::Buffer,
    scene_parameters_buffer: wgpu::Buffer,
    render_state_buffer: wgpu::Buffer,
    render_camera_buffer: wgpu::Buffer,
    storage_bind_group: wgpu::BindGroup,
    storage_bind_group_layout: wgpu::BindGroupLayout,
    primitives_buffer: wgpu::Buffer,
    lights_buffer: wgpu::Buffer,
    atmosphere_buffer: wgpu::Buffer,
    emissive_primitive_indices_buffer: wgpu::Buffer,
    progressive_rendering_bind_group: wgpu::BindGroup,
    progressive_rendering_bind_group_layout: wgpu::BindGroupLayout,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_parameters: Std430GPURayMarcher,
        scene_parameters: Std430GPUSceneParameters,
        render_camera: Std430GPUCamera,
        primitives: &Vec<Std430GPUPrimitive>,
        lights: &Vec<Std430GPULight>,
        atmosphere: Std430GPUMaterial,
        emissive_primitive_indices: &Vec<u32>,
        render_state: Std430GPURenderState,
    ) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            &self.render_parameters_buffer,
            0,
            bytemuck::cast_slice(&[render_parameters]),
        );
        queue.write_buffer(
            &self.scene_parameters_buffer,
            0,
            bytemuck::cast_slice(&[scene_parameters]),
        );
        queue.write_buffer(
            &self.render_state_buffer,
            0,
            bytemuck::cast_slice(&[render_state]),
        );
        queue.write_buffer(
            &self.render_camera_buffer,
            0,
            bytemuck::cast_slice(&[render_camera]),
        );
        queue.write_buffer(
            &self.primitives_buffer,
            0,
            bytemuck::cast_slice(primitives.as_slice()),
        );
        queue.write_buffer(
            &self.lights_buffer,
            0,
            bytemuck::cast_slice(lights.as_slice()),
        );
        queue.write_buffer(
            &self.atmosphere_buffer,
            0,
            bytemuck::cast_slice(&[atmosphere]),
        );
        queue.write_buffer(
            &self.emissive_primitive_indices_buffer,
            0,
            bytemuck::cast_slice(emissive_primitive_indices.as_slice()),
        );
    }

    fn paint<'render_pass>(&'render_pass self, render_pass: &mut wgpu::RenderPass<'render_pass>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_bind_group(1, &self.storage_bind_group, &[]);
        render_pass.set_bind_group(2, &self.progressive_rendering_bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }
}
