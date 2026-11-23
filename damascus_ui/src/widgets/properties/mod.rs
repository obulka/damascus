// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt,
    ops::{Range, RangeInclusive},
    sync::{Arc, Mutex},
};

use iced;
use indexmap::IndexSet;
use itertools::intersperse_with;
use slotmap::SlotMap;

use damascus::{
    Enumerator,
    data_types::{ExpressionData, Float},
    graph::node_graph::{
        inputs::{InputId, input::Input, input_data::InputData},
        nodes::NodeId,
    },
    naming_convensions::label_from_pascal,
};

use crate::app::Context;

use super::{Style, Widget, node_graph::NodeGraphContext, panel::tabs::Tabs};

slotmap::new_key_type! { pub struct PropertiesId; }

impl fmt::Display for PropertiesId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PropertiesContext {
    pub open_properties: IndexSet<NodeId>,
}

impl Default for PropertiesContext {
    fn default() -> Self {
        Self {
            open_properties: IndexSet::<NodeId>::new(),
        }
    }
}

impl PropertiesContext {
    pub fn open(&mut self, node_id: NodeId) {
        self.open_properties.shift_remove(&node_id);
        self.open_properties.insert(node_id);
    }

    pub fn close(&mut self, node_id: NodeId) {
        self.open_properties.shift_remove(&node_id);
    }
}

pub type PropertiesContexts = SlotMap<PropertiesId, PropertiesContext>;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum PropertiesMessage {
    CloseNodeProperties(Option<PropertiesId>, NodeId),
    OpenNodeProperties(NodeId),
    InputDataChanged(InputId, InputData),
    #[default]
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Properties {
    #[serde(skip)]
    context: Arc<Mutex<PropertiesContexts>>,
    #[serde(skip)]
    node_graph_context: Arc<Mutex<NodeGraphContext>>,
    properties_id: Option<PropertiesId>,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            context: Arc::default(),
            node_graph_context: Arc::default(),
            properties_id: None,
        }
    }
}

impl Properties {
    pub fn id(&self) -> &Option<PropertiesId> {
        &self.properties_id
    }

    pub fn set_id(&mut self, id: PropertiesId) {
        self.properties_id = Some(id);
    }

    pub fn with_id(mut self, id: PropertiesId) -> Self {
        self.set_id(id);
        self
    }

    pub fn default_label() -> String {
        Tabs::Properties.variant_pascal_label()
    }

    pub fn label(&self) -> String {
        if let Some(properties_id) = self.properties_id {
            format!("{}{}", Self::default_label(), properties_id)
        } else {
            Self::default_label()
        }
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
        suggested_range: Option<RangeInclusive<f64>>,
        range: Option<Range<f64>>,
        value: &Float,
    ) -> iced::Element<'a, PropertiesMessage> {
        let suggested_range: RangeInclusive<f64> = suggested_range.unwrap_or(0.0..=1.0);

        let on_change = move |new_value| -> PropertiesMessage {
            PropertiesMessage::InputDataChanged(
                input_id,
                InputData::Float(Float::new(new_value as f32)),
            )
        };
        let current_value: f32 = value.value();
        let on_input = move |expression| -> PropertiesMessage {
            PropertiesMessage::InputDataChanged(
                input_id,
                InputData::Float(Float::new(current_value).with_expression(expression)),
            )
        };

