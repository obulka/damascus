use std::borrow::Cow;
use std::ops::BitOr;
use std::sync::Arc;
use std::time::SystemTime;

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    wgpu::util::DeviceExt,
};
use glam;
use serde_hashkey::{to_key_with_ordered_float, Key, OrderedFloatPolicy};

use damascus_core::{
    geometry::{camera::Std430GPUCamera, Std430GPUPrimitive},
    lights::Std430GPULight,
    materials::Std430GPUMaterial,
    renderers::{RayMarcher, RenderStats, Std430GPURayMarcher, Std430GPURenderStats},
    scene::{Scene, Std430GPUSceneParameters},
    shaders,
};

pub struct Viewport3d {
    pub renderer: RayMarcher,
    pub enable_frame_rate_overlay: bool,
    pub frames_to_update_fps: u32,
    render_stats: RenderStats,
    renderer_hash: Key<OrderedFloatPolicy>,
}

impl Viewport3d {
    pub fn new<'a>(creation_context: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let renderer = RayMarcher::default();
        let renderer_hash = to_key_with_ordered_float(&renderer).ok()?;
        let viewport3d = Self {
            renderer: renderer,
            enable_frame_rate_overlay: true,
            frames_to_update_fps: 10,
            render_stats: RenderStats::default(),
            renderer_hash: renderer_hash,
        };

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        let wgpu_render_state = creation_context.wgpu_render_state.as_ref()?;
        let device = &wgpu_render_state.device;

        // Uniforms
        let (
            render_parameters_buffer,
            scene_parameters_buffer,
            render_stats_buffer,
            render_camera_buffer,
        ) = Self::create_uniform_buffers(device, &viewport3d);
        let (uniform_bind_group_layout, uniform_bind_group) = Self::create_uniform_binding(
            device,
            &render_parameters_buffer,
            &scene_parameters_buffer,
            &render_stats_buffer,
            &render_camera_buffer,
        );

        // Storage
        let (primitives_buffer, lights_buffer, atmosphere_buffer) =
            Self::create_storage_buffers(device, &viewport3d);
        let (storage_bind_group_layout, storage_bind_group) = Self::create_storage_binding(
            device,
            &primitives_buffer,
            &lights_buffer,
            &atmosphere_buffer,
        );

        // Create the texture to render to and initialize from
        let texture_view = Self::create_progressive_rendering_texture(device);
        let (progressive_rendering_bind_group_layout, progressive_rendering_bind_group) =
            Self::create_progressive_rendering_binding(device, &texture_view);

        // Create the pipeline
        let render_pipeline = Self::create_render_pipeline(
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
            .paint_callback_resources
            .insert(RenderResources {
                render_pipeline,
                uniform_bind_group,
                render_parameters_buffer,
                scene_parameters_buffer,
                render_stats_buffer,
                render_camera_buffer,
                storage_bind_group,
                primitives_buffer,
                lights_buffer,
                atmosphere_buffer,
                progressive_rendering_bind_group,
            });

        Some(viewport3d)
    }

    fn create_uniform_buffers(
        device: &Arc<wgpu::Device>,
        viewport3d: &Self,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let render_parameters_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d render parameter buffer"),
                contents: bytemuck::cast_slice(&[viewport3d.renderer.render_parameters()]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let scene_parameters_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d scene parameter buffer"),
                contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.scene_parameters()]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let render_stats_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d render progress buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.render_stats.as_std_430()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let render_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d camera buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.render_camera.as_std_430()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        (
            render_parameters_buffer,
            scene_parameters_buffer,
            render_stats_buffer,
            render_camera_buffer,
        )
    }

