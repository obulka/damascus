// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::ops::Range;

use image::Rgba32FImage;
use wgpu;

pub trait BindingResource {
    fn as_resource(&self) -> wgpu::BindingResource<'_>;
}

#[derive(Clone)]
pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub visibility: wgpu::ShaderStages,
}

impl BindingResource for Buffer {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}

#[derive(Clone)]
pub struct TextureView {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub texture_data: Rgba32FImage,
    pub visibility: wgpu::ShaderStages,
    pub view_dimension: wgpu::TextureViewDimension,
    pub size: wgpu::Extent3d,
}

impl BindingResource for TextureView {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BufferBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffers: Vec<Buffer>,
}

impl BufferBindGroup {
    pub fn write(&self, queue: &wgpu::Queue, buffer_data: &Vec<BufferDescriptor>) {
        for (buffer, data) in self.buffers.iter().zip(buffer_data) {
            queue.write_buffer(&buffer.buffer, 0, data.data.as_slice());
        }
    }
}

#[derive(Clone)]
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
                bytemuck::cast_slice(texture_view.texture_data.as_raw().as_slice()),
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

#[derive(Clone)]
pub struct StorageTextureViewBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub storage_texture_views: Vec<StorageTextureView>,
}

#[derive(Clone)]
pub struct BindGroups {
    pub vertex_bind_group: BufferBindGroup,
    pub uniform_bind_group: Option<BufferBindGroup>,
    pub storage_bind_group: Option<BufferBindGroup>,
    pub texture_bind_group: Option<TextureViewBindGroup>,
    pub storage_texture_bind_group: Option<StorageTextureViewBindGroup>,
}

impl BindGroups {
    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        let mut bind_group_layouts: Vec<&wgpu::BindGroupLayout> = vec![];

        bind_group_layouts.push(&self.vertex_bind_group.bind_group_layout);

        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            bind_group_layouts.push(&uniform_bind_group.bind_group_layout);
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            bind_group_layouts.push(&storage_bind_group.bind_group_layout);
        }
        if let Some(texture_bind_group) = &self.texture_bind_group {
            bind_group_layouts.push(&texture_bind_group.bind_group_layout);
        }
        if let Some(storage_texture_bind_group) = &self.storage_texture_bind_group {
            bind_group_layouts.push(&storage_texture_bind_group.bind_group_layout);
        }
        bind_group_layouts
    }

    pub fn set_bind_groups(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        let mut bind_group: u32 = 0;

        render_pass.set_bind_group(bind_group, &self.vertex_bind_group.bind_group, &[]);
        bind_group += 1;

        if let Some(uniform_bind_group) = &self.uniform_bind_group {
            render_pass.set_bind_group(bind_group, &uniform_bind_group.bind_group, &[]);
            bind_group += 1;
        }
        if let Some(storage_bind_group) = &self.storage_bind_group {
            render_pass.set_bind_group(bind_group, &storage_bind_group.bind_group, &[]);
            bind_group += 1;
        }
        if let Some(texture_bind_group) = &self.texture_bind_group {
            render_pass.set_bind_group(bind_group, &texture_bind_group.bind_group, &[]);
            bind_group += 1;
        }
        if let Some(storage_texture_bind_group) = &self.storage_texture_bind_group {
            render_pass.set_bind_group(bind_group, &storage_texture_bind_group.bind_group, &[]);
        }
    }
}

#[derive(Clone)]
pub struct BufferDescriptor {
    pub data: Vec<u8>,
    pub usage: wgpu::BufferUsages,
    pub visibility: wgpu::ShaderStages,
}

#[derive(Clone)]
pub struct BufferData {
    pub vertex: Vec<BufferDescriptor>,
    pub uniform: Vec<BufferDescriptor>,
    pub storage: Vec<BufferDescriptor>,
}

#[derive(Clone)]
pub struct RenderResource {
    pub render_pipeline: wgpu::RenderPipeline,
    pub index_buffer: Buffer,
    pub bind_groups: BindGroups,
    pub index_count: Range<u32>,
    pub base_vertex: i32,
    pub instance_count: Range<u32>,
}

impl RenderResource {
    pub fn write_bind_groups(&self, queue: &wgpu::Queue, buffer_data: &BufferData) {
        self.bind_groups
            .vertex_bind_group
            .write(queue, &buffer_data.vertex);

        if let Some(uniform_bind_group) = &self.bind_groups.uniform_bind_group {
            uniform_bind_group.write(queue, &buffer_data.uniform);
        }
        if let Some(storage_bind_group) = &self.bind_groups.storage_bind_group {
            storage_bind_group.write(queue, &buffer_data.storage);
        }
        if let Some(texture_bind_group) = &self.bind_groups.texture_bind_group {
            texture_bind_group.write(queue);
        }
        // self.storage_texture_bind_group.write(queue, storage_texture_data);
    }

    pub fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.set_pipeline(&self.render_pipeline);

        self.bind_groups.set_bind_groups(render_pass);

        render_pass.set_index_buffer(
            self.index_buffer.buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );

        // Reference each value w/in these ranges in the vertex shader
        // w/ @builtin(vertex_index) & @builtin(instance_index)
        // respectively
        render_pass.draw_indexed(
            self.index_count.clone(),
            self.base_vertex,
            self.instance_count.clone(),
        );
    }
}

#[derive(Clone)]
pub struct RenderResources {
    pub resources: Vec<RenderResource>,
}

impl RenderResources {
    pub fn new(resources: Vec<RenderResource>) -> Self {
        Self { resources }
    }
}
