// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{collections::BTreeMap, fmt::Debug};

use iced;
use macro_rules_attribute::derive;

use crate::{
    EnumTraits,
    app::Context,
    icons::Icons,
    widgets::{Widget, style::Style},
};

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
struct Pane {}

impl Default for Pane {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PanelMessage {
    Split(iced::widget::pane_grid::Axis, iced::widget::pane_grid::Pane),
    SplitFocused(iced::widget::pane_grid::Axis),
    FocusAdjacent(iced::widget::pane_grid::Direction),
    Clicked(iced::widget::pane_grid::Pane),
    Dragged(iced::widget::pane_grid::DragEvent),
    Resized(iced::widget::pane_grid::ResizeEvent),
    Maximize(iced::widget::pane_grid::Pane),
    Restore,
    Close(iced::widget::pane_grid::Pane),
    CloseFocused,
    Detach(iced::widget::pane_grid::Pane),
}

#[derive(Default, EnumTraits!)]
pub enum PanelAxis {
    #[default]
    Horizontal,
    Vertical,
}

impl From<iced::widget::pane_grid::Axis> for PanelAxis {
    fn from(axis: iced::widget::pane_grid::Axis) -> Self {
        match axis {
            iced::widget::pane_grid::Axis::Horizontal => Self::Horizontal,
            iced::widget::pane_grid::Axis::Vertical => Self::Vertical,
        }
    }
}

impl From<PanelAxis> for iced::widget::pane_grid::Axis {
    fn from(axis: PanelAxis) -> Self {
        match axis {
            PanelAxis::Horizontal => Self::Horizontal,
            PanelAxis::Vertical => Self::Vertical,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(bound = "T: Clone + Debug + Default + for<'a> serde::Deserialize<'a> + serde::Serialize")]
pub enum PanelContext<T>
where
    T: Clone + Debug + Default + for<'a> serde::Deserialize<'a> + serde::Serialize,
{
    Split {
        axis: PanelAxis,
        ratio: f32,
        a: Box<PanelContext<T>>,
        b: Box<PanelContext<T>>,
    },
    Pane(T),
}

impl<T> PanelContext<T>
where
    T: Clone + Debug + Default + for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    pub fn from_node(
        node: &iced::widget::pane_grid::Node,
        panes: &BTreeMap<iced::widget::pane_grid::Pane, T>,
    ) -> Self {
        match node {
            iced::widget::pane_grid::Node::Split {
                id: _,
                axis,
                ratio,
                a,
                b,
            } => Self::Split {
                axis: (*axis).into(),
                ratio: *ratio,
                a: Box::new(Self::from_node(a, panes)),
                b: Box::new(Self::from_node(b, panes)),
            },
            iced::widget::pane_grid::Node::Pane(pane) => match panes.get(&pane) {
                Some(state) => Self::Pane(state.clone()),
                None => Self::Pane(T::default()),
            },
        }
    }
}

impl<T> From<PanelContext<T>> for iced::widget::pane_grid::Configuration<T>
where
    T: Clone + Debug + Default + for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    fn from(panel_context: PanelContext<T>) -> Self {
        match panel_context {
            PanelContext::Split { axis, ratio, a, b } => Self::Split {
                axis: axis.into(),
                ratio: ratio,
                a: Box::new((*a).into()),
                b: Box::new((*b).into()),
            },
            PanelContext::Pane(state) => Self::Pane(state),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Panel {
    focus: Option<iced::widget::pane_grid::Pane>,
    panes: iced::widget::pane_grid::State<Pane>,
}

impl serde::Serialize for Panel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_context().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Panel {
    fn deserialize<D>(deserializer: D) -> Result<Panel, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match PanelContext::deserialize(deserializer) {
            Ok(context) => Ok(Self::from_context(context)),
            Err(error) => Err(error),
        }
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel::new()
    }
}

impl Panel {
    fn as_context(&self) -> PanelContext<Pane> {
        PanelContext::from_node(self.panes.layout(), &self.panes.panes)
    }

    fn from_context(context: PanelContext<Pane>) -> Self {
        Self {
            focus: None,
            panes: iced::widget::pane_grid::State::with_configuration(
                <PanelContext<Pane> as Into<iced::widget::pane_grid::Configuration<Pane>>>::into(
                    context,
                ),
            ),
        }
    }

    fn view_content<'a>(
        style: &'a Style,
        _pane: iced::widget::pane_grid::Pane,
        _total_panes: usize,
        size: iced::Size,
    ) -> iced::Element<'a, PanelMessage> {
        let content =
            iced::widget::column![style.text(format!("{:}x{:}", size.width, size.height))]
                .spacing(style.spacing)
                .align_x(iced::Center);

        iced::widget::center_y(iced::widget::scrollable(content))
            .padding(style.padding)
            .into()
    }

