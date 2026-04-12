// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::time::{Duration, SystemTime};

use iced;
use serde_hashkey::{Key, OrderedFloatPolicy, to_key_with_ordered_float};

use damascus;

use crate::{
    widgets::{Widget, node_graph::NodeGraphMessage, panel::PanelMessage, style},
    windows::{
        window::{Window, WindowMessage},
        window_manager::{WindowManager, WindowManagerMessage},
    },
};

//
//
//
//
//
//
//
// tmp Viewer & Node graph
//
//
//
//
//
//

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ViewerMessage {
    None,
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

#[derive(Clone, Debug)]
pub enum Message {
    WindowManager(WindowManagerMessage),
    NodeGraph(NodeGraphMessage),
    Viewer(ViewerMessage),
}

impl From<WindowMessage> for Message {
    fn from(window_message: WindowMessage) -> Self {
        Self::WindowManager(WindowManagerMessage::Window(window_message))
    }
}

impl From<WindowManagerMessage> for Message {
    fn from(window_manager_message: WindowManagerMessage) -> Self {
        Self::WindowManager(window_manager_message)
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Context {
    pub default_preferences: style::Preferences,
    working_file: Option<String>,
    working_file_hash: Option<Key<OrderedFloatPolicy>>,
    pub node_graph: damascus::graph::node_graph::NodeGraph,
}

impl Context {
    pub fn window_title(&self) -> String {
        if let Some(working_file) = &self.working_file {
            format!(
                "{:} - {:}{:}",
                Window::default_title(),
                working_file,
                if self.dirty() { "*" } else { "" }
            )
        } else {
            Window::default_title().to_string()
        }
    }

    pub fn working_file(&self) -> &Option<String> {
        &self.working_file
    }

    pub fn set_working_file(&mut self, working_file: String) {
        self.working_file = Some(working_file);
    }

    pub fn update_hash(&mut self) {
        self.working_file_hash = to_key_with_ordered_float(&self.node_graph).ok();
    }

    pub fn update(&mut self, working_file: String) {
        self.set_working_file(working_file);
        self.update_hash();
    }

    pub fn dirty(&self) -> bool {
        if let Some(working_file_hash) = &self.working_file_hash
            && let Ok(current_hash) = to_key_with_ordered_float(&self.node_graph)
        {
            return current_hash != *working_file_hash;
        }
        true
    }
}

pub struct Damascus {
    last_lazy_update: SystemTime,
    context: Context,
    window_manager: WindowManager,
    // viewport: Viewport,
}

impl Damascus {
    const LAZY_UPDATE_DELAY: f32 = 1.0;

    pub fn new() -> (Self, iced::Task<Message>) {
        // TODO read from disk
        let persistent_data = Context::default();

        let (_, open) = iced::window::open(iced::window::Settings::default());

        (
            Self {
                last_lazy_update: SystemTime::now()
                    - Duration::from_millis((Self::LAZY_UPDATE_DELAY * 1000.0) as u64),
                context: persistent_data,
                window_manager: WindowManager::new(),
                // viewports: Viewport::new(
                //     persistent_data.viewport_state,
                //     creation_context.wgpu_render_state.as_ref().unwrap(),
                // ),
            },
            open.map(|id| WindowMessage::Opened(id).into()),
        )
    }

    fn lazy_update(&self) -> iced::Task<Message> {
        iced::Task::done(WindowMessage::UpdateTitle.into())
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        // TODO might be able to use iced's native lazy functionality
        // this was only originally done to update title because egui
        // was checking dirty hash every frame
        let lazy_task: iced::Task<Message> = if let Ok(duration_since_lazy_update) =
            SystemTime::now().duration_since(self.last_lazy_update)
            && duration_since_lazy_update.as_secs_f32() >= Self::LAZY_UPDATE_DELAY
        {
            let task: iced::Task<Message> = self.lazy_update();
            self.last_lazy_update = SystemTime::now();
            task
        } else {
            iced::Task::none()
        };

        iced::Task::batch(
            std::iter::once(lazy_task).chain(std::iter::once(match message {
                Message::WindowManager(window_manager_message) => self
                    .window_manager
                    .update(&mut self.context, window_manager_message)
                    .map(|window_message| window_message.into()),
                _ => iced::Task::none(),
            })),
        )
    }

    pub fn view(&self, window_id: iced::window::Id) -> iced::Element<'_, Message> {
        self.window_manager
            .view(window_id, &self.context.default_preferences)
            .map(|window_manager_message| window_manager_message.into())
    }

    // fn display_error() {
    //     todo!("Display Error to User")
    // }

    // fn auto_save_interval(&self) -> Duration {
    //     Duration::from_secs(15)
    // }

    pub fn default_font() -> iced::Font {
        iced::Font::DEFAULT
    }

    pub fn title(&self, window: iced::window::Id) -> String {
        self.window_manager
            .windows
            .get(&window)
            .map(|window| window.title.clone())
            .unwrap_or_default()
    }

    pub fn theme(&self, window: iced::window::Id) -> Option<iced::Theme> {
        Some(
            self.window_manager
                .windows
                .get(&window)?
                .preferences
                .theme
                .into(),
        )
    }

    pub fn scale_factor(&self, window: iced::window::Id) -> f32 {
        self.window_manager
            .windows
            .get(&window)
            .map(|window| window.preferences.scale)
            .unwrap_or(1.0)
    }

    fn handle_ctrl_alt_shift_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_shift_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_alt_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_alt_shift_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_ctrl_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            iced::keyboard::key::Key::Character("w") => {
                Some(WindowMessage::Panel(None, PanelMessage::CloseFocused).into())
            }
            iced::keyboard::key::Key::Character("=") => {
                Some(WindowMessage::ScaleChanged(None, 1.05).into())
            }
            iced::keyboard::key::Key::Character("-") => {
                Some(WindowMessage::ScaleChanged(None, 0.95).into())
            }
            iced::keyboard::key::Key::Named(key) => {
                if let Some(direction) = match key {
                    iced::keyboard::key::Named::ArrowUp => {
                        Some(iced::widget::pane_grid::Direction::Up)
                    }
                    iced::keyboard::key::Named::ArrowDown => {
                        Some(iced::widget::pane_grid::Direction::Down)
                    }
                    iced::keyboard::key::Named::ArrowLeft => {
                        Some(iced::widget::pane_grid::Direction::Left)
                    }
                    iced::keyboard::key::Named::ArrowRight => {
                        Some(iced::widget::pane_grid::Direction::Right)
                    }
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

    fn handle_shift_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_alt_hotkey(key: iced::keyboard::Key) -> Option<Message> {
        match key.as_ref() {
            _ => None,
        }
    }

    fn handle_hotkeys(
        modifiers: iced::keyboard::Modifiers,
        key: iced::keyboard::Key,
    ) -> Option<Message> {
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

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(
            std::iter::once(iced::keyboard::listen().filter_map(|event| {
                let iced::keyboard::Event::KeyPressed { key, modifiers, .. } = event else {
                    return None;
                };

                Self::handle_hotkeys(modifiers, key)
            }))
            .chain(std::iter::once(
                iced::window::close_events()
                    .map(|window_id| WindowMessage::Closed(window_id).into()),
            ))
            .chain(std::iter::once(
                iced::window::events().map(|(id, event)| WindowMessage::Event(id, event).into()),
            )),
        )
    }
}
