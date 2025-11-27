// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::UVec2;
use image::{GenericImageView, ImageReader};
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};

use crate::{
    gpu::resources::TextureView,
    textures::evaluators::{FrameCounter, TextureEvaluator, TextureEvaluatorHashes},
};

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

// A change in the data within this struct will trigger the pass to
// reconstruct its pipeline
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextureReaderRenderData {
    pub layers: u32,
    pub filepath: String,
    pub resolution: UVec2,
    pub frame: u32,
}

impl Default for TextureReaderRenderData {
    fn default() -> Self {
        Self {
            layers: 1,
            filepath: String::new(),
            resolution: UVec2::ZERO,
            frame: 1001,
        }
    }
}

impl TextureReaderRenderData {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TextureReader {
    pub render_data: TextureReaderRenderData,
    pub frame_counter: FrameCounter,
    hashes: TextureEvaluatorHashes,
}

impl Default for TextureReader {
    fn default() -> Self {
        Self {
            render_data: TextureReaderRenderData::default(),
            frame_counter: FrameCounter::default(),
            hashes: TextureEvaluatorHashes::default(),
        }
    }
}

impl TextureEvaluator for TextureReader {
    fn label(&self) -> String {
        "texture reader".to_owned()
    }

    fn hashes(&self) -> &TextureEvaluatorHashes {
        &self.hashes
    }

    fn hashes_mut(&mut self) -> &mut TextureEvaluatorHashes {
        &mut self.hashes
    }

    fn create_reset_hash(&mut self) -> Result<Key<OrderedFloatPolicy>, Error> {
        to_key_with_ordered_float(&self.render_data)
    }

    fn frame_counter(&self) -> &FrameCounter {
        &self.frame_counter
    }

    fn frame_counter_mut(&mut self) -> &mut FrameCounter {
        &mut self.frame_counter
    }

    fn output_texture_dimensions(&self) -> Option<wgpu::Extent3d> {
        if self.render_data.resolution.x == 0 && self.render_data.resolution.y == 0 {
            None
        } else {
            Some(wgpu::Extent3d {
                width: self.render_data.resolution.x,
                height: self.render_data.resolution.y,
                depth_or_array_layers: self.render_data.layers,
            })
        }
    }

