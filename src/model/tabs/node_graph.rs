use iced::{Command, Container, Element, Length, Point};

use super::TabContent;
use crate::action::{
    tabs::{
        node_graph::{clear_node_caches_command, Message},
        Message as TabContentMessage,
    },
    Message as DamascusMessage,
};
use crate::state::{node::NodeType, tabs::node_graph::State, Config};

#[derive(Default)]
pub struct NodeGraph {
    state: State,
}

impl NodeGraph {
    pub fn new() -> Self {
        let mut state = State::default();
        state.add_node(NodeType::Viewer, Point::new(1.0, 5.0));
        Self { state: state }
    }
}

impl TabContent for NodeGraph {
    fn update(&mut self, message: TabContentMessage) -> Command<DamascusMessage> {
        if let TabContentMessage::NodeGraph(message) = message {
            match message {
                Message::ToggleGrid => self.state.toggle_lines(),
                Message::Next => {}
                Message::AddNode(node_type, position) => {
                    self.state.add_node(node_type, position);
                }
                Message::ClearCache => {
                    self.state.clear_cache();
                }
                Message::ClearNodeCaches => {
                    self.state.clear_node_caches();
                }
                Message::ClearSelected => {
                    self.state.clear_selected();
                    return clear_node_caches_command();
                }
                Message::DeselectNode(label) => {
                    self.state.deselect_node(label);
                    return clear_node_caches_command();
                }
                Message::SelectNode(label) => {
                    self.state.select_node(label);
                    return clear_node_caches_command();
                }
            }
        }
        Command::none()
    }

    fn view(&mut self, config: &Config) -> Element<DamascusMessage> {
        let content = self.state.view(config).map(|message| message.into());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(1)
            .into()
    }
}
