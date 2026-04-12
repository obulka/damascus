// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use crate::widgets::{
    Widget,
    panel::{Panel, PanelMessage},
    style::Style,
};

#[derive(Clone, Debug)]
pub enum WindowMessage {
    Event(iced::window::Id, iced::window::Event),
    Opened(iced::window::Id),
    Closed(iced::window::Id),
    ScaleChanged(Option<iced::window::Id>, f32),
    UpdateTitle,
    Panel(Option<iced::window::Id>, PanelMessage),
}

impl From<PanelMessage> for WindowMessage {
    fn from(panel_message: PanelMessage) -> Self {
        Self::Panel(None, panel_message)
    }
}

impl WindowMessage {
    pub fn from_panel_message_with_id(id: iced::window::Id, panel_message: PanelMessage) -> Self {
        Self::Panel(Some(id), panel_message)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Window {
    pub title: String,
    pub style: Style,
    pub panel: Panel,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: Self::default_title().to_string(),
            style: Style::default(),
            panel: Panel::default(),
        }
    }
}

impl Widget<WindowMessage> for Window {
    fn view<'a>(
        &'a self,
        window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, WindowMessage> {
        self.panel
            .view(window_id, style)
            .map(move |panel_message| WindowMessage::Panel(Some(window_id), panel_message))
    }
}

impl Window {
    pub fn default_title() -> &'static str {
        "damascus"
    }
}
