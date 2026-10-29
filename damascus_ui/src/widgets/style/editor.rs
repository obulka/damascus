// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use glam::Vec4;
use iced;
use macro_rules_attribute::derive;
use strum::Display;

use damascus::Enumerator;

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
    DefaultNodeColour,
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
            Self::ViewerZoomSensitivity => {
                style.viewer_zoom_sensitivity += style.viewer_zoom_sensitivity
            }
            Self::TextZoomCutoff => style.text_zoom_cutoff += style.text_zoom_cutoff,
            Self::ConnectionLineWidth => style.connection_line_width += style.connection_line_width,
            Self::InactiveConnectionLineScale => {
                style.inactive_connection_line_scale += style.inactive_connection_line_scale
            }
            Self::NodeInputRadius => style.node_input_radius += style.node_input_radius,
            Self::NodeOutputRadius => style.node_output_radius += style.node_output_radius,
            Self::NodeSelectedBorderWidth => {
                style.node_selected_border_width += style.node_selected_border_width
            }
            Self::SelectionBoxBorderWidth => {
                style.selection_box_border_width += style.selection_box_border_width
            }
            Self::SelectionBoxAlpha => style.selection_box_alpha += style.selection_box_alpha,
            _ => {}
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
            Self::ViewerZoomSensitivity => {
                style.viewer_zoom_sensitivity -= style.viewer_zoom_sensitivity
            }
            Self::TextZoomCutoff => style.text_zoom_cutoff -= style.text_zoom_cutoff,
            Self::ConnectionLineWidth => style.connection_line_width -= style.connection_line_width,
            Self::InactiveConnectionLineScale => {
                style.inactive_connection_line_scale -= style.inactive_connection_line_scale
            }
            Self::NodeInputRadius => style.node_input_radius -= style.node_input_radius,
            Self::NodeOutputRadius => style.node_output_radius -= style.node_output_radius,
            Self::NodeSelectedBorderWidth => {
                style.node_selected_border_width -= style.node_selected_border_width
            }
            Self::SelectionBoxBorderWidth => {
                style.selection_box_border_width -= style.selection_box_border_width
            }
            Self::SelectionBoxAlpha => style.selection_box_alpha -= style.selection_box_alpha,
            _ => {}
        }
    }

    fn toggle(&self, style: &mut Style) {
        match self {
            Self::DefaultNodeColour => {
                style.picking_default_node_colour = !style.picking_default_node_colour;
            }
            _ => {}
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
    ViewerZoomSensitivity(f32),
    DefaultNodeColour(Vec4),
    TextZoomCutoff(f32),
    ConnectionLineWidth(f32),
    InactiveConnectionLineScale(f32),
    NodeInputRadius(f32),
    NodeOutputRadius(f32),
    NodeSelectedBorderWidth(f32),
    SelectionBoxBorderWidth(f32),
    SelectionBoxAlpha(f32),
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
            Self::ViewerZoomSensitivity(viewer_zoom_sensitivity) => {
                style.viewer_zoom_sensitivity = *viewer_zoom_sensitivity
            }
            Self::DefaultNodeColour(colour) => {
                style.default_node_colour = *colour;
                style.picking_default_node_colour = !style.picking_default_node_colour;
            }
            Self::TextZoomCutoff(text_zoom_cutoff) => style.text_zoom_cutoff = *text_zoom_cutoff,
            Self::ConnectionLineWidth(connection_line_width) => {
                style.connection_line_width = *connection_line_width
            }
            Self::InactiveConnectionLineScale(inactive_connection_line_scale) => {
                style.inactive_connection_line_scale = *inactive_connection_line_scale
            }
            Self::NodeInputRadius(node_input_radius) => {
                style.node_input_radius = *node_input_radius
            }
            Self::NodeOutputRadius(node_output_radius) => {
                style.node_output_radius = *node_output_radius
            }
            Self::NodeSelectedBorderWidth(node_selected_border_width) => {
                style.node_selected_border_width = *node_selected_border_width
            }
            Self::SelectionBoxBorderWidth(selection_box_border_width) => {
                style.selection_box_border_width = *selection_box_border_width
            }
            Self::SelectionBoxAlpha(selection_box_alpha) => {
                style.selection_box_alpha = *selection_box_alpha
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum StyleEditorMessage {
    Increment(StyleEditorFields),
    Decrement(StyleEditorFields),
    Set(StyleEditorData),
    Toggle(StyleEditorFields),
    Error(StyleEditorErrors),
    StyleChanged,
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
            style.text_size,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::TextSize(value).into() },
            StyleEditorData::TextSize(style.text_size).into(),
            "The default global text size",
        );
        let scale = style.slider(
            StyleEditorFields::Scale.variant_label(),
            0.25..=5.0,
            style.float_input_step..,
            style.scale,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Scale(value).into() },
            StyleEditorData::Scale(style.scale).into(),
            "Scale the UI up or down",
        );
        let border_width = style.slider(
            StyleEditorFields::BorderWidth.variant_label(),
            0.0..=10.0,
            ..,
            style.border_width,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::BorderWidth(value).into() },
            StyleEditorData::BorderWidth(style.border_width).into(),
            "The width of borders in the UI",
        );
        let spacing = style.slider(
            StyleEditorFields::Spacing.variant_label(),
            0.0..=10.0,
            ..,
            style.spacing,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Spacing(value).into() },
            StyleEditorData::Spacing(style.spacing).into(),
            "The spacing between UI elements",
        );
        let padding = style.slider(
            StyleEditorFields::Padding.variant_label(),
            0.0..=10.0,
            ..,
            style.padding,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::Padding(value).into() },
            StyleEditorData::Padding(style.padding).into(),
            "The padding around UI elements",
        );
        let icon_size = style.slider(
            StyleEditorFields::IconSize.variant_label(),
            4..=36,
            ..,
            style.icon_size,
            1,
            |value| -> StyleEditorMessage { StyleEditorData::IconSize(value).into() },
            StyleEditorData::IconSize(style.icon_size).into(),
            "The size of icon buttons",
        );
        let leeway = style.slider(
            StyleEditorFields::Leeway.variant_label(),
            1.0..=100.0,
            ..,
            style.leeway,
            1.,
            |value| -> StyleEditorMessage { StyleEditorData::Leeway(value).into() },
            StyleEditorData::Leeway(style.leeway).into(),
            "The tolerance around grabbable elements",
        );
        let modal_opacity = style.slider(
            StyleEditorFields::ModalOpacity.variant_label(),
            0.0..=1.0,
            ..,
            style.modal_opacity,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::ModalOpacity(value).into() },
            StyleEditorData::ModalOpacity(style.modal_opacity).into(),
            "The opacity of the background when in a modal context",
        );
        let viewer_zoom_sensitivity = style.slider(
            StyleEditorFields::ViewerZoomSensitivity.variant_label(),
            1e-3..=2.0,
            ..,
            style.viewer_zoom_sensitivity,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::ViewerZoomSensitivity(value).into() },
            StyleEditorData::ViewerZoomSensitivity(style.viewer_zoom_sensitivity).into(),
            "The sensitivity of the mouse scroll wheel when zooming in the viewer",
        );
        let float_input_step = style.slider(
            StyleEditorFields::FloatInputStep.variant_label(),
            1e-32..=1e-1,
            ..,
            style.float_input_step,
            1e-32,
            |value| -> StyleEditorMessage { StyleEditorData::FloatInputStep(value).into() },
            StyleEditorData::FloatInputStep(style.float_input_step).into(),
            "The minimum step between floating point numbers when dragging sliders",
        );
        let default_node_colour = style.colour_picker(
            StyleEditorFields::DefaultNodeColour.variant_label(),
            style.picking_default_node_colour,
            iced::Color::from_rgba(
                style.default_node_colour.x,
                style.default_node_colour.y,
                style.default_node_colour.z,
                style.default_node_colour.w,
            ),
            StyleEditorMessage::Toggle(StyleEditorFields::DefaultNodeColour),
            |colour| -> StyleEditorMessage {
                StyleEditorData::DefaultNodeColour(Vec4::new(
                    colour.r, colour.g, colour.b, colour.a,
                ))
                .into()
            },
            "The default colour of nodes",
        );

        let text_zoom_cutoff = style.slider(
            StyleEditorFields::TextZoomCutoff.variant_label(),
            1.0..=10.,
            ..,
            style.text_zoom_cutoff,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::TextZoomCutoff(value).into() },
            StyleEditorData::TextZoomCutoff(style.text_zoom_cutoff).into(),
            "If you zoom out past this value, text will stop rendering in the node graph",
        );
        let connection_line_width = style.slider(
            StyleEditorFields::ConnectionLineWidth.variant_label(),
            0.5..=10.,
            ..,
            style.connection_line_width,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::ConnectionLineWidth(value).into() },
            StyleEditorData::ConnectionLineWidth(style.connection_line_width).into(),
            "The width of the lines representing connections between nodes",
        );
        let inactive_connection_line_scale = style.slider(
            StyleEditorFields::InactiveConnectionLineScale.variant_label(),
            0.1..=1.0,
            ..,
            style.inactive_connection_line_scale,
            style.float_input_step,
            |value| -> StyleEditorMessage {
                StyleEditorData::InactiveConnectionLineScale(value).into()
            },
            StyleEditorData::InactiveConnectionLineScale(style.inactive_connection_line_scale)
                .into(),
            "The width of lines representing the inactive viewport connections",
        );
        let node_input_radius = style.slider(
            StyleEditorFields::NodeInputRadius.variant_label(),
            1.0..=10.0,
            ..,
            style.node_input_radius,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::NodeInputRadius(value).into() },
            StyleEditorData::NodeInputRadius(style.node_input_radius).into(),
            "The radius of the circular inputs on nodes",
        );
        let node_output_radius = style.slider(
            StyleEditorFields::NodeOutputRadius.variant_label(),
            1.0..=10.0,
            ..,
            style.node_output_radius,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::NodeOutputRadius(value).into() },
            StyleEditorData::NodeOutputRadius(style.node_output_radius).into(),
            "The radius of the circular outputs on nodes",
        );
        let node_selected_border_width = style.slider(
            StyleEditorFields::NodeSelectedBorderWidth.variant_label(),
            1.0..=10.0,
            ..,
            style.node_selected_border_width,
            style.float_input_step,
            |value| -> StyleEditorMessage {
                StyleEditorData::NodeSelectedBorderWidth(value).into()
            },
            StyleEditorData::NodeSelectedBorderWidth(style.node_selected_border_width).into(),
            "The width of the border that is drawn on selected nodes",
        );

        let selection_box_border_width = style.slider(
            StyleEditorFields::SelectionBoxBorderWidth.variant_label(),
            0.0..=10.0,
            ..,
            style.selection_box_border_width,
            style.float_input_step,
            |value| -> StyleEditorMessage {
                StyleEditorData::SelectionBoxBorderWidth(value).into()
            },
            StyleEditorData::SelectionBoxBorderWidth(style.selection_box_border_width).into(),
            "The opaque border width of any selection boxes",
        );
        let selection_box_alpha = style.slider(
            StyleEditorFields::SelectionBoxAlpha.variant_label(),
            0.0..=1.0,
            ..,
            style.selection_box_alpha,
            style.float_input_step,
            |value| -> StyleEditorMessage { StyleEditorData::SelectionBoxAlpha(value).into() },
            StyleEditorData::SelectionBoxAlpha(style.selection_box_alpha).into(),
            "The opacity of any selection boxes",
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
                default_node_colour,
                text_zoom_cutoff,
                connection_line_width,
                inactive_connection_line_scale,
                node_input_radius,
                node_output_radius,
                node_selected_border_width,
                selection_box_border_width,
                selection_box_alpha,
            ]
            .spacing(style.spacing),
        )
        .into()
    }
}
