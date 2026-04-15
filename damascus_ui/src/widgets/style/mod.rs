// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    fmt::Display,
    ops::{AddAssign, DivAssign, MulAssign, RangeBounds, RangeInclusive, RemAssign, SubAssign},
    str::FromStr,
};

use iced;
use macro_rules_attribute::derive;
use num_traits::{Bounded, FromPrimitive, Num, NumAssignOps};

use crate::{EnumTraits, icons::Icons};

pub mod editor;
pub mod theme;

use theme::Theme;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Style {
    pub text_size: f32,
    pub scale: f32,
    pub border_width: f32,
    pub theme: Theme,
    pub spacing: f32,
    pub padding: f32,
    pub icon_size: u32,
    pub leeway: f32,
    pub modal_opacity: f32,
    pub float_input_step: f32,
}

impl Default for Style {
    fn default() -> Self {
        let theme = Theme::default();
        Self {
            text_size: 16.0,
            scale: 1.0,
            border_width: 2.0,
            theme: theme,
            spacing: 3.0,
            padding: 3.0,
            icon_size: 16,
            leeway: 5.0,
            modal_opacity: 0.69,
            float_input_step: 1e-4,
        }
    }
}

impl Style {
    pub fn text_size(&self) -> f32 {
        self.text_size
    }

    pub fn h1_size(&self) -> f32 {
        1.5 * self.text_size
    }

    pub fn h2_size(&self) -> f32 {
        1.375 * self.text_size
    }

    pub fn h3_size(&self) -> f32 {
        1.25 * self.text_size
    }

    pub fn h4_size(&self) -> f32 {
        1.125 * self.text_size
    }

    pub fn text<'a, Message>(&'a self, message: &'a str) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.text_size())
    }

    pub fn heading<'a, Message>(&'a self, message: &'a str) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h1_size())
    }

    pub fn subheading<'a, Message>(&'a self, message: &'a str) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h2_size())
    }

    pub fn subsubheading<'a, Message>(&'a self, message: &'a str) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h3_size())
    }

    pub fn subsubsubheading<'a, Message>(
        &'a self,
        message: &'a str,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h4_size())
    }

    pub fn title_bar(&self) -> iced::widget::container::Style {
        let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(self.theme);
        let palette: &iced::theme::palette::Extended = theme.extended_palette();

        iced::widget::container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(&self) -> iced::widget::container::Style {
        let mut style = self.title_bar();

        style.border = iced::Border {
            width: self.border_width,
            color: iced::Color::TRANSPARENT,
            ..iced::Border::default()
        };

        style
    }

    pub fn pane(&self) -> iced::widget::container::Style {
        let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(self.theme);
        let palette: &iced::theme::palette::Extended = theme.extended_palette();

        iced::widget::container::Style {
            background: Some(palette.background.weak.color.into()),
            border: iced::Border {
                width: self.border_width,
                color: palette.background.strong.color,
                ..iced::Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(&self) -> iced::widget::container::Style {
        let theme: iced::Theme = <Theme as Into<iced::Theme>>::into(self.theme);
        let palette: &iced::theme::palette::Extended = theme.extended_palette();

        let mut style = self.pane();

        style.border = iced::Border {
            width: self.border_width,
            color: palette.primary.strong.color,
            ..iced::Border::default()
        };

        style
    }

    pub fn close_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::Close
                .as_svg()
                .width(self.icon_size)
                .height(self.icon_size),
        )
        .style(iced::widget::button::secondary)
        .padding(self.padding)
    }

    pub fn error_button<'a, Message>(&'a self, text: &'a str) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::danger)
            .padding(self.padding)
    }

    pub fn button<'a, Message>(&'a self, text: &'a str) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::secondary)
            .padding(self.padding)
    }

    pub fn warning_button<'a, Message>(
        &'a self,
        text: &'a str,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::warning)
            .padding(self.padding)
    }

    pub fn success_button<'a, Message>(
        &'a self,
        text: &'a str,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::success)
            .padding(self.padding)
    }

    pub fn maximize_button<'a, Message>(
        &'a self,
        is_maximized: bool,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            if is_maximized {
                Icons::Minimize.as_svg()
            } else {
                Icons::Maximize.as_svg()
            }
            .width(self.icon_size)
            .height(self.icon_size),
        )
        .style(iced::widget::button::secondary)
        .padding(self.padding)
    }

    pub fn detach_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::Detach
                .as_svg()
                .width(self.icon_size)
                .height(self.icon_size),
        )
        .style(iced::widget::button::secondary)
        .padding(self.padding)
    }

    pub fn slider<'a, T, F, Message>(
        &'a self,
        name: &'a str,
        suggested_range: RangeInclusive<T>,
        limit_range: impl RangeBounds<T>,
        value: T,
        step: T,
        on_change: F,
        on_release: Message,
    ) -> iced::Element<'a, Message>
    where
        T: Num
            + NumAssignOps
            + PartialOrd
            + Display
            + FromStr
            + Clone
            + Bounded
            + FromPrimitive
            + From<u8>
            + Copy
            + 'static,
        F: Fn(T) -> Message + Copy + 'static,
        Message: Clone + 'a,
        f64: From<T> + 'a,
    {
        iced::widget::row![
            self.text(name).align_y(iced::alignment::Vertical::Center),
            iced_aw::widget::number_input(&value, limit_range, on_change)
                .step(step) // TODO dynamic based on highlight?
                .ignore_buttons(true)
                .width(iced::Length::Fixed(self.text_size * 3.))
                .padding(self.padding),
            iced::widget::slider(suggested_range, value, on_change)
                .step(step)
                .height(self.text_size + 2. * self.padding),
        ]
        .spacing(self.spacing)
        .into()
    }

    pub fn none<'a, Message>() -> iced::Element<'a, Message> {
        None::<iced::Element<'a, Message>>.into()
    }
}
