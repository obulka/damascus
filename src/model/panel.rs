// 3rd Party Imports
use iced::{pane_grid, Command, Container, Element, Length, Subscription};

// Local Imports
use crate::model::Config;
use crate::update::{tabs::Message as TabContentMessage, Message as DamascusMessage};
use crate::view::panel::State;

pub struct Panel {
    pub state: State,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        focus: Option<pane_grid::Focus>,
        config: &Config,
    ) -> Element<DamascusMessage> {
        let content = self.state.view(pane, config);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .center_y()
            .style(config.theme.pane_style(focus.is_some()))
            .into()
    }

    pub fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let Some(focused_label) = self.state.get_focused_label() {
            if message == *focused_label {
                if let Some(focused_content) = self.state.get_mut_focused_content() {
                    return focused_content.update(message);
                }
            }
        }
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<DamascusMessage> {
        if let Some(focused_content) = self.state.get_focused_content() {
            return focused_content.subscription();
        }
        Subscription::none()
    }
}
