// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::sync::{Arc, Mutex};

use glam::{UVec2, Vec2};
use iced;
use indexmap::IndexSet;
use itertools::intersperse_with;
use slotmap::{SecondaryMap, SlotMap};

use damascus::{
    graph::node_graph::{
        inputs::{InputId, input::Input, input_data::InputData},
        nodes::NodeId,
    },
    naming_convensions::label_from_pascal,
};

use crate::app::Context;

use super::{Style, Widget, node_graph::NodeGraphContext};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum PropertiesMessage {
    OpenNodeProperties(NodeId),
    CloseNodeProperties(NodeId),
    InputDataChanged(InputId, InputData),
    #[default]
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Properties {
    #[serde(skip)]
    node_graph_context: Arc<Mutex<NodeGraphContext>>,
    open_properties: IndexSet<NodeId>,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            node_graph_context: Arc::default(),
            open_properties: IndexSet::<NodeId>::new(),
        }
    }
}

impl Properties {
    pub fn open(&mut self, node_id: NodeId) -> iced::Task<PropertiesMessage> {
        self.open_properties.shift_remove(&node_id);
        self.open_properties.insert(node_id);

        iced::Task::none()
    }

    pub fn close(&mut self, node_id: NodeId) -> iced::Task<PropertiesMessage> {
        self.open_properties.shift_remove(&node_id);

        iced::Task::none()
    }

    pub fn bool_widget<'a>(
        style: &'a Style,
        input_id: InputId,
        label: String,
        tooltip: String,
        value: bool,
    ) -> iced::Element<'a, PropertiesMessage> {
        style.checkbox(label, tooltip, value, move |value| {
            PropertiesMessage::InputDataChanged(input_id, InputData::Bool(value))
        })
    }

    pub fn filepath_widget<'a>(
        style: &'a Style,
        input_id: InputId,
        label: String,
        tooltip: String,
        value: &str,
    ) -> iced::Element<'a, PropertiesMessage> {
        style.filepath(label, tooltip, value, move |value| {
            PropertiesMessage::InputDataChanged(input_id, InputData::Filepath(value))
        })
    }

    pub fn float_widget<'a>(
        style: &'a Style,
        input_id: InputId,
        label: String,
        tooltip: String,
        value: f32,
    ) -> iced::Element<'a, PropertiesMessage> {
        let parameter_name_length = iced::Length::FillPortion(2);
        let horizontal_text_alignment = iced::alignment::Horizontal::Left;
        style.slider(
            label,
            0.0..=1.0,
            ..,
            value,
            style.float_input_step,
            move |value| -> PropertiesMessage {
                PropertiesMessage::InputDataChanged(input_id, InputData::Float(value))
            },
            PropertiesMessage::InputDataChanged(input_id, InputData::Float(value)),
            tooltip,
            parameter_name_length,
            horizontal_text_alignment,
        )
    }

    pub fn widget_for_input<'a>(
        style: &'a Style,
        node_graph: &NodeGraphContext,
        input_id: InputId,
        input: &Input,
    ) -> iced::Element<'a, PropertiesMessage> {
        let label: String = label_from_pascal(input.name.clone());
        let tooltip: String = input.input_help(&node_graph.graph);
        match input.data {
            InputData::Bool(value) => Self::bool_widget(style, input_id, label, tooltip, value),
            // InputData::BVec3(value) => {
            //     Self::bvec3_widget(style, label, &value)
            // }
            // InputData::Int(value) => {
            //     Self::int_widget(style, label, &value)
            // }
            // InputData::UInt(value) => {
            //     Self::uint_widget(style, label, &value)
            // }
            // InputData::UVec2(value) => {
            //     Self::uvec2_widget(style, label, &value)
            // }
            // InputData::UVec3(value) => {
            //     Self::uvec3_widget(style, label, &value)
            // }
            InputData::Float(value) => Self::float_widget(style, input_id, label, tooltip, value),
            // InputData::Vec2(value) => {
            //     Self::vec2_widget(style, label, &value)
            // }
            // InputData::Vec3(value) => {
            //     Self::vec3_widget(style, label, &value)
            // }
            // InputData::Vec4(value) => {
            //     Self::vec4_widget(style, label, &value)
            // }
            // InputData::Mat3(value) => {
            //     Self::mat3_widget(style, label, &value)
            // }
            // InputData::Mat4(value) => {
            //     Self::mat4_widget(style, label, &value)
            // }
            // InputData::Enum(value) => {
            //     Self::enum_widget(style, label, &value)
            // }
            InputData::Filepath(ref value) => {
                Self::filepath_widget(style, input_id, label, tooltip, value)
            }
            _ => style.text(label_from_pascal(input.name.clone())).into(),
        }
    }

    pub fn input_widgets_for_node<'a>(
        style: &'a Style,
        node_graph: &NodeGraphContext,
        node_id: &NodeId,
    ) -> impl Iterator<Item = iced::Element<'a, PropertiesMessage>> {
        node_graph
            .graph
            .non_scene_graph_inputs(node_id)
            .map(|(input_id, input)| Self::widget_for_input(style, node_graph, *input_id, input))
    }

    pub fn node_title_bar<'a>(
        style: &'a Style,
        node_id: NodeId,
        label: String,
    ) -> iced::Element<'a, PropertiesMessage> {
        iced::widget::row![
            style.text(label),
            Style::horizontal_spacer(),
            style
                .close_button()
                .style(iced::widget::button::background)
                .on_press(PropertiesMessage::CloseNodeProperties(node_id)),
        ]
        .spacing(style.spacing)
        .into()
    }

    pub fn widgets_for_node<'a>(
        &'a self,
        style: &'a Style,
        node_id: &NodeId,
    ) -> iced::Element<'a, PropertiesMessage> {
        let Ok(node_graph) = self.node_graph_context.lock() else {
            return Style::none();
        };

        if !node_graph.graph.node_id_is_valid(*node_id) {
            return Style::none();
        }

        let label: String = if let Some(node_data) = node_graph.context.get(*node_id) {
            node_data.label.clone()
        } else {
            node_id.to_string()
        };

        let node_is_selected: bool = node_graph.selected_nodes.contains(node_id);

        iced::widget::container(iced::widget::column(
            std::iter::once(Self::node_title_bar(style, *node_id, label))
                .chain(Self::input_widgets_for_node(style, &node_graph, node_id)),
        ))
        .padding(style.border_width + style.padding)
        .style(move |theme| iced::widget::container::Style {
            border: if node_is_selected {
                iced::Border::default()
                    .color(theme.palette().primary)
                    .width(style.border_width)
            } else {
                iced::Border::default()
            },
            ..iced::widget::container::Style::default()
        })
        .into()
    }
}

