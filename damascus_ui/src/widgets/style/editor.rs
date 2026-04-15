// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::str::FromStr;

use iced;
use macro_rules_attribute::derive;
use strum::{Display, IntoEnumIterator};

use damascus::Enumerator;

use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
    icons::Icons,
    widgets::{
        Widget,
        style::{Style, theme::Theme},
    },
};

#[derive(Default, Display, ErrorTraits!)]
pub enum StyleEditorErrors {
    DeserializeError(String),
    #[default]
    UnknownError,
}

pub type StyleEditorResult<E> = Result<E, StyleEditorErrors>;

#[derive(Copy, Default, EnumTraits!)]
pub enum StyleEditorFields {
    #[default]
    TextSize,
    Scale,
    BorderWidth,
    Theme,
    Spacing,
    Padding,
    IconSize,
    Leeway,
    ModalOpacity,
    FloatInputStep,
}

impl StyleEditorFields {
    fn increment(&self, style: &mut Style) {
        match self {
            Self::TextSize => style.text_size += style.float_input_step,
            Self::Scale => style.scale *= 1.05,
            Self::BorderWidth => style.border_width += style.float_input_step,
            Self::Theme => {}
            Self::Spacing => style.spacing += style.float_input_step,
            Self::Padding => style.padding += style.float_input_step,
            Self::IconSize => style.icon_size += 1,
            Self::Leeway => style.leeway += style.float_input_step,
            Self::ModalOpacity => style.modal_opacity += style.float_input_step,
            Self::FloatInputStep => style.float_input_step += style.float_input_step,
        }
    }

