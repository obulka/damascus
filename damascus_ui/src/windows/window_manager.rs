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
        panel::PanelMessage,
        style::Style,
        toolbar::{Toolbar, ToolbarMessage},
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

impl Widget<WindowManagerMessage> for WindowManager {
    fn update(
        &mut self,
        context: &mut Context,
        message: WindowManagerMessage,
    ) -> iced::Task<WindowManagerMessage> {
        match message {
            WindowManagerMessage::Toolbar(toolbar_message) => self
                .toolbar
                .update(context, toolbar_message)
                .map(|toolbar_message| toolbar_message.into()),
            WindowManagerMessage::Window(window_message) => {
                match window_message {
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

                        let window = Window::new();
                        let focus_input = iced::widget::operation::focus(format!("input-{id}"));

                        self.windows.insert(id, window);

                        iced::Task::batch(std::iter::once(focus_input).chain(std::iter::once(
                            iced::Task::done(WindowMessage::UpdateTitle),
                        )))
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
                            window.style.scale *= scale;
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

            let is_main_window: bool = toolbar.is_some();

            let main_contents = iced::widget::column![
                toolbar,
                window
                    .view(window_id, &window.style)
                    .map(|window_message| { window_message.into() })
            ]
            .into();

            if is_main_window && let Some(dialog) = &self.toolbar.dialog {
                dialog.modal(
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
