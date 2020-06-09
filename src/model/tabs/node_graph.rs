use iced::{
    time,
    Canvas,
    Command,
    Container,
    Subscription,
    Element,
    Length,
};


use crate::action::{
    Message as DamascusMessage,
    tabs::{
        Message as TabContentMessage,
        node_graph::Message,
    },
};
use crate::state::{
    Config,
    tabs::node_graph::State,
};
use super::TabContent;


pub struct NodeGraph {
    state: State,
}

impl NodeGraph {
    pub fn new() -> Self {
        NodeGraph {
            state: State::new(),
        }
    }
}

impl TabContent for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let TabContentMessage::NodeGraph(message) = message {
            match message {
                Message::Tick(instant) => {
                    self.state.update(instant);
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<DamascusMessage> {
        time::every(std::time::Duration::from_millis(10))
            .map(|instant| DamascusMessage::TabContent(
                TabContentMessage::NodeGraph(
                    Message::Tick(instant)
                )
            ))
    }

    fn view(&mut self, _config: &Config) -> Element<DamascusMessage> {
        let content = Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill);
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3)
            .into()
    }
}
