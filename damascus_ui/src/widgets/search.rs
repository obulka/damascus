// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use damascus::Enumerator;

use crate::app::Context;

use super::{Style, Widget, modal};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SearchMessage {
    Close,
    Open,
    None,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Search<E: Enumerator> {
    search_results: E,
    visible: bool,
}

impl<E: Enumerator> Search<E> {
    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl<E> Widget<SearchMessage> for Search<E>
where
    E: Enumerator + for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    fn update(
        &mut self,
        _context: &mut Context,
        message: SearchMessage,
    ) -> iced::Task<SearchMessage> {
        match message {
            SearchMessage::Open => {
                self.visible = true;
            }
            SearchMessage::Close => {
                self.visible = false;
            }
            _ => {}
        }
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, SearchMessage> {
        if self.visible {
            iced::widget::container(modal(
                style,
                None::<iced::Element<'a, SearchMessage>>,
                iced::widget::column![
                    style.text_input("hello", "goodbye", |_| -> SearchMessage {
                        SearchMessage::None
                    }),
                    style.text("test"),
                ]
                .spacing(style.spacing())
                .padding(style.padding()),
                SearchMessage::Close,
            ))
            .padding(style.border_width())
            .into()
        } else {
            None::<iced::Element<'a, SearchMessage>>.into()
        }
    }
}
