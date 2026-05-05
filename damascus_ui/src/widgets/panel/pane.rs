// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    app::Context,
    widgets::{
        Widget,
        panel::tabs::{Tab, TabMessage, TabResult},
        style::Style,
    },
};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PaneMessage {
    Tab(TabMessage),
}

impl From<TabMessage> for PaneMessage {
    fn from(tab_message: TabMessage) -> Self {
        Self::Tab(tab_message)
    }
}

impl From<TabResult<TabMessage>> for PaneMessage {
    fn from(tab_result: TabResult<TabMessage>) -> Self {
        <TabResult<TabMessage> as Into<TabMessage>>::into(tab_result).into()
    }
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

impl Widget<PaneMessage> for Pane {
    fn update(&mut self, context: &mut Context, message: PaneMessage) -> iced::Task<PaneMessage> {
        match message {
            PaneMessage::Tab(tab_message) => {
                match tab_message {
                    TabMessage::Selected(tab_index) => {
                        self.active_tab = tab_index;
                    }
                    TabMessage::Closed(tab_index) => {
                        self.tabs.remove(tab_index);
                        self.active_tab = if self.tabs.is_empty() {
                            0
                        } else {
                            usize::max(0, usize::min(self.active_tab, self.tabs.len() - 1))
                        };
                    }
                    TabMessage::New(ref tab) => {
                        self.tabs.push(tab.clone().into());
                    }
                    _ => {}
                }
                if self.tabs.len() > self.active_tab {
                    self.tabs[self.active_tab]
                        .update(context, tab_message)
                        .map(|tab_message| tab_message.into())
                } else {
                    iced::Task::none()
                }
            }
        }
    }

    fn view<'a>(
        &'a self,
        window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, PaneMessage> {
        if self.tabs.len() > self.active_tab {
            self.tabs[self.active_tab]
                .view(window_id, style)
                .map(|tab_message| tab_message.into())
        } else {
            None::<iced::Element<'a, PaneMessage>>.into()
        }
    }
}
