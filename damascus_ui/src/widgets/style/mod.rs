// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    borrow::Borrow,
    fmt::Display,
    ops::{RangeBounds, RangeInclusive},
    str::FromStr,
};

use glam::Vec4;
use iced;
use iced_aw;
use macro_rules_attribute::derive;
use num_traits::{Bounded, FromPrimitive, Num, NumAssignOps};

use damascus::Enumerator;

use crate::{icons::Icons, widgets::menu};

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
    pub viewer_zoom_sensitivity: f32,
    pub picking_default_node_colour: bool,
    pub default_node_colour: Vec4,
    pub text_zoom_cutoff: f32,
    pub connection_line_width: f32,
    pub inactive_connection_line_scale: f32,
    pub node_input_radius: f32,
    pub node_output_radius: f32,
    pub node_selected_border_width: f32,
    pub selection_box_border_width: f32,
    pub selection_box_alpha: f32,
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
            viewer_zoom_sensitivity: 0.1,
            picking_default_node_colour: false,
            default_node_colour: Vec4::new(0.35, 0.35, 0.35, 1.),
            text_zoom_cutoff: 2.,
            connection_line_width: 3.5,
            inactive_connection_line_scale: 0.5,
            node_input_radius: 5.0,
            node_output_radius: 5.0,
            node_selected_border_width: 3.,
            selection_box_border_width: 2.,
            selection_box_alpha: 0.1,
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

    pub fn text<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message)
            .size(self.text_size)
            .wrapping(iced::widget::text::Wrapping::WordOrGlyph)
    }

    pub fn tooltip<'a, Message: 'a>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Tooltip<'a, Message> {
        iced::widget::tooltip(
            iced::widget::container(
                self.text(message)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Right)
                    .width(iced::Length::FillPortion(1)),
            )
            .padding(self.padding),
            iced::widget::container(
                self.text(tooltip)
                    .align_y(iced::alignment::Vertical::Center),
            )
            .style(iced::widget::container::bordered_box)
            .padding(self.padding),
            iced::widget::tooltip::Position::Bottom,
        )
    }

    pub fn filepath<'a, Message: Clone + 'a>(
        &'a self,
        label: impl iced::widget::text::IntoFragment<'a>,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
        path: &str,
        on_input: impl Fn(String) -> Message + 'a,
    ) -> iced::Element<'a, Message> {
        iced::widget::row![
            self.tooltip(label, tooltip,),
            iced::widget::text_input("", &path)
                .width(iced::Length::FillPortion(4))
                .padding(self.padding)
                .size(self.text_size)
                .on_input(on_input)
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing)
        .into()
    }

    pub fn checkbox<'a, Message: 'a>(
        &'a self,
        label: impl iced::widget::text::IntoFragment<'a>,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
        value: bool,
        on_toggle: impl Fn(bool) -> Message + 'a,
    ) -> iced::Element<'a, Message> {
        iced::widget::row![
            self.tooltip(label, tooltip,),
            iced::widget::checkbox(value)
                .spacing(self.spacing)
                .text_size(self.text_size)
                .width(iced::Length::FillPortion(4))
                .on_toggle(on_toggle),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing)
        .into()
    }

    pub fn heading<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h1_size())
    }

    pub fn subheading<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h2_size())
    }

    pub fn subsubheading<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message).size(self.h3_size())
    }

    pub fn subsubsubheading<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
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
            text_color: Some(palette.background.weakest.text),
            background: Some(palette.background.weakest.color.into()),
            border: iced::Border {
                width: self.border_width,
                color: palette.background.base.color,
                ..Default::default()
            },
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
            background: Some(palette.background.base.color.into()),
            border: iced::Border {
                color: palette.background.base.color,
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

    pub fn error_button<'a, Message>(
        &'a self,
        text: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::danger)
            .padding(self.padding)
    }

    pub fn button<'a, Message>(
        &'a self,
        text: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Button<'a, Message> {
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

    pub fn pick_list<'a, T, F, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        picked: T,
        on_change: F,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::Element<'a, Message>
    where
        T: Enumerator,
        F: Fn(T) -> Message + Copy + 'static,
        Message: Clone + 'a,
    {
        iced::widget::row![
            self.tooltip(name, tooltip,),
            iced::widget::pick_list(
                T::iter()
                    .map(|variant| variant.variant_pascal_label())
                    .collect::<Vec<String>>(),
                Some(picked.variant_pascal_label()),
                move |mut value| -> Message {
                    value = value.chars().filter(|c| !c.is_whitespace()).collect();
                    on_change(match T::from_str(&value) {
                        Ok(variant) => variant,
                        Err(_parse_error) => T::default(),
                    })
                },
            )
            .width(iced::Length::FillPortion(4))
            .style(
                |theme: &iced::Theme, status| -> iced::widget::pick_list::Style {
                    let base: iced::widget::pick_list::Style =
                        iced::widget::pick_list::default(theme, status);

                    iced::widget::pick_list::Style {
                        border: iced::border::rounded(self.border_width),
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
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing)
        .into()
    }

    pub fn slider<'a, T, F, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        suggested_range: RangeInclusive<T>,
        limit_range: impl RangeBounds<T>,
        value: T,
        step: T,
        on_change: F,
        on_release: Message,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
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
            self.tooltip(name, tooltip),
            // TODO BUG when highlighting ex 123 and hitting 1 can panic
            // bring this in so we can fix and add better editing control
            iced_aw::widget::number_input(&value, limit_range, on_change)
                .step(step) // TODO dynamic based on highlight?
                .ignore_buttons(true)
                .set_size(self.text_size)
                .width(iced::Length::FillPortion(1))
                .font(iced::Font::MONOSPACE)
                .padding(self.padding)
                .on_submit(on_release.clone())
                .input_style(|theme: &iced::Theme, status| {
                    let mut background_colour: iced::Color =
                        theme.extended_palette().background.base.color;
                    background_colour.a = 1.0;
                    iced::widget::text_input::Style {
                        background: background_colour.into(),
                        ..iced::widget::text_input::default(theme, status)
                    }
                }),
            iced::widget::slider(suggested_range, value, on_change)
                .step(step)
                .width(iced::Length::FillPortion(3))
                .height(self.text_size + 2. * self.padding)
                .on_release(on_release),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing)
        .into()
    }

    pub fn dropdown_menu<'a, T, L, Message>(
        &'a self,
        options: L,
        menu_title: String,
        on_select: impl Fn(T) -> Message + 'a,
    ) -> iced::Element<'a, Message>
    where
        T: ToString + PartialEq + Clone + 'a,
        L: Borrow<[T]> + 'a,
        Message: Clone + 'a,
    {
        menu::DropdownMenu::new(options, menu_title, on_select)
            .padding(self.padding)
            .text_size(self.text_size)
            .style(|theme: &iced::Theme, status| -> menu::Style { menu::default(theme, status) })
            .menu_style(|theme: &iced::Theme| -> iced::overlay::menu::Style {
                let palette = theme.extended_palette();
                iced::overlay::menu::Style {
                    background: palette.background.weakest.color.into(),
                    ..iced::overlay::menu::default(theme)
                }
            })
            .into()
    }

    pub fn colour_picker<'a, Message: Clone + 'static>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        picking: bool,
        color: iced::Color,
        toggle_message: Message,
        submit_message: impl Fn(iced::Color) -> Message + 'static,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::Element<'a, Message> {
        // TODO this should probably be absorbed for more control over behaviour

        iced::widget::row![
            self.tooltip(name, tooltip,),
            iced_aw::helpers::color_picker(
                picking,
                color,
                self.button(" ")
                    .style(move |_theme, _status| {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(color)),
                            ..iced::widget::button::Style::default()
                        }
                    })
                    .padding(self.padding)
                    .width(iced::Length::FillPortion(4))
                    .on_press(toggle_message.clone()),
                toggle_message,
                submit_message,
            ),
        ]
        .spacing(self.spacing)
        .align_y(iced::Alignment::Center)
        .into()
    }

    pub fn iced_theme(&self) -> iced::Theme {
        self.theme.into()
    }

    pub fn spacer<'a, Message: 'a>() -> iced::Element<'a, Message> {
        iced::widget::container(Self::none())
            .height(iced::Fill)
            .width(iced::Fill)
            .into()
    }

    pub fn horizontal_spacer<'a, Message: 'a>() -> iced::Element<'a, Message> {
        iced::widget::container(Self::none())
            .width(iced::Fill)
            .into()
    }

    pub fn vertical_spacer<'a, Message: 'a>() -> iced::Element<'a, Message> {
        iced::widget::container(Self::none())
            .height(iced::Fill)
            .into()
    }

    pub fn none<'a, Message>() -> iced::Element<'a, Message> {
        None::<iced::Element<'a, Message>>.into()
    }
}
