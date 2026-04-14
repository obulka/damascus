// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;
use macro_rules_attribute::derive;

use damascus::Enumerator;

use crate::{
    EnumTraits,
    app::Context,
    icons::Icons,
    widgets::{
        Widget,
        style::{Style, theme::Theme},
    },
};

#[derive(Copy, Default, EnumTraits!)]
pub enum StyleEditorMessage {
    TextSize(f32),
    Scale(f32),
    #[default]
    IncrementScale,
    DecrementScale,
    BorderWidth(f32),
    Theme(Theme),
    Spacing(f32),
    Padding(f32),
    IconSize(u32),
    Leeway(f32),
    ModalOpacity(f32),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct StyleEditor {}

impl StyleEditor {
    pub fn update_style(
        &mut self,
        style: &mut Style,
        message: StyleEditorMessage,
    ) -> iced::Task<StyleEditorMessage> {
        match message {
            StyleEditorMessage::TextSize(text_size) => {
                style.text_size = text_size;
            }
            StyleEditorMessage::Scale(scale) => {
                style.scale = scale;
            }
            StyleEditorMessage::BorderWidth(border_width) => {
                style.border_width = border_width;
            }
            StyleEditorMessage::Theme(theme) => {
                style.theme = theme;
            }
            StyleEditorMessage::Spacing(spacing) => {
                style.spacing = spacing;
            }
            StyleEditorMessage::Padding(padding) => {
                style.padding = padding;
            }
            StyleEditorMessage::IconSize(icon_size) => {
                style.icon_size = icon_size;
            }
            StyleEditorMessage::Leeway(leeway) => {
                style.leeway = leeway;
            }
            StyleEditorMessage::ModalOpacity(modal_opacity) => {
                style.modal_opacity = modal_opacity;
            }
            StyleEditorMessage::IncrementScale => {
                style.scale *= 1.05;
            }
            StyleEditorMessage::DecrementScale => {
                style.scale *= 0.95;
            }
        }

        iced::Task::none()
    }
}

impl Widget<StyleEditorMessage> for StyleEditor {
    fn update(
        &mut self,
        context: &mut Context,
        message: StyleEditorMessage,
    ) -> iced::Task<StyleEditorMessage> {
        self.update_style(&mut context.default_style, message)
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, StyleEditorMessage> {
        let text_size = style.slider(
            StyleEditorMessage::TextSize(0.).as_ref(),
            3.0..=36.0,
            style.text_size,
            0.25,
            StyleEditorMessage::TextSize,
        );
        let scale = style.slider(
            StyleEditorMessage::Scale(0.).as_ref(),
            0.1..=10.0,
            style.scale,
            0.1,
            StyleEditorMessage::Scale,
        );
        let border_width = style.slider(
            StyleEditorMessage::BorderWidth(0.).as_ref(),
            0.0..=10.0,
            style.border_width,
            0.1,
            StyleEditorMessage::BorderWidth,
        );
        // let theme = style.slider(1.0..=100.0, style.theme, StyleEditorMessage::Theme);
        let spacing = style.slider(
            StyleEditorMessage::Spacing(0.).as_ref(),
            0.0..=10.0,
            style.spacing,
            0.1,
            StyleEditorMessage::Spacing,
        );
        let padding = style.slider(
            StyleEditorMessage::Padding(0.).as_ref(),
            0.0..=10.0,
            style.padding,
            0.1,
            StyleEditorMessage::Padding,
        );
        let icon_size = style.slider(
            StyleEditorMessage::IconSize(0).as_ref(),
            4..=36,
            style.icon_size,
            1,
            StyleEditorMessage::IconSize,
        );
        let leeway = style.slider(
            StyleEditorMessage::Leeway(0.).as_ref(),
            1.0..=100.0,
            style.leeway,
            1.,
            StyleEditorMessage::Leeway,
        );
        let modal_opacity = style.slider(
            StyleEditorMessage::ModalOpacity(0.).as_ref(),
            0.0..=1.0,
            style.modal_opacity,
            0.1,
            StyleEditorMessage::ModalOpacity,
        );

        iced::widget::container(
            iced::widget::column![
                text_size,
                scale,
                border_width,
                spacing,
                padding,
                icon_size,
                leeway,
                modal_opacity,
            ]
            .spacing(style.spacing),
        )
        .width(250)
        .into()
    }
}
