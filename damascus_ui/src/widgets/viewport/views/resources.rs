// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use eframe::egui_wgpu::wgpu;
use image::RgbaImage;

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
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub texture_data: RgbaImage,
    pub visibility: wgpu::ShaderStages,
    pub view_dimension: wgpu::TextureViewDimension,
    pub size: wgpu::Extent3d,
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

pub struct TextureViewBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub texture_views: Vec<TextureView>,
}

impl TextureViewBindGroup {
    pub fn write(&self, queue: &wgpu::Queue) {
        for texture_view in self.texture_views.iter() {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture_view.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                texture_view.texture_data.as_raw(),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(16 * texture_view.texture_data.width()),
                    rows_per_image: Some(texture_view.texture_data.height()),
                },
                texture_view.size,
            );
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
    pub texture_bind_group: Option<TextureViewBindGroup>,
    pub storage_texture_bind_group: Option<StorageTextureViewBindGroup>,
}

impl RenderResources {
    pub fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniform_buffer_data: Vec<&[u8]>,
        storage_buffer_data: Vec<&[u8]>,
        // texture_buffer_data: Vec<&[u8]>,
    ) {
        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            uniform_bind_group.write(queue, uniform_buffer_data);
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            storage_bind_group.write(queue, storage_buffer_data);
        }
        if let Some(texture_bind_group) = &self.texture_bind_group {
            texture_bind_group.write(queue);
        }
        // self.storage_texture_bind_group.write(queue, storage_texture_data);
    }

    pub fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
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
        if let Some(texture_bind_group) = &self.texture_bind_group {
            render_pass.set_bind_group(bind_group, &texture_bind_group.bind_group, &[]);
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
