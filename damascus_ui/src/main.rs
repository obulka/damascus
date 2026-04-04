// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::collections::BTreeMap;

use iced::keyboard;
use iced::widget::{
    button, center, center_x, center_y, column, container, operation,
    pane_grid::{self, PaneGrid},
    responsive, row, scrollable, space, text, text_input,
};
use iced::window;
use iced::{Center, Color, Element, Fill, Size, Subscription, Task, Theme, Vector};

//
//
//
//
//
//
//
// App run
//
//
//
//
//
//

fn main() -> iced::Result {
    iced::daemon(Damascus::new, Damascus::update, Damascus::view)
        .subscription(Damascus::subscription)
        .title(Damascus::title)
        .theme(Damascus::theme)
        .scale_factor(Damascus::scale_factor)
        .run()
}

//
//
//
//
//
//
//
// Common window contents
//
//
//
//
//
//

struct Panel {
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone, Copy)]
enum PanelMessage {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    Detach(pane_grid::Pane),
}

impl Panel {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));

        Panel {
            panes,
            panes_created: 1,
            focus: None,
        }
    }

    fn update(&mut self, message: PanelMessage) {
        match message {
            PanelMessage::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            PanelMessage::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

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
            PanelMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            PanelMessage::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            PanelMessage::Dragged(_) => {}
            PanelMessage::TogglePin(pane) => {
                if let Some(Pane { is_pinned, .. }) = self.panes.get_mut(pane) {
                    *is_pinned = !*is_pinned;
                }
            }
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
                    && let Some(Pane { is_pinned, .. }) = self.panes.get(pane)
                    && !is_pinned
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
    }

    fn subscription(&self) -> Subscription<PanelMessage> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
                return None;
            };

            if !modifiers.command() {
                return None;
            }

            handle_hotkey(key)
        })
    }

    fn view(&self) -> Element<'_, PanelMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = focus == Some(id);

            let pin_button = button(text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14))
                .on_press(PanelMessage::TogglePin(id))
                .padding(3);

            let title = row![
                pin_button,
                "Pane",
                text(pane.id.to_string()).color(if is_focused {
                    PANE_ID_COLOR_FOCUSED
                } else {
                    PANE_ID_COLOR_UNFOCUSED
                }),
            ]
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .controls(pane_grid::Controls::dynamic(
                    view_controls(id, total_panes, pane.is_pinned, is_maximized),
                    button(text("X").size(14))
                        .style(button::danger)
                        .padding(3)
                        .on_press_maybe(if total_panes > 1 && !pane.is_pinned {
                            Some(PanelMessage::Close(id))
                        } else {
                            None
                        }),
                ))
                .padding(3)
                .style(if is_focused {
                    style::title_bar_focused
                } else {
                    style::title_bar_active
                });

            pane_grid::Content::new(responsive(move |size| {
                view_content(id, total_panes, pane.is_pinned, size)
            }))
            .title_bar(title_bar)
            .style(if is_focused {
                style::pane_focused
            } else {
                style::pane_active
            })
        })
        .width(Fill)
        .height(Fill)
        .on_click(PanelMessage::Clicked)
        .on_drag(PanelMessage::Dragged)
        .on_resize(10, PanelMessage::Resized);

        container(pane_grid).into()
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel::new()
    }
}

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

