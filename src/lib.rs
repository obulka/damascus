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
pub mod action;
pub mod state {
    pub mod content;
    pub mod style;
    pub mod pane;
}
pub mod model;
pub mod widgets;

use action::{Message, handle_hotkey};
use state::content::Content;
use state::style::Theme;

pub struct Damascus {
    theme: Theme,
    panes: pane_grid::State<Content>,
    panes_created: usize,
}


impl Application for Damascus {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Content::new(0));

        (
            Damascus {
                theme: Theme::default(),
                panes: panes,
                panes_created: 1,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Damascus")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::Split(axis, pane) => {
                let _ = self.panes.split(
                    axis,
                    &pane,
                    Content::new(self.panes_created),
                );

                self.panes_created += 1;
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.panes.active() {
                    let _ = self.panes.split(
                        axis,
                        &pane,
                        Content::new(self.panes_created),
                    );

                    self.panes_created += 1;
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
                let _ = self.panes.close(&pane);
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

        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, content, focus| {
                content.view(pane, focus, total_panes, theme)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2)
            .on_drag(Message::Dragged)
            .on_resize(Message::Resized)
            .on_key_press(handle_hotkey);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .style(self.theme)
            .into()
    }
}
