// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::BTreeSet;

use crevice::std430::AsStd430;
use glam::{UVec2, Vec2};
use macro_rules_attribute::derive;
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};
use wgpu;

use crate::{
    DualDevice, PreprocessorDirectivesTraits,
    gpu::{
        ShaderSource,
        resources::{BufferDescriptor, RenderResource, TextureView},
    },
    textures::evaluators::{
        FrameCounter, GPUTextureEvaluator, TextureEvaluator, TextureEvaluatorHashes, grade::Grade,
    },
};

#[derive(Copy, Default, PreprocessorDirectivesTraits!)]
pub enum TextureViewerPreprocessorDirectives {
    #[default]
    None,
}

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextureViewerConstructionData {
    #[serde(skip_deserializing)]
    pub input_texture_views: Vec<TextureView>,
}

impl Default for TextureViewerConstructionData {
    fn default() -> Self {
        Self {
            input_texture_views: vec![],
        }
    }
}

impl TextureViewerConstructionData {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUTextureViewer {
    pan: Vec2,
    zoom: f32,
    flags: u32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureViewer {
    pub pan: Vec2,
    pub zoom: f32,
    pub grade: Grade,
    pub frame_counter: FrameCounter,
    render_data: TextureViewerRenderData,
    construction_data: TextureViewerConstructionData,
    hashes: TextureEvaluatorHashes,
    preprocessor_directives: BTreeSet<TextureViewerPreprocessorDirectives>,
    #[serde(skip)]
    output_texture_view: Option<TextureView>,
    #[serde(skip)]
    render_resource: Option<RenderResource>,
}

impl TextureViewer {
    pub fn set_input_texture_view(&mut self, input_texture_view: TextureView) {
        // TODO this should be the viewport, not texture, resolution
        self.render_data.resolution = UVec2::new(
            input_texture_view.texture_view.texture().width(),
            input_texture_view.texture_view.texture().height(),
        );
        self.construction_data.input_texture_views = vec![input_texture_view];
    }

    pub fn with_input_texture_view(mut self, input_texture_view: TextureView) -> Self {
        self.set_input_texture_view(input_texture_view);
        self
    }

    pub fn grade(mut self, grade: Grade) -> Self {
        self.grade = grade;
        self
    }
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
            hashes: TextureEvaluatorHashes::default(),
            preprocessor_directives: BTreeSet::<TextureViewerPreprocessorDirectives>::new(),
            output_texture_view: None,
            render_resource: None,
        }
    }
}

impl TextureEvaluator for TextureViewer {
    fn label(&self) -> String {
        "texture viewer".to_owned()
    }

    fn hashes(&self) -> &TextureEvaluatorHashes {
        &self.hashes
    }

    fn hashes_mut(&mut self) -> &mut TextureEvaluatorHashes {
        &mut self.hashes
    }

    fn frame_counter(&self) -> &FrameCounter {
        &self.frame_counter
    }

    fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        &mut self.frame_counter
    }

    fn input_texture_views(&self) -> Vec<TextureView> {
        self.construction_data.input_texture_views.clone()
    }

    fn set_input_texture_views(&mut self, mut input_texture_views: Vec<TextureView>) {
        if let Some(input_texture_view) = input_texture_views.pop() {
            self.set_input_texture_view(input_texture_view);
        }
    }

    fn output_texture_dimensions(&self) -> Option<wgpu::Extent3d> {
        if !self.construction_data.input_texture_views.is_empty() {
            Some(wgpu::Extent3d {
                width: self.construction_data.input_texture_views[0]
                    .texture_view
                    .texture()
                    .width(),
                height: self.construction_data.input_texture_views[0]
                    .texture_view
                    .texture()
                    .height(),
                depth_or_array_layers: 1,
            })
        } else {
            None
        }
    }

    fn set_output_texture_view(&mut self, output_texture_view: TextureView) {
        self.output_texture_view = Some(output_texture_view);
    }

    fn output_texture_view(&self) -> Option<&TextureView> {
        self.output_texture_view.as_ref()
    }

    fn output_texture_view_mut(&mut self) -> Option<&mut TextureView> {
        self.output_texture_view.as_mut()
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
        include_str!("../../gpu/wgsl/textures/evaluators/view/vertex_shader.wgsl")
    }

    fn fragment_shader_raw(&self) -> &str {
        include_str!("../../gpu/wgsl/textures/evaluators/view/fragment_shader.wgsl")
    }

    fn current_directives(&self) -> &BTreeSet<TextureViewerPreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_directives_mut(&mut self) -> &mut BTreeSet<TextureViewerPreprocessorDirectives> {
        &mut self.preprocessor_directives
    }
}

impl GPUTextureEvaluator<TextureViewerPreprocessorDirectives> for TextureViewer {
    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.construction_data)
    }

    fn render_resource(&self) -> &Option<RenderResource> {
        &self.render_resource
    }

    fn render_resource_mut(&mut self) -> &mut Option<RenderResource> {
        &mut self.render_resource
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

    fn create_texture_views(&self, _device: &wgpu::Device) -> Vec<TextureView> {
        self.input_texture_views()
    }
}
