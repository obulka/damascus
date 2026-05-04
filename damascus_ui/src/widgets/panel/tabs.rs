// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use macro_rules_attribute::derive;

use crate::{
    EnumTraits,
    widgets::{
        Widget,
        node_graph::{NodeGraph, NodeGraphMessage},
        viewport::{Viewport, ViewportMessage},
    },
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum TabMessage {
    NodeGraph(NodeGraphMessage),
    Viewport(ViewportMessage),
    Properties,
    TabSelected(usize),
    TabClosed(usize),
    New(Tabs),
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

impl Widget<TabMessage> for Tab {}
