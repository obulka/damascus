use iced::{
    Container,
    Element,
    Text,
    Length,
    HorizontalAlignment,
    VerticalAlignment,
};

use crate::action::Message;
use crate::state::Config;
use super::TabContent;


pub struct Viewer {}

impl TabContent for Viewer {

    fn view(&self, config: &Config) -> Element<Message> {
        let content = Text::new("Viewer")
            .width(Length::Shrink)
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Center)
            .size(config.font_size)
            .color(config.theme.text_color());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3)
            .into()
    }
}
