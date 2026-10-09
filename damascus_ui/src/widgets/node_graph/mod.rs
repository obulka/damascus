// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::{UVec2, Vec2, Vec3};

use damascus::{
    geometry::{primitives::Shapes, rectangle::Rectangle},
    graph::node_graph::{
        inputs::{InputId, input_data::InputData},
        nodes::{
            NodeId,
            node_data::{
                AxisInputData, CameraInputData, LightInputData, MaterialInputData, NodeData,
                PrimitiveInputData, RayMarcherInputData, SceneInputData, TextureReadInputData,
            },
        },
    },
};

use crate::{app::Context, widgets::Style};

use super::{
    Widget,
    pan_zoom::{PanZoom, PanZoomMessage},
};

pub mod node;
use node::{NodeGraphUIState, node_data::NodeData as NodeUIData};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NodeGraphMessage {
    InputValueChanged(NodeId, String),
    CreateReadNode,
    CreateLiveNode,
    PanZoom(PanZoomMessage),
}

impl From<PanZoomMessage> for NodeGraphMessage {
    fn from(pan_zoom_message: PanZoomMessage) -> Self {
        Self::PanZoom(pan_zoom_message)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    state: NodeGraphUIState,
    pan_zoom: PanZoom,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            state: NodeGraphUIState::default(),
            pan_zoom: PanZoom::default(),
        }
    }
}

impl NodeGraph {
    pub fn handle_mouse_event(&self, mouse_event: &iced::mouse::Event) -> Option<NodeGraphMessage> {
        match mouse_event {
            iced::mouse::Event::CursorEntered => None,
            iced::mouse::Event::CursorLeft => None,
            iced::mouse::Event::CursorMoved { position } => {
                Some(PanZoomMessage::MoveCursor(Vec2::new(position.x, position.y)).into())
            }
            iced::mouse::Event::ButtonPressed(button) => match button {
                iced::mouse::Button::Left => None,
                iced::mouse::Button::Right => None,
                iced::mouse::Button::Middle => Some(PanZoomMessage::BeginPan.into()),
                iced::mouse::Button::Back => None,
                iced::mouse::Button::Forward => None,
                _ => None,
            },
            iced::mouse::Event::ButtonReleased(button) => match button {
                iced::mouse::Button::Left => None,
                iced::mouse::Button::Right => None,
                iced::mouse::Button::Middle => Some(PanZoomMessage::EndPan.into()),
                iced::mouse::Button::Back => None,
                iced::mouse::Button::Forward => None,
                _ => None,
            },
            iced::mouse::Event::WheelScrolled { delta: _ } => None,
        }
    }

