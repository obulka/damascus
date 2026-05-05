// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{fmt, str::FromStr};

use macro_rules_attribute::derive;

use damascus::Enumerator;

use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
    widgets::{
        Style, Widget,
        node_graph::{NodeGraph, NodeGraphMessage},
        viewport::{Viewport, ViewportMessage},
    },
};

#[derive(Default, ErrorTraits!)]
pub enum TabErrors {
    DeserializeError(String),
    #[default]
    UnknownError,
}

impl fmt::Display for TabErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeserializeError(error) => write!(
                formatter,
                "{}: Could not deserialize from: {}",
                self.variant(),
                error
            ),
            _ => write!(formatter, "{}: Sounds like a you problem.", self.variant()),
        }
    }
}

pub type TabResult<T> = Result<T, TabErrors>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TabMessage {
    NodeGraph(NodeGraphMessage),
    Viewport(ViewportMessage),
    Properties,
    Selected(usize),
    Closed(usize),
    New(Tabs),
    Error(TabErrors),
}

impl FromStr for TabMessage {
    type Err = TabErrors;

    fn from_str(option: &str) -> TabResult<Self> {
        let variant: String = option.chars().filter(|c| !c.is_whitespace()).collect();

        if let Ok(option) = Tabs::from_str(&variant) {
            Ok(Self::New(option))
        } else {
            Err(Self::Err::DeserializeError(variant))
        }
    }
}

impl From<TabResult<Self>> for TabMessage {
    fn from(result: TabResult<Self>) -> Self {
        match result {
            Ok(message) => message,
            Err(error) => TabMessage::Error(error),
        }
    }
}

impl From<NodeGraphMessage> for TabMessage {
    fn from(node_graph_message: NodeGraphMessage) -> Self {
        Self::NodeGraph(node_graph_message)
    }
}

impl From<ViewportMessage> for TabMessage {
    fn from(viewport_message: ViewportMessage) -> Self {
        Self::Viewport(viewport_message)
    }
}

#[derive(Default, EnumTraits!)]
pub enum Tabs {
    #[default]
    NodeGraph,
    Viewport,
    Properties,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub enum Tab {
    NodeGraph(NodeGraph),
    Viewport(Viewport),
    Properties,
    #[default]
    None,
}

impl Tab {
    pub fn as_tabs(&self) -> Tabs {
        match self {
            Self::NodeGraph(_) => Tabs::NodeGraph,
            Self::Viewport(_) => Tabs::Viewport,
            Self::Properties => Tabs::Properties,
            _ => Tabs::default(),
        }
    }
}

impl From<Tabs> for Tab {
    fn from(tab: Tabs) -> Self {
        match tab {
            Tabs::NodeGraph => Self::NodeGraph(NodeGraph::default()),
            Tabs::Viewport => Self::Viewport(Viewport::default()),
            Tabs::Properties => Self::Properties,
        }
    }
}

impl From<Tab> for Tabs {
    fn from(tab: Tab) -> Self {
        match tab {
            Tab::NodeGraph(_) => Self::NodeGraph,
            Tab::Viewport(_) => Self::Viewport,
            Tab::Properties => Self::Properties,
            _ => Self::default(),
        }
    }
}

impl Widget<TabMessage> for Tab {
    fn update(&mut self, context: &mut Context, message: TabMessage) -> iced::Task<TabMessage> {
        match self {
            Self::NodeGraph(node_graph) => match message {
                TabMessage::NodeGraph(node_graph_message) => node_graph
                    .update(context, node_graph_message)
                    .map(|node_graph_message| node_graph_message.into()),
                _ => iced::Task::none(),
            },
            Self::Viewport(viewport) => match message {
                TabMessage::Viewport(viewport_message) => viewport
                    .update(context, viewport_message)
                    .map(|viewport_message| viewport_message.into()),
                _ => iced::Task::none(),
            },
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        _style: &'a Style,
    ) -> iced::Element<'a, TabMessage> {
        // TODO
        None::<iced::Element<'a, TabMessage>>.into()
    }
}
