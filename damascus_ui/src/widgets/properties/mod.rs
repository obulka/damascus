// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::sync::{Arc, Mutex};

use glam::{UVec2, Vec2};
use iced;
use indexmap::IndexSet;
use slotmap::{SecondaryMap, SlotMap};

use damascus::{
    graph::node_graph::nodes::{NodeId, node_data::NodeData},
    naming_convensions::label_from_pascal,
};

use crate::app::Context;

use super::{
    Style, Widget,
    node_graph::{NodeGraphContext, node::NodeContext},
    pan_zoom::{PanZoom, PanZoomMessage},
};

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum PropertiesMessage {
    OpenNodeProperties(NodeId),
    #[default]
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Properties {
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

    pub fn input_widgets_for_node<'a>(
        &'a self,
        style: &'a Style,
        node_graph: &NodeGraphContext,
        node_id: &NodeId,
    ) -> impl Iterator<Item = iced::Element<'a, PropertiesMessage>> {
        node_graph
            .graph
            .non_scene_graph_inputs(node_id)
            .map(|(_input_id, input)| style.text(label_from_pascal(input.name.clone())).into())
    }

    pub fn widgets_for_node<'a>(
        &'a self,
        style: &'a Style,
        node_id: &NodeId,
    ) -> iced::Element<'a, PropertiesMessage> {
        let Ok(node_graph) = self.node_graph_context.lock() else {
            return None::<iced::Element<'a, PropertiesMessage>>.into();
        };

        if !node_graph.graph.node_id_is_valid(*node_id) {
            return None::<iced::Element<'a, PropertiesMessage>>.into();
        }

        let label: String = if let Some(node_data) = node_graph.context.get(*node_id) {
            node_data.label.clone()
        } else {
            node_id.to_string()
        };

        iced::widget::column(
            std::iter::once(style.text(label).into()).chain(self.input_widgets_for_node(
                style,
                &node_graph,
                node_id,
            )),
        )
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
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, PropertiesMessage> {
        let toolbar = iced::widget::row![];
        iced::widget::container(iced::widget::column![
            toolbar,
            iced::widget::column(
                self.open_properties
                    .iter()
                    .rev()
                    .map(|node_id| self.widgets_for_node(style, &node_id)),
            )
        ])
        .padding(style.padding)
        .into()
    }
}
