// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;

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
        outputs::OutputId,
    },
};

use crate::{app::Context, widgets::Style};

use super::{
    Widget,
    pan_zoom::{PanZoom, PanZoomMessage},
    selection_box::{SelectionBox, SelectionBoxMessage},
};

pub mod node;
use node::{NodeGraphUIState, node_data::NodeData as NodeUIData};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum NodeGraphMessage {
    InputValueChanged(NodeId, String),
    CreateReadNode,
    CreateLiveNode,
    PanZoom(PanZoomMessage),
    SelectionBox(SelectionBoxMessage),
    ClearSelection,
    UpdateSelection,
    ViewNodeOutput(u32, Option<OutputId>),
    #[serde(skip)]
    CanvasEvent(iced::Event),
    #[default]
    None,
}

impl From<PanZoomMessage> for NodeGraphMessage {
    fn from(pan_zoom_message: PanZoomMessage) -> Self {
        Self::PanZoom(pan_zoom_message)
    }
}

impl From<SelectionBoxMessage> for NodeGraphMessage {
    fn from(selection_box_message: SelectionBoxMessage) -> Self {
        Self::SelectionBox(selection_box_message)
    }
}

impl From<NodeGraphMessage> for iced::Task<NodeGraphMessage> {
    fn from(message: NodeGraphMessage) -> Self {
        iced::Task::done(message)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NodeGraph {
    state: NodeGraphUIState,
    pan_zoom: PanZoom,
    selection_box: SelectionBox,
    selected_nodes: HashSet<NodeId>,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            state: NodeGraphUIState::default(),
            pan_zoom: PanZoom::default(),
            selection_box: SelectionBox::default(),
            selected_nodes: HashSet::<NodeId>::default(),
        }
    }
}