fn handle_hotkey(key: keyboard::Key) -> Option<PanelMessage> {
    use keyboard::key::{self, Key};
    use pane_grid::{Axis, Direction};

    match key.as_ref() {
        Key::Character("v") => Some(PanelMessage::SplitFocused(Axis::Vertical)),
        Key::Character("h") => Some(PanelMessage::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(PanelMessage::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(PanelMessage::FocusAdjacent)
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, PanelMessage> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = column![
        button(
            "Split horizontally",
            PanelMessage::Split(pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split vertically",
            PanelMessage::Split(pane_grid::Axis::Vertical, pane),
        ),
        if total_panes > 1 && !is_pinned {
            Some(button("Close", PanelMessage::Close(pane)).style(button::danger))
        } else {
            None
        }
    ]
    .spacing(5)
    .max_width(160);

    let content = column![text!("{}x{}", size.width, size.height).size(24), controls,]
        .spacing(3)
        .align_x(Center);

    center_y(scrollable(content)).padding(5).into()
}

fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, PanelMessage> {
    let maximize = if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", PanelMessage::Restore)
        } else {
            ("Maximize", PanelMessage::Maximize(pane))
        };

        Some(
            button(text(content).size(14))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    };

    let detach = button(text("Detach").size(14))
        .style(button::secondary)
        .padding(3)
        .on_press(PanelMessage::Detach(pane));

    let close = button(text("Close").size(14))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if total_panes > 1 && !is_pinned {
            Some(PanelMessage::Close(pane))
        } else {
            None
        });

    row![maximize, detach, close].spacing(5).into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}

//
//
//
//
//
//
//
// Individual window
//
//
//
//
//
//

struct Window {
    title: String,
    scale_input: String,
    current_scale: f32,
    theme: Theme,
    panel: Panel,
}

#[derive(Debug, Clone)]
enum WindowMessage {
    Open,
    Opened(window::Id),
    Closed(window::Id),
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String),
    TitleChanged(window::Id, String),
    Panel(window::Id, PanelMessage),
}

impl Window {
    fn new(count: usize) -> Self {
        Self {
            title: format!("damascus-{count}"),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: Theme::ALL[count % Theme::ALL.len()].clone(),
            panel: Panel::new(),
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        // let scale_input = column![
        //     text("Window scale factor:"),
        //     text_input("Window Scale", &self.scale_input)
        //         .on_input(move |scale| WindowMessage::ScaleInputChanged(id, scale).into())
        //         .on_submit(WindowMessage::ScaleChanged(id, self.scale_input.to_string()).into())
        // ];

        // let title_input = column![
        //     text("Window title:"),
        //     text_input("Window Title", &self.title)
        //         .on_input(move |title| WindowMessage::TitleChanged(id, title).into())
        //         .id(format!("input-{id}"))
        // ];

        // let new_window_button = button(text("New Window")).on_press(WindowMessage::Open.into());

        // let content = column![
        //     scale_input,
        //     title_input,
        //     new_window_button,
        //     self.panel
        //         .view()
        //         .map(move |panel_message| WindowMessage::Panel(id, panel_message).into())
        // ]
        // .spacing(50)
        // .width(Fill)
        // .align_x(Center)
        // .width(200);

        self.panel
            .view()
            .map(move |panel_message| WindowMessage::Panel(id, panel_message).into())
    }
}

//
//
//
//
//
//
//
// Main App
//
//
//
//
//
//

struct Damascus {
    windows: BTreeMap<window::Id, Window>,
}

#[derive(Debug, Clone)]
enum Message {
    Window(WindowMessage),
}

impl From<WindowMessage> for Message {
    fn from(window_message: WindowMessage) -> Self {
        Self::Window(window_message)
    }
}

impl Damascus {
    fn new() -> (Self, Task<Message>) {
        let (_, open) = window::open(window::Settings::default());

        (
            Self {
                windows: BTreeMap::new(),
            },
            open.map(|id| WindowMessage::Opened(id).into()),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    fn update_window(&mut self, message: WindowMessage) -> Task<Message> {
        match message {
            WindowMessage::Open => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::position(*last_window)
                    .then(|last_position| {
                        let position =
                            last_position.map_or(window::Position::Default, |last_position| {
                                window::Position::Specific(last_position + Vector::new(20.0, 20.0))
                            });

                        let (_, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });

                        open
                    })
                    .map(|id| WindowMessage::Opened(id).into())
            }
            WindowMessage::Opened(id) => {
                let window = Window::new(self.windows.len() + 1);
                let focus_input = operation::focus(format!("input-{id}"));

                self.windows.insert(id, window);

                focus_input
            }
            WindowMessage::Closed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            WindowMessage::ScaleInputChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale_input = scale;
                }

                Task::none()
            }
            WindowMessage::ScaleChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.current_scale = scale
                        .parse()
                        .unwrap_or(window.current_scale)
                        .clamp(0.5, 5.0);
                }

                Task::none()
            }
            WindowMessage::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }

                Task::none()
            }
            WindowMessage::Panel(id, panel_message) => {
                let task: Task<Message> = match panel_message {
                    PanelMessage::Detach(_) => window::position(id)
                        .then(|last_position| {
                            let position =
                                last_position.map_or(window::Position::Default, |last_position| {
                                    window::Position::Specific(
                                        last_position + Vector::new(20.0, 20.0),
                                    )
                                });

                            let (_, open) = window::open(window::Settings {
                                position,
                                ..window::Settings::default()
                            });

                            open
                        })
                        .map(|id| WindowMessage::Opened(id).into()),
                    _ => Task::none(),
                };
                if let Some(window) = self.windows.get_mut(&id) {
                    window.panel.update(panel_message);
                }

                task
            }
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(window_message) => self.update_window(window_message),
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            space().into()
        }
    }

    fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(self.windows.get(&window)?.theme.clone())
    }

    fn scale_factor(&self, window: window::Id) -> f32 {
        self.windows
            .get(&window)
            .map(|window| window.current_scale)
            .unwrap_or(1.0)
    }

    fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(|id| WindowMessage::Closed(id).into())
    }
}
