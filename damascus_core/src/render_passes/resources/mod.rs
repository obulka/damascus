// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use image::Rgba32FImage;
use wgpu;

pub trait BindingResource {
    fn as_resource(&self) -> wgpu::BindingResource<'_>;
}

#[derive(Clone)]
pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
    pub vertex_count: u32,
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
    pub fn write(&self, queue: &wgpu::Queue, buffer_data: Vec<&[u8]>) {
        for (buffer, data) in self.buffers.iter().zip(buffer_data) {
            queue.write_buffer(&buffer.buffer, 0, data);
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
    pub uniform_bind_group: Option<BufferBindGroup>,
    pub storage_bind_group: Option<BufferBindGroup>,
    pub texture_bind_group: Option<TextureViewBindGroup>,
    pub storage_texture_bind_group: Option<StorageTextureViewBindGroup>,
}

impl BindGroups {
    pub fn bind_group_layouts(&self) -> Vec<&wgpu::BindGroupLayout> {
        let mut bind_group_layouts: Vec<&wgpu::BindGroupLayout> = vec![];
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
            bind_group += 1
        }
        if let Some(storage_texture_bind_group) = &self.storage_texture_bind_group {
            render_pass.set_bind_group(bind_group, &storage_texture_bind_group.bind_group, &[]);
        }
    }
}

#[derive(Clone)]
pub struct RenderResource {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffers: Vec<VertexBuffer>,
    pub bind_groups: BindGroups,
}

pub struct RenderResources {
    pub resources: Vec<RenderResource>,
}

pub struct BufferData<'a> {
    pub uniform: Vec<&'a [u8]>,
    pub storage: Vec<&'a [u8]>,
}

impl RenderResources {
    pub fn bind_groups(&self) -> Vec<BindGroups> {
        self.resources
            .iter()
            .map(|resource| resource.bind_groups.clone())
            .collect()
    }

    pub fn write_bind_groups(&self, queue: &wgpu::Queue, buffer_data: Vec<BufferData<'_>>) {
        for (resource, data) in self.resources.iter().zip(buffer_data) {
            if let Some(uniform_bind_group) = &resource.bind_groups.uniform_bind_group {
                uniform_bind_group.write(queue, data.uniform);
            }
            if let Some(storage_bind_group) = &resource.bind_groups.storage_bind_group {
                storage_bind_group.write(queue, data.storage);
            }
            if let Some(texture_bind_group) = &resource.bind_groups.texture_bind_group {
                texture_bind_group.write(queue);
            }
        }
        // self.storage_texture_bind_group.write(queue, storage_texture_data);
    }

    pub fn paint(&self, render_pass: &mut wgpu::RenderPass<'_>) {
        for resource in self.resources.iter() {
            render_pass.set_pipeline(&resource.render_pipeline);

            resource.bind_groups.set_bind_groups(render_pass);

            let mut vertices: u32 = 0;
            for (index, buffer) in resource.vertex_buffers.iter().enumerate() {
                render_pass.set_vertex_buffer(index as u32, buffer.buffer.slice(..));
                vertices += buffer.vertex_count;
            }

            render_pass.draw(0..vertices, 0..1);
        }
    }
}
