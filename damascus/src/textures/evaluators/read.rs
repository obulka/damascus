// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

// use std::collections::HashSet;

// use crevice::std430::AsStd430;
// use glam::{UVec2, Vec2};
// use image::{ImageReader, Rgba32FImage};
// use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};
// use wgpu;

// use crate::{
//     DualDevice,
//     gpu::{
//         ShaderSource,
//         resources::{BufferDescriptor, TextureView},
//         texture::view::{
//             TEXTURE_VIEWER_FRAGMENT_SHADER, TEXTURE_VIEWER_VERTEX_SHADER,
//             TextureReaderPreprocessorDirectives,
//         },
//     },
//     textures::evaluators::{FrameCounter, GPUTextureEvaluator, TextureEvaluatorHashes},
// };

// // A change in the data within this struct will trigger the pass to
// // reconstruct its pipeline
// #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
// pub struct TextureReaderConstructionData {
//     pub layers: u32,
//     pub filepath: String,
// }

// impl Default for TextureReaderConstructionData {
//     fn default() -> Self {
//         Self {
//             layers: 1,
//             filepath: String::new(),
//         }
//     }
// }

// impl TextureReaderConstructionData {}

// #[repr(C)]
// #[derive(Debug, Copy, Clone, AsStd430, PartialEq, serde::Serialize, serde::Deserialize)]
// pub struct GPUTextureReaderRenderData {
//     resolution: Vec2,
//     frame: u32,
//     flags: u32,
// }

// #[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(default)]
// pub struct TextureReaderRenderData {
//     pub resolution: UVec2,
//     pub frame: u32,
// }

// impl Default for TextureReaderRenderData {
//     fn default() -> Self {
//         Self {
//             resolution: UVec2::ZERO,
//             frame: 1001,
//         }
//     }
// }

// impl TextureReaderRenderData {}

// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// #[serde(default)]
// pub struct TextureReader {
//     pub render_data: TextureReaderRenderData,
//     pub construction_data: TextureReaderConstructionData,
//     pub frame_counter: FrameCounter,
//     hashes: TextureEvaluatorHashes,
//     preprocessor_directives: HashSet<TextureReaderPreprocessorDirectives>,
// }

// impl Default for TextureReader {
//     fn default() -> Self {
//         Self {
//             render_data: TextureReaderRenderData::default(),
//             construction_data: TextureReaderConstructionData::default(),
//             frame_counter: FrameCounter::default(),
//             hashes: TextureEvaluatorHashes::default(),
//         }
//     }
// }

// impl GPUTextureEvaluator<TextureReaderPreprocessorDirectives> for TextureReader {
//     fn label(&self) -> String {
//         "texture viewer".to_owned()
//     }

//     fn hashes(&self) -> &TextureEvaluatorHashes {
//         &self.hashes
//     }

//     fn hashes_mut(&mut self) -> &mut TextureEvaluatorHashes {
//         &mut self.hashes
//     }

//     fn create_reconstruction_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
//         to_key_with_ordered_float(&self.construction_data)
//     }

//     fn frame_counter(&self) -> &FrameCounter {
//         &self.frame_counter
//     }

//     fn frame_counter_mut(&mut self) -> &mut FrameCounter {
//         &mut self.frame_counter
//     }

//     fn uniform_buffer_data(&self) -> Vec<BufferDescriptor> {
//         vec![
//             BufferDescriptor {
//                 data: bytemuck::cast_slice(&[self.render_data.as_std430()]).to_vec(),
//                 usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
//                 visibility: wgpu::ShaderStages::VERTEX,
//             },
//             BufferDescriptor {
//                 data: bytemuck::cast_slice(&[self.as_std430()]).to_vec(),
//                 usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
//                 visibility: wgpu::ShaderStages::VERTEX,
//             },
//             BufferDescriptor {
//                 data: bytemuck::cast_slice(&[self.grade.as_std430()]).to_vec(),
//                 usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
//                 visibility: wgpu::ShaderStages::FRAGMENT,
//             },
//         ]
//     }

//     fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
//         let mut width: u32 = 10;
//         let mut height: u32 = 10;
//         let mut texture_data = Rgba32FImage::new(width, height);
//         if let Ok(image) = ImageReader::open(&self.construction_data.texture.filepath) {
//             if let Ok(decoded_image) = image.decode() {
//                 texture_data = decoded_image.to_rgba32f();
//                 (width, height) = texture_data.dimensions();
//             }
//         }

//         let texture_descriptor = wgpu::TextureDescriptor {
//             size: wgpu::Extent3d {
//                 width: width,
//                 height: height,
//                 depth_or_array_layers: self.construction_data.texture.layers,
//             },
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: wgpu::TextureDimension::D2,
//             format: wgpu::TextureFormat::Rgba32Float,
//             usage: wgpu::TextureUsages::COPY_DST
//                 | wgpu::TextureUsages::COPY_SRC
//                 | wgpu::TextureUsages::RENDER_ATTACHMENT
//                 | wgpu::TextureUsages::TEXTURE_BINDING,
//             label: Some("texture view"),
//             view_formats: &[],
//         };
//         let texture: wgpu::Texture = device.create_texture(&texture_descriptor);
//         let texture_view: wgpu::TextureView = texture.create_view(&Default::default());
//         vec![TextureView {
//             texture_view: texture_view,
//             texture_data: texture_data,
//             visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
//             view_dimension: wgpu::TextureViewDimension::D2,
//         }]
//     }
// }

// impl TextureReader {
//     pub fn texture(mut self, texture: TextureRead) -> Self {
//         self.construction_data.texture = texture;
//         self
//     }
// }
// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureRead {
    pub layers: u32,
    pub filepath: String,
}

impl Default for TextureRead {
    fn default() -> Self {
        Self {
            layers: 1,
            filepath: String::new(),
        }
    }
}
