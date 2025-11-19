// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::UVec2;
use image::{ImageReader, Rgba32FImage};
use serde_hashkey::{Error, Key, OrderedFloatPolicy, Result, to_key_with_ordered_float};

use crate::textures::evaluators::{FrameCounter, TextureEvaluator, TextureEvaluatorHashes};

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
                depth_or_array_layers: 1, // TODO could output AOVs to other layers
            })
        }
    }

    fn render_to_texture(&mut self, texture: &mut Rgba32FImage) {
        let (mut width, mut height) = texture.dimensions();
        if let Ok(image) = ImageReader::open(&self.render_data.filepath) {
            if let Ok(decoded_image) = image.decode() {
                *texture = decoded_image.to_rgba32f();
                (width, height) = texture.dimensions();
            }
        }
        self.render_data.resolution = UVec2::new(width, height);
    }
}

impl TextureReader {
    pub fn filepath(mut self, filepath: String) -> Self {
        self.render_data.filepath = filepath;
        self
    }
}
