// 3rd Party Imports
use iced::{Column, Container, Element, Length};

// Local Imports
use crate::model::tabs::Viewer;
use crate::update::{tabs::viewer::ViewerMessage, Message};
use crate::view::{Config, View};

impl View for Viewer {
    fn view(&mut self, config: &Config) -> Element<Message> {
        let selected_speed = self.next_speed.unwrap_or(self.speed);
        let controls = self
            .controls
            .view(
                self.is_playing,
                self.grid.are_lines_visible(),
                selected_speed,
                config,
            )
            .map(|message| message.into());

        let content = Column::new()
            .push(
                self.grid
                    .view()
                    .map(|message| ViewerMessage::Grid(message).into()),
            )
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(config.theme)
            .into()
    }
}
