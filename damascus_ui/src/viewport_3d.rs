use std::sync::Arc;

use eframe::{
    egui,
    egui_wgpu::{self, wgpu},
    wgpu::{util::DeviceExt, Limits},
};

use damascus_core::{geometry::camera::GPUCamera, materials::GPUMaterial, scene::Scene};

#[derive(Debug)]
pub struct Viewport3d {
    pub angle: f32,
    pub scene: Scene,
}

impl Viewport3d {
    pub fn new<'a>(creation_context: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let viewport3d = Self {
            angle: 0.0,
            scene: Scene::default(),
        };

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        let wgpu_render_state = creation_context.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;

        // Render globals buffer, TODO: actually use this for renderer's global params
        let render_globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d render globals buffer"),
            contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
            // Mapping at creation (as done by the create_buffer_init utility)
            // doesn't require us to to add the MAP_WRITE usage
            // (this *happens* to workaround this bug )
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let render_globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d render globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let render_globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d render globals bind group"),
            layout: &render_globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_globals_buffer.as_entire_binding(),
            }],
        });

        // Render camera uniform buffer
        let render_camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d camera buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.scene.render_camera.to_gpu_camera()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let render_camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d camera bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
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

        // Material storage buffer
        let materials_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("viewport 3d materials buffer"),
            contents: bytemuck::cast_slice(&[viewport3d.scene.create_gpu_materials()]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::STORAGE,
        });
        let materials_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("viewport 3d materials bind group layout"),
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

        let materials_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("viewport 3d materials bind group"),
            layout: &materials_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: materials_buffer.as_entire_binding(),
            }],
        });

        // Create the pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("viewport 3d pipeline layout"),
            bind_group_layouts: &[
                &render_globals_bind_group_layout,
                &render_camera_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport 3d source shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/ray_march.wgsl").into()),
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
            .egui_rpass
            .write()
            .paint_callback_resources
            .insert(RenderResources {
                render_pipeline,
                render_globals_bind_group,
                render_globals_buffer,
                render_camera_bind_group,
                render_camera_buffer,
                materials_bind_group,
                materials_buffer,
            });

        Some(viewport3d)
    }

    pub fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;
        self.scene.render_camera.aspect_ratio = (rect.max.x - rect.min.x) / (rect.max.y - rect.min.y);

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let render_camera = self.scene.render_camera.to_gpu_camera();
        let materials = self.scene.create_gpu_materials();

        // The callback function for WGPU is in two stages: prepare, and paint.
        //
        // The prepare callback is called every frame before paint and is given access to the wgpu
        // Device and Queue, which can be used, for instance, to update buffers and uniforms before
        // rendering.
        //
        // The paint callback is called after prepare and is given access to the render pass, which
        // can be used to issue draw commands.
        let cb = egui_wgpu::CallbackFn::new()
            .prepare(move |device, queue, paint_callback_resources| {
                let resources: &RenderResources = paint_callback_resources.get().unwrap();
                resources.prepare(device, queue, angle, render_camera, materials);
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
    }
}

struct RenderResources {
    render_pipeline: wgpu::RenderPipeline,
    render_globals_bind_group: wgpu::BindGroup,
    render_globals_buffer: wgpu::Buffer,
    render_camera_bind_group: wgpu::BindGroup,
    render_camera_buffer: wgpu::Buffer,
    materials_bind_group: wgpu::BindGroup,
    materials_buffer: wgpu::Buffer,
}

impl RenderResources {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        angle: f32,
        render_camera: GPUCamera,
        materials: [GPUMaterial; Scene::MAX_MATERIALS],
    ) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            &self.render_globals_buffer,
            0,
            bytemuck::cast_slice(&[angle]),
        );
        queue.write_buffer(
            &self.render_camera_buffer,
            0,
            bytemuck::cast_slice(&[render_camera]),
        );
        queue.write_buffer(
            &self.materials_buffer,
            0,
            bytemuck::cast_slice(&[materials]),
        );
    }

    fn paint<'render_pass>(&'render_pass self, render_pass: &mut wgpu::RenderPass<'render_pass>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.render_globals_bind_group, &[]);
        render_pass.set_bind_group(1, &self.render_camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.materials_bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }
}
