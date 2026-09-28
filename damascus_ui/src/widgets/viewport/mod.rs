// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam;
use iced;
use wgpu;

use damascus::{
    gpu::resources::{BufferData, TextureView},
    graph::node_graph::{
        NodeGraph,
        inputs::input_data::InputData,
        nodes::{
            NodeId,
            node_data::{NodeData, TextureReadInputData},
        },
        outputs::OutputId,
    },
    textures::evaluators::{
        GPUTextureEvaluator, TextureEvaluator, grade::Grade, view::TextureViewer,
    },
};

use crate::{app::Context, widgets::Style};

use super::Widget;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Pipeline {
    texture_evaluator: TextureViewer,
}

impl iced::widget::shader::Pipeline for Pipeline {
    fn new(_device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        Self {
            texture_evaluator: TextureViewer::default(), // .with_input_texture_view(output_texture_view.clone())
                                                         // .finalized(device),
        }
    }
}

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
    #[serde(skip)]
    ViewTexture(TextureView),
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ViewportPrimitive {
    zoom: f32,
    pan: glam::Vec2,
    grade: Grade,
    #[serde(skip)]
    input_texture: Option<TextureView>,
}

impl Default for ViewportPrimitive {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan: glam::Vec2::ZERO,
            grade: Grade::default(),
            input_texture: None,
        }
    }
}

impl iced::widget::shader::Primitive for ViewportPrimitive {
    type Pipeline = Pipeline;

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

        pipeline.texture_evaluator.zoom = self.zoom;
        pipeline.texture_evaluator.pan = self.pan;
        pipeline.texture_evaluator.grade.gain = self.grade.gain;
        pipeline.texture_evaluator.grade.gamma = self.grade.gamma;

        if let Some(ref input_texture) = self.input_texture
            && (pipeline.texture_evaluator.input_texture_views().is_empty()
                || *input_texture != pipeline.texture_evaluator.input_texture_views()[0])
        {
            pipeline
                .texture_evaluator
                .set_input_texture_view(input_texture.clone());

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
        _context: &mut Context,
        message: ViewportMessage,
    ) -> iced::Task<ViewportMessage> {
        println!("Viewport");
        match message {
            ViewportMessage::Zoom(zoom) => {
                let cursor_position = glam::Vec2::new(
                    self.last_cursor_position.x - self.bounds.x * 0.5,
                    self.bounds.y * 0.5 - self.last_cursor_position.y,
                );

                let hovered_image_pixel_before: glam::Vec2 =
                    cursor_position * self.viewport_primitive.zoom - self.viewport_primitive.pan;

                self.viewport_primitive.zoom /= zoom.exp();

                let hovered_image_pixel: glam::Vec2 =
                    cursor_position * self.viewport_primitive.zoom - self.viewport_primitive.pan;

                self.viewport_primitive.pan += hovered_image_pixel - hovered_image_pixel_before;
            }
            ViewportMessage::BeginPan => self.panning = true,
            ViewportMessage::EndPan => self.panning = false,
            ViewportMessage::Exit => self.panning = false,
            ViewportMessage::MoveCursor(cursor_position) => {
                if self.panning {
                    let drag_delta: glam::Vec2 = (cursor_position - self.last_cursor_position)
                        * self.viewport_primitive.zoom;
                    self.viewport_primitive.pan.x += drag_delta.x;
                    self.viewport_primitive.pan.y -= drag_delta.y;
                }

                self.last_cursor_position = cursor_position;
            }
            ViewportMessage::Resize(bounds) => self.bounds = bounds,
            ViewportMessage::GainChanged(gain) => self.viewport_primitive.grade.gain = gain,
            ViewportMessage::GammaChanged(gamma) => self.viewport_primitive.grade.gamma = gamma,
            ViewportMessage::ViewTexture(texture_view) => {
                self.viewport_primitive.input_texture = Some(texture_view);
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
            self.viewport_primitive.grade.gain,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GainChanged(value) },
            ViewportMessage::GainChanged(self.viewport_primitive.grade.gain),
            "The gain to apply in the viewer.",
            parameter_name_length,
            horizontal_text_alignment,
        );
        let gamma = style.slider(
            "γ",
            0.01..=64.0,
            0.01..,
            self.viewport_primitive.grade.gamma,
            style.float_input_step,
            |value| -> ViewportMessage { ViewportMessage::GammaChanged(value) },
            ViewportMessage::GammaChanged(self.viewport_primitive.grade.gamma),
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
