// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;
use wgpu;

use damascus::{
    gpu::resources::BufferData,
    graph::node_graph::{
        NodeGraph,
        inputs::input_data::InputData,
        nodes::{
            NodeId,
            node_data::{NodeData, TextureReadInputData},
        },
        outputs::OutputId,
    },
    textures::evaluators::{GPUTextureEvaluator, view::TextureViewer},
};

use crate::widgets::Style;

use super::Widget;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Pipeline {
    texture_evaluator: TextureViewer,
}

impl iced::widget::shader::Pipeline for Pipeline {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        // TODO this shouldnt be hardcoded, duh
        let mut graph = NodeGraph::new();

        let read_id: NodeId = graph.add_node(NodeData::TextureRead);

        let Ok(_input_id) = graph.set_input_data(
            &read_id,
            &TextureReadInputData::Filepath,
            InputData::Filepath("/home/ob1/software/rust/damascus/damascus/image.exr".to_string()),
        ) else {
            panic!("read could not set filepath.");
        };

        let read_output_id: OutputId = *graph.nodes_first_output_id(&read_id).unwrap();

        let mut encoder: wgpu::CommandEncoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let Ok(input_data) = graph.evaluate_output(&device, &queue, &mut encoder, &read_output_id)
        else {
            panic!("read did not produce an output.");
        };

        let Ok(texture_evaluator_id) = input_data.try_to_texture_evaluator_id() else {
            panic!("read output was not a texture evaluator id.");
        };

        let Some(output_texture_view) =
            graph.scene_graph()[texture_evaluator_id].output_texture_view()
        else {
            panic!("read did not produce an output TextureView.");
        };

        Self {
            texture_evaluator: TextureViewer::default()
                .with_input_texture_view(output_texture_view.clone())
                .finalized(device),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewportMessage {
    None,
}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ViewportPrimitive {}

impl iced::widget::shader::Primitive for ViewportPrimitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &iced::Rectangle,
        _viewport: &iced::widget::shader::Viewport,
    ) {
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
        _clip_bounds: &iced::Rectangle<u32>,
    ) {
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        a: 0.,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };

        if let Some(render_resource) = pipeline.texture_evaluator.render_resource() {
            render_resource.paint(&mut encoder.begin_render_pass(&render_pass_desc));
        }
    }
}

#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Viewport {
    viewport_primitive: ViewportPrimitive,
}

impl Widget<ViewportMessage> for Viewport {
    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, ViewportMessage> {
        let shader = iced::widget::shader(self)
            .width(iced::Fill)
            .height(iced::Fill);

        iced::widget::center(
            iced::widget::column![style.text("Viewport"), shader].align_x(iced::Center),
        )
        .into()
    }
}

impl<ViewportMessage> iced::widget::shader::Program<ViewportMessage> for Viewport {
    type State = ();
    type Primitive = ViewportPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced::mouse::Cursor,
        _bounds: iced::Rectangle,
    ) -> Self::Primitive {
        self.viewport_primitive
    }
}
