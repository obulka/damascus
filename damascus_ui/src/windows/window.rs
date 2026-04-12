// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use crate::widgets::{
    Widget,
    dialog::Dialog,
    panel::{Panel, PanelMessage},
    style,
};

#[derive(Clone, Debug)]
pub enum WindowMessage {
    Event(iced::window::Id, iced::window::Event),
    Opened(iced::window::Id),
    Closed(iced::window::Id),
    ScaleChanged(Option<iced::window::Id>, f32),
    TitleChanged(iced::window::Id, String),
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
    pub preferences: style::Preferences,
    pub panel: Panel,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: format!("damascus"),
            preferences: style::Preferences::default(),
            panel: Panel::default(),
        }
    }
}

impl Widget<WindowMessage> for Window {
    fn view<'a>(
        &'a self,
        window_id: iced::window::Id,
        preferences: &'a style::Preferences,
    ) -> iced::Element<'a, WindowMessage> {
        // let title_input = column![
        //     text("Window title:"),
        //     text_input("Window Title", &self.title)
        //         .on_input(move |title| WindowMessage::TitleChanged(id, title).into())
        //         .id(format!("input-{id}"))
        // ];

        // let new_window_button = button(text("New Window")).on_press(WindowMessage::Open.into());

        // let content = column![
        //     scale_input,
        //     title_input,
        //     new_window_button,
        //     self.panel
        //         .view()
        //         .map(move |panel_message| WindowMessage::Panel(id, panel_message).into())
        // ]
        // .spacing(50)
        // .width(Fill)
        // .align_x(Center)
        // .width(200);

        self.panel
            .view(window_id, preferences)
            .map(move |panel_message| WindowMessage::Panel(Some(window_id), panel_message))
    }
}
