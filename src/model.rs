// 3rd Party Imports
use iced::{
    Align,
    Application,
    Column,
    Command,
    Container,
    Element,
    executor,
    Length,
    PaneGrid,
    pane_grid,
    Row,
};

// Local Imports
use crate::action::{
    handle_hotkey,
    Message,
};

use crate::state::{
    Config,
    widget::Panel,
};


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
            Message::OpenTabFocused(label) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(panel) = self.panes.get_mut(&pane) {
                        (*panel).open_tab(&label);
                    }
                }
            }
            Message::CloseTab(pane, index) => {
                if let Some(panel) = self.panes.get_mut(&pane) {
                    let new_focus = (*panel).close_tab(index);
                    return Command::perform(async move {
                            (pane, new_focus)
                        },
                        Message::FocusTab,
                    );
                }
            }
            Message::FocusTab((pane, index)) => {
                if let Some(panel) = self.panes.get_mut(&pane) {
                    (*panel).focus_tab(index);
                }
            }
            Message::ThemeChanged(theme) => self.config.theme = theme,
            Message::Split(axis, pane) => {
                let _ = self.panes.split(
                    axis,
                    &pane,
                    Panel::new(),
                );
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.split(
                        axis,
                        &pane,
                        Panel::new(),
                    );
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.panes.active() {
                    if let Some(adjacent) =
                        self.panes.adjacent(&pane, direction)
                    {
                        self.panes.focus(&adjacent);
                    }
                }
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, ratio);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.swap(&pane, &target);
            }
            Message::PaneDragged(_) => {}
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
                    let _ = self.panes.close(&pane);
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let total_panes = self.panes.len();
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
                    .padding(0)
            )
            .push(
                // Panes
                PaneGrid::new(&mut self.panes, |pane, content, focus| {
                    content.view(pane, focus, total_panes, config)
                })
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(0) // Space between panes
                    .on_drag(Message::PaneDragged)
                    .on_resize(Message::Resized)
                    .on_key_press(handle_hotkey)
            );

        Container::new(app_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0) // Space between panes and window edge
            .style(config.theme)
            .into()
    }
}
