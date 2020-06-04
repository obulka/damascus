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


pub struct NodeGraph {}

impl TabContent for NodeGraph {

    fn view(&self, config: &Config) -> Element<Message> {
        let content = Text::new("Node Graph")
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
