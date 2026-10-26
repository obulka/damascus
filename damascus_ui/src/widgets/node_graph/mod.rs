// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use glam::{UVec2, Vec2, Vec3};
use slotmap::SparseSecondaryMap;

use damascus::{
    geometry::{primitives::Shapes, ray::Ray2d, rectangle::Rectangle},
    graph::node_graph::{
        NodeGraph as Graph,
        inputs::{InputId, input_data::InputData},
        nodes::{
            NodeId,
            node::Node,
            node_data::{
                AxisInputData, CameraInputData, LightInputData, MaterialInputData, NodeData,
                PrimitiveInputData, RayMarcherInputData, SceneInputData, TextureReadInputData,
            },
        },
        outputs::OutputId,
    },
};

use crate::{
    app::Context,
    type_conversion::{FromGlam, ToPath},
    widgets::{
        style::Style,
        viewport::{ViewportContext, ViewportId, Viewports},
    },
};

use super::{
    Widget,
    pan_zoom::{PanZoom, PanZoomMessage},
    selection_box::{SelectionBox, SelectionBoxMessage},
};

pub mod node;
use node::NodeContext;

// This is the data of which a single instance will be shared among widgets
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NodeGraphContext {
    pub context: SparseSecondaryMap<NodeId, NodeContext>,
    pub graph: Graph,
    pub selected_nodes: HashSet<NodeId>,
    pub selected_viewports: HashSet<ViewportId>,
    pub active_viewport: Option<ViewportId>,
}

impl NodeGraphContext {
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.selected_viewports.clear();
    }

    pub fn viewport_removed(&mut self, viewport_id: &ViewportId) {
        if let Some(active_viewport) = self.active_viewport
            && active_viewport == *viewport_id
        {
            self.active_viewport = None;
        }

        self.selected_viewports.remove(viewport_id);
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum NodeGraphMessage {
    InputValueChanged(NodeId, String),
    CreateNode(NodeData),
    CreateLiveNode,
    ViewportCreated(ViewportId),
    PanZoom(PanZoomMessage),
    SelectionBox(SelectionBoxMessage),
    ClearSelection,
    UpdateStyle,
    ViewNodeOutput(Option<ViewportId>, u32, Option<OutputId>),
    RemovedViewports(HashSet<ViewportId>),
    NewViewport(ViewportId),
    OpenProperties(NodeId),
    DoubleClick,
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
    context: Arc<Mutex<NodeGraphContext>>,
    viewport_context: Arc<Mutex<Viewports>>,
    pan_zoom: PanZoom,
    selection_box: SelectionBox,
    dragging_nodes: bool,
    dragged_nodes: bool,
    style: Style,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            context: Arc::default(),
            viewport_context: Arc::default(),
            pan_zoom: PanZoom::default(),
            selection_box: SelectionBox::default(),
            dragging_nodes: false,
            dragged_nodes: false,
            style: Style::default(),
        }
    }
}

impl NodeGraph {
    pub fn clear_selection(&self) {
        let Ok(mut node_graph) = self.context.lock() else {
            return;
        };

        node_graph.clear_selection();
    }

