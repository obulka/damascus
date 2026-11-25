// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::marker::PhantomData;

use iced;

use damascus::Enumerator;

use crate::app::Context;

use super::{Style, Widget, modal};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum SearchMessage<E: Enumerator> {
    Close,
    Open,
    UpdateSearch(String),
    VariantSelected(E),
    None,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Search<E: Enumerator> {
    search: String,
    visible: bool,
    _search_results: PhantomData<E>,
}

impl<E: Enumerator> Search<E> {
    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl<E> Widget<SearchMessage<E>> for Search<E>
where
    E: Enumerator + for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    fn update(
        &mut self,
        _context: &mut Context,
        message: SearchMessage<E>,
    ) -> iced::Task<SearchMessage<E>> {
        match message {
            SearchMessage::Open => {
                self.visible = true;
            }
            SearchMessage::Close => {
                self.visible = false;
            }
            SearchMessage::UpdateSearch(new_search) => {
                self.search = new_search;
            }
            _ => {}
        }
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, SearchMessage<E>> {
        if self.visible {
            iced::widget::container(modal(
                style,
                None::<iced::Element<'a, SearchMessage<E>>>,
                iced::widget::column![
                    style.text_input(
                        "Search for a node",
                        &self.search,
                        |new_search| -> SearchMessage<E> {
                            SearchMessage::UpdateSearch(new_search)
                        },
                    ),
                    iced::widget::scrollable(
                        iced::widget::column(E::iter().filter_map(|variant| {
                            let variant_label: String = variant.to_string();
                            if variant_label
                                .to_lowercase()
                                .starts_with(&self.search.to_lowercase())
                            {
                                Some(
                                    style
                                        .button(variant.to_string())
                                        .style(iced::widget::button::text)
                                        .on_press(SearchMessage::VariantSelected(variant))
                                        .into(),
                                )
                            } else {
                                None
                            }
                        }))
                        .align_x(iced::alignment::Horizontal::Center)
                        .spacing(style.spacing())
                    ),
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(style.spacing())
                .padding(style.padding()),
                SearchMessage::Close,
            ))
            .padding(style.border_width())
            .into()
        } else {
            None::<iced::Element<'a, SearchMessage<E>>>.into()
        }
    }
}
