// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam;
use wgpu;

use damascus::{
    geometry::primitives::Shapes,
    gpu::{GPUResult, get_device_queue, resources::TextureView},
    graph::{
        BidirectedGraph,
        node_graph::{
            self,
            inputs::{InputId, input_data::InputData},
            nodes::{
                NodeId,
                node_data::{
                    AxisInputData, CameraInputData, LightInputData, MaterialInputData, NodeData,
                    PrimitiveInputData, RayMarcherInputData, SceneInputData, TextureReadInputData,
                },
            },
            outputs::OutputId,
        },
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
    ViewActiveNode,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    pub active_node: Option<NodeId>,
    pub node_graph: node_graph::NodeGraph,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            active_node: None,
            node_graph: node_graph::NodeGraph::new(),
        }
    }
}

impl Widget<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        context: &mut Context,
        message: NodeGraphMessage,
    ) -> iced::Task<NodeGraphMessage> {
        match message {
            NodeGraphMessage::EvaluateActiveNode => {
                // TODO this shouldnt be hardcoded, duh
                let mut graph = context.node_graph.lock().unwrap();
                if graph.node_graph.is_empty() {
                    println!("setting up");

                    // let read_id: NodeId = graph.node_graph.add_node(NodeData::TextureRead);

                    // let Ok(_input_id) = graph.node_graph.set_input_data(
                    //     &read_id,
                    //     &TextureReadInputData::Filepath,
                    //     InputData::Filepath(
                    //         "/home/ob1/software/rust/damascus/damascus/image.exr".to_string(),
                    //     ),
                    // ) else {
                    //     panic!("read could not set filepath.");
                    // };

                    let primary_camera_axis_id: NodeId = graph.node_graph.add_node(NodeData::Axis);
                    let secondary_camera_axis_id: NodeId =
                        graph.node_graph.add_node(NodeData::Axis);
                    let camera_id: NodeId = graph.node_graph.add_node(NodeData::Camera);

                    let light_id: NodeId = graph.node_graph.add_node(NodeData::Light);

                    let primitive_id: NodeId = graph.node_graph.add_node(NodeData::Primitive);
                    let primitive_axis_id: NodeId = graph.node_graph.add_node(NodeData::Axis);
                    let primitive_material_id: NodeId =
                        graph.node_graph.add_node(NodeData::Material);

                    // let primitive1_id: NodeId = graph.node_graph.add_node(NodeData::Primitive);
                    // let primitive1_axis_id: NodeId = graph.node_graph.add_node(NodeData::Axis);
                    // let primitive1_material_id: NodeId =
                    //     graph.node_graph.add_node(NodeData::Material);

                    // let primitive2_id: NodeId = graph.node_graph.add_node(NodeData::Primitive);
                    // let primitive2_axis_id: NodeId = graph.node_graph.add_node(NodeData::Axis);

                    let scene_id: NodeId = graph.node_graph.add_node(NodeData::Scene);

                    let ray_marcher_id: NodeId = graph.node_graph.add_node(NodeData::RayMarcher);

                    // Connect camera to scene
                    // /ray_marcher/scene/camera/secondary_axis/primary_axis

                    let secondary_camera_axis_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&secondary_camera_axis_id, &AxisInputData::Axis)
                        .unwrap();
                    graph.node_graph.connect_node_to_input(
                        &primary_camera_axis_id,
                        &secondary_camera_axis_input_id,
                    );

                    let camera_axis_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&camera_id, &CameraInputData::Axis)
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&secondary_camera_axis_id, &camera_axis_input_id);

                    let scene_render_camera_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&scene_id, &SceneInputData::RenderCamera)
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&camera_id, &scene_render_camera_input_id);

                    // Connect light to scene
                    // /ray_marcher/scene/camera/secondary_axis/primary_axis
                    // |           |     /light

                    let scene_light_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&scene_id, &SceneInputData::Scene)
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&light_id, &scene_light_input_id);

                    // Connect primitives to scene
                    // /ray_marcher/scene/camera/secondary_axis/primary_axis
                    // |           |     /light
                    // |           |     /primitive/axis
                    // |           |     |         /material
                    // |           |     |         /primitive2/axis2
                    // |           |     |                    /material1
                    // |           |     /primitive1/axis1
                    // |           |     |          /material1

                    let primitive_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&primitive_id, &PrimitiveInputData::Axis)
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&primitive_axis_id, &primitive_input_id);

                    let primitive_material_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&primitive_id, &PrimitiveInputData::Material)
                        .unwrap();
                    graph.node_graph.connect_node_to_input(
                        &primitive_material_id,
                        &primitive_material_input_id,
                    );

                    let scene_primitive_input_id: InputId = graph
                        .node_graph
                        .node_input_id_from_str(&scene_id, "Scene1")
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&primitive_id, &scene_primitive_input_id);

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive1_axis_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id(&primitive1_id, &PrimitiveInputData::Axis)
                    //         .unwrap(),
                    // );

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive1_material_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id(&primitive1_id, &PrimitiveInputData::Material)
                    //         .unwrap(),
                    // );

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive1_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id_from_str(&scene_id, "Scene2")
                    //         .unwrap(),
                    // );

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive2_axis_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id(&primitive2_id, &PrimitiveInputData::Axis)
                    //         .unwrap(),
                    // );

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive1_material_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id(&primitive2_id, &PrimitiveInputData::Material)
                    //         .unwrap(),
                    // );

                    // graph.node_graph.connect_node_to_input(
                    //     &primitive2_id,
                    //     &graph
                    //         .node_graph
                    //         .node_input_id(&primitive_id, &PrimitiveInputData::Child)
                    //         .unwrap(),
                    // );

                    // Connect scene to ray marcher

                    let ray_marcher_scene_input_id: InputId = graph
                        .node_graph
                        .node_input_id(&ray_marcher_id, &RayMarcherInputData::SceneRoot)
                        .unwrap();
                    graph
                        .node_graph
                        .connect_node_to_input(&scene_id, &ray_marcher_scene_input_id);

                    let _ = graph.node_graph.set_input_data(
                        &camera_id,
                        &CameraInputData::SensorResolution,
                        InputData::UVec2(glam::UVec2::new(2048u32, 1024u32)),
                    );

                    let _ = graph.node_graph.set_input_data(
                        &secondary_camera_axis_id,
                        &AxisInputData::Translate,
                        InputData::Vec3(glam::Vec3::Z * 100.),
                    );

                    // Modify light data

                    let _ = graph.node_graph.set_input_data(
                        &light_id,
                        &LightInputData::Colour,
                        InputData::Vec3(glam::Vec3::new(1., 0.1, 0.1)),
                    );

                    // Modify primitive data

                    let _ = graph.node_graph.set_input_data(
                        &primitive_material_id,
                        &MaterialInputData::DiffuseColour,
                        InputData::Vec3(glam::Vec3::new(0.1, 0.1, 1.)),
                    );

                    let _ = graph.node_graph.set_input_data(
                        &primitive_id,
                        &PrimitiveInputData::Shape,
                        InputData::Enum(Shapes::Capsule.into()),
                    );

                    let _ = graph.node_graph.set_input_data(
                        &primitive_id,
                        &PrimitiveInputData::BlendStrength,
                        InputData::Float(0.5),
                    );

                    // graph.node_graph
                    //     .set_input_data(
                    //         &primitive1_axis_id,
                    //         &AxisInputData::Translate,
                    //         InputData::Vec3(glam::Vec3::X),
                    //     );

                    graph.active_node = Some(ray_marcher_id);
                }

                iced::Task::done(NodeGraphMessage::ViewActiveNode)
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
            .button("read from file")
            .on_press(NodeGraphMessage::EvaluateActiveNode)
            .into()
    }
}
