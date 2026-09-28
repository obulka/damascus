// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use wgpu;

use damascus::{
    gpu::{GPUResult, get_device_queue, resources::TextureView},
    graph::node_graph::{
        self,
        inputs::input_data::InputData,
        nodes::{
            NodeId,
            node_data::{NodeData, TextureReadInputData},
        },
        outputs::OutputId,
    },
};

use crate::{app::Context, widgets::Style};

use super::Widget;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NodeGraphMessage {
    SetActiveNode(NodeId),
    ClearActiveNode,
    InputValueChanged(NodeId, String),
    CheckPreprocessorDirectives,
    ReconstructRenderResources,
    EvaluateActiveNode,
    #[serde(skip)]
    ViewTexture(TextureView),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    pub active_node: Option<NodeId>,
    pub node_graph: node_graph::NodeGraph,
}

impl Default for NodeGraph {
    fn default() -> Self {
        // TODO this shouldnt be hardcoded, duh
        let mut graph = node_graph::NodeGraph::new();

        let read_id: NodeId = graph.add_node(NodeData::TextureRead);

        let Ok(_input_id) = graph.set_input_data(
            &read_id,
            &TextureReadInputData::Filepath,
            InputData::Filepath("/home/ob1/software/rust/damascus/damascus/image.exr".to_string()),
        ) else {
            panic!("read could not set filepath.");
        };

        Self {
            active_node: Some(read_id),
            node_graph: graph,
        }
    }
}

impl Widget<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        context: &mut Context,
        message: NodeGraphMessage,
    ) -> iced::Task<NodeGraphMessage> {
        println!("NodeGraph");
        match message {
            NodeGraphMessage::EvaluateActiveNode => {
                if let Some(active_node_id) = self.active_node
                    && let Some((ref device, ref queue)) = context.device_queue
                {
                    let read_output_id: OutputId = *self
                        .node_graph
                        .nodes_first_output_id(&active_node_id)
                        .unwrap();

                    let mut encoder: wgpu::CommandEncoder =
                        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                    let Ok(input_data) = self.node_graph.evaluate_output(
                        &device,
                        &queue,
                        &mut encoder,
                        &read_output_id,
                    ) else {
                        panic!("read did not produce an output.");
                    };

                    let Ok(texture_evaluator_id) = input_data.try_to_texture_evaluator_id() else {
                        panic!("read output was not a texture evaluator id.");
                    };

                    let Some(output_texture_view) =
                        self.node_graph.scene_graph()[texture_evaluator_id].output_texture_view()
                    else {
                        panic!("read did not produce an output TextureView.");
                    };

                    iced::Task::done(NodeGraphMessage::ViewTexture(output_texture_view.clone()))
                } else {
                    iced::Task::none()
                }
            }
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, NodeGraphMessage> {
        style
            .button("crash me")
            .on_press(NodeGraphMessage::EvaluateActiveNode)
            .into()
    }
}
