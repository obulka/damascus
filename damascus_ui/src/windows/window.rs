// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam;
use iced;

use crate::{
    app::Context,
    widgets::{
        Widget, modal,
        panel::{Panel, PanelMessage},
        style::{
            Style,
            editor::{StyleEditor, StyleEditorMessage},
        },
    },
};

#[derive(Clone, Debug)]
pub enum WindowMessage {
    Event(iced::window::Id, iced::window::Event),
    Open(Option<glam::Vec2>),
    Opened(iced::window::Id),
    Close(iced::window::Id),
    Closed(iced::window::Id),
    UpdateTitle,
    Panel(Option<iced::window::Id>, PanelMessage),
    StyleEditor(Option<iced::window::Id>, StyleEditorMessage),
    CloseStyleEditor(Option<iced::window::Id>),
}

impl From<StyleEditorMessage> for WindowMessage {
    fn from(style_editor_message: StyleEditorMessage) -> Self {
        Self::StyleEditor(None, style_editor_message)
    }
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
    pub position: Option<glam::Vec2>,
    pub title: String,
    pub style: Style,
    pub panel: Panel,
    pub style_editor: StyleEditor,
    pub style_editor_visible: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            position: None,
            title: Self::default_title().to_string(),
            style: Style::default(),
            panel: Panel::default(),
            style_editor: StyleEditor::default(),
            style_editor_visible: false,
        }
    }
}

impl Widget<WindowMessage> for Window {
    fn update(
        &mut self,
        _context: &mut Context,
        message: WindowMessage,
    ) -> iced::Task<WindowMessage> {
        match message {
            WindowMessage::StyleEditor(_id, style_editor_message) => self
                .style_editor
                .update_style(&mut self.style, style_editor_message)
                .map(|style_editor_message| style_editor_message.into()),
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, WindowMessage> {
        let main_contents: iced::Element<'_, WindowMessage> = self
            .panel
            .view(window_id, style)
            .map(move |panel_message| WindowMessage::Panel(Some(window_id), panel_message));

        if self.style_editor_visible {
            modal(
                style,
                main_contents,
                self.style_editor
                    .view(window_id, style)
                    .map(|style_editor_message| style_editor_message.into()),
                WindowMessage::CloseStyleEditor(Some(window_id)),
            )
        } else {
            main_contents
        }
    }
}

impl Window {
    pub fn default_title() -> &'static str {
        "damascus"
    }

    pub fn open_style_editor(&mut self) {
        self.style_editor_visible = true;
    }

    pub fn close_style_editor(&mut self) {
        self.style_editor_visible = false;
    }
}
