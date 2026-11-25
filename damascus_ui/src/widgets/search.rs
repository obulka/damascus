// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::marker::PhantomData;

use iced;

use damascus::Enumerator;

use crate::app::Context;

use super::{Style, Widget};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SearchMessage<E: Enumerator> {
    UpdateSearch(String),
    VariantSelected(E),
    SubmitSearch,
    ClearSearch,
    None,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Search<E: Enumerator> {
    search: String,
    #[serde(skip)]
    text_input_id: iced::widget::Id,
    _search_results: PhantomData<E>,
}

impl<E: Enumerator> Default for Search<E> {
    fn default() -> Self {
        Self {
            search: String::new(),
            text_input_id: iced::widget::Id::unique(),
            _search_results: PhantomData::<E>::default(),
        }
    }
}

impl<E: Enumerator> Search<E> {
    pub fn id(&self) -> &iced::widget::Id {
        &self.text_input_id
    }

    pub fn search_results(&self) -> impl Iterator<Item = E> {
        E::iter().filter(|variant| {
            let variant_label: String = variant.to_string();
            variant_label
                .to_lowercase()
                .starts_with(&self.search.to_lowercase())
        })
    }
}

impl<E> Widget<SearchMessage<E>> for Search<E>
where
    E: Enumerator
        + for<'de> serde::Deserialize<'de>
        + serde::Serialize
        + std::marker::Send
        + 'static,
{
    fn update(
        &mut self,
        _context: &mut Context,
        message: SearchMessage<E>,
    ) -> iced::Task<SearchMessage<E>> {
        match message {
            SearchMessage::UpdateSearch(new_search) => {
                self.search = new_search;
                iced::Task::none()
            }
            SearchMessage::SubmitSearch => {
                if let Some(search_result) = self.search_results().next() {
                    iced::Task::batch(
                        std::iter::once(SearchMessage::VariantSelected(search_result))
                            .chain(std::iter::once(SearchMessage::ClearSearch))
                            .map(|message| iced::Task::done(message)),
                    )
                } else {
                    iced::Task::none()
                }
            }
            SearchMessage::ClearSearch => {
                self.search.clear();
                iced::Task::none()
            }
            _ => iced::Task::none(),
        }
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, SearchMessage<E>> {
        iced::widget::column![
            style
                .text_input(
                    "Search for a node",
                    &self.search,
                    |new_search| -> SearchMessage<E> { SearchMessage::UpdateSearch(new_search) },
                )
                .id(self.text_input_id.clone()),
            iced::widget::scrollable(
                iced::widget::column(self.search_results().map(|variant| {
                    style
                        .button(variant.to_string())
                        .style(iced::widget::button::text)
                        .on_press(SearchMessage::VariantSelected(variant))
                        .into()
                }))
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(style.spacing())
            ),
        ]
        .align_x(iced::alignment::Horizontal::Center)
        .spacing(style.spacing())
        .padding(style.padding())
        .into()
    }
}
