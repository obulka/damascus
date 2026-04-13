// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use super::style::Style;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Dialog {
    Error(String),
    Info(String),
    Warning(String),
    Success(String),
}

impl Dialog {
    pub fn modal<'a, Message: Clone>(
        &'a self,
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
                    iced::widget::center(iced::widget::opaque(modal_widget)).style(|_theme| {
                        iced::widget::container::Style {
                            background: Some(
                                iced::Color {
                                    r: style.modal_background_colour.x,
                                    g: style.modal_background_colour.y,
                                    b: style.modal_background_colour.z,
                                    a: style.modal_background_colour.w,
                                }
                                .into(),
                            ),
                            ..iced::widget::container::Style::default()
                        }
                    })
                )
                .on_press(on_close)
            )
        ]
        .into()
    }

    pub fn inform_user<'a, Message: Clone>(
        &'a self,
        style: &'a Style,
        base: impl Into<iced::Element<'a, Message>>,
        on_close: Message,
    ) -> iced::Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        self.modal(
            style,
            base,
            match self {
                Dialog::Error(message) => Self::error(style, message, on_close.clone()),
                Dialog::Info(message) => Self::success(style, message, on_close.clone()),
                Dialog::Warning(message) => Self::success(style, message, on_close.clone()),
                Dialog::Success(message) => Self::success(style, message, on_close.clone()),
            },
            on_close,
        )
    }

    pub fn error<'a, Message: Clone + 'a>(
        style: &'a Style,
        message: &'a str,
        on_close: Message,
    ) -> iced::Element<'a, Message> {
        iced::widget::container(
            iced::widget::column![
                style.heading("Error"),
                style.text(message),
                style.error_button("Ok").on_press(on_close),
            ]
            .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(style.padding)
        .style(iced::widget::container::rounded_box)
        .into()
    }

    pub fn info<'a, Message: Clone + 'a>(
        style: &'a Style,
        message: &'a str,
        on_close: Message,
    ) -> iced::Element<'a, Message> {
        iced::widget::container(
            iced::widget::column![
                style.heading("Warning"),
                style.text(message),
                style.button("Ok").on_press(on_close),
            ]
            .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(style.padding)
        .style(iced::widget::container::rounded_box)
        .into()
    }

    pub fn warning<'a, Message: Clone + 'a>(
        style: &'a Style,
        message: &'a str,
        on_close: Message,
    ) -> iced::Element<'a, Message> {
        iced::widget::container(
            iced::widget::column![
                style.heading("Warning"),
                style.text(message),
                style.warning_button("Ok").on_press(on_close),
            ]
            .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(style.padding)
        .style(iced::widget::container::rounded_box)
        .into()
    }

    pub fn success<'a, Message: Clone + 'a>(
        style: &'a Style,
        message: &'a str,
        on_close: Message,
    ) -> iced::Element<'a, Message> {
        iced::widget::container(
            iced::widget::column![
                style.heading("Success"),
                style.text(message),
                style.success_button("Ok").on_press(on_close),
            ]
            .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(style.padding)
        .style(iced::widget::container::rounded_box)
        .into()
    }
}
