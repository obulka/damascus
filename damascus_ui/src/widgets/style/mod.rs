// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    ops::{RangeBounds, RangeInclusive},
    str::FromStr,
};

use iced;
use macro_rules_attribute::derive;
use num_traits::{AsPrimitive, Bounded, FromPrimitive, Num, NumAssignOps};

use damascus::{
    Enumerator,
    data_types::{ExpressionData, Float, Mat4, Vec4},
};

use crate::{icons::Icons, widgets::menu};

pub mod editor;
pub mod theme;

use theme::Theme;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Style {
    pub text_size: Float,
    pub scale: Float,
    pub border_width: Float,
    pub theme: Theme,
    pub spacing: Float,
    pub padding: Float,
    pub icon_size: u32,
    pub leeway: Float,
    pub modal_opacity: Float,
    pub float_input_step: Float,
    pub viewer_zoom_sensitivity: Float,
    pub text_zoom_cutoff: Float,
    pub connection_line_width: Float,
    pub inactive_connection_line_scale: Float,
    pub node_input_radius: Float,
    pub node_output_radius: Float,
    pub node_selected_border_width: Float,
    pub selection_box_border_width: Float,
    pub selection_box_alpha: Float,
}

impl Default for Style {
    fn default() -> Self {
        let theme = Theme::default();
        Self {
            text_size: 14.0.into(),
            scale: 1.0.into(),
            border_width: 2.0.into(),
            theme: theme,
            spacing: 3.0.into(),
            padding: 3.0.into(),
            icon_size: 14,
            leeway: 5.0.into(),
            modal_opacity: 0.69.into(),
            float_input_step: 1e-4.into(),
            viewer_zoom_sensitivity: 0.1.into(),
            text_zoom_cutoff: 2.0.into(),
            connection_line_width: 3.5.into(),
            inactive_connection_line_scale: 0.5.into(),
            node_input_radius: 5.0.into(),
            node_output_radius: 5.0.into(),
            node_selected_border_width: 3.0.into(),
            selection_box_border_width: 2.0.into(),
            selection_box_alpha: 0.1.into(),
        }
    }
}

impl Style {
    pub const MINIMUM_DASHED_LINE_SEGMENT_LENGTH: f32 = 1e-12;

    pub fn scale(&self) -> f32 {
        self.scale.value()
    }

    pub fn border_width(&self) -> f32 {
        self.border_width.value()
    }

    pub fn spacing(&self) -> f32 {
        self.spacing.value()
    }

    pub fn padding(&self) -> f32 {
        self.padding.value()
    }

    pub fn icon_size(&self) -> u32 {
        self.icon_size //.value()
    }

    pub fn leeway(&self) -> f32 {
        self.leeway.value()
    }

    pub fn modal_opacity(&self) -> f32 {
        self.modal_opacity.value()
    }

    pub fn float_input_step(&self) -> f32 {
        self.float_input_step.value()
    }

    pub fn viewer_zoom_sensitivity(&self) -> f32 {
        self.viewer_zoom_sensitivity.value()
    }

    pub fn text_zoom_cutoff(&self) -> f32 {
        self.text_zoom_cutoff.value()
    }

    pub fn connection_line_width(&self) -> f32 {
        self.connection_line_width.value()
    }

    pub fn inactive_connection_line_scale(&self) -> f32 {
        self.inactive_connection_line_scale.value()
    }

    pub fn node_input_radius(&self) -> f32 {
        self.node_input_radius.value()
    }

    pub fn node_output_radius(&self) -> f32 {
        self.node_output_radius.value()
    }

    pub fn node_selected_border_width(&self) -> f32 {
        self.node_selected_border_width.value()
    }

    pub fn selection_box_border_width(&self) -> f32 {
        self.selection_box_border_width.value()
    }

    pub fn selection_box_alpha(&self) -> f32 {
        self.selection_box_alpha.value()
    }

    pub fn text_size(&self) -> f32 {
        self.text_size.value()
    }

