// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::sync::{Arc, Mutex};

use glam;
use iced;
use wgpu;

use damascus::{
    gpu::resources::{BufferData, TextureView},
    graph::{
        BidirectedGraph,
        node_graph::{
            inputs::input_data::InputData,
            nodes::{
                NodeId,
                node_data::{NodeData, TextureReadInputData},
            },
            outputs::OutputId,
        },
    },
    textures::evaluators::{
        GPUTextureEvaluator, TextureEvaluator, grade::Grade, view::TextureViewer,
    },
};

use crate::{
    app::Context,
    widgets::{Style, node_graph::NodeGraph},
};

use super::Widget;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportMessage {
    Zoom(f32),
    BeginPan,
    EndPan,
    Exit,
    MoveCursor(glam::Vec2),
    Resize(glam::Vec2),
    GainChanged(f32),
    GammaChanged(f32),
    ViewActiveNode,
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewportPrimitive {
    texture_evaluator: TextureViewer,
    node_graph: Arc<Mutex<NodeGraph>>,
}

impl Default for ViewportPrimitive {
    fn default() -> Self {
        Self {
            texture_evaluator: TextureViewer::default(),
            node_graph: Arc::default(),
        }
    }
}

impl iced::widget::shader::Pipeline for ViewportPrimitive {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        let mut viewport_primitive = Self::default();

        if let Some(input_texture) = viewport_primitive
            .texture_evaluator
            .create_output_texture_view(device)
        {
            viewport_primitive
                .texture_evaluator
                .set_input_texture_view(input_texture);
        }

        viewport_primitive
    }
}

impl iced::widget::shader::Primitive for ViewportPrimitive {
    type Pipeline = Self;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &iced::Rectangle,
        _viewport: &iced::widget::shader::Viewport,
    ) {
        pipeline
            .texture_evaluator
            .set_output_resolution(glam::UVec2::new(bounds.width as u32, bounds.height as u32));

        pipeline.texture_evaluator.zoom = self.texture_evaluator.zoom;
        pipeline.texture_evaluator.pan = self.texture_evaluator.pan;
        pipeline.texture_evaluator.grade.gain = self.texture_evaluator.grade.gain;
        pipeline.texture_evaluator.grade.gamma = self.texture_evaluator.grade.gamma;

        {
            let mut node_graph = self.node_graph.lock().unwrap();

            if let Some(active_node_id) = node_graph.active_node {
                println!("nodes: {:?}", node_graph.node_graph.node_count());
                let active_output_id: OutputId = *node_graph
                    .node_graph
                    .nodes_first_output_id(&active_node_id)
                    .unwrap();

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                let Ok(input_data) = node_graph.node_graph.evaluate_output(
                    &device,
                    &queue,
                    &mut encoder,
                    &active_output_id,
                ) else {
                    panic!("Active node did not produce an output.");
                };

                let Ok(texture_evaluator_id) = input_data.try_to_texture_evaluator_id() else {
                    panic!("Active node output was not a texture evaluator id.");
                };

                let Some(input_texture) =
                    node_graph.node_graph.scene_graph()[texture_evaluator_id].output_texture_view()
                else {
                    panic!("Active node did not produce an output TextureView.");
                };

                pipeline
                    .texture_evaluator
                    .set_input_texture_view(input_texture.clone());
            }

            if !pipeline.texture_evaluator.update_if_hash_changed(device) {
                // No new data triggered a reset/recompile/reconstruction of
                // the pipeline, therefore we can build on top of the previous
                // render pass

                pipeline.texture_evaluator.update_for_reevaluation(device);
            }
        }

        let buffer_data: BufferData = pipeline.texture_evaluator.buffer_data();
        if let Some(render_resource) = pipeline.texture_evaluator.render_resource() {
            render_resource.write_bind_groups(queue, &buffer_data);
        }
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        if let Some(render_resource) = pipeline.texture_evaluator.render_resource() {
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

        if let Some(render_resource) = pipeline.texture_evaluator.render_resource() {
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
    viewport_primitive: ViewportPrimitive,
    scroll_to_zoom: f32,
    panning: bool,
    last_cursor_position: glam::Vec2,
    bounds: glam::Vec2,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            viewport_primitive: ViewportPrimitive::default(),
            scroll_to_zoom: 0.1,
            panning: false,
            last_cursor_position: glam::Vec2::ZERO,
            bounds: glam::Vec2::ZERO,
        }
    }
}

