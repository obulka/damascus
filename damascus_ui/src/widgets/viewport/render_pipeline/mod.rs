// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crevice::std430::AsStd430;
use eframe::{egui, egui_wgpu, epaint};

use damascus_core::renderers::Renderer;

use super::settings::{CompilerSettings, RayMarcherPipelineSettings, ViewportSettings};

mod ray_marcher_pipeline;

pub use ray_marcher_pipeline::RayMarcherPipeline;

pub trait RenderPipeline<R: Renderer<G, S>, G: Copy + Clone + AsStd430<Output = S>, S>:
    Default
{
    fn set_recompile_hash(&mut self);
    fn set_reconstruct_hash(&mut self, settings: &ViewportSettings);

    /// Create an instance of this render pipeline
    fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Option<Self> {
        let mut pipeline = Self::default();
        pipeline.set_recompile_hash();
        pipeline.set_reconstruct_hash(&settings);

        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        Self::construct(
            &mut pipeline,
            creation_context.wgpu_render_state.as_ref()?,
            &settings.ray_marcher_pipeline,
        );

        Some(pipeline)
    }

    /// Construict all uniform/storage/texture buffers and RenderResources
    fn construct(
        &mut self,
        _wgpu_render_state: &egui_wgpu::RenderState,
        _settings: &ViewportSettings,
    );

    /// Construict all uniform/storage/texture buffers and RenderResources
    fn reconstruct(
        &mut self,
        wgpu_render_state: &egui_wgpu::RenderState,
        settings: &ViewportSettings,
    ) {
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .clear();
        self.reset_render();
        self.construct(wgpu_render_state, settings);
    }

    fn recompile_shader(&mut self, wgpu_render_state: &egui_wgpu::RenderState);

    fn update_preprocessor_directives(&mut self, settings: &CompilerSettings) -> bool;

    fn disable(&mut self) {}
    fn enable(&mut self) {}
    fn enabled(&mut self) -> bool {
        true
    }
    fn pause(&mut self) {}
    fn play(&mut self) {}
    fn toggle_play_pause(&mut self) {}
    fn paused(&self) -> bool {
        false
    }
    fn reset_render(&mut self) {}

    fn custom_painting(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        available_size: egui::Vec2,
        settings: &ViewportSettings,
    ) -> Option<epaint::PaintCallback>;
}

pub enum RenderPipelines {
    RayMarcher {
        render_pipeline: Option<RayMarcherPipeline>,
    },
    Texture,
}

impl RenderPipelines {
    pub fn new<'a>(
        creation_context: &'a eframe::CreationContext<'a>,
        settings: &ViewportSettings,
    ) -> Self {
        Self::RayMarcher {
            value: RayMarcherPipeline::new(creation_context, settings),
        }
    }

    pub fn reconstruct(&mut self, frame: &eframe::Frame, settings: &ViewportSettings) {
        if let Some(wgpu_render_state) = frame.wgpu_render_state() {
            match self {
                Self::RayMarcher { render_pipeline } => {
                    if let Some(pipeline) = render_pipeline {
                        pipeline.reconstruct(wgpu_render_state, settings)
                    }
                }
                _ => {}
            }
        }
    }

    pub fn recompile_shader(&mut self, frame: &eframe::Frame) {
        if let Some(wgpu_render_state) = frame.wgpu_render_state() {
            match self {
                Self::RayMarcher { render_pipeline } => {
                    if let Some(pipeline) = render_pipeline {
                        pipeline.recompile_shader(wgpu_render_state)
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update_preprocessor_directives(&mut self, settings: &CompilerSettings) -> bool {
        match self {
            Self::RayMarcher { render_pipeline } => {
                if let Some(pipeline) = render_pipeline {
                    pipeline.update_preprocessor_directives(settings)
                }
            }
            _ => false,
        }
    }

    pub fn recompile_if_preprocessor_directives_changed(&mut self, frame: &mut eframe::Frame) {
        if self.update_preprocessor_directives() {
            self.recompile_shader(frame);
        }
    }

    // pub fn enable_and_play(&mut self) {
    //     if let Some(render_pipeline) = &mut self.render_pipeline {
    //         render_pipeline.enable();
    //         render_pipeline.play();
    //     }
    // }

    // pub fn enable(&mut self) {
    //     if let Some(render_pipeline) = &mut self.render_pipeline {
    //         render_pipeline.enable();
    //     }
    // }

    // pub fn disable(&mut self) {
    //     if let Some(render_pipeline) = &mut self.render_pipeline {
    //         render_pipeline.disable();
    //     }
    // }
}
