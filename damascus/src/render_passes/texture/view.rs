// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2};
use image::{ImageReader, Rgba32FImage};
use serde_hashkey::{to_key_with_ordered_float, Error, Key, OrderedFloatPolicy, Result};
use wgpu;

use crate::{
    render_passes::{
        resources::{BufferDescriptor, TextureView},
        FrameCounter, RenderPass, RenderPassHashes,
    },
    shaders::{
        texture::view::{
            TextureViewerPreprocessorDirectives, TEXTURE_VIEWER_FRAGMENT_SHADER,
            TEXTURE_VIEWER_VERTEX_SHADER,
        },
        ShaderSource,
    },
    textures::{Grade, Texture},
    DualDevice,
};

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextureViewerConstructionData {
    pub texture: Texture,
}

impl Default for TextureViewerConstructionData {
    fn default() -> Self {
        Self {
            texture: Texture::default(),
        }
    }
}

impl TextureViewerConstructionData {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUTextureViewerRenderData {
    resolution: Vec2,
    frame: u32,
    flags: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewerRenderData {
    pub resolution: UVec2,
    pub frame: u32,
}

impl Default for TextureViewerRenderData {
    fn default() -> Self {
        Self {
            resolution: UVec2::ZERO,
            frame: 1001,
        }
    }
}

impl TextureViewerRenderData {}

impl DualDevice<GPUTextureViewerRenderData, Std430GPUTextureViewerRenderData>
    for TextureViewerRenderData
{
    fn to_gpu(&self) -> GPUTextureViewerRenderData {
        GPUTextureViewerRenderData {
            resolution: self.resolution.as_vec2(),
            frame: self.frame,
            flags: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUTextureViewer {
    pan: Vec2,
    zoom: f32,
    flags: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewer {
    pub render_data: TextureViewerRenderData,
    pub construction_data: TextureViewerConstructionData,
    pub pan: Vec2,
    pub zoom: f32,
    pub grade: Grade,
    pub frame_counter: FrameCounter,
    hashes: RenderPassHashes,
    preprocessor_directives: HashSet<TextureViewerPreprocessorDirectives>,
}

impl Default for TextureViewer {
    fn default() -> Self {
        Self {
            render_data: TextureViewerRenderData::default(),
            construction_data: TextureViewerConstructionData::default(),
            pan: Vec2::ZERO,
            zoom: 1.0,
            grade: Grade::default(),
            frame_counter: FrameCounter::default(),
            hashes: RenderPassHashes::default(),
            preprocessor_directives: HashSet::<TextureViewerPreprocessorDirectives>::new(), //TODO update the directives here
        }
    }
}

impl DualDevice<GPUTextureViewer, Std430GPUTextureViewer> for TextureViewer {
    fn to_gpu(&self) -> GPUTextureViewer {
        GPUTextureViewer {
            pan: self.pan,
            zoom: self.zoom,
            flags: 0,
        }
    }
}

impl ShaderSource<TextureViewerPreprocessorDirectives> for TextureViewer {
    fn vertex_shader_raw(&self) -> &str {
        TEXTURE_VIEWER_VERTEX_SHADER
    }

    fn fragment_shader_raw(&self) -> &str {
        TEXTURE_VIEWER_FRAGMENT_SHADER
    }

    fn current_directives(&self) -> &HashSet<TextureViewerPreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_directives_mut(&mut self) -> &mut HashSet<TextureViewerPreprocessorDirectives> {
        &mut self.preprocessor_directives
    }
}

impl RenderPass<TextureViewerPreprocessorDirectives> for TextureViewer {
    fn label(&self) -> String {
        "texture viewer".to_owned()
    }

    fn hashes(&self) -> &RenderPassHashes {
        &self.hashes
    }

    fn hashes_mut(&mut self) -> &mut RenderPassHashes {
        &mut self.hashes
    }

    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.construction_data)
    }

    fn frame_counter(&self) -> &FrameCounter {
        &self.frame_counter
    }

    fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        &mut self.frame_counter
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.render_data.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::VERTEX,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::VERTEX,
            },
            BufferDescriptor {
                data: bytemuck::cast_slice(&[self.grade.as_std430()]).to_vec(),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
        let mut width: u32 = 10;
        let mut height: u32 = 10;
        let mut texture_data = Rgba32FImage::new(width, height);
        if let Ok(image) = ImageReader::open(&self.construction_data.texture.filepath) {
            if let Ok(decoded_image) = image.decode() {
                texture_data = decoded_image.to_rgba32f();
                (width, height) = texture_data.dimensions();
            }
        }

        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: self.construction_data.texture.layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("texture view"),
            view_formats: &[],
        };
        let texture: wgpu::Texture = device.create_texture(&texture_descriptor);
        let texture_view: wgpu::TextureView = texture.create_view(&Default::default());
        vec![TextureView {
            texture: texture,
            texture_view: texture_view,
            texture_data: texture_data,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            view_dimension: wgpu::TextureViewDimension::D2,
        }]
    }
}

impl TextureViewer {
    pub fn texture(mut self, texture: Texture) -> Self {
        self.construction_data.texture = texture;
        self
    }
}