        if let Some(range) = range {
            style.slider(
                label,
                suggested_range,
                range,
                current_value as f64,
                value.expression(),
                style.float_input_step() as f64,
                on_input,
                on_change,
                tooltip,
                1,
            )
        } else {
            style.slider(
                label,
                suggested_range,
                ..,
                current_value as f64,
                value.expression(),
                style.float_input_step() as f64,
                on_input,
                on_change,
                tooltip,
                1,
            )
        }
    }

    pub fn widget_for_input<'a>(
        style: &'a Style,
        node_graph: &NodeGraphContext,
        input_id: InputId,
        input: &Input,
    ) -> iced::Element<'a, PropertiesMessage> {
        let label: String = label_from_pascal(&input.name);
        let tooltip: String = input.input_help(&node_graph.graph);
        let suggested_range: Option<RangeInclusive<f64>> =
            input.input_suggested_range(&node_graph.graph);
        let range: Option<Range<f64>> = input.input_range(&node_graph.graph);
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
            InputData::Float(ref value) => Self::float_widget(
                style,
                input_id,
                label,
                tooltip,
                suggested_range,
                range,
                value,
            ),
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
            _ => style.text(label_from_pascal(&input.name)).into(),
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
        &'a self,
        style: &'a Style,
        node_id: NodeId,
        label: String,
    ) -> iced::Element<'a, PropertiesMessage> {
        iced::widget::container(iced::widget::row![
            style.text(label),
            Style::horizontal_spacer(),
            style.tooltip(
                style
                    .close_button()
                    .style(|theme, status| iced::widget::button::Style {
                        background: Some(iced::Background::Color(match status {
                            iced::widget::button::Status::Hovered =>
                                theme.extended_palette().background.weaker.color,
                            _ => theme.extended_palette().background.weakest.color,
                        })),
                        ..iced::widget::button::Style::default()
                    })
                    .on_press(PropertiesMessage::CloseNodeProperties(
                        self.properties_id,
                        node_id
                    )),
                "Close properties",
            ),
        ])
        .style(|theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(
                theme.extended_palette().background.weakest.color,
            )),
            ..iced::widget::container::Style::default()
        })
        .padding(style.padding())
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
            std::iter::once(self.node_title_bar(style, *node_id, label)).chain(std::iter::once(
                iced::widget::column(Self::input_widgets_for_node(style, &node_graph, node_id))
                    .padding(style.padding())
                    .spacing(style.spacing())
                    .into(),
            )),
        ))
        .padding(style.border_width())
        .style(move |theme| iced::widget::container::Style {
            border: if node_is_selected {
                iced::Border::default()
                    .color(theme.palette().primary)
                    .width(style.border_width())
            } else {
                iced::Border::default()
            },
            ..iced::widget::container::Style::default()
        })
        .into()
    }

    pub fn create_properties(&mut self, context: &mut Context) {
        if self.properties_id.is_none()
            && let Ok(mut properties) = context.properties.lock()
            && let Ok(mut node_graph) = context.node_graph.lock()
        {
            self.properties_id = Some(properties.insert(PropertiesContext::default()));
            node_graph.active_properties = self.properties_id;
        }
    }
}

impl Widget<PropertiesMessage> for Properties {
    fn on_create(&mut self, context: &mut Context) {
        // Initialize our pointers to the shared app data
        if !Arc::ptr_eq(&self.node_graph_context, &context.node_graph) {
            self.node_graph_context = Arc::clone(&context.node_graph);
        }
        if !Arc::ptr_eq(&self.context, &context.properties) {
            self.context = Arc::clone(&context.properties);
            self.create_properties(context);
        }
    }

    fn update(
        &mut self,
        context: &mut Context,
        message: PropertiesMessage,
    ) -> iced::Task<PropertiesMessage> {
        match message {
            PropertiesMessage::OpenNodeProperties(node_id) => {
                if let Ok(mut properties) = context.properties.lock()
                    && let Some(properties_id) = self.properties_id
                    && let Some(properties_context) = properties.get_mut(properties_id)
                {
                    properties_context.open(node_id);
                }

                iced::Task::none()
            }
            PropertiesMessage::CloseNodeProperties(properties_id, node_id) => {
                if properties_id == self.properties_id
                    && let Some(id) = properties_id
                    && let Ok(mut properties) = context.properties.lock()
                    && let Some(properties_context) = properties.get_mut(id)
                {
                    properties_context.close(node_id);
                }

                iced::Task::none()
            }
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
            if let Some(properties_id) = self.properties_id
                && let Ok(properties) = self.context.lock()
                && let Some(properties_context) = properties.get(properties_id)
            {
                iced::widget::column(intersperse_with(
                    properties_context
                        .open_properties
                        .iter()
                        .rev()
                        .map(|node_id| {
                            iced::widget::column![self.widgets_for_node(style, node_id)].into()
                        }),
                    || -> iced::Element<'a, PropertiesMessage> {
                        iced::widget::rule::horizontal(style.border_width())
                            .style(|theme: &iced::Theme| iced::widget::rule::Style {
                                color: theme.extended_palette().background.strong.color,
                                radius: (0.5 * style.border_width()).into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                                snap: false,
                            })
                            .into()
                    },
                ))
            } else {
                iced::widget::column![]
            }
            .padding(style.padding())
            .spacing(style.spacing()),
        )
        .spacing(style.spacing())
        .into()
    }
}
