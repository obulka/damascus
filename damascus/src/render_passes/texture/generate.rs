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
        RenderPass, RenderPassHashes,
    },
    shaders,
    textures::Texture,
    DualDevice,
};

pub trait TextureGenerationPass<Directives: shaders::PreprocessorDirectives>:
    RenderPass<Directives>
{
    fn label(&self) -> String {
        "texture generation".to_owned()
    }

    fn descriptors(&self) -> Vec<wgpu::TextureDescriptor<'_>>;

    // fn create_texture_views(&self, device: &wgpu::Device) -> Vec<TextureView> {
    //     self.descriptors()
    //         .iter()
    //         .map(|descriptor| {
    //             let texture: wgpu::Texture = device.create_texture(&descriptor);
    //             let texture_view: wgpu::TextureView = texture.create_view(&Default::default());
    //             TextureView {
    //                 texture: texture,
    //                 texture_view: texture_view,
    //                 texture_data: texture_data,
    //                 visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
    //                 view_dimension: wgpu::TextureViewDimension::D2,
    //                 size: descriptor.size,
    //             }
    //         })
    //         .collect()
    // }
}
