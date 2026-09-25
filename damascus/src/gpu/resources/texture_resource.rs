// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::future::Future;

use image::{
    GrayAlphaImage, GrayImage, ImageBuffer, ImageError, Luma, Rgba32FImage, RgbaImage,
    error::{LimitError, LimitErrorKind},
};
use wgpu;

use crate::gpu::GPUResult;

use super::BindingResource;

pub trait TextureResource: BindingResource {
    fn format(&self) -> wgpu::TextureFormat;

    fn texture_view(&self) -> &wgpu::TextureView;

    fn colour_target_state(&self) -> wgpu::ColorTargetState {
        self.format().into()
    }

    fn buffer_size(&self) -> wgpu::BufferAddress {
        self.format()
            .theoretical_memory_footprint(self.texture_view().texture().size())
            as wgpu::BufferAddress
    }

    fn bytes_per_row(&self) -> u32 {
        self.format().theoretical_memory_footprint(wgpu::Extent3d {
            width: self.texture_view().texture().width(),
            height: 1,
            depth_or_array_layers: 1,
        }) as u32
    }

    fn rows_per_image(&self) -> u32 {
        self.texture_view().texture().height()
    }

    fn copy_to_buffer(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) -> wgpu::Buffer {
        let buffer: wgpu::Buffer = device.create_buffer(&wgpu::BufferDescriptor {
            size: self.buffer_size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("texture copy buffer"),
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            self.as_texel_copy_texture_info(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row()),
                    rows_per_image: Some(self.rows_per_image()),
                },
            },
            self.texture_view().texture().size(),
        );

