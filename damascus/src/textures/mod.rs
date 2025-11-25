// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use image::{ImageBuffer, Luma, Primitive, Rgb, Rgba};
use macro_rules_attribute::derive;

use crate::EnumHashTraits;

pub mod evaluators;

#[derive(Copy, Default, EnumHashTraits!)]
pub enum AOVs {
    #[default]
    Beauty,
    WorldPosition,
    LocalPosition,
    Normals,
    Depth,
    Cryptomatte,
    Stats,
}

// pub enum TextureFormats {
//     SingleChannelTexture,//(ImageBuffer<Luma<ChannelType>, Vec<ChannelType>>),
//     // UVTexture(ImageBuffer<[ChannelType; 2], Vec<ChannelType>>),
//     RGBTexture,//(ImageBuffer<Rgb<ChannelType>, Vec<ChannelType>>),
//     RGBATexture,//(ImageBuffer<Rgba<ChannelType>, Vec<ChannelType>>),
// }

// pub struct Texture {
//     texture_format: TextureFormats,
//     buffer: ImageBuffer<PixelType, Vec<ChannelType>>,
// }

// impl Texture {
//     fn buffer<PixelType: image::Pixel, Vec<ChannelType>>(&self) -> ImageBuffer<PixelType, Vec<ChannelType>> {

//     }
// }
