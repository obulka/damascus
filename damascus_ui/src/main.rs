// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

fn main() {
    use eframe::egui::Visuals;

    let mut options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    options.wgpu_options.device_descriptor.features =
        eframe::wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;

    let _ = eframe::run_native(
        "damascus",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(damascus_ui::Damascus::new(cc))
        }),
    );
}
