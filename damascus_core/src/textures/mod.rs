// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;

use glam;

use crate::DualDevice;

/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
pub fn uv_to_screen(pixel_coordinates: glam::Vec2, resolution: glam::Vec2) -> glam::Vec2 {
    return (pixel_coordinates + glam::Vec2::ONE) * resolution * 0.5;
}

/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
pub fn screen_to_uv(pixel_coordinates: glam::Vec2, resolution: glam::Vec2) -> glam::Vec2 {
    return scale_screen_to_uv(pixel_coordinates, resolution) - glam::Vec2::ONE;
}

/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
pub fn scale_uv_to_screen(pixel_coordinates: glam::Vec2, resolution: glam::Vec2) -> glam::Vec2 {
    return pixel_coordinates * resolution * 0.5;
}

/**
 * Convert location of a pixel in screen/image space from uvs.
 *
 * @arg pixel_coordinates: The x, and y positions of the pixel in uv space.
 * @arg resolution: The image width, and height.
 *
 * @returns: The pixel indices.
 */
pub fn scale_screen_to_uv(pixel_coordinates: glam::Vec2, resolution: glam::Vec2) -> glam::Vec2 {
    return pixel_coordinates * 2. / resolution;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUGrade {
    black_point: f32,
    white_point: f32,
    lift: f32,
    gain: f32,
    gamma: f32,
    flags: u32,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Grade {
    pub black_point: f32,
    pub white_point: f32,
    pub lift: f32,
    pub gain: f32,
    pub gamma: f32,
    pub invert: bool,
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
        }
    }
}

impl DualDevice<GPUGrade, Std430GPUGrade> for Grade {
    fn to_gpu(&self) -> GPUGrade {
        GPUGrade {
            black_point: self.black_point,
            white_point: self.white_point,
            lift: self.lift,
            gain: self.gain,
            gamma: self.gamma,
            flags: self.invert as u32,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Texture {
    pub layers: u32,
    pub filepath: String,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            layers: 1,
            filepath: String::new(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsStd430)]
pub struct GPUVertex {
    texture_coordinate: glam::Vec2,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Vertex {
    pub texture_coordinate: glam::Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            texture_coordinate: glam::Vec2::ZERO,
        }
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            texture_coordinate: glam::Vec2::new(x, y),
        }
    }
}

impl DualDevice<GPUVertex, Std430GPUVertex> for Vertex {
    fn to_gpu(&self) -> GPUVertex {
        GPUVertex {
            texture_coordinate: self.texture_coordinate,
        }
    }
}
