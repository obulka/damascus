// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use crate::{
    style,
    widgets::panel::{Panel, PanelMessage},
};

#[derive(Debug, Clone)]
pub enum WindowMessage {
    Event(iced::window::Id, iced::window::Event),
    Opened(iced::window::Id),
    Closed(iced::window::Id),
    ScaleChanged(Option<iced::window::Id>, f32),
    TitleChanged(iced::window::Id, String),
    Panel(Option<iced::window::Id>, PanelMessage),
}

pub struct Window {
    pub title: String,
    pub preferences: style::Preferences,
    pub panel: Panel,
}

impl Window {
    pub fn new(count: usize) -> Self {
        Self {
            title: format!("damascus-{count}"),
            preferences: style::Preferences::default(),
            panel: Panel::default(),
        }
    }

    pub fn view(&self, id: iced::window::Id) -> iced::Element<'_, WindowMessage> {
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
            .view(&self.preferences)
            .map(move |panel_message| WindowMessage::Panel(Some(id), panel_message))
    }
}
