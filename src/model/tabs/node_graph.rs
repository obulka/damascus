use iced::{Command, Container, Element, Length};

mod grid;

use grid::Grid;

use super::TabContent;
use crate::action::{
    tabs::{node_graph::Message, Message as TabContentMessage},
    Message as DamascusMessage,
};
use crate::state::Config;

#[derive(Default)]
pub struct NodeGraph {
    grid: Grid,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl TabContent for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let TabContentMessage::NodeGraph(message) = message {
            match message {
                Message::Grid(message) => {
                    self.grid.update(message);
                }
                Message::Clear => {
                    self.grid.clear();
                }
                Message::Next => {}
            }
        }
        Command::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage> {
        let content = self.grid.view(config).map(|message| {
            DamascusMessage::TabContent(TabContentMessage::NodeGraph(Message::Grid(message)))
        });

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}