    pub fn h1_size(&self) -> f32 {
        1.5 * self.text_size.value()
    }

    pub fn h2_size(&self) -> f32 {
        1.375 * self.text_size.value()
    }

    pub fn h3_size(&self) -> f32 {
        1.25 * self.text_size.value()
    }

    pub fn h4_size(&self) -> f32 {
        1.125 * self.text_size.value()
    }

    pub fn text<'a, Message>(
        &'a self,
        message: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Text<'a, Message>
    where
        Message: iced::widget::text::Catalog + 'a,
    {
        iced::widget::text(message)
            .size(self.text_size())
            .wrapping(iced::widget::text::Wrapping::WordOrGlyph)
    }

    pub fn tooltip_label<'a, Message: 'a>(
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
            .padding(self.padding()),
            iced::widget::container(
                self.text(tooltip)
                    .align_y(iced::alignment::Vertical::Center),
            )
            .style(iced::widget::container::bordered_box)
            .padding(self.padding()),
            iced::widget::tooltip::Position::Bottom,
        )
    }

    pub fn tooltip<'a, Message: 'a>(
        &'a self,
        widget: impl Into<iced::Element<'a, Message>>,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Tooltip<'a, Message> {
        iced::widget::tooltip(
            widget,
            iced::widget::container(
                self.text(tooltip)
                    .align_y(iced::alignment::Vertical::Center),
            )
            .style(iced::widget::container::bordered_box)
            .padding(self.padding()),
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
            self.tooltip_label(label, tooltip),
            iced::widget::text_input("", &path)
                .width(iced::Length::FillPortion(4))
                .padding(self.padding())
                .size(self.text_size())
                .on_input(on_input)
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing())
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
            self.tooltip_label(label, tooltip,),
            iced::widget::checkbox(value)
                .spacing(self.spacing())
                .text_size(self.text_size())
                .width(iced::Length::FillPortion(4))
                .on_toggle(on_toggle),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing())
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
                width: self.border_width(),
                color: palette.background.base.color,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn title_bar_focused(&self) -> iced::widget::container::Style {
        let mut style = self.title_bar();

        style.border = iced::Border {
            width: self.border_width(),
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
            width: self.border_width(),
            color: palette.primary.strong.color,
            ..iced::Border::default()
        };

        style
    }

    pub fn close_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::Close
                .as_svg()
                .width(self.icon_size())
                .height(self.icon_size()),
        )
        .style(iced::widget::button::subtle)
        .padding(self.padding())
    }

    pub fn error_button<'a, Message>(
        &'a self,
        text: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::danger)
            .padding(self.padding())
    }

    pub fn button<'a, Message>(
        &'a self,
        text: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::secondary)
            .padding(self.padding())
    }

    pub fn warning_button<'a, Message>(
        &'a self,
        text: &'a str,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::warning)
            .padding(self.padding())
    }

    pub fn success_button<'a, Message>(
        &'a self,
        text: &'a str,
    ) -> iced::widget::Button<'a, Message> {
        iced::widget::button(self.text(text))
            .style(iced::widget::button::success)
            .padding(self.padding())
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
            .width(self.icon_size())
            .height(self.icon_size()),
        )
        .style(iced::widget::button::subtle)
        .padding(self.padding())
    }

    pub fn detach_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::Detach
                .as_svg()
                .width(self.icon_size())
                .height(self.icon_size()),
        )
        .style(iced::widget::button::subtle)
        .padding(self.padding())
    }

    pub fn horizontal_split_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::HorizontalSplit
                .as_svg()
                .width(self.icon_size())
                .height(self.icon_size()),
        )
        .style(iced::widget::button::subtle)
        .padding(self.padding())
    }

    pub fn vertical_split_button<'a, Message>(&'a self) -> iced::widget::Button<'a, Message> {
        iced::widget::button(
            Icons::VerticalSplit
                .as_svg()
                .width(self.icon_size())
                .height(self.icon_size()),
        )
        .style(iced::widget::button::subtle)
        .padding(self.padding())
    }

    pub fn pick_list<'a, E, F, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        picked: E,
        on_change: F,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::Element<'a, Message>
    where
        E: Enumerator,
        F: Fn(E) -> Message + Copy + 'static,
        Message: Clone + 'a,
    {
        iced::widget::row![
            self.tooltip_label(name, tooltip),
            iced::widget::pick_list(
                E::iter()
                    .map(|variant| variant.variant_pascal_label())
                    .collect::<Vec<String>>(),
                Some(picked.variant_pascal_label()),
                move |mut value| -> Message {
                    value = value.chars().filter(|c| !c.is_whitespace()).collect();
                    on_change(match E::from_str(&value) {
                        Ok(variant) => variant,
                        Err(_parse_error) => E::default(),
                    })
                },
            )
            .width(iced::Length::FillPortion(4))
            .style(
                |theme: &iced::Theme, status| -> iced::widget::pick_list::Style {
                    let base: iced::widget::pick_list::Style =
                        iced::widget::pick_list::default(theme, status);

                    iced::widget::pick_list::Style {
                        border: iced::border::rounded(self.border_width()),
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
        .spacing(self.spacing())
        .into()
    }

    pub fn text_input<'a, Message>(
        &'a self,
        placeholder: &str,
        value: &str,
        on_input: impl Fn(String) -> Message + 'a,
    ) -> iced::widget::TextInput<'a, Message>
    where
        Message: Clone + 'a,
    {
        iced::widget::TextInput::new(placeholder, value)
            .size(self.text_size())
            .padding(self.padding())
            .on_input(on_input)
    }

    pub fn number_input<'a, T, Message>(
        &'a self,
        value: T,
        expression: &str,
        on_input: impl Fn(String) -> Message + 'a,
        fill_portion: u16,
    ) -> iced::widget::Tooltip<'a, Message>
    where
        T: Num
            + NumAssignOps
            + PartialOrd
            + Display
            + Debug
            + FromStr
            + Clone
            + Bounded
            + FromPrimitive
            + From<u8>
            + Copy
            + 'static,
        <T as FromStr>::Err: std::fmt::Debug,
        Message: Clone + Default + 'a,
        f64: AsPrimitive<T> + From<T> + 'a,
    {
        let default_expression: String = format!("{:}", value);
        self.tooltip(
            self.text_input(
                "",
                if expression.is_empty() {
                    &default_expression
                } else {
                    expression
                },
                on_input,
            )
            .font(iced::Font::MONOSPACE)
            .width(iced::Length::FillPortion(fill_portion))
            .style(|theme: &iced::Theme, status| {
                let palette: &iced::theme::palette::Extended = theme.extended_palette();
                let mut background_colour: iced::Color = palette.background.base.color;
                background_colour.a = 1.0;

                let mut style: iced::widget::text_input::Style =
                    iced::widget::text_input::default(theme, status);
                style.background = background_colour.into();
                style.placeholder = style.value;
                style
            }),
            format!("={:.3}", value),
        )
    }

    pub fn vec4<'a, T, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        value: &impl Vec4<T>,
        on_input: impl Fn(usize, String) -> Message + 'a + Clone,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
        scale_portion: u16,
    ) -> iced::Element<'a, Message>
    where
        T: Num
            + NumAssignOps
            + PartialOrd
            + Display
            + Debug
            + FromStr
            + Clone
            + Bounded
            + FromPrimitive
            + From<u8>
            + Copy
            + 'static,
        <T as FromStr>::Err: std::fmt::Debug,
        Message: Clone + Default + 'a,
        f64: AsPrimitive<T> + From<T> + 'a,
    {
        iced::widget::row![
            self.tooltip_label(name, tooltip),
            self.number_input(
                value.x().value(),
                value.x().expression(),
                {
                    let on_input_clone = on_input.clone();
                    move |value| on_input_clone.clone()(0, value)
                },
                scale_portion,
            ),
            self.number_input(
                value.y().value(),
                value.y().expression(),
                {
                    let on_input_clone = on_input.clone();
                    move |value| on_input_clone.clone()(1, value)
                },
                scale_portion,
            ),
            self.number_input(
                value.z().value(),
                value.z().expression(),
                {
                    let on_input_clone = on_input.clone();
                    move |value| on_input_clone.clone()(2, value)
                },
                scale_portion,
            ),
            self.number_input(
                value.w().value(),
                value.w().expression(),
                {
                    let on_input_clone = on_input.clone();
                    move |value| on_input_clone.clone()(3, value)
                },
                scale_portion,
            ),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing())
        .into()
    }

    pub fn mat4<'a, T, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        value: &impl Mat4<T>,
        on_input: impl Fn(usize, usize, String) -> Message + 'a + Clone,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
    ) -> iced::Element<'a, Message>
    where
        T: Num
            + NumAssignOps
            + PartialOrd
            + Display
            + Debug
            + FromStr
            + Clone
            + Bounded
            + FromPrimitive
            + From<u8>
            + Copy
            + 'static,
        <T as FromStr>::Err: std::fmt::Debug,
        Message: Clone + Default + 'a,
        f64: AsPrimitive<T> + From<T> + 'a,
    {
        // TODO Transpose
        iced::widget::row![
            self.tooltip_label(name, tooltip),
            iced::widget::column![
                self.vec4(
                    "x",
                    value.x_axis(),
                    {
                        let on_input_clone = on_input.clone();
                        move |row, value| on_input_clone(0, row, value)
                    },
                    "first column",
                    4,
                ),
                self.vec4(
                    "y",
                    value.y_axis(),
                    {
                        let on_input_clone = on_input.clone();
                        move |row, value| on_input_clone(1, row, value)
                    },
                    "second column",
                    4,
                ),
                self.vec4(
                    "z",
                    value.z_axis(),
                    {
                        let on_input_clone = on_input.clone();
                        move |row, value| on_input_clone(2, row, value)
                    },
                    "third column",
                    4,
                ),
                self.vec4(
                    "w",
                    value.w_axis(),
                    {
                        let on_input_clone = on_input.clone();
                        move |row, value| on_input_clone(3, row, value)
                    },
                    "fourth column",
                    4,
                ),
            ]
            .width(iced::Length::FillPortion(4)),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing())
        .into()
    }

    pub fn slider<'a, T, Message>(
        &'a self,
        name: impl iced::widget::text::IntoFragment<'a>,
        suggested_range: RangeInclusive<T>,
        _limit_range: impl RangeBounds<T>,
        value: T,
        expression: &str,
        step: T,
        on_input: impl Fn(String) -> Message + 'a,
        on_change: impl Fn(T) -> Message + 'static,
        tooltip: impl iced::widget::text::IntoFragment<'a>,
        scale_portion: u16,
    ) -> iced::Element<'a, Message>
    where
        T: Num
            + NumAssignOps
            + PartialOrd
            + Display
            + Debug
            + FromStr
            + Clone
            + Bounded
            + FromPrimitive
            + From<u8>
            + Copy
            + 'static,
        <T as FromStr>::Err: std::fmt::Debug,
        Message: Clone + Default + 'a,
        f64: AsPrimitive<T> + From<T> + 'a,
    {
        iced::widget::row![
            self.tooltip_label(name, tooltip),
            self.number_input(value, expression, on_input, scale_portion),
            iced::widget::slider(suggested_range, value, on_change)
                .step(step)
                .width(iced::Length::FillPortion(3 * scale_portion))
                .height(self.text_size() + 2. * self.padding()),
        ]
        .align_y(iced::Alignment::Center)
        .spacing(self.spacing())
        .into()
    }

    pub fn dropdown_menu<'a, T, L, Message>(
        &'a self,
        options: L,
        menu_title: String,
        icon: Option<iced_core::Svg>,
        on_select: impl Fn(T) -> Message + 'a,
    ) -> iced::Element<'a, Message>
    where
        T: ToString + PartialEq + Clone + 'a,
        L: Borrow<[T]> + 'a,
        Message: Clone + 'a,
    {
        let icon_size: u32 = if icon.is_some() { self.icon_size() } else { 0 };
        menu::DropdownMenu::new(options, menu_title, icon, on_select)
            .padding(self.padding())
            .text_size(self.text_size())
            .spacing(self.spacing())
            .icon_size(icon_size)
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

    // pub fn colour_picker<'a, Message: Clone + 'static>(
    //     &'a self,
    //     name: impl iced::widget::text::IntoFragment<'a>,
    //     picking: bool,
    //     color: iced::Color,
    //     toggle_message: Message,
    //     submit_message: impl Fn(iced::Color) -> Message + 'static,
    //     tooltip: impl iced::widget::text::IntoFragment<'a>,
    // ) -> iced::Element<'a, Message> {
    //     // TODO this should probably be absorbed for more control over behaviour

    //     iced::widget::row![
    //         self.tooltip_label(name, tooltip),
    //         ColourPicker::new(
    //             picking,
    //             color,
    //             self.button(" ")
    //                 .style(move |_theme, _status| {
    //                     iced::widget::button::Style {
    //                         background: Some(iced::Background::Color(color)),
    //                         ..iced::widget::button::Style::default()
    //                     }
    //                 })
    //                 .padding(self.padding)
    //                 .width(iced::Length::FillPortion(4))
    //                 .on_press(toggle_message.clone()),
    //             toggle_message,
    //             submit_message,
    //         ),
    //     ]
    //     .spacing(self.spacing)
    //     .align_y(iced::Alignment::Center)
    //     .into()
    // }

    pub fn iced_theme(&self) -> iced::Theme {
        self.theme.into()
    }

    pub fn spacer<'a, Message: 'a>() -> iced::Element<'a, Message> {
        iced::widget::container(Self::none())
            .height(iced::Fill)
            .width(iced::Fill)
            .into()
    }

    pub fn horizontal_spacer<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        iced::widget::container(Self::none())
            .width(iced::Fill)
            .into()
    }

    pub fn vertical_spacer<'a, Message: 'a>() -> iced::widget::Container<'a, Message> {
        iced::widget::container(Self::none())
            .height(iced::Fill)
            .into()
    }

    pub fn node_text_orbit_distance(&self, max_distance: f32, index: usize) -> f32 {
        max_distance
            .min(1.5 * self.text_size() * (index + 1) as f32)
            .max(0.)
    }

    pub fn connection_stroke(
        &self,
        reciprocal_zoom: f32,
        colour: iced::Color,
    ) -> iced::widget::canvas::Stroke<'_> {
        iced::widget::canvas::Stroke::default()
            .with_width(self.connection_line_width() * reciprocal_zoom)
            .with_color(colour)
    }

    pub fn inactive_connection_stroke<'a>(
        &'a self,
        reciprocal_zoom: f32,
        colour: iced::Color,
        segments: &'a mut Vec<f32>,
    ) -> iced::widget::canvas::Stroke<'a> {
        let inactive_connection_line_width: f32 =
            self.connection_line_width() * reciprocal_zoom * self.inactive_connection_line_scale();

        let dashed_line_segment_length = 2. * inactive_connection_line_width;

        segments.push(dashed_line_segment_length);
        segments.push(dashed_line_segment_length);
        segments.push(Self::MINIMUM_DASHED_LINE_SEGMENT_LENGTH);
        segments.push(dashed_line_segment_length);

        let mut inactive_view_connection_stroke = iced::widget::canvas::Stroke::default()
            .with_width(inactive_connection_line_width)
            .with_line_cap(iced::widget::canvas::LineCap::Round)
            .with_color(colour);
        inactive_view_connection_stroke.line_dash = iced::widget::canvas::LineDash {
            segments: segments,
            offset: 0,
        };

        inactive_view_connection_stroke
    }

    pub fn none<'a, Message>() -> iced::Element<'a, Message> {
        None::<iced::Element<'a, Message>>.into()
    }
}
