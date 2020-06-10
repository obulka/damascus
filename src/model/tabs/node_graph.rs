use iced::{Command, Container, Element, Length};

use super::TabContent;
use crate::action::{
    tabs::{node_graph::Message, Message as TabContentMessage},
    Message as DamascusMessage,
    panel::Message as PanelMessage,
};
use crate::state::{tabs::node_graph::State, Config};

#[derive(Default)]
pub struct NodeGraph {
    state: State,
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
                Message::ToggleGrid => self.state.toggle_lines(),
                Message::Clear => {
                    self.state.clear();
                }
                Message::Next => {}
            }
        }
        Command::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage> {
        let content = self
            .state
            .view(config)
            .map(|message| DamascusMessage::Panel(
                PanelMessage::TabContent(TabContentMessage::NodeGraph(message))
            ));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}