impl NodeGraph {
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
    }

    pub fn update_selection(&mut self, context: &mut Context) {
        for (node_id, node_data) in self.state.iter() {
            if self.selection_box.contains(node_data.shape.center) {
                if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                    self.selected_nodes.remove(&node_id);
                } else {
                    self.selected_nodes.insert(node_id);
                }
            }
        }
    }

    pub fn handle_mouse_event(
        &mut self,
        context: &mut Context,
        mouse_event: &iced::mouse::Event,
    ) -> iced::Task<NodeGraphMessage> {
        match mouse_event {
            iced::mouse::Event::CursorEntered => iced::Task::none(),
            iced::mouse::Event::CursorLeft => iced::Task::none(),
            iced::mouse::Event::CursorMoved { position } => iced::Task::batch(
                std::iter::once(
                    // Update the PanZoom widget with the new global
                    // cursor position
                    self.pan_zoom
                        .update(
                            context,
                            PanZoomMessage::MoveCursor(Vec2::new(position.x, position.y)),
                        )
                        .map(|pan_zoom_message| pan_zoom_message.into()),
                )
                .chain(if self.selection_box.selecting {
                    // If a selection box is active, update its bounds
                    // with the new local cursor position
                    let task: iced::Task<NodeGraphMessage> = self
                        .selection_box
                        .update(
                            context,
                            SelectionBoxMessage::MoveCursor(
                                self.pan_zoom
                                    .global_to_local(Vec2::new(position.x, position.y)),
                            ),
                        )
                        .map(|selection_box_message| selection_box_message.into());

                    // Update the selection based on the selection box  bounds
                    if !context.keyboard_modifiers.control() {
                        self.clear_selection();
                    }
                    self.update_selection(context);

                    // Pass the selection box task back to the application
                    Some(task).into_iter()
                } else {
                    None.into_iter()
                }),
            ),
            iced::mouse::Event::ButtonPressed(button) => match button {
                iced::mouse::Button::Left => {
                    if self.pan_zoom.cursor_in_bounds() {
                        // Initiate a box selection
                        let task: iced::Task<NodeGraphMessage> = self
                            .selection_box
                            .update(
                                context,
                                SelectionBoxMessage::BeginSelection(
                                    self.pan_zoom.local_cursor_position(),
                                ),
                            )
                            .map(|selection_box_message| selection_box_message.into());

                        // Potentially reset the selection

                        if !context.keyboard_modifiers.control() {
                            self.clear_selection();
                        }

                        task
                    } else {
                        iced::Task::none()
                    }
                }
                iced::mouse::Button::Right => iced::Task::none(),
                iced::mouse::Button::Middle => iced::Task::done(PanZoomMessage::BeginPan.into()),
                iced::mouse::Button::Back => iced::Task::none(),
                iced::mouse::Button::Forward => iced::Task::none(),
                _ => iced::Task::none(),
            },
            iced::mouse::Event::ButtonReleased(button) => match button {
                iced::mouse::Button::Left => {
                    iced::Task::done(SelectionBoxMessage::EndSelection.into())
                }
                iced::mouse::Button::Right => iced::Task::none(),
                iced::mouse::Button::Middle => iced::Task::done(PanZoomMessage::EndPan.into()),
                iced::mouse::Button::Back => iced::Task::none(),
                iced::mouse::Button::Forward => iced::Task::none(),
                _ => iced::Task::none(),
            },
            iced::mouse::Event::WheelScrolled { delta: _ } => iced::Task::none(),
        }
    }

    pub fn handle_keyboard_event(
        &mut self,
        context: &mut Context,
        keyboard_event: &iced::keyboard::Event,
    ) -> iced::Task<NodeGraphMessage> {
        match keyboard_event {
            iced::keyboard::Event::KeyPressed { key, .. } => match key {
                iced::keyboard::Key::Character(character) => {
                    if let Ok(digits) = character.as_str().parse() {
                        iced::Task::done(
                            if let Some(node_id) = self.selected_nodes.iter().next()
                                && let Ok(node_graph) = context.node_graph.lock()
                                && let Some(output_id) = node_graph.nodes_first_output_id(node_id)
                            {
                                NodeGraphMessage::ViewNodeOutput(digits, Some(*output_id))
                            } else {
                                NodeGraphMessage::ViewNodeOutput(digits, None)
                            },
                        )
                    } else {
                        iced::Task::none()
                    }
                }
                _ => iced::Task::none(),
            },
            iced::keyboard::Event::KeyReleased {
                key,
                modified_key: _,
                physical_key: _,
                location: _,
                modifiers: _,
            } => match key {
                iced::keyboard::Key::Character(character) => {
                    if character == "r" {
                        iced::Task::done(NodeGraphMessage::CreateReadNode)
                    } else if character == "l" {
                        iced::Task::done(NodeGraphMessage::CreateLiveNode)
                    } else {
                        iced::Task::none()
                    }
                }
                _ => iced::Task::none(),
            },
            iced::keyboard::Event::ModifiersChanged(_modifiers) => iced::Task::none(),
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
        } else {
            match event {
                iced::Event::Mouse(_) | iced::Event::Keyboard(_) => Some(
                    iced::widget::Action::publish(NodeGraphMessage::CanvasEvent(event.clone())),
                ),
                _ => None,
            }
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
        for (node_id, node_data) in self.state.iter() {
            let node_center: Vec2 = self.pan_zoom.local_to_canvas(node_data.shape.center);
            let node_top_left: Vec2 = self.pan_zoom.local_to_canvas(node_data.shape.top_left());
            let node = iced::widget::canvas::Path::rectangle(
                iced::Point::new(node_top_left.x, node_top_left.y),
                iced::Size::new(node_data.shape.size.x, node_data.shape.size.y)
                    / self.pan_zoom.zoom,
            );
            frame.fill(
                &node,
                iced::Color::from_rgb(node_data.colour.x, node_data.colour.y, node_data.colour.z),
            );
            if self.selected_nodes.contains(&node_id) {
                frame.stroke(
                    &node,
                    iced::widget::canvas::Stroke::default()
                        .with_width(1.)
                        .with_color(iced::Color::WHITE),
                );
            }
            frame.fill_text(iced::widget::canvas::Text {
                content: node_data.label.clone(),
                position: iced::Point::new(node_center.x, node_center.y),
                color: iced::Color::WHITE,
                size: iced::Pixels(16.0 / self.pan_zoom.zoom),
                align_x: iced::advanced::text::Alignment::Center,
                align_y: iced::alignment::Vertical::Center,
                ..iced::widget::canvas::Text::default()
            });
        }

        if self.selection_box.selecting {
            let bounds: Rectangle = self.selection_box.bounds();
            let selection_box_top_left: Vec2 = self.pan_zoom.local_to_canvas(bounds.top_left());
            let selection_box = iced::widget::canvas::Path::rectangle(
                iced::Point::new(selection_box_top_left.x, selection_box_top_left.y),
                iced::Size::new(bounds.size.x, bounds.size.y) / self.pan_zoom.zoom,
            );
            frame.fill(&selection_box, iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1));
            frame.stroke(
                &selection_box,
                iced::widget::canvas::Stroke::default()
                    .with_width(1.)
                    .with_color(iced::Color::WHITE),
            );
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
            NodeGraphMessage::CanvasEvent(event) => match event {
                iced::Event::Mouse(mouse_event) => self.handle_mouse_event(context, &mouse_event),
                iced::Event::Keyboard(keyboard_event) => {
                    if self.pan_zoom.cursor_in_bounds() {
                        self.handle_keyboard_event(context, &keyboard_event)
                    } else {
                        iced::Task::none()
                    }
                }
                _ => iced::Task::none(),
            },
            NodeGraphMessage::PanZoom(pan_zoom_message) => self
                .pan_zoom
                .update(context, pan_zoom_message)
                .map(|pan_zoom_message| pan_zoom_message.into()),
            NodeGraphMessage::SelectionBox(selection_box_message) => self
                .selection_box
                .update(context, selection_box_message)
                .map(|selection_box_message| selection_box_message.into()),
            NodeGraphMessage::ClearSelection => {
                self.selected_nodes.clear();

                iced::Task::none()
            }
            NodeGraphMessage::UpdateSelection => {
                for (node_id, node_data) in self.state.iter() {
                    if self.selection_box.contains(node_data.shape.center) {
                        self.selected_nodes.insert(node_id);
                    }
                }

                iced::Task::none()
            }
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
                    NodeUIData::new(format!("Read{}", read_id))
                        .shape(
                            Rectangle::default()
                                .center(self.pan_zoom.local_cursor_position())
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
                    NodeUIData::new(format!("RayMarcher{}", ray_marcher_id))
                        .shape(
                            Rectangle::default()
                                .center(self.pan_zoom.local_cursor_position())
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
