// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Range,
    rc::Rc,
};

use wgpu;

pub trait BindingResource {
    fn as_resource(&self) -> wgpu::BindingResource<'_>;
}

pub trait TextureResource: BindingResource {
    fn format(&self) -> wgpu::TextureFormat;

    fn texture_view(&self) -> &wgpu::TextureView;

    fn colour_target_state(&self) -> wgpu::ColorTargetState {
        self.format().into()
    }

    fn buffer_size(&self) -> wgpu::BufferAddress {
        self.format()
            .theoretical_memory_footprint(self.texture_view().texture().size())
            as wgpu::BufferAddress
    }

    fn bytes_per_row(&self) -> u32 {
        self.format().theoretical_memory_footprint(wgpu::Extent3d {
            width: self.texture_view().texture().width(),
            height: 1,
            depth_or_array_layers: 1,
        }) as u32
    }

    fn rows_per_image(&self) -> u32 {
        self.texture_view().texture().height()
    }

    fn copy_to_buffer(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) -> wgpu::Buffer {
        let buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: self.buffer_size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("texture copy buffer"),
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture_view().texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row()),
                    rows_per_image: Some(self.rows_per_image()),
                },
            },
            self.texture_view().texture().size(),
        );

        buffer
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub buffer: wgpu::Buffer,
    pub visibility: wgpu::ShaderStages,
}

impl BindingResource for Buffer {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextureView {
    pub texture_view: wgpu::TextureView,
    pub data: Rc<Vec<u8>>,
    pub visibility: wgpu::ShaderStages,
    pub view_dimension: wgpu::TextureViewDimension,
    pub format: wgpu::TextureFormat,
}

impl serde::Serialize for TextureView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        serializer.serialize_u64(hasher.finish())
    }
}

impl BindingResource for TextureView {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}

impl TextureResource for TextureView {
    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

#[derive(Debug, Clone)]
pub struct StorageTextureView {
    pub texture_view: wgpu::TextureView,
    pub visibility: wgpu::ShaderStages,
    pub access: wgpu::StorageTextureAccess,
    pub view_dimension: wgpu::TextureViewDimension,
    pub format: wgpu::TextureFormat,
}

impl BindingResource for StorageTextureView {
    fn as_resource(&self) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.texture_view)
    }
}

impl TextureResource for StorageTextureView {
    fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TextureViewBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub texture_views: Vec<TextureView>,
}

impl TextureViewBindGroup {
    pub fn write(&self, queue: &wgpu::Queue) {
        for texture_view in self.texture_views.iter() {
            if !texture_view.data.is_empty() {
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture_view.texture_view.texture(),
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    texture_view.data.as_slice(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(texture_view.bytes_per_row()),
                        rows_per_image: Some(texture_view.rows_per_image()),
                    },
                    texture_view.texture_view.texture().size(),
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageTextureViewBindGroup {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub storage_texture_views: Vec<StorageTextureView>,
}

#[derive(Debug, Clone)]
pub struct BindGroups {
    pub vertex_bind_group: BufferBindGroup,
    pub uniform_bind_group: Option<BufferBindGroup>,
    pub storage_bind_group: Option<BufferBindGroup>,
    pub texture_bind_group: Option<TextureViewBindGroup>,
    pub storage_texture_bind_group: Option<StorageTextureViewBindGroup>,
}

impl BindGroups {
    pub fn num_bind_groups(&self) -> u32 {
        let mut bind_group_count: u32 = 1;

        if let Some(_uniform_bind_group) = &self.uniform_bind_group {
            bind_group_count += 1;
        }
        if let Some(_storage_bind_group) = &self.storage_bind_group {
            bind_group_count += 1;
        }
        if let Some(_texture_bind_group) = &self.texture_bind_group {
            bind_group_count += 1;
        }
        if let Some(_storage_texture_bind_group) = &self.storage_texture_bind_group {
            bind_group_count += 1;
        }
        bind_group_count
    }

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

#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    pub data: Vec<u8>,
    pub usage: wgpu::BufferUsages,
    pub visibility: wgpu::ShaderStages,
}

#[derive(Debug, Default, Clone)]
pub struct BufferData {
    pub vertex: Vec<BufferDescriptor>,
    pub uniform: Vec<BufferDescriptor>,
    pub storage: Vec<BufferDescriptor>,
}

#[derive(Debug, Clone)]
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

        render_pass.draw_indexed(
            self.index_count.clone(),
            self.base_vertex,
            self.instance_count.clone(),
        );
    }
}

#[derive(Debug, Clone)]
pub struct RenderResources {
    pub resources: Vec<Option<RenderResource>>,
}

impl RenderResources {
    pub fn new(resources: Vec<Option<RenderResource>>) -> Self {
        Self { resources }
    }
}
