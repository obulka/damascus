// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use glam::{UVec2, Vec2};
use iced;
use wgpu;

use damascus::{
    gpu::resources::BufferData,
    graph::node_graph::{NodeGraph, outputs::OutputId},
    textures::evaluators::{GPUTextureEvaluator, TextureEvaluator, view::TextureViewer},
};

use crate::{app::Context, widgets::Style};

use super::Widget;

pub type ViewMap = HashMap<u32, OutputId>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportMessage {
    Zoom(f32),
    BeginPan,
    EndPan,
    Exit,
    MoveCursor(Vec2),
    Resize(Vec2),
    GainChanged(f32),
    GammaChanged(f32),
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewportPipeline {
    texture_viewer: TextureViewer,
}

impl Default for ViewportPipeline {
    fn default() -> Self {
        Self {
            texture_viewer: TextureViewer::default().output_srgb(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewportPrimitive {
    node_graph: Arc<Mutex<NodeGraph>>,
    active_output_id: Option<OutputId>,
    zoom: f32,
    pan: Vec2,
    gain: f32,
    gamma: f32,
}

impl Default for ViewportPrimitive {
    fn default() -> Self {
        Self {
            node_graph: Arc::default(),
            active_output_id: None,
            zoom: 1.0,
            pan: Vec2::ZERO,
            gain: 1.0,
            gamma: 1.0,
        }
    }
}

impl iced::widget::shader::Pipeline for ViewportPipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        let mut viewport_pipeline = Self::default();

        if let Some(input_texture) = viewport_pipeline
            .texture_viewer
            .create_output_texture_view(device)
        {
            viewport_pipeline
                .texture_viewer
                .set_input_texture_view(input_texture);
        }

        viewport_pipeline
    }
}

impl iced::widget::shader::Primitive for ViewportPrimitive {
    type Pipeline = ViewportPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &iced::Rectangle,
        _viewport: &iced::widget::shader::Viewport,
    ) {
        let Some(active_output_id) = self.active_output_id else {
            return;
        };

        pipeline
            .texture_viewer
            .set_output_resolution(UVec2::new(bounds.width as u32, bounds.height as u32));

        pipeline.texture_viewer.zoom = self.zoom;
        pipeline.texture_viewer.pan = self.pan;
        pipeline.texture_viewer.grade.gain = self.gain;
        pipeline.texture_viewer.grade.gamma = self.gamma;

        let mut encoder: wgpu::CommandEncoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        if let Ok(mut node_graph) = self.node_graph.lock()
            && node_graph.output_id_is_valid(active_output_id)
            && let Ok(input_data) =
                node_graph.evaluate_output(&device, &queue, &mut encoder, &active_output_id)
        {
            queue.submit(Some(encoder.finish()));

            if let Ok(texture_evaluator_id) = input_data.try_to_texture_evaluator_id()
                && let Some(input_texture) =
                    node_graph.scene_graph()[texture_evaluator_id].output_texture_view()
            {
                pipeline
                    .texture_viewer
                    .set_input_texture_view(input_texture.clone());
            }
        }

        if pipeline.texture_viewer.reconstruct_if_hash_changed(device) {
            pipeline.texture_viewer.update_recompilation_hash();
            pipeline.texture_viewer.update_reset_hash();
        } else if pipeline.texture_viewer.recompile_if_hash_changed(device) {
            pipeline.texture_viewer.update_reset_hash();
        } else if pipeline.texture_viewer.reset_if_hash_changed() {
            pipeline.texture_viewer.update_for_reevaluation(device);
        }

        let buffer_data: BufferData = pipeline.texture_viewer.buffer_data();
        if let Some(render_resource) = pipeline.texture_viewer.render_resource() {
            render_resource.write_bind_groups(queue, &buffer_data);
        }
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        if let Some(render_resource) = pipeline.texture_viewer.render_resource() {
            render_resource.paint(render_pass);
            true
        } else {
            false
        }
    }

    fn render(
        &self,
        pipeline: &Self::Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        if let Some(render_resource) = pipeline.texture_viewer.render_resource() {
            let mut pass = encoder.begin_render_pass(&render_pass_desc);
            pass.set_viewport(
                clip_bounds.x as f32,
                clip_bounds.y as f32,
                clip_bounds.width as f32,
                clip_bounds.height as f32,
                0.0,
                1.0,
            );
            render_resource.paint(&mut pass);
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Viewport {
    active_view: Option<u32>,
    view_map: ViewMap,
    viewport_primitive: ViewportPrimitive,
    panning: bool,
    last_cursor_position: Vec2,
    bounds: Vec2,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            active_view: None,
            view_map: ViewMap::default(),
            viewport_primitive: ViewportPrimitive::default(),
            panning: false,
            last_cursor_position: Vec2::ZERO,
            bounds: Vec2::ZERO,
        }
    }
}

impl Viewport {
    pub fn active_output_id(&self) -> Option<&OutputId> {
        if let Some(active_view) = self.active_view {
            self.view_map.get(&active_view)
        } else {
            None
        }
    }
}

impl Widget<ViewportMessage> for Viewport {
    fn update(
        &mut self,
        context: &mut Context,
        message: ViewportMessage,
    ) -> iced::Task<ViewportMessage> {
        if !Arc::ptr_eq(&self.viewport_primitive.node_graph, &context.node_graph) {
            self.viewport_primitive.node_graph = Arc::clone(&context.node_graph);
        }

        // TODO nuh uh uh
        if let Ok(mut node_graph) = self.viewport_primitive.node_graph.lock() {
            for (index, node_id) in node_graph.iter().enumerate() {
                if let Some(output_id) = node_graph.nodes_first_output_id(&node_id) {
                    self.view_map.insert(index as u32 + 1, *output_id);
                    self.active_view = Some(index as u32 + 1);
                }
            }
        }

        match message {
            ViewportMessage::Zoom(zoom) => {
                let cursor_position = Vec2::new(
                    self.last_cursor_position.x - self.bounds.x * 0.5,
                    self.bounds.y * 0.5 - self.last_cursor_position.y,
                );

                let hovered_image_pixel_before: Vec2 =
                    cursor_position * self.viewport_primitive.zoom - self.viewport_primitive.pan;

                self.viewport_primitive.zoom /= zoom.exp();

                let hovered_image_pixel: Vec2 =
                    cursor_position * self.viewport_primitive.zoom - self.viewport_primitive.pan;

                self.viewport_primitive.pan += hovered_image_pixel - hovered_image_pixel_before;
            }
            ViewportMessage::BeginPan => self.panning = true,
            ViewportMessage::EndPan => self.panning = false,
            ViewportMessage::Exit => self.panning = false,
            ViewportMessage::MoveCursor(cursor_position) => {
                if self.panning {
                    let drag_delta: Vec2 = (cursor_position - self.last_cursor_position)
                        * self.viewport_primitive.zoom;
                    self.viewport_primitive.pan.x += drag_delta.x;
                    self.viewport_primitive.pan.y -= drag_delta.y;
                }

                self.last_cursor_position = cursor_position;
            }
            ViewportMessage::Resize(bounds) => self.bounds = bounds,
            ViewportMessage::GainChanged(gain) => self.viewport_primitive.gain = gain,
            ViewportMessage::GammaChanged(gamma) => self.viewport_primitive.gamma = gamma,
            _ => {}
        }
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, ViewportMessage> {
        let parameter_name_length = iced::Length::Fill;
        let horizontal_text_alignment = iced::alignment::Horizontal::Center;

        let gain = style.slider(
            "f/4",
            0.0..=64.0,
            ..,
            self.viewport_primitive.gain,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GainChanged(value) },
            ViewportMessage::GainChanged(self.viewport_primitive.gain),
            "The gain to apply in the viewer.",
            parameter_name_length,
            horizontal_text_alignment,
        );
        let gamma = style.slider(
            "γ",
            0.01..=64.0,
            0.01..,
            self.viewport_primitive.gamma,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GammaChanged(value) },
            ViewportMessage::GammaChanged(self.viewport_primitive.gamma),
            "The gamma to apply in the viewer.",
            parameter_name_length,
            horizontal_text_alignment,
        );

        let toolbar = iced::widget::row![gain, gamma];

        let shader = iced::widget::mouse_area(
            iced::widget::shader(self)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .on_scroll(|scroll_delta| {
            ViewportMessage::Zoom(match scroll_delta {
                iced::mouse::ScrollDelta::Lines { y, .. } => y * style.viewer_zoom_sensitivity,
                iced::mouse::ScrollDelta::Pixels { y, .. } => y * style.viewer_zoom_sensitivity,
            })
        })
        .on_middle_press(ViewportMessage::BeginPan)
        .on_middle_release(ViewportMessage::EndPan)
        .on_exit(ViewportMessage::Exit)
        .on_move(|point| ViewportMessage::MoveCursor(Vec2::new(point.x, point.y)));

        iced::widget::container(iced::widget::column![toolbar, shader])
            .padding(style.padding)
            .into()
    }
}

impl iced::widget::shader::Program<ViewportMessage> for Viewport {
    type State = iced::Rectangle;
    type Primitive = ViewportPrimitive;

    fn update(
        &self,
        state: &mut Self::State,
        _event: &iced::Event,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Option<iced::widget::Action<ViewportMessage>> {
        if *state != bounds {
            *state = bounds;
            Some(iced::widget::Action::publish(ViewportMessage::Resize(
                Vec2::new(bounds.width, bounds.height),
            )))
        } else {
            None
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced::mouse::Cursor,
        _bounds: iced::Rectangle,
    ) -> Self::Primitive {
        let mut viewport_primitive: ViewportPrimitive = self.viewport_primitive.clone();
        viewport_primitive.active_output_id = self.active_output_id().copied();
        viewport_primitive
    }
}