    fn create_uniform_binding(
        device: &Arc<wgpu::Device>,
        render_parameters_buffer: &wgpu::Buffer,
        scene_parameters_buffer: &wgpu::Buffer,
        render_stats_buffer: &wgpu::Buffer,
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
                    resource: render_stats_buffer.as_entire_binding(),
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
        device: &Arc<wgpu::Device>,
        viewport3d: &Self,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let primitives_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d primitives buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.create_gpu_primitives()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let lights_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d lights buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.create_gpu_lights()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let atmosphere_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d render globals buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.atmosphere()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });

        (primitives_buffer, lights_buffer, atmosphere_buffer)
    }

    fn create_storage_binding(
        device: &Arc<wgpu::Device>,
        primitives_buffer: &wgpu::Buffer,
        lights_buffer: &wgpu::Buffer,
        atmosphere_buffer: &wgpu::Buffer,
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
            ],
        });

        (storage_bind_group_layout, storage_bind_group)
    }

    fn create_progressive_rendering_texture(device: &Arc<wgpu::Device>) -> wgpu::TextureView {
        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 4096u32,
                height: 2160u32,
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

    fn create_render_pipeline(
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
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&shaders::ray_march_shader())).into(),
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

    pub fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(
            ui.available_size(),
            // egui::vec2(2048., 1024.),
            egui::Sense::drag(),
        );

        self.renderer.scene.render_camera.aspect_ratio =
            (rect.max.x - rect.min.x) / (rect.max.y - rect.min.y);

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
                    -0.015 * ui.input(|i| i.scroll_delta.y)
                } else {
                    0.
                },
            ))
        };
        self.renderer.scene.render_camera.world_matrix *= camera_transform;

        // Check if the nodegraph has changed and reset the render if it has
        if let Ok(new_hash) = to_key_with_ordered_float(&self.renderer) {
            if new_hash != self.renderer_hash {
                self.render_stats.paths_rendered_per_pixel = 0;
                self.renderer_hash = new_hash;
            }
        } else {
            panic!("Cannot hash node graph!")
        }

        // Clone locals so we can move them into the paint callback
        let render_parameters = self.renderer.render_parameters();
        let scene_parameters = self.renderer.scene.scene_parameters();
        let render_camera = self.renderer.scene.render_camera.as_std_430();
        let primitives = self.renderer.scene.create_gpu_primitives();
        let lights = self.renderer.scene.create_gpu_lights();
        let atmosphere = self.renderer.scene.atmosphere();
        let render_stats = self.render_stats.as_std_430();

        // The callback function for WGPU is in two stages: prepare, and paint.
        //
        // The prepare callback is called every frame before paint and is given access to the wgpu
        // Device and Queue, which can be used, for instance, to update buffers and uniforms before
        // rendering.
        //
        // The paint callback is called after prepare and is given access to the render pass, which
        // can be used to issue draw commands.
        let cb = egui_wgpu::CallbackFn::new()
            .prepare(move |device, queue, _encoder, paint_callback_resources| {
                let resources: &RenderResources = paint_callback_resources.get().unwrap();
                resources.prepare(
                    device,
                    queue,
                    render_parameters,
                    scene_parameters,
                    render_camera,
                    primitives,
                    lights,
                    atmosphere,
                    render_stats,
                );
                Vec::new()
            })
            .paint(move |_info, rpass, paint_callback_resources| {
                let resources: &RenderResources = paint_callback_resources.get().unwrap();
                resources.paint(rpass);
            });

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(cb),
        };

        ui.painter().add(callback);

        if self.enable_frame_rate_overlay {
            if self.render_stats.frame_counter % self.frames_to_update_fps == 0 {
                match SystemTime::now().duration_since(self.render_stats.previous_frame_time) {
                    Ok(frame_time) => {
                        self.render_stats.fps =
                            self.frames_to_update_fps as f32 / frame_time.as_secs_f32();
                    }
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                }

                self.render_stats.previous_frame_time = SystemTime::now();
                self.render_stats.frame_counter = 1;
            } else {
                self.render_stats.frame_counter += 1;
            }
            ui.label(format!("{:?} fps", self.render_stats.fps));
        }

        self.render_stats.paths_rendered_per_pixel += 1;
    }
}

struct RenderResources {
    render_pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    render_parameters_buffer: wgpu::Buffer,
    scene_parameters_buffer: wgpu::Buffer,
    render_stats_buffer: wgpu::Buffer,
    render_camera_buffer: wgpu::Buffer,
    storage_bind_group: wgpu::BindGroup,
    primitives_buffer: wgpu::Buffer,
    lights_buffer: wgpu::Buffer,
    atmosphere_buffer: wgpu::Buffer,
    progressive_rendering_bind_group: wgpu::BindGroup,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_parameters: Std430GPURayMarcher,
        scene_parameters: Std430GPUSceneParameters,
        render_camera: Std430GPUCamera,
        primitives: [Std430GPUPrimitive; Scene::MAX_PRIMITIVES],
        lights: [Std430GPULight; Scene::MAX_LIGHTS],
        atmosphere: Std430GPUMaterial,
        render_stats: Std430GPURenderStats,
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
            &self.render_stats_buffer,
            0,
            bytemuck::cast_slice(&[render_stats]),
        );
        queue.write_buffer(
            &self.render_camera_buffer,
            0,
            bytemuck::cast_slice(&[render_camera]),
        );
        queue.write_buffer(
            &self.primitives_buffer,
            0,
            bytemuck::cast_slice(&[primitives]),
        );
        queue.write_buffer(&self.lights_buffer, 0, bytemuck::cast_slice(&[lights]));
        queue.write_buffer(
            &self.atmosphere_buffer,
            0,
            bytemuck::cast_slice(&[atmosphere]),
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