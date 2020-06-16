// 3rd Party Imports
use iced::{
    canvas::{Cursor, Event},
    keyboard::KeyCode,
    mouse,
    pane_grid::{self, Direction},
    Command, Rectangle, Subscription,
};

// Local Imports
pub mod panel;
pub mod tabs;
mod widget;

pub use tabs::node_graph::node;
pub use widget::*;

use crate::model::{panel::Panel, tabs::TabType};
use crate::update::tabs::TabContentMessage;
use crate::view::Theme;
use crate::Damascus;
use tabs::node_graph::{clear_cache_command, NodeGraphMessage};

#[derive(Debug, Clone)]
pub enum Message {
    TabContent(TabContentMessage),
    MoveTab((String, pane_grid::Pane)),
    OpenTabFocused(TabType),
    CloseTab(String),
    FocusTab(String),
    MoveTabAdjacent(pane_grid::Direction),
    ThemeChanged(Theme),
    ToggleTheme,
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    PaneDragged(pane_grid::DragEvent),
    FloatPane(pane_grid::Pane),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

fn get_direction(event: pane_grid::KeyPressEvent) -> Option<Direction> {
    match event.key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    }
}

fn handle_ctrl(event: pane_grid::KeyPressEvent) -> Option<Message> {
    match event.key_code {
        KeyCode::V => Some(Message::OpenTabFocused(TabType::Viewer)),
        KeyCode::G => Some(Message::OpenTabFocused(TabType::NodeGraph)),
        KeyCode::T => Some(Message::ToggleTheme),
        KeyCode::W => Some(Message::CloseFocused),
        KeyCode::F => {
            Some(TabContentMessage::NodeGraph((None, NodeGraphMessage::ToggleGrid)).into())
        }
        _ => get_direction(event).map(Message::MoveTabAdjacent),
    }
}

fn handle_ctrl_shift(event: pane_grid::KeyPressEvent) -> Option<Message> {
    match event.key_code {
        _ => get_direction(event).map(Message::FocusAdjacent),
    }
}

fn handle_ctrl_alt(_event: pane_grid::KeyPressEvent) -> Option<Message> {
    None
}

fn handle_ctrl_alt_shift(_event: pane_grid::KeyPressEvent) -> Option<Message> {
    None
}

pub fn handle_hotkey(event: pane_grid::KeyPressEvent) -> Option<Message> {
    if event.modifiers.shift {
        if event.modifiers.alt {
            // Ctrl + Alt + Shift
            return handle_ctrl_alt_shift(event);
        }
        // Ctrl + Shift
        return handle_ctrl_shift(event);
    }
    if event.modifiers.alt {
        // Ctrl + Alt
        return handle_ctrl_alt(event);
    }
    // Ctrl
    handle_ctrl(event)
}

pub trait Update<UpdateMessage> {
    fn update(&mut self, _message: UpdateMessage) -> Command<Message> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

pub trait CanvasUpdate<EmittedMessage> {
    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor)
        -> Option<EmittedMessage>;

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction;
}

pub trait CanvasItemUpdate {}

impl Update<Message> for Damascus {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabContent(tab_content_message) => {
                return Command::batch(
                    self.panes
                        .iter_mut()
                        .map(|(_, panel)| (*panel).update(tab_content_message.clone())),
                );
            }
            Message::MoveTab((tab_label, target_pane)) => {
                if let Some(new_focus) = self.move_tab(&tab_label, target_pane) {
                    return Command::perform(async move { new_focus }, Message::FocusTab);
                }
            }
            Message::OpenTabFocused(tab_type) => {
                self.open_tab_focused(tab_type);
            }
            Message::CloseTab(tab_label) => {
                if let Some(new_focus) = self.close_tab(&tab_label) {
                    return Command::perform(async move { new_focus }, Message::FocusTab);
                }
            }
            Message::FocusTab(tab_label) => {
                if let Some(pane) = self.tabs.get(&tab_label) {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        (*panel).focus_tab(tab_label);
                    }
                }
            }
            Message::MoveTabAdjacent(direction) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        if let Some(focused_tab) = panel.get_focused_label() {
                            if let Some(adjacent) = self.panes.adjacent(&pane, direction) {
                                self.panes.focus(&adjacent);
                                return Command::perform(
                                    async move { (focused_tab, adjacent) },
                                    Message::MoveTab,
                                );
                            }
                        }
                    }
                }
            }
            Message::ThemeChanged(theme) => {
                self.config.theme = theme;
                return clear_cache_command();
            }
            Message::ToggleTheme => {
                self.config.theme = match self.config.theme {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                };
                return clear_cache_command();
            }
            Message::Split(axis, pane) => {
                let _ = self.panes.split(axis, &pane, Panel::new());
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(adjacent) = self.panes.adjacent(&pane, direction) {
                        self.panes.focus(&adjacent);
                    }
                }
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.split(axis, &pane, Panel::new());
                }
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, ratio);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.swap(&pane, &target);
            }
            Message::PaneDragged(_) => {}
            Message::FloatPane(_) => {
                println!("Floating panes not implemented.");
            }
            Message::Close(pane) => {
                let panel = self.panes.close(&pane);
                if panel.is_none() {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        (*panel).close_all_tabs();
                    }
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.panes.active() {
                    let panel = self.panes.close(&pane);
                    if panel.is_none() {
                        if let Some(panel) = self.panes.get_mut(&pane) {
                            (*panel).close_all_tabs();
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(self.panes.iter().map(|(_, panel)| (*panel).subscription()))
    }
}
