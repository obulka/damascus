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

use damascus_core::{
    geometry::{camera::Std430GPUCamera, Std430GPUPrimitive},
    lights::Std430GPULight,
    renderers::{RayMarcher, Std430RenderParameters},
    scene::Scene,
    shaders,
};

pub struct Viewport3d {
    pub enable_frame_rate_overlay: bool,
    pub frames_to_update_fps: u32,
    frame_counter: u32,
    previous_frame_time: SystemTime,
    fps: f32,
    pub renderer: RayMarcher,
}

impl Viewport3d {
    pub fn new<'a>(creation_context: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let viewport3d = Self {
            enable_frame_rate_overlay: true,
            frames_to_update_fps: 10,
            frame_counter: 1,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            renderer: RayMarcher::default(),
        };

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        let wgpu_render_state = creation_context.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;

        // Render globals buffer
        let render_parameters_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("viewport 3d render globals buffer"),
                contents: bytemuck::cast_slice(&[viewport3d.renderer.as_render_parameters()]),
                // Mapping at creation (as done by the create_buffer_init utility)
                // doesn't require us to to add the MAP_WRITE usage
                // (this *happens* to workaround this bug )
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            });
        let render_parameters_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d render globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let render_parameters_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d render globals bind group"),
            layout: &render_parameters_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_parameters_buffer.as_entire_binding(),
            }],
        });

        // Render camera uniform buffer
        let render_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d camera buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.render_camera.as_std_430()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let render_camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d camera bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT.bitor(wgpu::ShaderStages::VERTEX),
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let render_camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d render camera bind group"),
            layout: &render_camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_camera_buffer.as_entire_binding(),
            }],
        });

        // Primitive storage buffer
        let primitives_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d primitives buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.create_gpu_primitives()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let primitives_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d primitives bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let primitives_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d primitives bind group"),
            layout: &primitives_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: primitives_buffer.as_entire_binding(),
            }],
        });

        // Light storage buffer
        let lights_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d lights buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.renderer.scene.create_gpu_lights()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        });
        let lights_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d lights bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let lights_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d lights bind group"),
            layout: &lights_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: lights_buffer.as_entire_binding(),
            }],
        });

        // Create the pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viewport 3d pipeline layout"),
            bind_group_layouts: &[
                &render_parameters_bind_group_layout,
                &render_camera_bind_group_layout,
                &primitives_bind_group_layout,
                &lights_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport 3d source shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shaders::RAY_MARCH_SHADER)).into(),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                targets: &[Some(wgpu_render_state.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .paint_callback_resources
            .insert(RenderResources {
                render_pipeline,
                render_parameters_bind_group,
                render_parameters_buffer,
                render_camera_bind_group,
                render_camera_buffer,
                primitives_bind_group,
                primitives_buffer,
                lights_bind_group,
                lights_buffer,
            });

        Some(viewport3d)
    }

    pub fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        self.renderer.scene.render_camera.aspect_ratio =
            (rect.max.x - rect.min.x) / (rect.max.y - rect.min.y);

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
                -0.015 * ui.input(|i| i.scroll_delta.y),
            ))
        };
        self.renderer.scene.render_camera.world_matrix *= camera_transform;

        // Clone locals so we can move them into the paint callback:
        let render_parameters = self.renderer.as_render_parameters();
        let render_camera = self.renderer.scene.render_camera.as_std_430();
        let primitives = self.renderer.scene.create_gpu_primitives();
        let lights = self.renderer.scene.create_gpu_lights();

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
                    render_camera,
                    primitives,
                    lights,
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
            if self.frame_counter % self.frames_to_update_fps == 0 {
                match SystemTime::now().duration_since(self.previous_frame_time) {
                    Ok(frame_time) => {
                        self.fps = self.frames_to_update_fps as f32 / frame_time.as_secs_f32();
                    }
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                }

                self.previous_frame_time = SystemTime::now();
                self.frame_counter = 1;
            } else {
                self.frame_counter += 1;
            }
            ui.label(format!("{:?} fps", self.fps));
        }
    }
}

struct RenderResources {
    render_pipeline: wgpu::RenderPipeline,
    render_parameters_bind_group: wgpu::BindGroup,
    render_parameters_buffer: wgpu::Buffer,
    render_camera_bind_group: wgpu::BindGroup,
    render_camera_buffer: wgpu::Buffer,
    primitives_bind_group: wgpu::BindGroup,
    primitives_buffer: wgpu::Buffer,
    lights_bind_group: wgpu::BindGroup,
    lights_buffer: wgpu::Buffer,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_parameters: Std430RenderParameters,
        render_camera: Std430GPUCamera,
        primitives: [Std430GPUPrimitive; Scene::MAX_PRIMITIVES],
        lights: [Std430GPULight; Scene::MAX_LIGHTS],
    ) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            &self.render_parameters_buffer,
            0,
            bytemuck::cast_slice(&[render_parameters]),
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
    }

    fn paint<'render_pass>(&'render_pass self, render_pass: &mut wgpu::RenderPass<'render_pass>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.render_parameters_bind_group, &[]);
        render_pass.set_bind_group(1, &self.render_camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.primitives_bind_group, &[]);
        render_pass.set_bind_group(3, &self.lights_bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }
}
