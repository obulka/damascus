// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui_wgpu::wgpu;

pub trait BindingResource {
    fn as_resource(&self) -> wgpu::BindingResource<'_>;
}

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub visibility: wgpu::ShaderStages,
}

impl BindingResource for Buffer {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}

pub struct TextureView {
    pub texture_view: wgpu::TextureView,
    pub visibility: wgpu::ShaderStages,
    pub format: wgpu::TextureFormat,
    pub view_dimension: wgpu::TextureViewDimension,
}

impl BindingResource for TextureView {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}

pub struct StorageTextureView {
    pub texture_view: wgpu::TextureView,
    pub visibility: wgpu::ShaderStages,
    pub access: wgpu::StorageTextureAccess,
    pub format: wgpu::TextureFormat,
    pub view_dimension: wgpu::TextureViewDimension,
}

impl BindingResource for StorageTextureView {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}

pub struct BufferBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffers: Vec<Buffer>,
}

impl BufferBindGroup {
    pub fn write(&self, queue: &wgpu::Queue, buffer_data: Vec<&[u8]>) {
        for (buffer, data) in self.buffers.iter().zip(buffer_data) {
            queue.write_buffer(&buffer.buffer, 0, data);
        }
    }
}

pub struct StorageTextureViewBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub storage_texture_views: Vec<StorageTextureView>,
}

pub struct RenderResources {
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub uniform_bind_group: Option<BufferBindGroup>,
    pub storage_bind_group: Option<BufferBindGroup>,
    pub storage_texture_bind_group: Option<StorageTextureViewBindGroup>,
}

impl RenderResources {
    pub fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniform_buffer_data: Vec<&[u8]>,
        storage_buffer_data: Vec<&[u8]>,
    ) {
        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            uniform_bind_group.write(queue, uniform_buffer_data);
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            storage_bind_group.write(queue, storage_buffer_data);
        }
        // self.storage_texture_bind_group.write(queue, storage_texture_data);
    }

    pub fn paint<'render_pass>(
        &'render_pass self,
        render_pass: &mut wgpu::RenderPass<'render_pass>,
    ) {
        if let Some(render_pipeline) = &self.render_pipeline {
            render_pass.set_pipeline(&render_pipeline);
        } else {
            return;
        }

        let mut bind_group: u32 = 0;
        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            render_pass.set_bind_group(bind_group, &uniform_bind_group.bind_group, &[]);
            bind_group += 1
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            render_pass.set_bind_group(bind_group, &storage_bind_group.bind_group, &[]);
            bind_group += 1
        }
        if let Some(storage_texture_bind_group) = &self.storage_texture_bind_group {
            render_pass.set_bind_group(bind_group, &storage_texture_bind_group.bind_group, &[]);
        }

        render_pass.draw(0..4, 0..1);
    }

    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        let mut bind_group_layouts: Vec<&wgpu::BindGroupLayout> = vec![];
        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            bind_group_layouts.push(&uniform_bind_group.bind_group_layout);
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            bind_group_layouts.push(&storage_bind_group.bind_group_layout);
        }
        if let Some(storage_texture_bind_group) = &self.storage_texture_bind_group {
            bind_group_layouts.push(&storage_texture_bind_group.bind_group_layout);
        }
        bind_group_layouts
    }
}
