// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::Arc;

use damascus_ui::{MAX_BUFFER_SIZE, MAX_TEXTURE_DIMENSION};

fn main() {
    use eframe::{egui::Visuals, egui_wgpu, wgpu};

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: egui_wgpu::WgpuConfiguration {
            device_descriptor: Arc::new(|adapter| {
                let base_limits = if adapter.get_info().backend == wgpu::Backend::Gl {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                };

                wgpu::DeviceDescriptor {
                    label: Some("egui wgpu device"),
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits {
                        // When using a depth buffer, we have to be able to create a texture
                        // large enough for the entire surface, and we want to support 4k+ displays.
                        max_texture_dimension_2d: MAX_TEXTURE_DIMENSION,
                        max_buffer_size: MAX_BUFFER_SIZE as u64,
                        max_storage_buffer_binding_size: MAX_BUFFER_SIZE as u32,
                        ..base_limits
                    },
                }
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = eframe::run_native(
        "damascus",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(damascus_ui::Damascus::new(cc))
        }),
    );
}