    pub fn drag_select_nodes(&self, context: &Context) {
        let Ok(mut node_graph) = self.context.lock() else {
            return;
        };

        for node_id in node_graph
            .context
            .iter()
            .filter_map(|(node_id, node_context)| {
                if self.selection_box.contains(node_context.shape.center) {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .iter()
        {
            if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                node_graph.selected_nodes.remove(node_id);
            } else {
                node_graph.selected_nodes.insert(*node_id);
            }
        }
    }

    pub fn drag_select_viewports(&self, context: &Context) {
        let Ok(viewports) = context.viewports.lock() else {
            return;
        };
        let Ok(mut node_graph) = context.node_graph.lock() else {
            return;
        };
        for (viewport_id, viewport_context) in viewports.iter() {
            if self
                .selection_box
                .contains(viewport_context.node_context.shape.center)
            {
                if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                    node_graph.selected_viewports.remove(&viewport_id);
                } else {
                    node_graph.selected_viewports.insert(viewport_id);
                    node_graph.active_viewport = Some(viewport_id);
                }
            }
        }
    }

    pub fn drag_select(&self, context: &Context) {
        self.drag_select_nodes(context);
        self.drag_select_viewports(context);
    }

    pub fn click_select_selected_viewports(
        &self,
        context: &Context,
        clear_selection: bool,
    ) -> bool {
        let Ok(viewports) = context.viewports.lock() else {
            return false;
        };
        let Ok(mut node_graph) = context.node_graph.lock() else {
            return false;
        };

        let mut any_selected = false;

        let cursor_position: Vec2 = self.pan_zoom.local_cursor_position();
        for viewport_id in node_graph.selected_viewports.clone().iter() {
            let Some(viewport_context) = viewports.get(*viewport_id) else {
                continue;
            };

            if !viewport_context
                .node_context
                .shape
                .contains(cursor_position)
            {
                continue;
            }

            if clear_selection {
                node_graph.selected_nodes.clear();
                node_graph.selected_viewports.clear();
            }

            if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                node_graph.selected_viewports.remove(viewport_id);
            } else {
                node_graph.selected_viewports.insert(*viewport_id);
                node_graph.active_viewport = Some(*viewport_id);
                any_selected = true;
            }

            break;
        }

        any_selected
    }

    pub fn click_select_selected_nodes(&self, context: &Context, clear_selection: bool) -> bool {
        let Ok(mut node_graph) = context.node_graph.lock() else {
            return false;
        };

        let mut any_selected = false;

        let cursor_position: Vec2 = self.pan_zoom.local_cursor_position();

        for node_id in node_graph.selected_nodes.clone().iter() {
            let Some(node_context) = node_graph.context.get(*node_id) else {
                continue;
            };

            if !node_context.shape.contains(cursor_position) {
                continue;
            }

            if clear_selection {
                node_graph.selected_nodes.clear();
                node_graph.selected_viewports.clear();
            }

            if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                node_graph.selected_nodes.remove(node_id);
            } else {
                node_graph.selected_nodes.insert(*node_id);
                any_selected = true;
            }

            break;
        }

