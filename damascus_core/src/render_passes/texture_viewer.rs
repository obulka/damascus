// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::HashSet, time::SystemTime};

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2};
use image::{ImageReader, Rgba32FImage};
use serde_hashkey::{Key, OrderedFloatPolicy};

use super::{
    resources::{Buffer, TextureView},
    RenderPass, TextureProcessingPass,
};

use crate::{
    shaders::{
        texture_viewer::{
            TextureViewerPreprocessorDirectives, TEXTURE_VIEWER_FRAGMENT_SHADER,
            TEXTURE_VIEWER_VERTEX_SHADER,
        },
        ShaderSource,
    },
    textures::{GPUTextureVertex, Grade, Std430GPUTextureVertex, Texture, TextureVertex},
    DualDevice, Hashable,
};

// A change in the data within this struct will trigger the pass to
// recompile
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewerCompilationData {}

impl Default for TextureViewerCompilationData {
    fn default() -> Self {
        Self {}
    }
}

impl TextureViewerCompilationData {}

impl Hashable for TextureViewerCompilationData {}

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TextureViewerPipelineData {
    pub texture: Texture,
}

impl Hashable for TextureViewerPipelineData {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUTextureViewerRenderData {
    resolution: Vec2,
    pan: Vec2,
    zoom: f32,
    flags: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewerRenderData {
    pub frame_counter: u32,
    pub previous_frame_time: SystemTime,
    pub fps: f32,
    pub resolution: UVec2,
    pub pan: Vec2,
    pub zoom: f32,
    pub grade: Grade,
    pub paused: bool,
}

impl Default for TextureViewerRenderData {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            previous_frame_time: SystemTime::now(),
            fps: 0.,
            resolution: UVec2::ZERO,
            pan: Vec2::ZERO,
            zoom: 1.0,
            grade: Grade::default(),
            paused: true,
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
            pan: self.pan,
            zoom: self.zoom,
            flags: 0,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewer {
    pub render_data: TextureViewerRenderData,
    pub pipeline_data: TextureViewerPipelineData,
    recompile_hash: Key<OrderedFloatPolicy>,
    reconstruct_hash: Key<OrderedFloatPolicy>,
    preprocessor_directives: HashSet<TextureViewerPreprocessorDirectives>,
}

impl Default for TextureViewer {
    fn default() -> Self {
        TextureViewer {
            render_data: TextureViewerRenderData::default(),
            pipeline_data: TextureViewerPipelineData::default(),
            recompile_hash: Key::<OrderedFloatPolicy>::Unit,
            reconstruct_hash: Key::<OrderedFloatPolicy>::Unit,
            preprocessor_directives: HashSet::<TextureViewerPreprocessorDirectives>::new(),
        }
    }
}

impl ShaderSource<TextureViewerPreprocessorDirectives> for TextureViewer {
    fn vertex_shader_raw(&self) -> String {
        TEXTURE_VIEWER_VERTEX_SHADER
    }

    fn fragment_shader_raw(&self) -> String {
        TEXTURE_VIEWER_FRAGMENT_SHADER
    }

    fn current_directives(&self) -> &HashSet<TextureViewerPreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_directives_mut(&mut self) -> &mut HashSet<TextureViewerPreprocessorDirectives> {
        &mut self.preprocessor_directives
    }
}

impl
    RenderPass<
        TextureViewerCompilationData,
        TextureViewerPipelineData,
        TextureVertex,
        GPUTextureVertex,
        Std430GPUTextureVertex,
        TextureViewerPreprocessorDirectives,
    > for TextureViewer
{
    fn compilation_data(&self) -> &TextureViewerCompilationData {
        &TextureViewerCompilationData {}
    }

    fn pipeline_data(&self) -> &TextureViewerPipelineData {
        &self.pipeline_data
    }

    fn label(&self) -> String {
        "texture viewer"
    }

    fn recompile_hash(&self) -> &Key<OrderedFloatPolicy> {
        &self.recompile_hash
    }

    fn recompile_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy> {
        &mut self.recompile_hash
    }

    fn reconstruct_hash(&self) -> &Key<OrderedFloatPolicy> {
        &self.reconstruct_hash
    }

    fn reconstruct_hash_mut(&mut self) -> &mut Key<OrderedFloatPolicy> {
        &mut self.reconstruct_hash
    }

    fn create_uniform_buffers(&self, device: &wgpu::Device) -> Vec<Buffer> {
        vec![
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("compositor render parameter buffer"),
                    contents: bytemuck::cast_slice(&[self.render_data.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            },
            Buffer {
                buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("viewer grade buffer"),
                    contents: bytemuck::cast_slice(&[self.grade.as_std430()]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                }),
                visibility: wgpu::ShaderStages::FRAGMENT,
            },
        ]
    }

    fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
        let mut width: u32 = 10;
        let mut height: u32 = 10;
        let mut texture_data = Rgba32FImage::new(width, height);
        if let Ok(image) = ImageReader::open(&self.pipeline_data.texture.filepath) {
            if let Ok(decoded_image) = image.decode() {
                texture_data = decoded_image.to_rgba32f();
                (width, height) = texture_data.dimensions();
            }
        }

        let size = wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: self.pipeline_data.texture.layers,
        };
        let texture_descriptor = wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("compositor texture"),
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
            size: size,
        }]
    }
}

impl TextureProcessingPass for TextureViewer {}
