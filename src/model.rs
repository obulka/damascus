// 3rd Party Imports
use iced::{
    executor, pane_grid, Align, Application, Column, Command, Container, Element, Length, PaneGrid,
    Row, Subscription,
};

// Local Imports
mod panel;
pub mod tabs;

use crate::action::{
    handle_hotkey, panel::Message as PanelMessage, tabs::node_graph::clear_cache_command, Message,
};
use crate::state::{style::Theme, Config};
use panel::Panel;

pub struct Damascus {
    config: Config,
    panes: pane_grid::State<Panel>,
}

impl Application for Damascus {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Panel::new());

        (
            Damascus {
                config: flags,
                panes: panes,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Damascus")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Panel(message) => match message {
                PanelMessage::TabContent(tab_content_message) => {
                    return Command::batch(
                        self.panes
                            .iter_mut()
                            .map(|(_, panel)| (*panel).update(tab_content_message.clone())),
                    );
                }
                PanelMessage::MoveTab((pane, tab_index, target_pane)) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        let (new_focus, tab, tab_content) = (*panel).state.close_tab(tab_index);
                        if let Some(target_panel) = self.panes.get_mut(&target_pane) {
                            (*target_panel)
                                .state
                                .open_tab_with_content(tab, tab_content);
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
                            if let Some(index) = (*panel).state.index_of_tab_type(tab_type.clone())
                            {
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
                            (*panel).state.open_tab(tab_type);
                        }
                    }
                }
                PanelMessage::CloseTab(pane, index) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        let (new_focus, _, _) = (*panel).state.close_tab(index);
                        return Command::perform(
                            async move { PanelMessage::FocusTab((pane, new_focus)) },
                            Message::Panel,
                        );
                    }
                }
                PanelMessage::FocusTab((pane, index)) => {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        (*panel).state.focus_tab(index);
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
                        (*panel).state.close_all_tabs();
                    }
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.panes.active() {
                    let panel = self.panes.close(&pane);
                    if panel.is_none() {
                        if let Some(panel) = self.panes.get_mut(&pane) {
                            (*panel).state.close_all_tabs();
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

    fn view(&mut self) -> Element<Message> {
        let config = &self.config;

        let app_content = Column::new()
            .push(
                // Toolbar
                Row::new()
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .max_height(config.tab_bar_height)
                    .align_items(Align::End)
                    .spacing(1)
                    .padding(0),
            )
            .push(
                // Panes
                PaneGrid::new(&mut self.panes, |pane, content, focus| {
                    content.view(pane, focus, config)
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(0) // Space between panes
                .on_drag(Message::PaneDragged)
                .on_resize(10, Message::Resized)
                .on_key_press(handle_hotkey),
            );

        Container::new(app_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0) // Space between panes and window edge
            .style(config.theme)
            .into()
    }
}
