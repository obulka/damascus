// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

// use glam::Vec4;
use iced;
use macro_rules_attribute::derive;
use strum::Display;

use damascus::{
    Enumerator,
    data_types::{ExpressionData, Float},
};

use crate::{
    EnumTraits, ErrorTraits,
    app::Context,
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
    ViewerZoomSensitivity,
    // ExampleColour,
    TextZoomCutoff,
    ConnectionLineWidth,
    InactiveConnectionLineScale,
    NodeInputRadius,
    NodeOutputRadius,
    NodeSelectedBorderWidth,
    SelectionBoxBorderWidth,
    SelectionBoxAlpha,
}

impl StyleEditorFields {
    fn increment(&self, style: &mut Style) {
        match self {
            Self::Scale => style.scale *= 1.05,
            Self::IconSize => style.icon_size += 1,
            _ => {}
        }
    }

    fn decrement(&self, style: &mut Style) {
        match self {
            Self::Scale => style.scale *= 0.95,
            Self::IconSize => style.icon_size -= 1,
            _ => {}
        }
    }

    fn toggle(&self, _style: &mut Style) {
        match self {
            // Self::ExampleColour => {
            //     style.picking_example_colour = !style.picking_example_colour;
            // }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub enum StyleEditorData {
    TextSize(Float),
    Scale(Float),
    BorderWidth(Float),
    Theme(Theme),
    Spacing(Float),
    Padding(Float),
    IconSize(u32),
    Leeway(Float),
    ModalOpacity(Float),
    FloatInputStep(Float),
    ViewerZoomSensitivity(Float),
    // ExampleColour(Vec4),
    TextZoomCutoff(Float),
    ConnectionLineWidth(Float),
    InactiveConnectionLineScale(Float),
    NodeInputRadius(Float),
    NodeOutputRadius(Float),
    NodeSelectedBorderWidth(Float),
    SelectionBoxBorderWidth(Float),
    SelectionBoxAlpha(Float),
}

impl StyleEditorData {
    fn set(self, style: &mut Style) {
        match self {
            Self::TextSize(text_size) => style.text_size = text_size,
            Self::Scale(scale) => style.scale = scale,
            Self::BorderWidth(border_width) => style.border_width = border_width,
            Self::Theme(theme) => style.theme = theme,
            Self::Spacing(spacing) => style.spacing = spacing,
            Self::Padding(padding) => style.padding = padding,
            Self::IconSize(icon_size) => style.icon_size = icon_size,
            Self::Leeway(leeway) => style.leeway = leeway,
            Self::ModalOpacity(modal_opacity) => style.modal_opacity = modal_opacity,
            Self::FloatInputStep(float_input_step) => style.float_input_step = float_input_step,
            Self::ViewerZoomSensitivity(viewer_zoom_sensitivity) => {
                style.viewer_zoom_sensitivity = viewer_zoom_sensitivity
            }
            // Self::ExampleColour(colour) => {
            //     style.example_colour = *colour;
            //     style.picking_example_colour = !style.picking_example_colour;
            // }
            Self::TextZoomCutoff(text_zoom_cutoff) => style.text_zoom_cutoff = text_zoom_cutoff,
            Self::ConnectionLineWidth(connection_line_width) => {
                style.connection_line_width = connection_line_width
            }
            Self::InactiveConnectionLineScale(inactive_connection_line_scale) => {
                style.inactive_connection_line_scale = inactive_connection_line_scale
            }
            Self::NodeInputRadius(node_input_radius) => style.node_input_radius = node_input_radius,
            Self::NodeOutputRadius(node_output_radius) => {
                style.node_output_radius = node_output_radius
            }
            Self::NodeSelectedBorderWidth(node_selected_border_width) => {
                style.node_selected_border_width = node_selected_border_width
            }
            Self::SelectionBoxBorderWidth(selection_box_border_width) => {
                style.selection_box_border_width = selection_box_border_width
            }
            Self::SelectionBoxAlpha(selection_box_alpha) => {
                style.selection_box_alpha = selection_box_alpha
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum StyleEditorMessage {
    Increment(StyleEditorFields),
    Decrement(StyleEditorFields),
    Set(StyleEditorData),
    Toggle(StyleEditorFields),
    Error(StyleEditorErrors),
    StyleChanged,
    #[default]
    None,
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
        let changed: bool = match message {
            StyleEditorMessage::Set(field) => {
                field.set(style);
                true
            }
            StyleEditorMessage::Increment(field) => {
                field.increment(style);
                true
            }
            StyleEditorMessage::Decrement(field) => {
                field.decrement(style);
                true
            }
            StyleEditorMessage::Toggle(field) => {
                field.toggle(style);
                true
            }
            _ => false,
        };

        if changed {
            iced::Task::done(StyleEditorMessage::StyleChanged)
        } else {
            iced::Task::none()
        }
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
        let theme = style.pick_list(
            StyleEditorFields::Theme.variant_label(),
            style.theme,
            |theme| -> StyleEditorMessage { StyleEditorData::Theme(theme).into() },
            "The colour scheme of the UI",
        );
        let text_size = style.slider(
            StyleEditorFields::TextSize.variant_label(),
            6.0..=36.0,
            ..,
            style.text_size(),
            style.text_size.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::TextSize(
                    Float::new(style.text_size())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::TextSize(value.into()).into() },
            "The default global text size",
            1,
        );
        let scale = style.slider(
            StyleEditorFields::Scale.variant_label(),
            0.25..=5.0,
            ..,
            style.scale(),
            style.scale.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::Scale(Float::new(style.scale()).with_expression(expression).into())
                    .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::Scale(value.max(0.1).into()).into() },
            "Scale the UI up or down",
            1,
        );
        let border_width = style.slider(
            StyleEditorFields::BorderWidth.variant_label(),
            0.0..=10.0,
            ..,
            style.border_width(),
            style.border_width.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::BorderWidth(
                    Float::new(style.border_width())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::BorderWidth(value.into()).into() },
            "The width of borders in the UI",
            1,
        );
        let spacing = style.slider(
            StyleEditorFields::Spacing.variant_label(),
            0.0..=10.0,
            ..,
            style.spacing(),
            style.spacing.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::Spacing(
                    Float::new(style.spacing())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::Spacing(value.into()).into() },
            "The spacing between UI elements",
            1,
        );
        let padding = style.slider(
            StyleEditorFields::Padding.variant_label(),
            0.0..=10.0,
            ..,
            style.padding(),
            style.padding.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::Padding(
                    Float::new(style.padding())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::Padding(value.into()).into() },
            "The padding around UI elements",
            1,
        );
        let icon_size = style.slider(
            StyleEditorFields::IconSize.variant_label(),
            4..=36,
            ..,
            style.icon_size,
            "",
            1,
            |_value| -> StyleEditorMessage { StyleEditorMessage::None },
            |value| -> StyleEditorMessage { StyleEditorData::IconSize(value.into()).into() },
            "The size of icon buttons",
            1,
        );
        let leeway = style.slider(
            StyleEditorFields::Leeway.variant_label(),
            1.0..=100.0,
            ..,
            style.leeway(),
            style.leeway.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::Leeway(
                    Float::new(style.leeway())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::Leeway(value.into()).into() },
            "The tolerance around grabbable elements",
            1,
        );
        let modal_opacity = style.slider(
            StyleEditorFields::ModalOpacity.variant_label(),
            0.0..=1.0,
            ..,
            style.modal_opacity(),
            style.modal_opacity.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::ModalOpacity(
                    Float::new(style.modal_opacity())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::ModalOpacity(value.into()).into() },
            "The opacity of the background when in a modal context",
            1,
        );
        let viewer_zoom_sensitivity = style.slider(
            StyleEditorFields::ViewerZoomSensitivity.variant_label(),
            1e-3..=2.0,
            ..,
            style.viewer_zoom_sensitivity(),
            style.viewer_zoom_sensitivity.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::ViewerZoomSensitivity(
                    Float::new(style.viewer_zoom_sensitivity())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::ViewerZoomSensitivity(value.into()).into()
            },
            "The sensitivity of the mouse scroll wheel when zooming in the viewer",
            1,
        );
        let float_input_step = style.slider(
            StyleEditorFields::FloatInputStep.variant_label(),
            1e-32..=1e-1,
            ..,
            style.float_input_step(),
            style.float_input_step.expression(),
            1e-32,
            move |expression| -> StyleEditorMessage {
                StyleEditorData::FloatInputStep(
                    Float::new(style.float_input_step())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::FloatInputStep(value.into()).into() },
            "The minimum step between floating point numbers when dragging sliders",
            1,
        );
        // let example_colour = style.colour_picker(
        //     StyleEditorFields::ExampleColour.variant_label(),
        //     style.picking_example_colour,
        //     iced::Color::from_rgba(
        //         style.example_colour.x,
        //         style.example_colour.y,
        //         style.example_colour.z,
        //         style.example_colour.w,
        //     ),
        //     StyleEditorMessage::Toggle(StyleEditorFields::ExampleColour),
        //     |colour| -> StyleEditorMessage {
        //         StyleEditorData::ExampleColour(Vec4::new(
        //             colour.r, colour.g, colour.b, colour.a,
        //         ))
        //         .into()
        //     },
        //     "An example colour",
        // );

        let text_zoom_cutoff = style.slider(
            StyleEditorFields::TextZoomCutoff.variant_label(),
            1.0..=10.,
            ..,
            style.text_zoom_cutoff(),
            style.text_zoom_cutoff.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::TextZoomCutoff(
                    Float::new(style.text_zoom_cutoff())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::TextZoomCutoff(value.into()).into() },
            "If you zoom out past this value, text will stop rendering in the node graph",
            1,
        );
        let connection_line_width = style.slider(
            StyleEditorFields::ConnectionLineWidth.variant_label(),
            0.5..=10.,
            ..,
            style.connection_line_width(),
            style.connection_line_width.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::ConnectionLineWidth(
                    Float::new(style.connection_line_width())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::ConnectionLineWidth(value.into()).into()
            },
            "The width of the lines representing connections between nodes",
            1,
        );
        let inactive_connection_line_scale = style.slider(
            StyleEditorFields::InactiveConnectionLineScale.variant_label(),
            0.1..=1.0,
            ..,
            style.inactive_connection_line_scale(),
            style.inactive_connection_line_scale.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::InactiveConnectionLineScale(
                    Float::new(style.inactive_connection_line_scale())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::InactiveConnectionLineScale(value.into()).into()
            },
            "The width of lines representing the inactive viewport connections",
            1,
        );
        let node_input_radius = style.slider(
            StyleEditorFields::NodeInputRadius.variant_label(),
            1.0..=10.0,
            ..,
            style.node_input_radius(),
            style.node_input_radius.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::NodeInputRadius(
                    Float::new(style.node_input_radius())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage { StyleEditorData::NodeInputRadius(value.into()).into() },
            "The radius of the circular inputs on nodes",
            1,
        );
        let node_output_radius = style.slider(
            StyleEditorFields::NodeOutputRadius.variant_label(),
            1.0..=10.0,
            ..,
            style.node_output_radius(),
            style.node_output_radius.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::NodeOutputRadius(
                    Float::new(style.node_output_radius())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::NodeOutputRadius(value.into()).into()
            },
            "The radius of the circular outputs on nodes",
            1,
        );
        let node_selected_border_width = style.slider(
            StyleEditorFields::NodeSelectedBorderWidth.variant_label(),
            1.0..=10.0,
            ..,
            style.node_selected_border_width(),
            style.node_selected_border_width.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::NodeSelectedBorderWidth(
                    Float::new(style.node_selected_border_width())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::NodeSelectedBorderWidth(value.into()).into()
            },
            "The width of the border that is drawn on selected nodes",
            1,
        );

        let selection_box_border_width = style.slider(
            StyleEditorFields::SelectionBoxBorderWidth.variant_label(),
            0.0..=10.0,
            ..,
            style.selection_box_border_width(),
            style.selection_box_border_width.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::SelectionBoxBorderWidth(
                    Float::new(style.selection_box_border_width())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::SelectionBoxBorderWidth(value.into()).into()
            },
            "The opaque border width of any selection boxes",
            1,
        );
        let selection_box_alpha = style.slider(
            StyleEditorFields::SelectionBoxAlpha.variant_label(),
            0.0..=1.0,
            ..,
            style.selection_box_alpha(),
            style.selection_box_alpha.expression(),
            style.float_input_step(),
            move |expression| -> StyleEditorMessage {
                StyleEditorData::SelectionBoxAlpha(
                    Float::new(style.selection_box_alpha())
                        .with_expression(expression)
                        .into(),
                )
                .into()
            },
            |value| -> StyleEditorMessage {
                StyleEditorData::SelectionBoxAlpha(value.into()).into()
            },
            "The opacity of any selection boxes",
            1,
        );
        iced::widget::scrollable(
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
                viewer_zoom_sensitivity,
                float_input_step,
                // example_colour,
                text_zoom_cutoff,
                connection_line_width,
                inactive_connection_line_scale,
                node_input_radius,
                node_output_radius,
                node_selected_border_width,
                selection_box_border_width,
                selection_box_alpha,
            ]
            .spacing(style.spacing()),
        )
        .into()
    }
}
