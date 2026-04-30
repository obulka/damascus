// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::BTreeMap;

use iced;

use super::window::{Window, WindowMessage};
use crate::{
    app::Context,
    widgets::{
        Widget,
        panel::{Panel, PanelMessage},
        style::Style,
        toolbar::{
            Toolbar, ToolbarMessage,
            file::{FileMenuOptions, FileMessage},
            preferences::PreferenceMessage,
        },
    },
};

#[derive(Clone, Debug)]
pub enum WindowManagerMessage {
    Toolbar(ToolbarMessage),
    Window(WindowMessage),
}

impl From<WindowMessage> for WindowManagerMessage {
    fn from(window_message: WindowMessage) -> Self {
        Self::Window(window_message)
    }
}

impl From<ToolbarMessage> for WindowManagerMessage {
    fn from(toolbar_message: ToolbarMessage) -> Self {
        Self::Toolbar(toolbar_message)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct WindowManagerContext {
    pub windows: Vec<Window>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct WindowManager {
    pub toolbar: Toolbar,
    pub main_window_id: Option<iced::window::Id>,
    pub focused_window_id: Option<iced::window::Id>,
    pub windows: BTreeMap<iced::window::Id, Window>,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            toolbar: Toolbar::default(),
            main_window_id: None,
            focused_window_id: None,
            windows: BTreeMap::new(),
        }
    }
}

impl From<WindowManager> for WindowManagerContext {
    fn from(window_manager: WindowManager) -> Self {
        Self {
            windows: window_manager
                .windows
                .iter()
                .map(|(_id, window)| window)
                .cloned()
                .collect(),
        }
    }
}

impl WindowManagerContext {
    pub fn from_window_manager(window_manager: &WindowManager) -> Self {
        Self {
            windows: window_manager
                .windows
                .iter()
                .map(|(_id, window)| window)
                .cloned()
                .collect(),
        }
    }
}

impl Widget<WindowManagerMessage> for WindowManager {
    fn update(
        &mut self,
        context: &mut Context,
        message: WindowManagerMessage,
    ) -> iced::Task<WindowManagerMessage> {
        match message {
            WindowManagerMessage::Toolbar(toolbar_message) => {
                let tasks: iced::Task<WindowManagerMessage> = match toolbar_message {
                    ToolbarMessage::File(ref file_message) => match file_message {
                        FileMessage::OptionSelected(option) => {
                            match option {
                                FileMenuOptions::Save | FileMenuOptions::SaveAs => {
                                    (*context).window_manager_context =
                                        WindowManagerContext::from_window_manager(self);
                                }
                                _ => {}
                            };
                            iced::Task::none()
                        }
                        _ => iced::Task::none(),
                    },
                    ToolbarMessage::Preferences(ref preference_message) => match preference_message
                    {
                        PreferenceMessage::Style => {
                            for (_id, window) in self.windows.iter_mut() {
                                window.open_style_editor();
                            }
                            iced::Task::none()
                        }
                    },
                    ToolbarMessage::RestoreWindows => {
                        let mut windows_to_restore: std::slice::Iter<'_, Window> =
                            context.window_manager_context.windows.iter();

                        if let Some(main_window_id) = &self.main_window_id
                            && let Some(window) = self.windows.get_mut(main_window_id)
                            && let Some(window_context) = windows_to_restore.next()
                        {
                            *window = window_context.clone();
                        }

                        iced::Task::batch(
                            self.windows
                                .iter()
                                .flat_map(|(id, _window)| {
                                    if let Some(main_window_id) = self.main_window_id
                                        && main_window_id == *id
                                    {
                                        None
                                    } else {
                                        Some(*id)
                                    }
                                })
                                .map(|id| iced::Task::done(WindowMessage::Close(id).into()))
                                .chain(windows_to_restore.map(|window| {
                                    iced::Task::done(
                                        WindowMessage::Open(Some(window.clone())).into(),
                                    )
                                })),
                        )
                    }
                    _ => iced::Task::none(),
                };

                iced::Task::batch(
                    std::iter::once(tasks).chain(std::iter::once(
                        self.toolbar
                            .update(context, toolbar_message)
                            .map(|toolbar_message| toolbar_message.into()),
                    )),
                )
            }
            WindowManagerMessage::Window(window_message) => {
                match window_message {
                    WindowMessage::Event(id, event) => {
                        match event {
                            // iced::window::Event::Opened { position, size } => {
                            // }
                            // iced::window::Event::Closed,
                            iced::window::Event::Moved(point) => {
                                if let Some(window) = self.windows.get_mut(&id) {
                                    window.position = Some(glam::Vec2::new(point.x, point.y));
                                }
                            }
                            iced::window::Event::Resized(size) => {
                                if let Some(window) = self.windows.get_mut(&id) {
                                    window.size = Some(glam::Vec2::new(size.width, size.height));
                                }
                            }
                            // iced::window::Event::Rescaled(scale) => {
                            // }
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
                        iced::Task::none()
                    }
                    WindowMessage::Open(window) => {
                        let mut position = iced::window::Position::Default;
                        let mut size = iced::Size::default();
                        if let Some(ref window) = window {
                            if let Some(window_position) = window.position {
                                position = iced::window::Position::Specific(iced::Point::new(
                                    window_position.x,
                                    window_position.y,
                                ));
                            }
                            if let Some(window_size) = window.size {
                                size = iced::Size::new(window_size.x, window_size.y);
                            }
                        }

                        let (_, open) = iced::window::open(iced::window::Settings {
                            position: position,
                            size: size,
                            ..iced::window::Settings::default()
                        });

                        open.map(move |id| WindowMessage::Opened(id, window.clone()).into())
                    }
                    WindowMessage::Opened(id, window) => {
                        if self.main_window_id.is_none() {
                            self.main_window_id = Some(id);
                        }

                        let focus_input = iced::widget::operation::focus(format!("input-{id}"));

                        self.windows
                            .insert(id, window.map_or(Window::new(), |window| window));

                        iced::Task::batch(std::iter::once(focus_input).chain(std::iter::once(
                            iced::Task::done(WindowMessage::UpdateTitle),
                        )))
                    }
                    WindowMessage::Close(id) => iced::window::close(id),
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
                    WindowMessage::StyleEditor(window_id, style_editor_message) => {
                        let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                            return iced::Task::none();
                        };

                        if let Some(window) = self.windows.get_mut(&id) {
                            window.update(
                                context,
                                WindowMessage::StyleEditor(Some(id), style_editor_message),
                            )
                        } else {
                            iced::Task::none()
                        }
                    }
                    WindowMessage::CloseStyleEditor(_window_id) => {
                        for (_id, window) in self.windows.iter_mut() {
                            window.close_style_editor();
                        }

                        iced::Task::none()
                    }
                    WindowMessage::UpdateTitle => {
                        let title: String = context.window_title();
                        for (_id, window) in &mut self.windows {
                            window.title = title.clone();
                        }

                        iced::Task::none()
                    }
                    WindowMessage::Panel(window_id, panel_message) => {
                        let Some(id) = window_id.or_else(|| self.focused_window_id) else {
                            return iced::Task::none();
                        };

                        let task: iced::Task<WindowMessage> = match panel_message {
                            PanelMessage::Detach(_) => {
                                let mut window: Option<Window> = self.windows.get(&id).cloned();

                                if let Some(ref mut window) = window {
                                    if let Some(ref mut position) = window.position {
                                        *position += glam::Vec2::splat(20.0);
                                    }
                                    window.panel = Panel::default();
                                }

                                iced::Task::done(WindowMessage::Open(window).into())
                            }
                            _ => iced::Task::none(),
                        };

                        iced::Task::batch(
                            std::iter::once(task).chain(
                                std::iter::once(if let Some(window) = self.windows.get_mut(&id) {
                                    Some(window.panel.update(context, panel_message).map(
                                        move |panel_message| {
                                            WindowMessage::from_panel_message_with_id(
                                                id,
                                                panel_message,
                                            )
                                        },
                                    ))
                                } else {
                                    None
                                })
                                .flatten(),
                            ),
                        )
                    }
                }
                .map(|window_message| window_message.into())
            }
        }
    }

    fn view(
        &self,
        window_id: iced::window::Id,
        _global_style: &Style,
    ) -> iced::Element<'_, WindowManagerMessage> {
        if let Some(window) = self.windows.get(&window_id) {
            let toolbar: Option<iced::Element<'_, WindowManagerMessage>> =
                if let Some(main_window_id) = self.main_window_id
                    && main_window_id == window_id
                {
                    Some(
                        self.toolbar
                            .view(window_id, &window.style)
                            .map(|toolbar_message| toolbar_message.into()),
                    )
                } else {
                    None
                };

            let main_contents: iced::Element<'_, WindowManagerMessage> = iced::widget::column![
                toolbar,
                window
                    .view(window_id, &window.style)
                    .map(|window_message| { window_message.into() })
            ]
            .into();

            if let Some(dialog) = &self.toolbar.dialog {
                dialog.inform_user(
                    &window.style,
                    main_contents,
                    ToolbarMessage::HideModal.into(),
                )
            } else {
                main_contents
            }
        } else {
            iced::widget::space().into()
        }
    }
}
