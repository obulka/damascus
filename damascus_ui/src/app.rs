// Copyright (c) 2024, Owen Bulka
// All rights reserved.
// This source code is licensed under the BSD-style license found in the
// LICENSE file in the root directory of this source tree.

use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use iced;
use serde_hashkey::{Key, OrderedFloatPolicy, to_key_with_ordered_float};

use crate::{
    widgets::{
        Widget,
        node_graph::{NodeGraphContext, NodeGraphMessage},
        panel::{
            PanelMessage,
            pane::PaneMessage,
            tabs::{Tab, TabMessage},
        },
        properties::PropertiesMessage,
        style::{
            Style,
            editor::{StyleEditorFields, StyleEditorMessage},
        },
        toolbar::{
            ToolbarMessage,
            file::{FileMenuOptions, FileMessage},
        },
        viewport::{Viewport, ViewportMessage, Viewports},
    },
    windows::{
        window::{Window, WindowMessage},
        window_manager::{WindowManager, WindowManagerContext, WindowManagerMessage},
    },
};

#[derive(Clone, Debug)]
pub enum Message {
    WindowManager(WindowManagerMessage),
    NodeGraph(NodeGraphMessage),
    Viewport(ViewportMessage),
    IcedEvent(iced::Event),
}

impl From<WindowMessage> for Message {
    fn from(window_message: WindowMessage) -> Self {
        Self::WindowManager(WindowManagerMessage::Window(window_message))
    }
}

impl From<StyleEditorMessage> for Message {
    fn from(style_editor_message: StyleEditorMessage) -> Self {
        <StyleEditorMessage as Into<WindowMessage>>::into(style_editor_message).into()
    }
}

impl From<WindowManagerMessage> for Message {
    fn from(window_manager_message: WindowManagerMessage) -> Self {
        Self::WindowManager(window_manager_message)
    }
}

impl From<FileMenuOptions> for Message {
    fn from(file_option: FileMenuOptions) -> Self {
        Self::WindowManager(<FileMenuOptions as Into<ToolbarMessage>>::into(file_option).into())
    }
}

impl From<FileMessage> for Message {
    fn from(file_message: FileMessage) -> Self {
        Self::WindowManager(<FileMessage as Into<ToolbarMessage>>::into(file_message).into())
    }
}

impl From<Message> for iced::Task<Message> {
    fn from(message: Message) -> Self {
        iced::Task::done(message)
    }
}

impl From<WindowMessage> for iced::Task<Message> {
    fn from(window_message: WindowMessage) -> Self {
        <WindowMessage as Into<Message>>::into(window_message).into()
    }
}

impl From<StyleEditorMessage> for iced::Task<Message> {
    fn from(style_editor_message: StyleEditorMessage) -> Self {
        <StyleEditorMessage as Into<Message>>::into(style_editor_message).into()
    }
}

impl From<WindowManagerMessage> for iced::Task<Message> {
    fn from(window_manager_message: WindowManagerMessage) -> Self {
        <WindowManagerMessage as Into<Message>>::into(window_manager_message).into()
    }
}

impl From<FileMenuOptions> for iced::Task<Message> {
    fn from(file_option: FileMenuOptions) -> Self {
        <FileMenuOptions as Into<Message>>::into(file_option).into()
    }
}

impl From<FileMessage> for iced::Task<Message> {
    fn from(file_message: FileMessage) -> Self {
        <FileMessage as Into<Message>>::into(file_message).into()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Context {
    pub default_style: Style,
    working_file: Option<String>,
    working_file_hash: Option<Key<OrderedFloatPolicy>>,
    pub window_manager_context: WindowManagerContext,
    pub node_graph: Arc<Mutex<NodeGraphContext>>,
    pub viewports: Arc<Mutex<Viewports>>,
    #[serde(skip)]
    pub keyboard_modifiers: iced::keyboard::Modifiers,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            default_style: Style::default(),
            working_file: None,
            working_file_hash: None,
            window_manager_context: WindowManagerContext::default(),
            node_graph: Arc::new(Mutex::new(NodeGraphContext::default())),
            viewports: Arc::new(Mutex::new(Viewports::default())),
            keyboard_modifiers: iced::keyboard::Modifiers::NONE,
        }
    }
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
        self.working_file_hash = to_key_with_ordered_float(&self.window_manager_context).ok();
    }

    pub fn update(&mut self, working_file: String) {
        self.set_working_file(working_file);
        self.update_hash();
    }

    pub fn dirty(&self) -> bool {
        if let Some(working_file_hash) = &self.working_file_hash
            && let Ok(current_hash) = to_key_with_ordered_float(&self.window_manager_context)
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
}

impl Damascus {
    const LAZY_UPDATE_DELAY: f32 = 1.0;

