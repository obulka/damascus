// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use super::style;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Dialog {
    Error(String, String),
    Info(String, String),
    Warning(String, String),
    Success(String, String),
}

pub fn modal<'a, Message: Clone>(
    preferences: &'a style::Preferences,
    base: impl Into<iced::Element<'a, Message>>,
    dialog: &'a Dialog,
    message: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    iced::widget::stack![
        base.into(),
        iced::widget::opaque(
            iced::widget::mouse_area(
                iced::widget::center(iced::widget::opaque(match dialog {
                    Dialog::Error(title, body) => {
                        success(preferences, title, body, message.clone())
                    }
                    Dialog::Info(title, body) => {
                        success(preferences, title, body, message.clone())
                    }
                    Dialog::Warning(title, body) => {
                        success(preferences, title, body, message.clone())
                    }
                    Dialog::Success(title, body) => {
                        success(preferences, title, body, message.clone())
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
            .on_press(message)
        )
    ]
    .into()
}

// pub fn error(modal: &Modal, title: &str, body: &str) {
//     modal
//         .dialog()
//         .with_title(title)
//         .with_body(body)
//         .with_icon(Icon::Error)
//         .open();
// }

// pub fn info(modal: &Modal, title: &str, body: &str) {
//     modal
//         .dialog()
//         .with_title(title)
//         .with_body(body)
//         .with_icon(Icon::Info)
//         .open();
// }

// pub fn warning(modal: &Modal, title: &str, body: &str) {
//     modal
//         .dialog()
//         .with_title(title)
//         .with_body(body)
//         .with_icon(Icon::Warning)
//         .open();
// }

pub fn success<'a, Message: Clone + 'a>(
    preferences: &'a style::Preferences,
    title: &'a str,
    body: &'a str,
    message: Message,
) -> iced::Element<'a, Message> {
    iced::widget::container(iced::widget::column![
        iced::widget::text(title),
        iced::widget::text(body),
        style::success_button(preferences, "Ok").on_press(message),
    ])
    .padding(preferences.padding)
    .style(iced::widget::container::rounded_box)
    .into()
}
