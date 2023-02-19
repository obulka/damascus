#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::egui::Visuals;

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "damascus",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Box::new(damascus::Damascus::new(cc))
        }),
    );
}
