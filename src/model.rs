// 3rd Party Imports
use iced::{
    Application,
    Command,
    Container,
    Element,
    executor,
    Length,
    PaneGrid,
    pane_grid,
};

// Local Imports
use crate::action::{Message, handle_hotkey};
use crate::state::Config;
use crate::state::widget::Panel;
use crate::state::style::Theme;


pub struct Damascus {
    theme: Theme,
    config: Config,
    panes: pane_grid::State<Panel>,
}


impl Application for Damascus {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let config = Config::default();
        let (panes, _) = pane_grid::State::new(Panel::new());

        (
            Damascus {
                theme: Theme::default(),
                config: config,
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
                    (*panel).close_tab(index);
                }
            }
            Message::FocusTab(pane, index) => {
                if let Some(panel) = self.panes.get_mut(&pane) {
                    (*panel).focus_tab(index);
                }
            }
            Message::ThemeChanged(theme) => self.theme = theme,
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
            Message::Dragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.swap(&pane, &target);
            }
            Message::Dragged(_) => {}
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
        let theme = self.theme;
        let config = &self.config;

        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, content, focus| {
                content.view(pane, focus, total_panes, theme, config)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0) // Space between panes
            .on_drag(Message::Dragged)
            .on_resize(Message::Resized)
            .on_key_press(handle_hotkey);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0) // Space between panes and window edge
            .style(self.theme)
            .into()
    }
}
