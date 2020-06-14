// 3rd Party Imports
use iced::{
    canvas::{Cursor, Event},
    keyboard, mouse, pane_grid, Command, Rectangle, Subscription,
};

// Local Imports
pub mod panel;
pub mod tabs;

use crate::model::{panel::Panel, tabs::TabType};
use crate::view::style::Theme;
use crate::Damascus;
use panel::PanelMessage;
use tabs::node_graph::{clear_cache_command, NodeGraphMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Panel(PanelMessage),
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

pub fn handle_hotkey(event: pane_grid::KeyPressEvent) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::Direction;

    let direction = match event.key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match event.key_code {
        KeyCode::V => Some(PanelMessage::OpenTabFocused(TabType::Viewer).into()),
        KeyCode::G => Some(PanelMessage::OpenTabFocused(TabType::NodeGraph).into()),
        KeyCode::T => Some(Message::ToggleTheme),
        KeyCode::W => Some(Message::CloseFocused),
        KeyCode::F => Some(NodeGraphMessage::ToggleGrid.into()),
        _ => direction.map(Message::FocusAdjacent),
    }
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

impl Update<Message> for Damascus {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Panel(panel_message) => match panel_message {
                PanelMessage::TabContent(tab_content_message) => {
                    return Command::batch(
                        self.panes
                            .iter_mut()
                            .map(|(_, panel)| (*panel).update(tab_content_message.clone())),
                    );
                }
                PanelMessage::MoveTab((pane, tab_index, target_pane)) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        let (new_focus, tab, tab_content) = (*panel).close_tab(tab_index);
                        if let Some(target_panel) = self.panes.get_mut(&target_pane) {
                            (*target_panel).open_tab_with_content(tab, tab_content);
                        }
                        return Command::perform(
                            async move { PanelMessage::FocusTab((pane, new_focus)) },
                            Message::Panel,
                        );
                    }
                }
                PanelMessage::OpenTabFocused(tab_type) => {
                    if let Some(active_pane) = self.panes.active() {
                        for (pane, panel) in self.panes.iter_mut() {
                            if let Some(index) = (*panel).index_of_tab_type(tab_type.clone()) {
                                let pane = *pane;
                                if pane == active_pane {
                                    return Command::perform(
                                        async move { PanelMessage::FocusTab((pane, index)) },
                                        Message::Panel,
                                    );
                                } else {
                                    return Command::perform(
                                        async move { PanelMessage::MoveTab((pane, index, active_pane)) },
                                        Message::Panel,
                                    );
                                }
                            }
                        }

                        if let Some(panel) = self.panes.get_mut(&active_pane) {
                            (*panel).open_tab(tab_type);
                        }
                    }
                }
                PanelMessage::CloseTab(pane, index) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        let (new_focus, _, _) = (*panel).close_tab(index);
                        return Command::perform(
                            async move { PanelMessage::FocusTab((pane, new_focus)) },
                            Message::Panel,
                        );
                    }
                }
                PanelMessage::FocusTab((pane, index)) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        (*panel).focus_tab(index);
                    }
                }
            },
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
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.split(axis, &pane, Panel::new());
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(adjacent) = self.panes.adjacent(&pane, direction) {
                        self.panes.focus(&adjacent);
                    }
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