        buffer
    }

    fn as_texel_copy_texture_info(&self) -> wgpu::TexelCopyTextureInfo<'_> {
        wgpu::TexelCopyTextureInfo {
            aspect: wgpu::TextureAspect::All,
            texture: &self.texture_view().texture(),
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        }
    }

    fn write_to_file(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        mut encoder: wgpu::CommandEncoder,
        filepath: String,
    ) -> impl Future<Output = GPUResult<()>> + Send
    where
        Self: Sync,
    {
        async {
            let output_buffer: wgpu::Buffer = self.copy_to_buffer(device, &mut encoder);

            queue.submit(Some(encoder.finish()));

            {
                // Wait for the buffer to be populated with the rendered data

                let (transmitter, receiver) = smol::channel::bounded(1);

                let buffer_slice: wgpu::BufferSlice<'_> = output_buffer.slice(..);
                buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                    assert!(transmitter.try_send(result).is_ok());
                });

                device.poll(wgpu::PollType::Wait {
                    submission_index: None, // None for most recent submission
                    timeout: Some(core::time::Duration::new(5, 0)),
                })?;

                let _ = receiver.recv().await?;

                // Get a read-only view into the buffer
                let buffer_view: wgpu::BufferView = buffer_slice.get_mapped_range();

                // Cast the buffer data into an image and save it to disk

                match self.format() {
                    wgpu::TextureFormat::R8Unorm
                    | wgpu::TextureFormat::R8Snorm
                    | wgpu::TextureFormat::R8Uint
                    | wgpu::TextureFormat::R8Sint
                    | wgpu::TextureFormat::Stencil8 => GrayImage::from_raw(
                        self.texture_view().texture().width(),
                        self.texture_view().texture().height(),
                        buffer_view.to_vec(),
                    )
                    .ok_or(ImageError::Limits(LimitError::from_kind(
                        LimitErrorKind::DimensionError,
                    )))?
                    .save(filepath)?,
                    wgpu::TextureFormat::R16Uint
                    | wgpu::TextureFormat::R16Sint
                    | wgpu::TextureFormat::R16Unorm
                    | wgpu::TextureFormat::R16Snorm
                    | wgpu::TextureFormat::Depth16Unorm => {
                        ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
                            self.texture_view().texture().width(),
                            self.texture_view().texture().height(),
                            bytemuck::cast_slice::<u8, u16>(&buffer_view).to_vec(),
                        )
                        .ok_or(ImageError::Limits(LimitError::from_kind(
                            LimitErrorKind::DimensionError,
                        )))?
                        .save(filepath)?
                    }
                    // wgpu::TextureFormat::R16Float,
                    wgpu::TextureFormat::Rg8Unorm
                    | wgpu::TextureFormat::Rg8Snorm
                    | wgpu::TextureFormat::Rg8Uint
                    | wgpu::TextureFormat::Rg8Sint => GrayAlphaImage::from_raw(
                        self.texture_view().texture().width(),
                        self.texture_view().texture().height(),
                        buffer_view.to_vec(),
                    )
                    .ok_or(ImageError::Limits(LimitError::from_kind(
                        LimitErrorKind::DimensionError,
                    )))?
                    .save(filepath)?,
                    // wgpu::TextureFormat::R32Uint
                    // | wgpu::TextureFormat::R32Sint,
                    // wgpu::TextureFormat::R32Float | wgpu::TextureFormat::Depth32Float => {
                    //     ImageBuffer::<Luma<f32>, Vec<f32>>::from_raw(
                    //         self.texture_view().texture().width(),
                    //         self.texture_view().texture().height(),
                    //         bytemuck::cast_slice::<u8, f32>(&buffer_view).to_vec(),
                    //     )
                    //     .ok_or(ImageError::Limits(LimitError::from_kind(
                    //         LimitErrorKind::DimensionError,
                    //     )))?
                    //     .save(filepath)?
                    // }
                    // wgpu::TextureFormat::Rg16Uint
                    // | wgpu::TextureFormat::Rg16Sint
                    // | wgpu::TextureFormat::Rg16Unorm
                    // | wgpu::TextureFormat::Rg16Snorm => {
                    //     ImageBuffer::<LumaA<u16>, Vec<u16>>::from_raw(
                    //         self.texture_view().texture().width(),
                    //         self.texture_view().texture().height(),
                    //         bytemuck::cast_slice::<u8, u16>(&buffer_view).to_vec(),
                    //     )
                    //     .ok_or(ImageError::Limits(LimitError::from_kind(
                    //         LimitErrorKind::DimensionError,
                    //     )))?
                    //     .save(filepath)?
                    // }
                    // wgpu::TextureFormat::Rg16Float,
                    wgpu::TextureFormat::Rgba8Unorm
                    | wgpu::TextureFormat::Rgba8UnormSrgb
                    | wgpu::TextureFormat::Rgba8Snorm
                    | wgpu::TextureFormat::Rgba8Uint
                    | wgpu::TextureFormat::Rgba8Sint => RgbaImage::from_raw(
                        self.texture_view().texture().width(),
                        self.texture_view().texture().height(),
                        buffer_view.to_vec(),
                    )
                    .ok_or(ImageError::Limits(LimitError::from_kind(
                        LimitErrorKind::DimensionError,
                    )))?
                    .save(filepath)?,
                    // wgpu::TextureFormat::Bgra8Unorm,
                    // wgpu::TextureFormat::Bgra8UnormSrgb,
                    // wgpu::TextureFormat::Rgb9e5Ufloat,
                    // wgpu::TextureFormat::Rgb10a2Uint,
                    // wgpu::TextureFormat::Rgb10a2Unorm,
                    // wgpu::TextureFormat::Rg11b10Ufloat,
                    // wgpu::TextureFormat::R64Uint,
                    // wgpu::TextureFormat::Rg32Uint,
                    // wgpu::TextureFormat::Rg32Sint,
                    // wgpu::TextureFormat::Rg32Float => {
                    //     ImageBuffer::<LumaA<f32>, Vec<f32>>::from_raw(
                    //         self.texture_view().texture().width(),
                    //         self.texture_view().texture().height(),
                    //         bytemuck::cast_slice::<u8, f32>(&buffer_view).to_vec(),
                    //     )
                    //     .ok_or(ImageError::Limits(LimitError::from_kind(
                    //         LimitErrorKind::DimensionError,
                    //     )))?
                    //     .save(filepath)?
                    // }
                    // wgpu::TextureFormat::Rgba16Uint,
                    // wgpu::TextureFormat::Rgba16Sint,
                    // wgpu::TextureFormat::Rgba16Unorm,
                    // wgpu::TextureFormat::Rgba16Snorm,
                    // wgpu::TextureFormat::Rgba16Float,
                    // wgpu::TextureFormat::Rgba32Uint,
                    // wgpu::TextureFormat::Rgba32Sint,
                    wgpu::TextureFormat::Rgba32Float => Rgba32FImage::from_raw(
                        self.texture_view().texture().width(),
                        self.texture_view().texture().height(),
                        bytemuck::cast_slice::<u8, f32>(&buffer_view).to_vec(),
                    )
                    .ok_or(ImageError::Limits(LimitError::from_kind(
                        LimitErrorKind::DimensionError,
                    )))?
                    .save(filepath)?,
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
                    _ => {}
                }
            }

            // Release the buffer back to the GPU
            output_buffer.unmap();

            Ok(())
        }
    }
}
