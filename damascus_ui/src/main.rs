// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::collections::BTreeMap;

use iced::{
    Center, Element, Fill, Size, Subscription, Task, Theme, Vector,
    keyboard::{
        self,
        key::{self, Key},
    },
    widget::{
        button, center, center_y, column, container, operation,
        pane_grid::{self, Axis, Direction, PaneGrid},
        responsive, row, scrollable, space, text,
    },
    window,
};

use damascus;

use damascus_ui::{icons::Icons, style};

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
        .default_font(iced::Font::MONOSPACE)
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
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    Detach(pane_grid::Pane),
}

impl Panel {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new());

        Panel {
            panes,
            panes_created: 1,
            focus: None,
        }
    }

    fn update(&mut self, message: PanelMessage) {
        match message {
            PanelMessage::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new());

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            PanelMessage::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new());

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
    }

    fn view_content<'a>(
        _pane: pane_grid::Pane,
        _total_panes: usize,
        size: Size,
    ) -> Element<'a, PanelMessage> {
        let content = column![text!("{}x{}", size.width, size.height).size(24)]
            .spacing(3)
            .align_x(Center);

        center_y(scrollable(content)).padding(5).into()
    }

    fn view_controls<'a>(
        pane: pane_grid::Pane,
        total_panes: usize,
        is_maximized: bool,
    ) -> Element<'a, PanelMessage> {
        let horizontal_split = button(Icons::HorizontalSplit.as_svg().width(16).height(16))
            .style(button::secondary)
            .padding(3)
            .on_press(PanelMessage::Split(pane_grid::Axis::Horizontal, pane));

        let vertical_split = button(Icons::VerticalSplit.as_svg().width(16).height(16))
            .style(button::secondary)
            .padding(3)
            .on_press(PanelMessage::Split(pane_grid::Axis::Vertical, pane));

        let maximize = if total_panes > 1 {
            let (content, message) = if is_maximized {
                (Icons::Minimize.as_svg(), PanelMessage::Restore)
            } else {
                (Icons::Maximize.as_svg(), PanelMessage::Maximize(pane))
            };

            Some(
                button(content.width(16).height(16))
                    .style(button::secondary)
                    .padding(3)
                    .on_press(message),
            )
        } else {
            None
        };

        let detach = button(Icons::Detach.as_svg().width(16).height(16))
            .style(button::secondary)
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

        row![horizontal_split, vertical_split, maximize, detach, close]
            .spacing(3)
            .into()
    }

    fn view<'a>(&'a self, preferences: &'a style::Preferences) -> Element<'a, PanelMessage> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, _pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title_bar = pane_grid::TitleBar::new(row![])
                .controls(pane_grid::Controls::dynamic(
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

            pane_grid::Content::new(responsive(move |size| {
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
        .width(Fill)
        .height(Fill)
        .spacing(2)
        .on_click(PanelMessage::Clicked)
        .on_drag(PanelMessage::Dragged)
        .on_resize(5, PanelMessage::Resized);

        container(pane_grid).into()
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel::new()
    }
}

#[derive(Clone, Copy)]
struct Pane {}

impl Pane {
    fn new() -> Self {
        Self {}
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
    preferences: style::Preferences,
    panel: Panel,
}

#[derive(Debug, Clone)]
enum WindowMessage {
    Event(window::Id, window::Event),
    Opened(window::Id),
    Closed(window::Id),
    ScaleChanged(Option<window::Id>, f32),
    TitleChanged(window::Id, String),
    Panel(Option<window::Id>, PanelMessage),
}

impl Window {
    fn new(count: usize) -> Self {
        Self {
            title: format!("damascus-{count}"),
            preferences: style::Preferences::default(),
            panel: Panel::new(),
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
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
            .view(&self.preferences)
            .map(move |panel_message| WindowMessage::Panel(Some(id), panel_message).into())
    }
}

//
//
//
//
//
//
//
// Node graph
//
//
//
//
//
//

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
struct NodeGraph {
    node_graph: damascus::graph::node_graph::NodeGraph,
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

struct WindowManager {
    main_window: Option<window::Id>,
    focused_window_id: Option<window::Id>,
    windows: BTreeMap<window::Id, Window>,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            main_window: None,
            focused_window_id: None,
            windows: BTreeMap::new(),
        }
    }
}

impl WindowManager {
    fn update_window(&mut self, message: WindowMessage) -> Task<Message> {
        match message {
            WindowMessage::Event(id, event) => {
                match event {
                    window::Event::Opened {
                        position: _,
                        size: _,
                    } => {}
                    // window::Event::Closed,
                    // window::Event::Moved(point),
                    // window::Event::Resized(Size),
                    // window::Event::Rescaled(f32),
                    // window::Event::RedrawRequested(Instant),
                    // window::Event::CloseRequested,
                    window::Event::Focused => self.focused_window_id = Some(id),
                    window::Event::Unfocused => {
                        if let Some(focused_window_id) = self.focused_window_id
                            && id == focused_window_id
                        {
                            self.focused_window_id = None;
                        }
                    } // window::Event::FileHovered(PathBuf),
                    // window::Event::FileDropped(PathBuf),
                    // window::Event::FilesHoveredLeft,
                    _ => {}
                };
                println!("{:?}, {:?}", id, event);
                Task::none()
            }
            WindowMessage::Opened(id) => {
                if self.main_window.is_none() {
                    self.main_window = Some(id);
                }

                let window = Window::new(self.windows.len() + 1);
                let focus_input = operation::focus(format!("input-{id}"));

                self.windows.insert(id, window);

                focus_input
            }
            WindowMessage::Closed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else if let Some(main_window) = self.main_window
                    && main_window == id
                {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            WindowMessage::ScaleChanged(window_id, scale) => {
                let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                    return Task::none();
                };

                if let Some(window) = self.windows.get_mut(&id) {
                    window.preferences.scale *= scale;
                }

                Task::none()
            }
            WindowMessage::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }

                Task::none()
            }
            WindowMessage::Panel(window_id, panel_message) => {
                let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                    return Task::none();
                };

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
}

enum NodeGraphMessage {
    None,
}

enum ViewerMessage {
    None,
}

struct Damascus {
    window_manager: WindowManager,
    node_graph: NodeGraph,
}

#[derive(Debug, Clone)]
enum Message {
    Window(WindowMessage),
    NodeGraphMessage,
    ViewerMessage,
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
                window_manager: WindowManager::default(),
                node_graph: NodeGraph::default(),
            },
            open.map(|id| WindowMessage::Opened(id).into()),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.window_manager
            .windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(window_message) => self.window_manager.update_window(window_message),
            _ => {
                todo!("Add the meat")
            }
        }
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.window_manager.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            space().into()
        }
    }

    fn theme(&self, window: window::Id) -> Option<Theme> {
        Some(
            self.window_manager
                .windows
                .get(&window)?
                .preferences
                .theme
                .into(),
        )
    }

    fn scale_factor(&self, window: window::Id) -> f32 {
        self.window_manager
            .windows
            .get(&window)
            .map(|window| window.preferences.scale)
            .unwrap_or(1.0)
    }

    fn handle_ctrl_alt_shift_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_shift_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_alt_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_alt_shift_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            Key::Character("w") => {
                Some(WindowMessage::Panel(None, PanelMessage::CloseFocused).into())
            }
            Key::Character("=") => Some(WindowMessage::ScaleChanged(None, 1.05).into()),
            Key::Character("-") => Some(WindowMessage::ScaleChanged(None, 0.95).into()),
            Key::Named(key) => {
                if let Some(direction) = match key {
                    key::Named::ArrowUp => Some(Direction::Up),
                    key::Named::ArrowDown => Some(Direction::Down),
                    key::Named::ArrowLeft => Some(Direction::Left),
                    key::Named::ArrowRight => Some(Direction::Right),
                    _ => None,
                } {
                    Some(WindowMessage::Panel(None, PanelMessage::FocusAdjacent(direction)).into())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn handle_shift_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_alt_hotkey(key: keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_hotkeys(modifiers: keyboard::Modifiers, key: keyboard::Key) -> Option<Message> {
        let ctrl_down: bool = modifiers.command();
        let shift_down: bool = modifiers.shift();
        let alt_down: bool = modifiers.alt();

        if ctrl_down && shift_down && alt_down {
            Self::handle_ctrl_alt_shift_hotkey(key)
        } else if ctrl_down && shift_down {
            Self::handle_ctrl_shift_hotkey(key)
        } else if ctrl_down && alt_down {
            Self::handle_ctrl_alt_hotkey(key)
        } else if shift_down && alt_down {
            Self::handle_alt_shift_hotkey(key)
        } else if ctrl_down {
            Self::handle_ctrl_hotkey(key)
        } else if alt_down {
            Self::handle_alt_hotkey(key)
        } else if shift_down {
            Self::handle_shift_hotkey(key)
        } else {
            None
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            std::iter::once(keyboard::listen().filter_map(|event| {
                let keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
                    return None;
                };

                Self::handle_hotkeys(modifiers, key)
            }))
            .chain(std::iter::once(
                window::close_events().map(|window_id| WindowMessage::Closed(window_id).into()),
            ))
            .chain(std::iter::once(
                window::events().map(|(id, event)| WindowMessage::Event(id, event).into()),
            )),
        )
    }
}