    fn evaluate_texture(&mut self, device: &wgpu::Device) -> Option<TextureView> {
        if let Some(mut texture_view) = self.create_output_texture_view(device)
            && let Ok(image) = ImageReader::open(&self.render_data.filepath)
            && let Ok(decoded_image) = image.decode()
        {
            let (width, height) = decoded_image.dimensions();
            self.render_data.resolution = UVec2::new(width, height);

            match texture_view.format {
                wgpu::TextureFormat::R8Unorm
                | wgpu::TextureFormat::R8Snorm
                | wgpu::TextureFormat::R8Uint
                | wgpu::TextureFormat::R8Sint
                | wgpu::TextureFormat::Stencil8 => {
                    texture_view.data = bytemuck::cast_slice(&decoded_image.into_luma8()).to_vec();
                }
                wgpu::TextureFormat::R16Uint
                | wgpu::TextureFormat::R16Sint
                | wgpu::TextureFormat::R16Unorm
                | wgpu::TextureFormat::R16Snorm
                | wgpu::TextureFormat::Depth16Unorm => {
                    texture_view.data = bytemuck::cast_slice(&decoded_image.into_luma16()).to_vec();
                }
                // wgpu::TextureFormat::R16Float,
                wgpu::TextureFormat::Rg8Unorm
                | wgpu::TextureFormat::Rg8Snorm
                | wgpu::TextureFormat::Rg8Uint
                | wgpu::TextureFormat::Rg8Sint => {
                    texture_view.data =
                        bytemuck::cast_slice(&decoded_image.into_luma_alpha8()).to_vec();
                }
                // wgpu::TextureFormat::R32Uint
                // | wgpu::TextureFormat::R32Sint,
                wgpu::TextureFormat::R32Float | wgpu::TextureFormat::Depth32Float => {
                    texture_view.data = bytemuck::cast_slice(&decoded_image.to_luma32f()).to_vec();
                }
                wgpu::TextureFormat::Rg16Uint
                | wgpu::TextureFormat::Rg16Sint
                | wgpu::TextureFormat::Rg16Unorm
                | wgpu::TextureFormat::Rg16Snorm => {
                    texture_view.data =
                        bytemuck::cast_slice(&decoded_image.into_luma_alpha16()).to_vec();
                }
                // wgpu::TextureFormat::Rg16Float,
                wgpu::TextureFormat::Rgba8Unorm
                | wgpu::TextureFormat::Rgba8UnormSrgb
                | wgpu::TextureFormat::Rgba8Snorm
                | wgpu::TextureFormat::Rgba8Uint
                | wgpu::TextureFormat::Rgba8Sint => {
                    texture_view.data = bytemuck::cast_slice(&decoded_image.into_rgba8()).to_vec();
                }
                // wgpu::TextureFormat::Bgra8Unorm,
                // wgpu::TextureFormat::Bgra8UnormSrgb,
                // wgpu::TextureFormat::Rgb9e5Ufloat,
                // wgpu::TextureFormat::Rgb10a2Uint,
                // wgpu::TextureFormat::Rgb10a2Unorm,
                // wgpu::TextureFormat::Rg11b10Ufloat,
                // wgpu::TextureFormat::R64Uint,
                // wgpu::TextureFormat::Rg32Uint,
                // wgpu::TextureFormat::Rg32Sint,
                wgpu::TextureFormat::Rg32Float => {
                    texture_view.data =
                        bytemuck::cast_slice(&decoded_image.to_luma_alpha32f()).to_vec();
                }
                // wgpu::TextureFormat::Rgba16Uint,
                // wgpu::TextureFormat::Rgba16Sint,
                // wgpu::TextureFormat::Rgba16Unorm,
                // wgpu::TextureFormat::Rgba16Snorm,
                // wgpu::TextureFormat::Rgba16Float,
                // wgpu::TextureFormat::Rgba32Uint,
                // wgpu::TextureFormat::Rgba32Sint,
                wgpu::TextureFormat::Rgba32Float => {
                    texture_view.data =
                        bytemuck::cast_slice(&decoded_image.into_rgba32f()).to_vec();
                }
                // wgpu::TextureFormat::Depth24Plus,
                // wgpu::TextureFormat::Depth24PlusStencil8,
                // wgpu::TextureFormat::Depth32FloatStencil8,
                // wgpu::TextureFormat::NV12,
                // wgpu::TextureFormat::P010,
                // wgpu::TextureFormat::Bc1RgbaUnorm,
                // wgpu::TextureFormat::Bc1RgbaUnormSrgb,
                // wgpu::TextureFormat::Bc2RgbaUnorm,
                // wgpu::TextureFormat::Bc2RgbaUnormSrgb,
                // wgpu::TextureFormat::Bc3RgbaUnorm,
                // wgpu::TextureFormat::Bc3RgbaUnormSrgb,
                // wgpu::TextureFormat::Bc4RUnorm,
                // wgpu::TextureFormat::Bc4RSnorm,
                // wgpu::TextureFormat::Bc5RgUnorm,
                // wgpu::TextureFormat::Bc5RgSnorm,
                // wgpu::TextureFormat::Bc6hRgbUfloat,
                // wgpu::TextureFormat::Bc6hRgbFloat,
                // wgpu::TextureFormat::Bc7RgbaUnorm,
                // wgpu::TextureFormat::Bc7RgbaUnormSrgb,
                // wgpu::TextureFormat::Etc2Rgb8Unorm,
                // wgpu::TextureFormat::Etc2Rgb8UnormSrgb,
                // wgpu::TextureFormat::Etc2Rgb8A1Unorm,
                // wgpu::TextureFormat::Etc2Rgb8A1UnormSrgb,
                // wgpu::TextureFormat::Etc2Rgba8Unorm,
                // wgpu::TextureFormat::Etc2Rgba8UnormSrgb,
                // wgpu::TextureFormat::EacR11Unorm,
                // wgpu::TextureFormat::EacR11Snorm,
                // wgpu::TextureFormat::EacRg11Unorm,
                // wgpu::TextureFormat::EacRg11Snorm,
                // wgpu::TextureFormat::Astc {
                //     block: AstcBlock,
                //     channel: AstcChannel,
                // },
                _ => {
                    return None;
                }
            }

            Some(texture_view)
        } else {
            None
        }
    }
}

impl TextureReader {
    pub fn filepath(mut self, filepath: String) -> Self {
        self.render_data.filepath = filepath;
        self
    }
}