impl Widget<PropertiesMessage> for Properties {
    fn update(
        &mut self,
        context: &mut Context,
        message: PropertiesMessage,
    ) -> iced::Task<PropertiesMessage> {
        // Initialize our pointers to the shared app data
        if !Arc::ptr_eq(&self.node_graph_context, &context.node_graph) {
            self.node_graph_context = Arc::clone(&context.node_graph);
        }

        match message {
            PropertiesMessage::OpenNodeProperties(node_id) => self.open(node_id),
            PropertiesMessage::CloseNodeProperties(node_id) => self.close(node_id),
            PropertiesMessage::InputDataChanged(input_id, input_data) => {
                let Ok(mut node_graph) = self.node_graph_context.lock() else {
                    return iced::Task::none();
                };

                if !node_graph.graph.input_id_is_valid(input_id) {
                    return iced::Task::none();
                }

                node_graph.graph[input_id].data = input_data;

                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, PropertiesMessage> {
        iced::widget::scrollable(
            iced::widget::column(intersperse_with(
                self.open_properties.iter().rev().map(|node_id| {
                    iced::widget::column![self.widgets_for_node(style, node_id)].into()
                }),
                || -> iced::Element<'a, PropertiesMessage> {
                    iced::widget::rule::horizontal(style.border_width)
                        .style(|theme: &iced::Theme| iced::widget::rule::Style {
                            color: theme.extended_palette().background.strong.color,
                            radius: (0.5 * style.border_width).into(),
                            fill_mode: iced::widget::rule::FillMode::Full,
                            snap: false,
                        })
                        .into()
                },
            ))
            .padding(style.padding)
            .spacing(style.spacing),
        )
        .spacing(style.spacing)
        .into()
    }
}
