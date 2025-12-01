// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

use crevice::std430::AsStd430;
use glam::Mat4;
use macro_rules_attribute::derive;
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};
use wgpu;

use crate::{
    DualDevice, PreprocessorDirectivesTraits,
    gpu::{
        ShaderSource,
        resources::{BufferDescriptor, TextureView},
    },
    textures::evaluators::{
        FrameCounter, GPUTextureEvaluator, TextureEvaluator, TextureEvaluatorHashes,
    },
};

#[derive(Copy, Default, PreprocessorDirectivesTraits!)]
pub enum GradePreprocessorDirectives {
    #[default]
    None,
}

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GradeConstructionData {
    #[serde(skip_deserializing)]
    pub input_texture_view: Option<TextureView>,
}

impl Default for GradeConstructionData {
    fn default() -> Self {
        Self {
            input_texture_view: None,
        }
    }
}

impl GradeConstructionData {}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GPUGrade {
    flags: u32,
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    gamma: f32,
    inverse_transform: Mat4,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Grade {
    pub black_point: f32,
    pub white_point: f32,
    pub lift: f32,
    pub gain: f32,
    pub gamma: f32,
    pub invert: bool,
    pub transform: Mat4,
    pub frame_counter: FrameCounter,
    construction_data: GradeConstructionData,
    hashes: TextureEvaluatorHashes,
    preprocessor_directives: HashSet<GradePreprocessorDirectives>,
    #[serde(skip)]
    output_texture_view: Option<TextureView>,
}

impl Default for Grade {
    fn default() -> Self {
        Self {
            black_point: 0.,
            white_point: 1.,
            lift: 0.,
            gain: 1.,
            gamma: 1.,
            invert: false,
            transform: Mat4::IDENTITY,
            frame_counter: FrameCounter::default(),
            construction_data: GradeConstructionData::default(),
            hashes: TextureEvaluatorHashes::default(),
            preprocessor_directives: HashSet::<GradePreprocessorDirectives>::new(),
            output_texture_view: None,
        }
    }
}

impl Grade {
    pub fn black_point(mut self, black_point: f32) -> Self {
        self.black_point = black_point;
        self
    }

    pub fn white_point(mut self, white_point: f32) -> Self {
        self.white_point = white_point;
        self
    }

    pub fn lift(mut self, lift: f32) -> Self {
        self.lift = lift;
        self
    }

    pub fn gain(mut self, gain: f32) -> Self {
        self.gain = gain;
        self
    }

    pub fn gamma(mut self, gamma: f32) -> Self {
        self.gamma = gamma;
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    pub fn transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    pub fn input_texture_view(mut self, input_texture_view: TextureView) -> Self {
        self.construction_data.input_texture_view = Some(input_texture_view);
        self
    }
}

impl DualDevice<GPUGrade, Std430GPUGrade> for Grade {
    fn to_gpu(&self) -> GPUGrade {
        GPUGrade {
            flags: self.invert as u32,
            black_point: self.black_point,
            white_point: self.white_point,
            lift: self.lift,
            gain: self.gain,
            gamma: 1. / self.gamma,
            inverse_transform: self.transform.inverse(),
        }
    }
}

impl TextureEvaluator for Grade {
    fn label(&self) -> String {
        "grade".to_owned()
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

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.to_gpu())
    }

    fn input_texture_views(&self) -> Vec<TextureView> {
        self.construction_data
            .input_texture_view
            .iter()
            .cloned()
            .collect()
    }

    fn with_input_texture_views(self, mut input_texture_views: Vec<TextureView>) -> Self {
        if let Some(input_texture_view) = input_texture_views.pop() {
            self.input_texture_view(input_texture_view)
        } else {
            self
        }
    }

    fn output_texture_dimensions(&self) -> Option<wgpu::Extent3d> {
        if let Some(input_texture_view) = &self.construction_data.input_texture_view {
            Some(wgpu::Extent3d {
                width: input_texture_view.texture_view.texture().width(),
                height: input_texture_view.texture_view.texture().height(),
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

impl ShaderSource<GradePreprocessorDirectives> for Grade {
    fn vertex_shader_raw(&self) -> &str {
        include_str!("../../gpu/wgsl/textures/evaluators/grade/vertex_shader.wgsl")
    }

    fn fragment_shader_raw(&self) -> &str {
        include_str!("../../gpu/wgsl/textures/evaluators/grade/fragment_shader.wgsl")
    }

    fn current_directives(&self) -> &HashSet<GradePreprocessorDirectives> {
        &self.preprocessor_directives
    }

    fn current_directives_mut(&mut self) -> &mut HashSet<GradePreprocessorDirectives> {
        &mut self.preprocessor_directives
    }
}

impl GPUTextureEvaluator<GradePreprocessorDirectives> for Grade {
    fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.construction_data)
    }

    fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
        vec![BufferDescriptor {
            data: bytemuck::cast_slice(&[self.as_std430()]).to_vec(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            visibility: wgpu::ShaderStages::FRAGMENT,
        }]
    }

    fn create_texture_views(&self, _device: &wgpu::Device) -> Vec<TextureView> {
        self.input_texture_views()
    }
}