    fn view_controls<'a>(
        style: &'a Style,
        pane: iced::widget::pane_grid::Pane,
        total_panes: usize,
        is_maximized: bool,
    ) -> iced::Element<'a, PanelMessage> {
        let horizontal_split = iced::widget::button(
            Icons::HorizontalSplit
                .as_svg()
                .width(style.icon_size)
                .height(style.icon_size),
        )
        .style(iced::widget::button::secondary)
        .padding(style.padding)
        .on_press(PanelMessage::Split(
            iced::widget::pane_grid::Axis::Horizontal,
            pane,
        ));

        let vertical_split = iced::widget::button(
            Icons::VerticalSplit
                .as_svg()
                .width(style.icon_size)
                .height(style.icon_size),
        )
        .style(iced::widget::button::secondary)
        .padding(style.padding)
        .on_press(PanelMessage::Split(
            iced::widget::pane_grid::Axis::Vertical,
            pane,
        ));

        let maximize = if total_panes > 1 {
            Some(
                style
                    .maximize_button(is_maximized)
                    .on_press(if is_maximized {
                        PanelMessage::Restore
                    } else {
                        PanelMessage::Maximize(pane)
                    }),
            )
        } else {
            None
        };

        let detach = style.detach_button().on_press_maybe(if total_panes > 1 {
            Some(PanelMessage::Detach(pane))
        } else {
            None
        });

        let close = style.close_button().on_press_maybe(if total_panes > 1 {
            Some(PanelMessage::Close(pane))
        } else {
            None
        });

        iced::widget::row![horizontal_split, vertical_split, maximize, detach, close,]
            .spacing(style.spacing)
            .into()
    }
}

impl Widget<PanelMessage> for Panel {
    fn new() -> Self {
        let (panes, _) = iced::widget::pane_grid::State::new(Pane::default());

        Self { focus: None, panes }
    }

    fn update(
        &mut self,
        _context: &mut Context,
        message: PanelMessage,
    ) -> iced::Task<PanelMessage> {
        match message {
            PanelMessage::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::default());

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
            }
            PanelMessage::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::default());

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }
                }
            }
            PanelMessage::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus
                    && let Some(adjacent) = self.panes.adjacent(pane, direction)
                {
                    self.focus = Some(adjacent);
                }
            }
            PanelMessage::Clicked(pane) => {
                self.focus = Some(pane);
            }
            PanelMessage::Resized(iced::widget::pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            PanelMessage::Dragged(iced::widget::pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            PanelMessage::Dragged(_drag_event) => {}
            PanelMessage::Maximize(pane) => self.panes.maximize(pane),
            PanelMessage::Restore => {
                self.panes.restore();
            }
            PanelMessage::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            PanelMessage::CloseFocused => {
                if let Some(pane) = self.focus
                    && let Some((_, sibling)) = self.panes.close(pane)
                {
                    self.focus = Some(sibling);
                }
            }
            PanelMessage::Detach(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
        }

        iced::Task::none()
    }

    fn view<'a>(
        &'a self,
        _window_id: iced::window::Id,
        style: &'a Style,
    ) -> iced::Element<'a, PanelMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |id, _pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title_bar = iced::widget::pane_grid::TitleBar::new(iced::widget::row![])
                .controls(iced::widget::pane_grid::Controls::dynamic(
                    Self::view_controls(style, id, total_panes, is_maximized),
                    style.close_button().on_press_maybe(if total_panes > 1 {
                        Some(PanelMessage::Close(id))
                    } else {
                        None
                    }),
                ))
                .padding(style.padding)
                .style(move |_theme| {
                    if is_focused {
                        style.title_bar_focused()
                    } else {
                        style.title_bar()
                    }
                });

            iced::widget::pane_grid::Content::new(iced::widget::responsive(move |size| {
                Self::view_content(style, id, total_panes, size)
            }))
            .title_bar(title_bar)
            .style(move |_theme| {
                if is_focused {
                    style.pane_focused()
                } else {
                    style.pane()
                }
            })
        })
        .width(iced::Fill)
        .height(iced::Fill)
        .spacing(style.spacing)
        .on_click(PanelMessage::Clicked)
        .on_drag(PanelMessage::Dragged)
        .on_resize(style.leeway, PanelMessage::Resized);

        iced::widget::container(pane_grid).into()
    }
}
