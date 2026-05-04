// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crate::widgets::{
    Widget,
    panel::tabs::{Tab, TabMessage},
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PaneMessage {
    Tab(TabMessage),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Pane {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

impl Default for Pane {
    fn default() -> Self {
        Self {
            active_tab: 0,
            tabs: vec![],
        }
    }
}

impl Widget<PaneMessage> for Pane {}
