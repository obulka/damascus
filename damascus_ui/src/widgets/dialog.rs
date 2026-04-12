// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use super::style;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Dialog {
    Error(String),
    Info(String),
    Warning(String),
    Success(String),
}

pub fn modal<'a, Message: Clone>(
    preferences: &'a style::Preferences,
    base: impl Into<iced::Element<'a, Message>>,
    dialog: &'a Dialog,
    on_close: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    iced::widget::stack![
        base.into(),
        iced::widget::opaque(
            iced::widget::mouse_area(
                iced::widget::center(iced::widget::opaque(match dialog {
                    Dialog::Error(message) => {
                        error(preferences, message, on_close.clone())
                    }
                    Dialog::Info(message) => {
                        success(preferences, message, on_close.clone())
                    }
                    Dialog::Warning(message) => {
                        success(preferences, message, on_close.clone())
                    }
                    Dialog::Success(message) => {
                        success(preferences, message, on_close.clone())
                    }
                }))
                .style(|_theme| {
                    iced::widget::container::Style {
                        background: Some(
                            iced::Color {
                                a: 0.8,
                                ..iced::Color::BLACK
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

pub fn error<'a, Message: Clone + 'a>(
    preferences: &'a style::Preferences,
    message: &'a str,
    on_close: Message,
) -> iced::Element<'a, Message> {
    iced::widget::container(iced::widget::column![
        iced::widget::text("Error"),
        iced::widget::text(message),
        style::error_button(preferences, "Ok").on_press(on_close),
    ])
    .padding(preferences.padding)
    .style(iced::widget::container::rounded_box)
    .into()
}

pub fn info<'a, Message: Clone + 'a>(
    preferences: &'a style::Preferences,
    message: &'a str,
    on_close: Message,
) -> iced::Element<'a, Message> {
    iced::widget::container(iced::widget::column![
        iced::widget::text("Warning"),
        iced::widget::text(message),
        style::button(preferences, "Ok").on_press(on_close),
    ])
    .padding(preferences.padding)
    .style(iced::widget::container::rounded_box)
    .into()
}

pub fn warning<'a, Message: Clone + 'a>(
    preferences: &'a style::Preferences,
    message: &'a str,
    on_close: Message,
) -> iced::Element<'a, Message> {
    iced::widget::container(iced::widget::column![
        iced::widget::text("Warning"),
        iced::widget::text(message),
        style::warning_button(preferences, "Ok").on_press(on_close),
    ])
    .padding(preferences.padding)
    .style(iced::widget::container::rounded_box)
    .into()
}

pub fn success<'a, Message: Clone + 'a>(
    preferences: &'a style::Preferences,
    message: &'a str,
    on_close: Message,
) -> iced::Element<'a, Message> {
    iced::widget::container(iced::widget::column![
        iced::widget::text("Success"),
        iced::widget::text(message),
        style::success_button(preferences, "Ok").on_press(on_close),
    ])
    .padding(preferences.padding)
    .style(iced::widget::container::rounded_box)
    .into()
}
