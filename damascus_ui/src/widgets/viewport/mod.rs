// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use glam;
use iced;
use wgpu;

use damascus::{
    gpu::resources::BufferData,
    graph::node_graph::{
        NodeGraph,
        nodes::{NodeId, node_data::NodeData},
        outputs::OutputId,
    },
};

use crate::{app::Context, widgets::Style};

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
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewportPrimitive {
    active_texture_viewer_output_id: Option<OutputId>,
    texture_viewer_output_ids: HashMap<u32, OutputId>,
    node_graph: Arc<Mutex<NodeGraph>>,
}

impl Default for ViewportPrimitive {
    fn default() -> Self {
        Self {
            active_texture_viewer_output_id: None,
            texture_viewer_output_ids: HashMap::default(),
            node_graph: Arc::default(),
        }
    }
}

impl iced::widget::shader::Pipeline for ViewportPrimitive {
    fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        // let mut viewport_primitive = Self::default();

        // if let Some(input_texture) = viewport_primitive
        //     .texture_evaluator
        //     .create_output_texture_view(device)
        // {
        //     viewport_primitive
        //         .texture_evaluator
        //         .set_input_texture_view(input_texture);
        // }

        // viewport_primitive
        Self::default()
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
        let Some(active_texture_viewer_output_id) = self.active_texture_viewer_output_id.as_ref()
        else {
            return;
        };

        let mut encoder: wgpu::CommandEncoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let Ok(mut node_graph) = self.node_graph.lock() else {
                return;
            };

            let Ok(input_data) = node_graph.evaluate_output(
                &device,
                &queue,
                &mut encoder,
                active_texture_viewer_output_id,
            ) else {
                return;
            };
        }

        // TODO this is gonna need to go into the node evaluation yeah?
        queue.submit(Some(encoder.finish()));
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        let Some(active_texture_viewer_output_id) = self.active_texture_viewer_output_id.as_ref()
        else {
            return false;
        };

        let Ok(mut node_graph) = self.node_graph.lock() else {
            return false;
        };

        node_graph[node_graph[active_texture_viewer_output_id].node_id]

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
    panning: bool,
    last_cursor_position: glam::Vec2,
    bounds: glam::Vec2,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            viewport_primitive: ViewportPrimitive::default(),
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
        if !Arc::ptr_eq(&self.viewport_primitive.node_graph, &context.node_graph) {
            self.viewport_primitive.node_graph = Arc::clone(&context.node_graph);

            if self.viewport_primitive.texture_viewer_output_ids.is_empty()
                && let Ok(mut node_graph) = self.viewport_primitive.node_graph.lock()
            {
                let texture_viewer_id: NodeId = node_graph.add_node(NodeData::TextureViewer);
                if let Some(texture_viewer_output_id) =
                    node_graph.nodes_first_output_id(&texture_viewer_id)
                {
                    self.viewport_primitive
                        .texture_viewer_output_ids
                        .insert(1, *texture_viewer_output_id);
                    self.viewport_primitive.active_texture_viewer_output_id =
                        Some(*texture_viewer_output_id)
                }
            }
        }

        match message {
            ViewportMessage::Zoom(zoom) => {
                // let cursor_position = glam::Vec2::new(
                //     self.last_cursor_position.x - self.bounds.x * 0.5,
                //     self.bounds.y * 0.5 - self.last_cursor_position.y,
                // );

                // let hovered_image_pixel_before: glam::Vec2 = cursor_position
                //     * self.viewport_primitive.texture_evaluator.zoom
                //     - self.viewport_primitive.texture_evaluator.pan;

                // self.viewport_primitive.texture_evaluator.zoom /= zoom.exp();

                // let hovered_image_pixel: glam::Vec2 = cursor_position
                //     * self.viewport_primitive.texture_evaluator.zoom
                //     - self.viewport_primitive.texture_evaluator.pan;

                // self.viewport_primitive.texture_evaluator.pan +=
                //     hovered_image_pixel - hovered_image_pixel_before;
            }
            ViewportMessage::BeginPan => self.panning = true,
            ViewportMessage::EndPan => self.panning = false,
            ViewportMessage::Exit => self.panning = false,
            ViewportMessage::MoveCursor(cursor_position) => {
                // if self.panning {
                //     let drag_delta: glam::Vec2 = (cursor_position - self.last_cursor_position)
                //         * self.viewport_primitive.texture_evaluator.zoom;
                //     self.viewport_primitive.texture_evaluator.pan.x += drag_delta.x;
                //     self.viewport_primitive.texture_evaluator.pan.y -= drag_delta.y;
                // }

                // self.last_cursor_position = cursor_position;
            }
            ViewportMessage::Resize(bounds) => {
                self.bounds = bounds;
                if let Some(active_texture_viewer_output_id) =
                    self.viewport_primitive.active_texture_viewer_output_id
                    && let Ok(mut node_graph) = self.viewport_primitive.node_graph.lock()
                {
                    node_graph.set_input_data(
                        &node_graph[active_texture_viewer_output_id].node_id,
                        &TextureViewerInputData::OutputResolution,
                        InputData::UVec2(self.bounds.as_u32()),
                    );
                }
            }
            ViewportMessage::GainChanged(gain) => {
                // self.viewport_primitive.texture_evaluator.grade.gain = gain
            }
            ViewportMessage::GammaChanged(gamma) => {
                // self.viewport_primitive.texture_evaluator.grade.gamma = gamma
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

        // let gain = style.slider(
        //     "f/4",
        //     0.0..=64.0,
        //     ..,
        //     self.viewport_primitive.texture_evaluator.grade.gain,
        //     style.float_input_step,
        //     |value| -> ViewportMessage { ViewportMessage::GainChanged(value) },
        //     ViewportMessage::GainChanged(self.viewport_primitive.texture_evaluator.grade.gain),
        //     "The gain to apply in the viewer.",
        //     parameter_name_length,
        //     horizontal_text_alignment,
        // );
        // let gamma = style.slider(
        //     "γ",
        //     0.01..=64.0,
        //     0.01..,
        //     self.viewport_primitive.texture_evaluator.grade.gamma,
        //     style.float_input_step,
        //     |value| -> ViewportMessage { ViewportMessage::GammaChanged(value) },
        //     ViewportMessage::GammaChanged(self.viewport_primitive.texture_evaluator.grade.gamma),
        //     "The gamma to apply in the viewer.",
        //     parameter_name_length,
        //     horizontal_text_alignment,
        // );

        let toolbar = iced::widget::row![]; //gain, gamma];

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
