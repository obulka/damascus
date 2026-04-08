// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::BTreeMap;

use iced;

use super::window::{Window, WindowMessage};
use crate::widgets::{panel::PanelMessage, toolbar::ToolbarMessage};

pub enum WindowManagerMessage {
    Toolbar(ToolbarMessage),
    Window(WindowMessage),
}

pub struct WindowManager {
    pub toolbar: Toolbar,
    pub main_window_id: Option<iced::window::Id>,
    pub focused_window_id: Option<iced::window::Id>,
    pub windows: BTreeMap<iced::window::Id, Window>,
}

impl From<WindowManagerMessage> for WindowMessage {
    fn from(window_message: WindowMessage) -> Self {
        Self::Window(window_message)
    }
}

impl From<WindowManagerMessage> for ToolbarMessage {
    fn from(toolbar_message: ToolbarMessage) -> Self {
        Self::Toolbar(toolbar_message)
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            main_window_id: None,
            focused_window_id: None,
            windows: BTreeMap::new(),
        }
    }
}

impl WindowManager {
    pub fn new() -> (Self, iced::Task<WindowManagerMessage>) {
        let (_, open) = iced::window::open(iced::window::Settings::default());

        (Self::default(), open.map(|id| WindowMessage::Opened(id)))
    }

    pub fn update(&mut self, message: WindowManagerMessage) -> iced::Task<WindowManagerMessage> {
        match message {
            WindowMessage::Toolbar(toolbar_message) => self.toolbar.update(toolbar_message),
            WindowMessage::Window(window_message) => {
                match message {
                    WindowMessage::Event(id, event) => {
                        match event {
                            // iced::window::Event::Opened {
                            //     position: _,
                            //     size: _,
                            // } => {}
                            // iced::window::Event::Closed,
                            // iced::window::Event::Moved(point),
                            // iced::window::Event::Resized(Size),
                            // iced::window::Event::Rescaled(f32),
                            // iced::window::Event::RedrawRequested(Instant),
                            // iced::window::Event::CloseRequested,
                            iced::window::Event::Focused => self.focused_window_id = Some(id),
                            iced::window::Event::Unfocused => {
                                if let Some(focused_window_id) = self.focused_window_id
                                    && id == focused_window_id
                                {
                                    self.focused_window_id = None;
                                }
                            } // iced::window::Event::FileHovered(PathBuf),
                            // iced::window::Event::FileDropped(PathBuf),
                            // iced::window::Event::FilesHoveredLeft,
                            _ => {}
                        };
                        // println!("{:?}, {:?}", id, event);
                        iced::Task::none()
                    }
                    WindowMessage::Opened(id) => {
                        if self.main_window_id.is_none() {
                            self.main_window_id = Some(id);
                        }

                        let window = Window::new(self.windows.len() + 1);
                        let focus_input = iced::widget::operation::focus(format!("input-{id}"));

                        self.windows.insert(id, window);

                        focus_input
                    }
                    WindowMessage::Closed(id) => {
                        self.windows.remove(&id);

                        if self.windows.is_empty() {
                            iced::exit()
                        } else if let Some(main_window_id) = self.main_window_id
                            && main_window_id == id
                        {
                            iced::exit()
                        } else {
                            iced::Task::none()
                        }
                    }
                    WindowMessage::ScaleChanged(window_id, scale) => {
                        let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                            return iced::Task::none();
                        };

                        if let Some(window) = self.windows.get_mut(&id) {
                            window.preferences.scale *= scale;
                        }

                        iced::Task::none()
                    }
                    WindowMessage::TitleChanged(id, title) => {
                        if let Some(window) = self.windows.get_mut(&id) {
                            window.title = title;
                        }

                        iced::Task::none()
                    }
                    WindowMessage::Panel(window_id, panel_message) => {
                        let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                            return iced::Task::none();
                        };

                        let task: iced::Task<WindowMessage> = match panel_message {
                            PanelMessage::Detach(_) => iced::window::position(id)
                                .then(|last_position| {
                                    let position = last_position.map_or(
                                        iced::window::Position::Default,
                                        |last_position| {
                                            iced::window::Position::Specific(
                                                last_position + iced::Vector::new(20.0, 20.0),
                                            )
                                        },
                                    );

                                    let (_, open) = iced::window::open(iced::window::Settings {
                                        position,
                                        ..iced::window::Settings::default()
                                    });

                                    open
                                })
                                .map(|id| WindowMessage::Opened(id)),
                            _ => iced::Task::none(),
                        };

                        if let Some(window) = self.windows.get_mut(&id) {
                            window.panel.update(panel_message);
                        }

                        task
                    }
                }
            }
        }
    }

    pub fn view(&self, window_id: iced::window::Id) -> iced::Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            let toolbar: iced::Element<'_, ToolbarMessage> = if let Some(main_window_id) =
                self.main_window_id
                && main_window_id == window_id
            {
                self.toolbar
                    .view()
                    .map(|toolbar_message| toolbar_message.into())
            } else {
                iced::widget::Space::new(0, 0).into()
            };

            iced::widget::column![
                toolbar,
                iced::widget::center(
                    window
                        .view(window_id)
                        .map(|window_message| window_message.into()),
                )
                .into()
            ]
        } else {
            iced::widget::space().into()
        }
    }
}
