// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use iced;

use crate::{icons::Icons, widgets::style};

#[derive(Clone, Copy)]
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

pub struct Panel {
    panes: iced::widget::pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<iced::widget::pane_grid::Pane>,
}

impl Default for Panel {
    fn default() -> Self {
        Panel::new()
    }
}

impl Panel {
    pub fn new() -> Self {
        let (panes, _) = iced::widget::pane_grid::State::new(Pane::default());

        Self {
            panes,
            panes_created: 1,
            focus: None,
        }
    }

    pub fn update(&mut self, message: PanelMessage) -> iced::Task<PanelMessage> {
        match message {
            PanelMessage::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::default());

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            PanelMessage::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::default());

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
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

    fn view_content<'a>(
        _pane: iced::widget::pane_grid::Pane,
        _total_panes: usize,
        size: iced::Size,
    ) -> iced::Element<'a, PanelMessage> {
        let content =
            iced::widget::column![iced::widget::text!("{}x{}", size.width, size.height).size(24)]
                .spacing(3)
                .align_x(iced::Center);

        iced::widget::center_y(iced::widget::scrollable(content))
            .padding(5)
            .into()
    }

    fn view_controls<'a>(
        pane: iced::widget::pane_grid::Pane,
        total_panes: usize,
        is_maximized: bool,
    ) -> iced::Element<'a, PanelMessage> {
        let horizontal_split =
            iced::widget::button(Icons::HorizontalSplit.as_svg().width(16).height(16))
                .style(iced::widget::button::secondary)
                .padding(3)
                .on_press(PanelMessage::Split(
                    iced::widget::pane_grid::Axis::Horizontal,
                    pane,
                ));

        let vertical_split =
            iced::widget::button(Icons::VerticalSplit.as_svg().width(16).height(16))
                .style(iced::widget::button::secondary)
                .padding(3)
                .on_press(PanelMessage::Split(
                    iced::widget::pane_grid::Axis::Vertical,
                    pane,
                ));

        let maximize = if total_panes > 1 {
            let (content, message) = if is_maximized {
                (Icons::Minimize.as_svg(), PanelMessage::Restore)
            } else {
                (Icons::Maximize.as_svg(), PanelMessage::Maximize(pane))
            };

            Some(
                iced::widget::button(content.width(16).height(16))
                    .style(iced::widget::button::secondary)
                    .padding(3)
                    .on_press(message),
            )
        } else {
            None
        };

        let detach = iced::widget::button(Icons::Detach.as_svg().width(16).height(16))
            .style(iced::widget::button::secondary)
            .padding(3)
            .on_press_maybe(if total_panes > 1 {
                Some(PanelMessage::Detach(pane))
            } else {
                None
            });

        let close = style::close_button().on_press_maybe(if total_panes > 1 {
            Some(PanelMessage::Close(pane))
        } else {
            None
        });

        iced::widget::row![horizontal_split, vertical_split, maximize, detach, close]
            .spacing(3)
            .into()
    }

    pub fn view<'a>(
        &'a self,
        preferences: &'a style::Preferences,
    ) -> iced::Element<'a, PanelMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |id, _pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title_bar = iced::widget::pane_grid::TitleBar::new(iced::widget::row![])
                .controls(iced::widget::pane_grid::Controls::dynamic(
                    Self::view_controls(id, total_panes, is_maximized),
                    style::close_button().on_press_maybe(if total_panes > 1 {
                        Some(PanelMessage::Close(id))
                    } else {
                        None
                    }),
                ))
                .padding(3)
                .style(move |_theme| {
                    if is_focused {
                        style::title_bar_focused(preferences)
                    } else {
                        style::title_bar(preferences)
                    }
                });

            iced::widget::pane_grid::Content::new(iced::widget::responsive(move |size| {
                Self::view_content(id, total_panes, size)
            }))
            .title_bar(title_bar)
            .style(move |_theme| {
                if is_focused {
                    style::pane_focused(preferences)
                } else {
                    style::pane(preferences)
                }
            })
        })
        .width(iced::Fill)
        .height(iced::Fill)
        .spacing(2)
        .on_click(PanelMessage::Clicked)
        .on_drag(PanelMessage::Dragged)
        .on_resize(5, PanelMessage::Resized);

        iced::widget::container(pane_grid).into()
    }
}
