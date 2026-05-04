// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::fmt::Debug;

use iced;

use crate::app::Context;

pub mod dialog;
pub mod menu;
pub mod node_graph;
pub mod panel;
pub mod style;
pub mod toolbar;
pub mod viewport;

use style::Style;

pub trait Widget<WidgetMessage: Clone>:
    Clone + Debug + Default + for<'de> serde::Deserialize<'de> + serde::Serialize
{
    fn new() -> Self {
        Self::default()
    }

    fn update(
        &mut self,
        _context: &mut Context,
        _message: WidgetMessage,
    ) -> iced::Task<WidgetMessage> {
        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        _style: &'a Style,
    ) -> iced::Element<'a, WidgetMessage> {
        None::<iced::Element<'a, WidgetMessage>>.into()
    }
}

pub fn modal<'a, Message: Clone>(
    style: &'a Style,
    base_widget: impl Into<iced::Element<'a, Message>>,
    modal_widget: impl Into<iced::Element<'a, Message>>,
    on_close: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    iced::widget::stack![
        base_widget.into(),
        iced::widget::opaque(
            iced::widget::mouse_area(
                iced::widget::center(iced::widget::row![
                        <iced::widget::Container<'_, Message> as Into<
                            iced::Element<'a, Message>,
                        >>::into(
                            iced::widget::container(Style::none())
                                .width(iced::Length::FillPortion(1))
                        ),
                        <iced::widget::Container<'_, Message> as Into<
                            iced::Element<'a, Message>,
                        >>::into(
                            iced::widget::container(iced::widget::opaque(modal_widget))
                                .width(iced::Length::FillPortion(1))
                        ),
                        <iced::widget::Container<'_, Message> as Into<
                            iced::Element<'a, Message>,
                        >>::into(
                            iced::widget::container(Style::none())
                                .width(iced::Length::FillPortion(1))
                        ),
                    ])
                .style(|theme: &iced::Theme| {
                    let mut background_colour: iced::Color = theme.palette().background;
                    background_colour.a = style.modal_opacity;
                    iced::widget::container::Style {
                        background: Some(background_colour.into()),
                        ..iced::widget::container::Style::default()
                    }
                }),
            )
            .on_press(on_close),
        )
    ]
    .into()
}
