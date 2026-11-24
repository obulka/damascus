// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use damascus::Enumerator;

use crate::app::Context;

use super::{Style, Widget};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SearchMessage {
    None,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Search<E: Enumerator> {
    search_results: E,
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
            _ => {}
        }
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, SearchMessage> {
        iced::widget::column![
            // style.text_input("hello", "goodbye", |_| -> SearchMessage {
            //     SearchMessage::None
            // }),
            // style.text("test"),
        ]
        .spacing(style.spacing())
        .padding(style.padding())
        .width(iced::Length::FillPortion(1))
        .into()
    }
}