        any_selected
    }

    pub fn click_select_selected(&mut self, context: &Context, clear_selection: bool) -> bool {
        self.click_select_selected_nodes(context, clear_selection)
            || self.click_select_selected_viewports(context, clear_selection)
    }

    pub fn hovered_node(&self, node_graph: &mut NodeGraphContext) -> Option<NodeId> {
        let cursor_position: Vec2 = self.pan_zoom.local_cursor_position();

        node_graph
            .context
            .iter()
            .filter_map(|(node_id, node_context)| {
                if node_context.shape.contains(cursor_position) {
                    Some(node_id)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn click_select_nodes(&self, context: &Context) -> bool {
        let mut any_selected = false;

        let Ok(mut node_graph) = context.node_graph.lock() else {
            return any_selected;
        };

        if let Some(node_id) = self.hovered_node(&mut node_graph) {
            if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                node_graph.selected_nodes.remove(&node_id);
            } else {
                node_graph.selected_nodes.insert(node_id);
                any_selected = true;
            }
        }

        any_selected
    }

    pub fn click_select_viewports(&self, context: &Context) -> bool {
        let Ok(viewports) = context.viewports.lock() else {
            return false;
        };
        let Ok(mut node_graph) = context.node_graph.lock() else {
            return false;
        };

        let mut any_selected = false;

        let cursor_position: Vec2 = self.pan_zoom.local_cursor_position();
        for (viewport_id, viewport_context) in viewports.iter() {
            if !viewport_context
                .node_context
                .shape
                .contains(cursor_position)
            {
                continue;
            }

            if context.keyboard_modifiers.control() && context.keyboard_modifiers.shift() {
                node_graph.selected_viewports.remove(&viewport_id);
            } else {
                node_graph.selected_viewports.insert(viewport_id);
                node_graph.active_viewport = Some(viewport_id);
                any_selected = true;
            }

            break;
        }

        any_selected
    }

    pub fn click_select(&mut self, context: &Context) -> bool {
        self.click_select_nodes(context) || self.click_select_viewports(context)
    }

    pub fn drag_nodes(&mut self, context: &Context) {
        if self.dragging_nodes
            && let Ok(mut node_graph) = context.node_graph.lock()
        {
            for node_id in node_graph
                .context
                .iter()
                .filter_map(|(node_id, _node_context)| {
                    if node_graph.selected_nodes.contains(&node_id) {
                        Some(node_id)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .iter()
            {
                if let Some(node_context) = node_graph.context.get_mut(*node_id) {
                    node_context.shape.center += self.pan_zoom.local_cursor_velocity();
                    self.dragged_nodes = true;
                }
            }
        }
    }

    pub fn drag_viewports(&mut self, context: &Context) {
        if self.dragging_nodes
            && let Ok(mut viewports) = context.viewports.lock()
            && let Ok(node_graph) = context.node_graph.lock()
        {
            for (viewport_id, viewport_context) in viewports.iter_mut() {
                if !node_graph.selected_viewports.contains(&viewport_id) {
                    continue;
                }
                viewport_context.node_context.shape.center += self.pan_zoom.local_cursor_velocity();
                self.dragged_nodes = true;
            }
        }
    }

    pub fn drag(&mut self, context: &Context) {
        self.drag_nodes(context);
        self.drag_viewports(context);
    }

    pub fn remove_selected(&self) -> iced::Task<NodeGraphMessage> {
        let Ok(mut node_graph) = self.context.lock() else {
            return iced::Task::none();
        };

        let task: iced::Task<NodeGraphMessage> = if !node_graph.selected_viewports.is_empty()
            && let Ok(mut viewports) = self.viewport_context.lock()
        {
            let removed_viewports: HashSet<ViewportId> = node_graph.selected_viewports.clone();
            for viewport_id in removed_viewports.iter() {
                viewports.remove(*viewport_id);
                node_graph.viewport_removed(viewport_id);
            }

            node_graph.selected_viewports.clear();

            iced::Task::done(NodeGraphMessage::RemovedViewports(removed_viewports))
        } else {
            iced::Task::none()
        };

        if !node_graph.selected_nodes.is_empty() {
            for node_id in node_graph.selected_nodes.clone().iter() {
                node_graph.graph.remove_node(*node_id);
                node_graph.context.remove(*node_id);
            }

            if let Ok(mut viewports) = self.viewport_context.lock() {
                for viewport in viewports.values_mut() {
                    viewport
                        .views
                        .retain(|_, output_id| node_graph.graph.output_id_is_valid(*output_id));
                    viewport.clear_active_view_if_invalid();
                }
            }

            node_graph.selected_nodes.clear();
        }

        task
    }

    pub fn create_viewport(&self, context: &Context) -> Option<ViewportId> {
        let Ok(mut viewports) = self.viewport_context.lock() else {
            return None;
        };

        Some(viewports.insert_with_key(|id| {
            let mut viewport_context = ViewportContext::default();
            viewport_context.node_context = NodeContext::new(format!("Viewport{}", id))
                .colour(context.default_style.default_node_colour);
            viewport_context
        }))
    }

    pub fn handle_mouse_event(
        &mut self,
        context: &mut Context,
        mouse_event: &iced::mouse::Event,
    ) -> iced::Task<NodeGraphMessage> {
        match mouse_event {
            iced::mouse::Event::CursorEntered => iced::Task::none(),
            iced::mouse::Event::CursorLeft => iced::Task::none(),
            iced::mouse::Event::CursorMoved { position } => {
                let mut task: iced::Task<NodeGraphMessage> = self
                    .pan_zoom
                    .update(
                        context,
                        PanZoomMessage::MoveCursor(Vec2::new(position.x, position.y)),
                    )
                    .map(|pan_zoom_message| pan_zoom_message.into());

                self.drag(context);

                if self.selection_box.selecting {
                    // If a selection box is active, update its bounds
                    // with the new local cursor position
                    let selection_task: iced::Task<NodeGraphMessage> = self
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
                    self.drag_select(context);

                    // Pass the selection box task back to the application
                    task = task.chain(selection_task)
                }

                task
            }
            iced::mouse::Event::ButtonPressed(button) => match button {
                iced::mouse::Button::Left => {
                    if self.pan_zoom.cursor_in_bounds() {
                        let mut node_clicked = false;

                        if !self.click_select_selected(context, false) {
                            // Potentially reset the selection
                            if !context.keyboard_modifiers.control() {
                                self.clear_selection();
                            }

                            if self.click_select(context) {
                                node_clicked = true;
                            }
                        } else {
                            node_clicked = true;
                        }

                        if node_clicked {
                            self.dragging_nodes = true;
                            self.dragged_nodes = false;

                            iced::Task::none()
                        } else {
                            // Initiate a box selection
                            self.selection_box
                                .update(
                                    context,
                                    SelectionBoxMessage::BeginSelection(
                                        self.pan_zoom.local_cursor_position(),
                                    ),
                                )
                                .map(|selection_box_message| selection_box_message.into())
                        }
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
                    if self.dragging_nodes && !self.dragged_nodes {
                        self.click_select_selected(context, true);
                    }
                    self.dragging_nodes = false;

                    let task: iced::Task<NodeGraphMessage> = self
                        .selection_box
                        .update(context, SelectionBoxMessage::EndSelection)
                        .map(|selection_box_message| selection_box_message.into());

                    task
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
                iced::keyboard::Key::Named(key) => match key {
                    iced::keyboard::key::Named::Delete | iced::keyboard::key::Named::Backspace => {
                        self.remove_selected()
                    }
                    _ => iced::Task::none(),
                },
                iced::keyboard::Key::Character(character) => {
                    if let Ok(digits) = character.as_str().parse() {
                        if let Ok(mut node_graph) = self.context.lock() {
                            let task: iced::Task<NodeGraphMessage> =
                                if node_graph.active_viewport.is_none() {
                                    if let Ok(viewports) = context.viewports.lock()
                                        && let Some(id) = viewports.keys().next()
                                    {
                                        node_graph.active_viewport = Some(id);
                                        iced::Task::none()
                                    } else {
                                        node_graph.active_viewport = self.create_viewport(context);
                                        if let Some(new_viewport) = node_graph.active_viewport {
                                            iced::Task::done(NodeGraphMessage::NewViewport(
                                                new_viewport,
                                            ))
                                        } else {
                                            iced::Task::none()
                                        }
                                    }
                                } else {
                                    iced::Task::none()
                                };
                            iced::Task::batch(std::iter::once(task).chain(std::iter::once(
                                iced::Task::done(
                                    if let Some(node_id) = node_graph.selected_nodes.iter().next()
                                        && let Some(output_id) =
                                            node_graph.graph.nodes_first_output_id(node_id)
                                    {
                                        NodeGraphMessage::ViewNodeOutput(
                                            node_graph.active_viewport,
                                            digits,
                                            Some(*output_id),
                                        )
                                    } else {
                                        NodeGraphMessage::ViewNodeOutput(
                                            node_graph.active_viewport,
                                            digits,
                                            None,
                                        )
                                    },
                                ),
                            )))
                        } else {
                            iced::Task::none()
                        }
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
                        iced::Task::done(NodeGraphMessage::CreateNode(NodeData::TextureRead))
                    } else if character == "g" {
                        iced::Task::done(NodeGraphMessage::CreateNode(NodeData::Grade))
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

    pub fn handle_double_click_event(
        &mut self,
        context: &mut Context,
    ) -> iced::Task<NodeGraphMessage> {
        let Ok(mut node_graph) = context.node_graph.lock() else {
            return iced::Task::none();
        };

        if let Some(node_id) = self.hovered_node(&mut node_graph) {
            iced::Task::done(NodeGraphMessage::OpenProperties(node_id))
        } else {
            iced::Task::none()
        }
    }

    pub fn draw_selection_box(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        theme: &iced::theme::Theme,
    ) {
        if !self.selection_box.selecting {
            return;
        }

        let selection_box: iced::widget::canvas::Path = self
            .pan_zoom
            .local_to_canvas_rectangle(&self.selection_box.bounds())
            .to_path();

        frame.fill(
            &selection_box,
            theme
                .palette()
                .primary
                .scale_alpha(self.style.selection_box_alpha),
        );
        frame.stroke(
            &selection_box,
            iced::widget::canvas::Stroke::default()
                .with_width(self.style.selection_box_border_width)
                .with_color(theme.palette().primary),
        );
    }

    pub fn draw_node_inputs(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        node_graph: &NodeGraphContext,
        node_id: &NodeId,
        node_shape: &Rectangle,
        theme: &iced::theme::Theme,
    ) {
        let connectable_input_count: usize = node_graph.graph.scene_graph_input_count(node_id);
        if connectable_input_count == 0 {
            return;
        }

        let input_path_radius: f32 = self.style.node_input_radius * self.pan_zoom.reciprocal_zoom();

        let node_bottom_left: Vec2 = node_shape.bottom_left();

        let step_size: f32 = node_shape.size.x / (connectable_input_count as f32);
        let half_step_size: f32 = 0.5 * step_size;

        for index in 0..connectable_input_count {
            let input = iced::widget::canvas::Path::circle(
                iced::Point::new(
                    node_bottom_left.x + (index as f32) * step_size + half_step_size,
                    node_bottom_left.y,
                ),
                input_path_radius,
            );

            frame.fill(&input, theme.palette().primary);
        }
    }

    pub fn draw_node_outputs(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        node_graph: &NodeGraphContext,
        node_id: &NodeId,
        node_shape: &Rectangle,
        theme: &iced::theme::Theme,
    ) {
        let output_count: usize = node_graph.graph[*node_id].output_count();
        if output_count == 0 {
            return;
        }

        let output_path_radius: f32 =
            self.style.node_output_radius * self.pan_zoom.reciprocal_zoom();

        let node_top_left: Vec2 = node_shape.top_left();

        let step_size: f32 = node_shape.size.x / (output_count as f32);
        let half_step_size: f32 = 0.5 * step_size;

        for index in 0..output_count {
            let output = iced::widget::canvas::Path::circle(
                iced::Point::new(
                    node_top_left.x + (index as f32) * step_size + half_step_size,
                    node_top_left.y,
                ),
                output_path_radius,
            );

            frame.fill(&output, theme.palette().primary);
        }
    }

    pub fn draw_nodes(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        node_graph: &NodeGraphContext,
        theme: &iced::theme::Theme,
    ) {
        let text_size = iced::Pixels(self.style.text_size * self.pan_zoom.reciprocal_zoom());
        let render_text: bool = self.pan_zoom.zoom() <= self.style.text_zoom_cutoff;

        let node_selected_border_width =
            self.style.node_selected_border_width * self.pan_zoom.reciprocal_zoom();

        let primary_colour: iced::Color = theme.palette().primary;
        let text_colour: iced::Color = theme.palette().text;

        for (node_id, node_context) in node_graph.context.iter() {
            let node_shape: Rectangle =
                self.pan_zoom.local_to_canvas_rectangle(&node_context.shape);

            let node: iced::widget::canvas::Path = node_shape.to_path();
            frame.fill(&node, iced::Color::from_glam(&node_context.colour));
            if node_graph.selected_nodes.contains(&node_id) {
                frame.stroke(
                    &node,
                    iced::widget::canvas::Stroke::default()
                        .with_width(node_selected_border_width)
                        .with_color(primary_colour),
                );
            }

            if render_text {
                frame.fill_text(iced::widget::canvas::Text {
                    content: node_context.label.clone(),
                    position: iced::Point::from_glam(&node_shape.center),
                    color: text_colour,
                    size: text_size,
                    align_x: iced::advanced::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    ..iced::widget::canvas::Text::default()
                });
            }

            if node_graph.graph.node_id_is_valid(node_id) {
                self.draw_node_inputs(frame, node_graph, &node_id, &node_shape, &theme);
                self.draw_node_outputs(frame, node_graph, &node_id, &node_shape, &theme);
            }
        }
    }

    pub fn draw_viewport_nodes(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        node_graph: &NodeGraphContext,
        viewports: &Viewports,
        theme: &iced::theme::Theme,
    ) {
        let text_size = iced::Pixels(self.style.text_size * self.pan_zoom.reciprocal_zoom());
        let render_text: bool = self.pan_zoom.zoom() <= self.style.text_zoom_cutoff;

        let node_selected_border_width =
            self.style.node_selected_border_width * self.pan_zoom.reciprocal_zoom();

        let primary_colour: iced::Color = theme.palette().primary;
        let text_colour: iced::Color = theme.palette().text;

        for (viewport_id, viewport_context) in viewports.iter() {
            let viewport_shape: Rectangle = self
                .pan_zoom
                .local_to_canvas_rectangle(&viewport_context.node_context.shape);
            let node = viewport_shape.to_path();
            frame.fill(
                &node,
                iced::Color::from_glam(&viewport_context.node_context.colour),
            );
            if node_graph.selected_viewports.contains(&viewport_id) {
                frame.stroke(
                    &node,
                    iced::widget::canvas::Stroke::default()
                        .with_width(node_selected_border_width)
                        .with_color(primary_colour),
                );
            }
            if render_text {
                frame.fill_text(iced::widget::canvas::Text {
                    content: viewport_context.node_context.label.clone(),
                    position: iced::Point::from_glam(&viewport_shape.center),
                    color: text_colour,
                    size: text_size,
                    align_x: iced::advanced::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    ..iced::widget::canvas::Text::default()
                });
            }
        }
    }

    pub fn draw_viewport_connections(
        &self,
        frame: &mut iced::widget::canvas::Frame,
        node_graph: &NodeGraphContext,
        viewports: &Viewports,
        theme: &iced::theme::Theme,
    ) {
        let text_size = iced::Pixels(self.style.text_size * self.pan_zoom.reciprocal_zoom());
        let render_text: bool = self.pan_zoom.zoom() <= self.style.text_zoom_cutoff;

        let primary_colour: iced::Color = theme.palette().primary;
        let text_colour: iced::Color = theme.palette().text;

        let connection_line_width: f32 =
            self.style.connection_line_width * self.pan_zoom.reciprocal_zoom();
        let inactive_connection_line_width: f32 =
            connection_line_width * self.style.inactive_connection_line_scale;

        let dashed_line_segment_length = 2. * inactive_connection_line_width;
        let dashed_line_pattern: &[f32] = &[
            dashed_line_segment_length,
            dashed_line_segment_length,
            1e-12,
            dashed_line_segment_length,
        ];

        let connection_stroke = iced::widget::canvas::Stroke::default()
            .with_width(connection_line_width)
            .with_color(primary_colour);
        let mut inactive_view_connection_stroke = iced::widget::canvas::Stroke::default()
            .with_width(inactive_connection_line_width)
            .with_line_cap(iced::widget::canvas::LineCap::Round)
            .with_color(primary_colour);
        inactive_view_connection_stroke.line_dash = iced::widget::canvas::LineDash {
            segments: dashed_line_pattern,
            offset: 0,
        };

        for (_viewport_id, viewport_context) in viewports.iter() {
            for (view, output_id) in viewport_context.views.iter() {
                if !node_graph.graph.output_id_is_valid(*output_id) {
                    continue;
                }

                let node_id: NodeId = node_graph.graph[*output_id].node_id;
                let Some(node_context) = node_graph.context.get(node_id) else {
                    continue;
                };

                let node: &Node = &node_graph.graph[node_id];
                let Some(output_index) = node.output_index(output_id) else {
                    continue;
                };

                let node_bottom_left: Vec2 = node_context.shape.bottom_left();

                let output_count: usize = node.output_count();
                let step_size: f32 = node_context.shape.size.x / (output_count as f32);
                let half_step_size: f32 = 0.5 * step_size;

                let output_position = Vec2::new(
                    node_bottom_left.x + (output_index as f32) * step_size + half_step_size,
                    node_bottom_left.y,
                );
                let canvas_output_position: Vec2 = self.pan_zoom.local_to_canvas(output_position);

                let viewport_center: Vec2 = viewport_context.node_context.shape.center;
                let canvas_viewport_center: Vec2 = self.pan_zoom.local_to_canvas(viewport_center);

                let connection = iced::widget::canvas::Path::line(
                    iced::Point::from_glam(&canvas_output_position),
                    iced::Point::from_glam(&canvas_viewport_center),
                );

                if let Some(active_view) = viewport_context.active_view
                    && active_view == *view
                {
                    frame.stroke(&connection, connection_stroke);
                } else {
                    frame.stroke(&connection, inactive_view_connection_stroke);
                }

                if !render_text {
                    continue;
                }

                let connection_vector: Vec2 = output_position - viewport_center;
                let mut ray = Ray2d::new(viewport_center, connection_vector);
                ray.advance_in_direction(
                    self.style.text_size,
                    if ray.direction().y > 0. {
                        ray.perpendicular_direction_ccw()
                    } else {
                        ray.perpendicular_direction_cw()
                    },
                );

                let mut text_path: Rectangle = viewport_context.node_context.shape;
                text_path.uniform_expansion(
                    (connection_vector.length() - text_path.half_size().length())
                        .min(
                            1.5 * (if *view == 0 { 10. } else { *view as f32 })
                                * self.style.text_size,
                        )
                        .max(0.),
                );

                let Some(intersection_distances) = ray.intersect_rectangle(&text_path) else {
                    continue;
                };
                ray.advance(intersection_distances.y);

                let canvas_text_position: Vec2 = self.pan_zoom.local_to_canvas(*ray.origin());
                frame.fill_text(iced::widget::canvas::Text {
                    content: view.to_string(),
                    position: iced::Point::from_glam(&canvas_text_position),
                    color: text_colour,
                    size: text_size,
                    align_x: iced::advanced::text::Alignment::Right,
                    align_y: iced::alignment::Vertical::Center,
                    ..iced::widget::canvas::Text::default()
                });
            }
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
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<iced::Renderer>> {
        let mut frame = iced::widget::canvas::Frame::new(renderer, bounds.size());

        if let Ok(viewports) = self.viewport_context.lock()
            && let Ok(node_graph) = self.context.lock()
        {
            self.draw_viewport_connections(&mut frame, &node_graph, &viewports, theme);
        }

        if let Ok(node_graph) = self.context.lock() {
            self.draw_nodes(&mut frame, &node_graph, theme);

            if let Ok(viewports) = self.viewport_context.lock() {
                self.draw_viewport_nodes(&mut frame, &node_graph, &viewports, theme);
            }
        }

        self.draw_selection_box(&mut frame, theme);

        vec![frame.into_geometry()]
    }
}

impl Widget<NodeGraphMessage> for NodeGraph {
    fn update(
        &mut self,
        context: &mut Context,
        message: NodeGraphMessage,
    ) -> iced::Task<NodeGraphMessage> {
        // Initialize our pointers to the shared app data
        if !Arc::ptr_eq(&self.context, &context.node_graph) {
            self.context = Arc::clone(&context.node_graph);
            self.style = context.default_style.clone();
        }
        if !Arc::ptr_eq(&self.viewport_context, &context.viewports) {
            self.viewport_context = Arc::clone(&context.viewports);
        }

        match message {
            NodeGraphMessage::UpdateStyle => {
                self.style = context.default_style.clone();
                iced::Task::none()
            }
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
            NodeGraphMessage::DoubleClick => self.handle_double_click_event(context),
            NodeGraphMessage::ClearSelection => {
                let Ok(mut node_graph) = self.context.lock() else {
                    return iced::Task::none();
                };

                node_graph.selected_nodes.clear();

                iced::Task::none()
            }
            NodeGraphMessage::CreateNode(node_data) => {
                let Ok(mut node_graph) = self.context.lock() else {
                    return iced::Task::none();
                };

                let node_id: NodeId = node_graph.graph.add_node(node_data);
                match node_data {
                    NodeData::TextureRead => {
                        let Ok(_input_id) = node_graph.graph.set_input_data(
                            &node_id,
                            &TextureReadInputData::Filepath,
                            InputData::Filepath(
                                "/home/ob1/software/rust/damascus/damascus/image.exr".to_string(),
                            ),
                        ) else {
                            panic!("read could not set filepath.");
                        };
                    }
                    _ => {}
                }

                node_graph.context.insert(
                    node_id,
                    NodeContext::new(format!("{}{}", node_data, node_id))
                        .position(self.pan_zoom.local_cursor_position())
                        .colour(context.default_style.default_node_colour),
                );

                iced::Task::none()
            }
            NodeGraphMessage::CreateLiveNode => {
                let Ok(mut node_graph) = self.context.lock() else {
                    return iced::Task::none();
                };

                let primary_camera_axis_id: NodeId = node_graph.graph.add_node(NodeData::Axis);
                let secondary_camera_axis_id: NodeId = node_graph.graph.add_node(NodeData::Axis);
                let camera_id: NodeId = node_graph.graph.add_node(NodeData::Camera);

                let light_id: NodeId = node_graph.graph.add_node(NodeData::Light);

                let primitive_id: NodeId = node_graph.graph.add_node(NodeData::Primitive);
                let primitive_axis_id: NodeId = node_graph.graph.add_node(NodeData::Axis);
                let primitive_material_id: NodeId = node_graph.graph.add_node(NodeData::Material);

                let scene_id: NodeId = node_graph.graph.add_node(NodeData::Scene);

                let ray_marcher_id: NodeId = node_graph.graph.add_node(NodeData::RayMarcher);

                // Connect camera to scene
                // /ray_marcher/scene/camera/secondary_axis/primary_axis

                let secondary_camera_axis_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&secondary_camera_axis_id, &AxisInputData::Axis)
                    .unwrap();
                node_graph.graph.connect_node_to_input(
                    &primary_camera_axis_id,
                    &secondary_camera_axis_input_id,
                );

                let camera_axis_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&camera_id, &CameraInputData::Axis)
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&secondary_camera_axis_id, &camera_axis_input_id);

                let scene_render_camera_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&scene_id, &SceneInputData::RenderCamera)
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&camera_id, &scene_render_camera_input_id);

                // Connect light to scene
                // /ray_marcher/scene/camera/secondary_axis/primary_axis
                // |           |     /light

                let scene_light_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&scene_id, &SceneInputData::Scene)
                    .unwrap();

                node_graph
                    .graph
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

                let primitive_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&primitive_id, &PrimitiveInputData::Axis)
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&primitive_axis_id, &primitive_input_id);

                let primitive_material_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&primitive_id, &PrimitiveInputData::Material)
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&primitive_material_id, &primitive_material_input_id);

                let scene_primitive_input_id: InputId = node_graph
                    .graph
                    .node_input_id_from_str(&scene_id, "Scene1")
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&primitive_id, &scene_primitive_input_id);

                // Connect scene to ray marcher

                let ray_marcher_scene_input_id: InputId = node_graph
                    .graph
                    .node_input_id(&ray_marcher_id, &RayMarcherInputData::SceneRoot)
                    .unwrap();

                node_graph
                    .graph
                    .connect_node_to_input(&scene_id, &ray_marcher_scene_input_id);

                // Modify camera data

                let _ = node_graph.graph.set_input_data(
                    &camera_id,
                    &CameraInputData::SensorResolution,
                    InputData::UVec2(UVec2::new(2048u32, 1024u32)),
                );

                let _ = node_graph.graph.set_input_data(
                    &secondary_camera_axis_id,
                    &AxisInputData::Translate,
                    InputData::Vec3(Vec3::Z * 10.),
                );

                // Modify light data

                let _ = node_graph.graph.set_input_data(
                    &light_id,
                    &LightInputData::Colour,
                    InputData::Vec3(Vec3::new(1., 0.1, 0.1)),
                );

                // Modify primitive data

                let _ = node_graph.graph.set_input_data(
                    &primitive_material_id,
                    &MaterialInputData::DiffuseColour,
                    InputData::Vec3(Vec3::new(0.1, 0.1, 1.)),
                );

                let _ = node_graph.graph.set_input_data(
                    &primitive_id,
                    &PrimitiveInputData::Shape,
                    InputData::Enum(Shapes::Capsule.into()),
                );

                let _ = node_graph.graph.set_input_data(
                    &primitive_id,
                    &PrimitiveInputData::BlendStrength,
                    InputData::Float(0.5),
                );

                node_graph.context.insert(
                    ray_marcher_id,
                    NodeContext::new(format!("RayMarcher{}", ray_marcher_id))
                        .position(self.pan_zoom.local_cursor_position())
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
        .on_double_click(NodeGraphMessage::DoubleClick)
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