    pub fn new() -> (Self, iced::Task<Message>) {
        let args: Vec<String> = std::env::args().collect();

        let context = Context::default();

        let (_, open) = iced::window::open(iced::window::Settings::default());

        (
            Self {
                last_lazy_update: SystemTime::now()
                    - Duration::from_millis((Self::LAZY_UPDATE_DELAY * 1000.0) as u64),
                context: context,
                window_manager: WindowManager::new(),
            },
            open.map(|id| WindowMessage::Opened(id, None).into())
                .chain(if args.len() > 1 {
                    iced::Task::done(FileMessage::Restore(args[args.len() - 1].clone()).into())
                } else {
                    iced::Task::none()
                }),
        )
    }

    fn lazy_update(&mut self) -> iced::Task<Message> {
        iced::Task::batch(std::iter::once(iced::Task::done(
            WindowMessage::UpdateTitle.into(),
        )))
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
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
            std::iter::once(match message {
                Message::IcedEvent(event) => self.handle_iced_event(event),
                Message::WindowManager(WindowManagerMessage::Window(
                    WindowMessage::StyleEditor(mut window_id, StyleEditorMessage::StyleChanged),
                )) => {
                    // Update node graph styles
                    if window_id.is_none() {
                        window_id = self.window_manager.focused_window_id;
                    }
                    if let Some(window_id) = window_id
                        && let Some(window) = self.window_manager.windows.get_mut(&window_id)
                    {
                        let mut tasks = Vec::<(
                            iced::window::Id,
                            iced::widget::pane_grid::Pane,
                            iced::Task<NodeGraphMessage>,
                        )>::new();
                        for (pane_id, pane) in window.panel.panes.panes.iter_mut() {
                            for tab in pane.tabs.iter_mut() {
                                let Tab::NodeGraph(node_graph) = tab else {
                                    continue;
                                };
                                self.context.default_style = window.style.clone();
                                tasks.push((
                                    window_id,
                                    *pane_id,
                                    node_graph
                                        .update(&mut self.context, NodeGraphMessage::UpdateStyle),
                                ))
                            }
                        }

                        iced::Task::batch(tasks.into_iter().map(|(window_id, pane_id, task)| {
                            task.map(move |message| {
                                Message::WindowManager(WindowManagerMessage::Window(
                                    WindowMessage::Panel(
                                        Some(window_id),
                                        PanelMessage::Pane(
                                            pane_id,
                                            PaneMessage::Tab(message.into()),
                                        ),
                                    ),
                                ))
                            })
                        }))
                    } else {
                        iced::Task::none()
                    }
                }
                Message::WindowManager(WindowManagerMessage::Window(WindowMessage::Panel(
                    _window_id,
                    PanelMessage::Pane(
                        _pane,
                        PaneMessage::Tab(TabMessage::NodeGraph(NodeGraphMessage::OpenProperties(
                            node_id,
                        ))),
                    ),
                ))) => {
                    let mut task: Option<(
                        iced::window::Id,
                        iced::widget::pane_grid::Pane,
                        iced::Task<PropertiesMessage>,
                    )> = None;
                    for (window_id, window) in self.window_manager.windows.iter_mut() {
                        for (pane_id, pane) in window.panel.panes.panes.iter_mut() {
                            for tab in pane.tabs.iter_mut() {
                                let Tab::Properties(properties) = tab else {
                                    continue;
                                };

                                task = Some((
                                    *window_id,
                                    *pane_id,
                                    properties.update(
                                        &mut self.context,
                                        PropertiesMessage::OpenNodeProperties(node_id),
                                    ),
                                ));

                                break;
                            }
                            if task.is_some() {
                                break;
                            }
                        }
                        if task.is_some() {
                            break;
                        }
                    }
                    if let Some((window_id, pane_id, task)) = task {
                        task.map(move |message| {
                            Message::WindowManager(WindowManagerMessage::Window(
                                WindowMessage::Panel(
                                    Some(window_id),
                                    PanelMessage::Pane(pane_id, PaneMessage::Tab(message.into())),
                                ),
                            ))
                        })
                    } else {
                        iced::Task::none()
                    }
                }
                Message::WindowManager(WindowManagerMessage::Window(WindowMessage::Panel(
                    _window_id,
                    PanelMessage::Pane(
                        _pane,
                        PaneMessage::Tab(TabMessage::NodeGraph(NodeGraphMessage::NewViewport(
                            new_viewport_id,
                        ))),
                    ),
                ))) => {
                    // Create a new tab for a newly created viewport
                    for window in self.window_manager.windows.values_mut() {
                        for pane in window.panel.panes.panes.values_mut() {
                            pane.tabs
                                .push(Tab::Viewport(Viewport::default().with_id(new_viewport_id)));

                            return iced::Task::none();
                        }
                    }
                    iced::Task::none()
                }
                Message::WindowManager(WindowManagerMessage::Window(WindowMessage::Panel(
                    _window_id,
                    PanelMessage::Pane(
                        _pane,
                        PaneMessage::Tab(TabMessage::NodeGraph(
                            NodeGraphMessage::RemovedViewports(viewport_ids),
                        )),
                    ),
                ))) => {
                    // Close the tab widget of a recently removed viewport
                    let mut tasks = Vec::<Message>::new();
                    for (window_id, window) in self.window_manager.windows.iter_mut() {
                        for (pane_id, pane) in window.panel.panes.panes.iter_mut() {
                            for (tab_index, tab) in pane.tabs.iter_mut().enumerate() {
                                if let Tab::Viewport(viewport) = tab
                                    && let Some(viewport_id) = viewport.id()
                                    && viewport_ids.contains(viewport_id)
                                {
                                    tasks.push(Message::WindowManager(
                                        WindowManagerMessage::Window(WindowMessage::Panel(
                                            Some(*window_id),
                                            PanelMessage::Pane(
                                                *pane_id,
                                                PaneMessage::Tab(TabMessage::Closed(tab_index)),
                                            ),
                                        )),
                                    ))
                                }
                            }
                        }
                    }

                    iced::Task::batch(tasks.into_iter().map(|message| iced::Task::done(message)))
                }
                Message::WindowManager(WindowManagerMessage::Window(WindowMessage::Panel(
                    _window_id,
                    PanelMessage::Pane(
                        _pane,
                        PaneMessage::Tab(TabMessage::NodeGraph(NodeGraphMessage::ViewNodeOutput(
                            viewport_id,
                            view_index,
                            node_output_id,
                        ))),
                    ),
                ))) => {
                    // Update the active output of a viewport
                    if let Ok(mut viewports) = self.context.viewports.lock() {
                        if let Some(id) = viewport_id
                            && let Some(viewport_context) = viewports.get_mut(id)
                        {
                            if let Some(output_id) = node_output_id {
                                viewport_context.views.insert(view_index, output_id);
                            }
                            viewport_context.active_view = Some(view_index);
                        }
                    }
                    iced::Task::none()
                }
                Message::WindowManager(window_manager_message) => self
                    .window_manager
                    .update(&mut self.context, window_manager_message)
                    .map(|window_message| window_message.into()),
                _ => iced::Task::none(),
            })
            .chain(std::iter::once(lazy_task)),
        )
    }

    pub fn view(&self, window_id: iced::window::Id) -> iced::Element<'_, Message> {
        self.window_manager
            .view(window_id, &self.context.default_style)
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
        Some(self.window_manager.windows.get(&window)?.style.theme.into())
    }

    pub fn scale_factor(&self, window: iced::window::Id) -> f32 {
        self.window_manager
            .windows
            .get(&window)
            .map(|window| window.style.scale)
            .unwrap_or(1.0)
    }

    fn handle_ctrl_alt_shift_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            _ => iced::Task::none(),
        }
    }

    fn handle_ctrl_shift_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            iced::keyboard::key::Key::Character("s") => FileMenuOptions::SaveAs.into(),
            _ => iced::Task::none(),
        }
    }

    fn handle_ctrl_alt_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            _ => iced::Task::none(),
        }
    }

    fn handle_alt_shift_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            _ => iced::Task::none(),
        }
    }

    fn handle_ctrl_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            iced::keyboard::key::Key::Character("s") => FileMenuOptions::Save.into(),
            iced::keyboard::key::Key::Character("l") => FileMenuOptions::Load.into(),
            iced::keyboard::key::Key::Character("w") => {
                WindowMessage::Panel(None, PanelMessage::CloseFocused).into()
            }
            iced::keyboard::key::Key::Character("=") => WindowMessage::StyleEditor(
                None,
                StyleEditorMessage::Increment(StyleEditorFields::Scale),
            )
            .into(),
            iced::keyboard::key::Key::Character("-") => WindowMessage::StyleEditor(
                None,
                StyleEditorMessage::Decrement(StyleEditorFields::Scale),
            )
            .into(),
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
                    WindowMessage::Panel(None, PanelMessage::FocusAdjacent(direction)).into()
                } else {
                    iced::Task::none()
                }
            }
            _ => iced::Task::none(),
        }
    }

    fn handle_shift_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            _ => iced::Task::none(),
        }
    }

    fn handle_alt_hotkey(key: iced::keyboard::Key) -> iced::Task<Message> {
        match key.as_ref() {
            _ => iced::Task::none(),
        }
    }

    fn handle_hotkeys(
        modifiers: iced::keyboard::Modifiers,
        key: iced::keyboard::Key,
    ) -> iced::Task<Message> {
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
            iced::Task::none()
        }
    }

    pub fn handle_iced_event(&mut self, event: iced::Event) -> iced::Task<Message> {
        match event {
            iced::Event::Mouse(_) => iced::Task::none(),
            iced::Event::Keyboard(keyboard_event) => match keyboard_event {
                iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                    Self::handle_hotkeys(modifiers, key)
                }
                iced::keyboard::Event::ModifiersChanged(modifiers) => {
                    self.context.keyboard_modifiers = modifiers;
                    iced::Task::none()
                }
                _ => iced::Task::none(),
            },
            _ => iced::Task::none(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(
            std::iter::once(iced::event::listen().filter_map(|event| match event {
                iced::Event::Mouse(_) => Some(Message::IcedEvent(event.clone())),
                iced::Event::Keyboard(_) => Some(Message::IcedEvent(event.clone())),
                _ => None,
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