impl Widget<ViewportMessage> for Viewport {
    fn update(
        &mut self,
        context: &mut Context,
        message: ViewportMessage,
    ) -> iced::Task<ViewportMessage> {
        match message {
            ViewportMessage::Zoom(zoom) => {
                let cursor_position = glam::Vec2::new(
                    self.last_cursor_position.x - self.bounds.x * 0.5,
                    self.bounds.y * 0.5 - self.last_cursor_position.y,
                );

                let hovered_image_pixel_before: glam::Vec2 = cursor_position
                    * self.viewport_primitive.texture_evaluator.zoom
                    - self.viewport_primitive.texture_evaluator.pan;

                self.viewport_primitive.texture_evaluator.zoom /= zoom.exp();

                let hovered_image_pixel: glam::Vec2 = cursor_position
                    * self.viewport_primitive.texture_evaluator.zoom
                    - self.viewport_primitive.texture_evaluator.pan;

                self.viewport_primitive.texture_evaluator.pan +=
                    hovered_image_pixel - hovered_image_pixel_before;
            }
            ViewportMessage::BeginPan => self.panning = true,
            ViewportMessage::EndPan => self.panning = false,
            ViewportMessage::Exit => self.panning = false,
            ViewportMessage::MoveCursor(cursor_position) => {
                if self.panning {
                    let drag_delta: glam::Vec2 = (cursor_position - self.last_cursor_position)
                        * self.viewport_primitive.texture_evaluator.zoom;
                    self.viewport_primitive.texture_evaluator.pan.x += drag_delta.x;
                    self.viewport_primitive.texture_evaluator.pan.y -= drag_delta.y;
                }

                self.last_cursor_position = cursor_position;
            }
            ViewportMessage::Resize(bounds) => self.bounds = bounds,
            ViewportMessage::GainChanged(gain) => {
                self.viewport_primitive.texture_evaluator.grade.gain = gain
            }
            ViewportMessage::GammaChanged(gamma) => {
                self.viewport_primitive.texture_evaluator.grade.gamma = gamma
            }
            ViewportMessage::ViewActiveNode => {
                self.viewport_primitive.node_graph = Arc::clone(&context.node_graph);
            }
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
            self.viewport_primitive.texture_evaluator.grade.gain,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GainChanged(value) },
            ViewportMessage::GainChanged(self.viewport_primitive.texture_evaluator.grade.gain),
            "The gain to apply in the viewer.",
            parameter_name_length,
            horizontal_text_alignment,
        );
        let gamma = style.slider(
            "γ",
            0.01..=64.0,
            0.01..,
            self.viewport_primitive.texture_evaluator.grade.gamma,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GammaChanged(value) },
            ViewportMessage::GammaChanged(self.viewport_primitive.texture_evaluator.grade.gamma),
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
                iced::mouse::ScrollDelta::Lines { y, .. } => y * self.scroll_to_zoom,
                iced::mouse::ScrollDelta::Pixels { .. } => 0.0,
            })
        })
        .on_middle_press(ViewportMessage::BeginPan)
        .on_middle_release(ViewportMessage::EndPan)
        .on_exit(ViewportMessage::Exit)
        .on_move(|point| ViewportMessage::MoveCursor(glam::Vec2::new(point.x, point.y)));

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
                glam::Vec2::new(bounds.width, bounds.height),
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
        self.viewport_primitive.clone()
    }
}