    pub fn handle_keyboard_event(
        &self,
        keyboard_event: &iced::keyboard::Event,
    ) -> Option<NodeGraphMessage> {
        match keyboard_event {
            iced::keyboard::Event::KeyPressed { .. } => None,
            iced::keyboard::Event::KeyReleased {
                key,
                modified_key: _,
                physical_key: _,
                location: _,
                modifiers: _,
            } => match key {
                iced::keyboard::Key::Character(character) => {
                    if character == "r" {
                        Some(NodeGraphMessage::CreateReadNode)
                    } else if character == "l" {
                        Some(NodeGraphMessage::CreateLiveNode)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            iced::keyboard::Event::ModifiersChanged(_modifiers) => None,
        }
    }
}

impl iced::widget::canvas::Program<NodeGraphMessage> for NodeGraph {
    type State = iced::Rectangle;

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Option<iced::widget::Action<NodeGraphMessage>> {
        if *state != bounds {
            *state = bounds;
            Some(iced::widget::Action::publish(
                PanZoomMessage::Resize(Rectangle {
                    center: Vec2::new(bounds.center_x(), bounds.center_y()),
                    size: Vec2::new(bounds.width, bounds.height),
                })
                .into(),
            ))
        } else if let Some(message) = match event {
            iced::Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            iced::Event::Keyboard(keyboard_event) => self.handle_keyboard_event(keyboard_event),
            _ => None,
        } {
            Some(iced::widget::Action::publish(message))
        } else {
            None
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<iced::Renderer>> {
        let mut frame = iced::widget::canvas::Frame::new(renderer, bounds.size());
        for node_data in self.state.values() {
            let node_center: Vec2 = self.pan_zoom.local_to_canvas(node_data.shape.center);
            let node_top_left: Vec2 = self.pan_zoom.local_to_canvas(node_data.shape.top_left());
            let node: iced::widget::canvas::Path = iced::widget::canvas::Path::rectangle(
                iced::Point::new(node_top_left.x, node_top_left.y),
                iced::Size::new(node_data.shape.size.x, node_data.shape.size.y),
            );
            frame.fill(
                &node,
                iced::Color::from_rgb(node_data.colour.x, node_data.colour.y, node_data.colour.z),
            );
            frame.fill_text(iced::widget::canvas::Text {
                content: node_data.label.clone(),
                position: iced::Point::new(node_center.x, node_center.y),
                color: iced::Color::WHITE,
                size: iced::Pixels(16.0),
                align_x: iced::advanced::text::Alignment::Center,
                align_y: iced::alignment::Vertical::Center,
                ..iced::widget::canvas::Text::default()
            });
        }

        vec![frame.into_geometry()]
    }
}

impl Widget<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        context: &mut Context,
        message: NodeGraphMessage,
    ) -> iced::Task<NodeGraphMessage> {
        match message {
            NodeGraphMessage::PanZoom(pan_zoom_message) => self
                .pan_zoom
                .update(context, pan_zoom_message)
                .map(|pan_zoom_message| pan_zoom_message.into()),
            NodeGraphMessage::CreateReadNode => {
                let mut node_graph = context.node_graph.lock().unwrap();

                let read_id: NodeId = node_graph.add_node(NodeData::TextureRead);

                let Ok(_input_id) = node_graph.set_input_data(
                    &read_id,
                    &TextureReadInputData::Filepath,
                    InputData::Filepath(
                        "/home/ob1/software/rust/damascus/damascus/image.exr".to_string(),
                    ),
                ) else {
                    panic!("read could not set filepath.");
                };

                self.state.insert(
                    read_id,
                    NodeUIData::new(format!("Read{:?}", read_id))
                        .shape(
                            Rectangle::default()
                                .center(self.pan_zoom.local_cursor_position)
                                .size(Vec2::new(100.0, 33.0)),
                        )
                        .colour(context.default_style.default_node_colour),
                );

                iced::Task::none()
            }
            NodeGraphMessage::CreateLiveNode => {
                let mut node_graph = context.node_graph.lock().unwrap();

                let primary_camera_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
                let secondary_camera_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
                let camera_id: NodeId = node_graph.add_node(NodeData::Camera);

                let light_id: NodeId = node_graph.add_node(NodeData::Light);

                let primitive_id: NodeId = node_graph.add_node(NodeData::Primitive);
                let primitive_axis_id: NodeId = node_graph.add_node(NodeData::Axis);
                let primitive_material_id: NodeId = node_graph.add_node(NodeData::Material);

                let scene_id: NodeId = node_graph.add_node(NodeData::Scene);

                let ray_marcher_id: NodeId = node_graph.add_node(NodeData::RayMarcher);

                // Connect camera to scene
                // /ray_marcher/scene/camera/secondary_axis/primary_axis

                let secondary_camera_axis_input_id: InputId = node_graph
                    .node_input_id(&secondary_camera_axis_id, &AxisInputData::Axis)
                    .unwrap();
                node_graph.connect_node_to_input(
                    &primary_camera_axis_id,
                    &secondary_camera_axis_input_id,
                );

                let camera_axis_input_id: InputId = node_graph
                    .node_input_id(&camera_id, &CameraInputData::Axis)
                    .unwrap();

                node_graph.connect_node_to_input(&secondary_camera_axis_id, &camera_axis_input_id);

                let scene_render_camera_input_id: InputId = node_graph
                    .node_input_id(&scene_id, &SceneInputData::RenderCamera)
                    .unwrap();

                node_graph.connect_node_to_input(&camera_id, &scene_render_camera_input_id);

                // Connect light to scene
                // /ray_marcher/scene/camera/secondary_axis/primary_axis
                // |           |     /light

                let scene_light_input_id: InputId = node_graph
                    .node_input_id(&scene_id, &SceneInputData::Scene)
                    .unwrap();

                node_graph.connect_node_to_input(&light_id, &scene_light_input_id);

                // Connect primitives to scene
                // /ray_marcher/scene/camera/secondary_axis/primary_axis
                // |           |     /light
                // |           |     /primitive/axis
                // |           |     |         /material
                // |           |     |         /primitive2/axis2
                // |           |     |                    /material1
                // |           |     /primitive1/axis1
                // |           |     |          /material1

                let primitive_input_id: InputId = node_graph
                    .node_input_id(&primitive_id, &PrimitiveInputData::Axis)
                    .unwrap();

                node_graph.connect_node_to_input(&primitive_axis_id, &primitive_input_id);

                let primitive_material_input_id: InputId = node_graph
                    .node_input_id(&primitive_id, &PrimitiveInputData::Material)
                    .unwrap();

                node_graph
                    .connect_node_to_input(&primitive_material_id, &primitive_material_input_id);

                let scene_primitive_input_id: InputId = node_graph
                    .node_input_id_from_str(&scene_id, "Scene1")
                    .unwrap();

                node_graph.connect_node_to_input(&primitive_id, &scene_primitive_input_id);

                // Connect scene to ray marcher

                let ray_marcher_scene_input_id: InputId = node_graph
                    .node_input_id(&ray_marcher_id, &RayMarcherInputData::SceneRoot)
                    .unwrap();

                node_graph.connect_node_to_input(&scene_id, &ray_marcher_scene_input_id);

                // Modify camera data

                let _ = node_graph.set_input_data(
                    &camera_id,
                    &CameraInputData::SensorResolution,
                    InputData::UVec2(UVec2::new(2048u32, 1024u32)),
                );

                let _ = node_graph.set_input_data(
                    &secondary_camera_axis_id,
                    &AxisInputData::Translate,
                    InputData::Vec3(Vec3::Z * 10.),
                );

                // Modify light data

                let _ = node_graph.set_input_data(
                    &light_id,
                    &LightInputData::Colour,
                    InputData::Vec3(Vec3::new(1., 0.1, 0.1)),
                );

                // Modify primitive data

                let _ = node_graph.set_input_data(
                    &primitive_material_id,
                    &MaterialInputData::DiffuseColour,
                    InputData::Vec3(Vec3::new(0.1, 0.1, 1.)),
                );

                let _ = node_graph.set_input_data(
                    &primitive_id,
                    &PrimitiveInputData::Shape,
                    InputData::Enum(Shapes::Capsule.into()),
                );

                let _ = node_graph.set_input_data(
                    &primitive_id,
                    &PrimitiveInputData::BlendStrength,
                    InputData::Float(0.5),
                );

                self.state.insert(
                    ray_marcher_id,
                    NodeUIData::new(format!("RayMarcher{:?}", ray_marcher_id))
                        .shape(
                            Rectangle::default()
                                .center(self.pan_zoom.local_cursor_position)
                                .size(Vec2::new(100.0, 33.0)),
                        )
                        .colour(context.default_style.default_node_colour),
                );

                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, NodeGraphMessage> {
        iced::widget::mouse_area(
            iced::widget::canvas::Canvas::new(self)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .on_scroll(|scroll_delta| {
            PanZoomMessage::Zoom(match scroll_delta {
                iced::mouse::ScrollDelta::Lines { y, .. } => y * style.viewer_zoom_sensitivity,
                iced::mouse::ScrollDelta::Pixels { y, .. } => y * style.viewer_zoom_sensitivity,
            })
            .into()
        })
        .into()
    }
}