    fn decrement(&self, style: &mut Style) {
        match self {
            Self::TextSize => style.text_size -= style.float_input_step,
            Self::Scale => style.scale *= 0.95,
            Self::BorderWidth => style.border_width -= style.float_input_step,
            Self::Theme => {}
            Self::Spacing => style.spacing -= style.float_input_step,
            Self::Padding => style.padding -= style.float_input_step,
            Self::IconSize => style.icon_size -= 1,
            Self::Leeway => style.leeway -= style.float_input_step,
            Self::ModalOpacity => style.modal_opacity -= style.float_input_step,
            Self::FloatInputStep => style.float_input_step -= style.float_input_step,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StyleEditorData {
    TextSize(f32),
    Scale(f32),
    BorderWidth(f32),
    Theme(Theme),
    Spacing(f32),
    Padding(f32),
    IconSize(u32),
    Leeway(f32),
    ModalOpacity(f32),
    FloatInputStep(f32),
}

impl StyleEditorData {
    fn set(&self, style: &mut Style) {
        match self {
            Self::TextSize(text_size) => style.text_size = *text_size,
            Self::Scale(scale) => style.scale = scale.max(0.01),
            Self::BorderWidth(border_width) => style.border_width = *border_width,
            Self::Theme(theme) => style.theme = *theme,
            Self::Spacing(spacing) => style.spacing = *spacing,
            Self::Padding(padding) => style.padding = *padding,
            Self::IconSize(icon_size) => style.icon_size = *icon_size,
            Self::Leeway(leeway) => style.leeway = *leeway,
            Self::ModalOpacity(modal_opacity) => style.modal_opacity = *modal_opacity,
            Self::FloatInputStep(float_input_step) => style.float_input_step = *float_input_step,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StyleEditorMessage {
    Increment(StyleEditorFields),
    Decrement(StyleEditorFields),
    Set(StyleEditorData),
    Error(StyleEditorErrors),
}

impl From<StyleEditorErrors> for StyleEditorMessage {
    fn from(style_editor_error: StyleEditorErrors) -> Self {
        Self::Error(style_editor_error)
    }
}

impl From<StyleEditorData> for StyleEditorMessage {
    fn from(style_editor_data: StyleEditorData) -> Self {
        Self::Set(style_editor_data)
    }
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
            StyleEditorMessage::Set(field) => {
                field.set(style);
            }
            StyleEditorMessage::Increment(field) => {
                field.increment(style);
            }
            StyleEditorMessage::Decrement(field) => {
                field.decrement(style);
            }
            _ => {}
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
            StyleEditorFields::TextSize.as_ref(),
            6.0..=36.0,
            ..,
            style.text_size,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::TextSize(value).into() },
            StyleEditorData::TextSize(style.text_size).into(),
        );
        let scale = style.slider(
            StyleEditorFields::Scale.as_ref(),
            0.25..=5.0,
            style.float_input_step..,
            style.scale,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Scale(value).into() },
            StyleEditorData::Scale(style.scale).into(),
        );
        let border_width = style.slider(
            StyleEditorFields::BorderWidth.as_ref(),
            0.0..=10.0,
            ..,
            style.border_width,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::BorderWidth(value).into() },
            StyleEditorData::BorderWidth(style.border_width).into(),
        );
        let theme = iced::widget::row![
            style
                .text(StyleEditorFields::Theme.as_ref())
                .align_y(iced::alignment::Vertical::Center),
            iced::widget::pick_list(
                Theme::iter()
                    .map(|variant| variant.variant_pascal_label())
                    .collect::<Vec<String>>(),
                Some(style.theme.to_string()),
                |mut value| -> StyleEditorMessage {
                    value = value.chars().filter(|c| !c.is_whitespace()).collect();
                    if let Ok(theme) = Theme::from_str(&value) {
                        StyleEditorData::Theme(theme).into()
                    } else {
                        StyleEditorErrors::DeserializeError(value.into()).into()
                    }
                },
            )
            .style(
                |theme: &iced::Theme, status| -> iced::widget::pick_list::Style {
                    let base: iced::widget::pick_list::Style =
                        iced::widget::pick_list::default(theme, status);

                    iced::widget::pick_list::Style {
                        border: iced::border::rounded(style.border_width),
                        ..base
                    }
                },
            )
            .menu_style(|theme: &iced::Theme| -> iced::overlay::menu::Style {
                let palette = theme.extended_palette();
                iced::overlay::menu::Style {
                    background: palette.background.weakest.color.into(),
                    ..iced::overlay::menu::default(theme)
                }
            })
        ]
        .spacing(style.spacing);
        let spacing = style.slider(
            StyleEditorFields::Spacing.as_ref(),
            0.0..=10.0,
            ..,
            style.spacing,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Spacing(value).into() },
            StyleEditorData::Spacing(style.spacing).into(),
        );
        let padding = style.slider(
            StyleEditorFields::Padding.as_ref(),
            0.0..=10.0,
            ..,
            style.padding,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Padding(value).into() },
            StyleEditorData::Padding(style.padding).into(),
        );
        let icon_size = style.slider(
            StyleEditorFields::IconSize.as_ref(),
            4..=36,
            ..,
            style.icon_size,
            1,
            |value| -> StyleEditorMessage { StyleEditorData::IconSize(value).into() },
            StyleEditorData::IconSize(style.icon_size).into(),
        );
        let leeway = style.slider(
            StyleEditorFields::Leeway.as_ref(),
            1.0..=100.0,
            ..,
            style.leeway,
            1.,
            |value| -> StyleEditorMessage { StyleEditorData::Leeway(value).into() },
            StyleEditorData::Leeway(style.leeway).into(),
        );
        let modal_opacity = style.slider(
            StyleEditorFields::ModalOpacity.as_ref(),
            0.0..=1.0,
            ..,
            style.modal_opacity,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::ModalOpacity(value).into() },
            StyleEditorData::ModalOpacity(style.modal_opacity).into(),
        );
        let float_input_step = style.slider(
            StyleEditorFields::FloatInputStep.as_ref(),
            1e-32..=1e-1,
            ..,
            style.float_input_step,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::FloatInputStep(value).into() },
            StyleEditorData::FloatInputStep(style.float_input_step).into(),
        );

        iced::widget::column![
            text_size,
            scale,
            theme,
            border_width,
            spacing,
            padding,
            icon_size,
            leeway,
            modal_opacity,
            float_input_step,
        ]
        .spacing(style.spacing)
        .into()
    }
}
