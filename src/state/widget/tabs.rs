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

#[derive(Debug, Clone)]
pub enum TabType {
    NodeGraph,
    Viewer,
}

impl From<TabType> for String {
    fn from(tab_type: TabType) -> Self {
        match tab_type {
            TabType::NodeGraph => "NodeGraph".to_string(),
            TabType::Viewer => "Viewer".to_string(),
        }
    }
}

pub trait TabContent {
    fn view(&self, config: &Config) -> Element<Message>;
}

pub struct Viewer {}

impl TabContent for Viewer {

    fn view(&self, config: &Config) -> Element<Message> {
        Container::new(
            Text::new("Test")
                .width(Length::Shrink)
                .horizontal_alignment(HorizontalAlignment::Left)
                .vertical_alignment(VerticalAlignment::Center)
                .size(config.font_size)
                .color(config.theme.text_color())
        ).into()
    }
}
